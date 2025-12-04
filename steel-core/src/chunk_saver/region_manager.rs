//! Region file manager with open handle tracking.
//!
//! Keeps region files open in memory while any chunk from that region is loaded,
//! avoiding repeated disk I/O for nearby chunk operations.

use std::{
    fs::{self, File},
    io::{self, BufReader, BufWriter, Read, Write},
    path::PathBuf,
};

use parking_lot::RwLock;
use rustc_hash::FxHashMap;
use steel_utils::{BlockStateId, ChunkPos, Identifier};

use crate::chunk::{
    chunk_access::ChunkStatus,
    level_chunk::LevelChunk,
    paletted_container::PalettedContainer,
    section::{ChunkSection, Sections},
};

use super::{
    bit_pack::{bits_for_palette_len, pack_indices, unpack_indices},
    format::{
        BIOMES_PER_SECTION, BLOCKS_PER_SECTION, FORMAT_VERSION, PersistentBiomeData,
        PersistentBlockState, PersistentChunk, PersistentSection, REGION_MAGIC, RegionFile,
        RegionPos,
    },
};

/// Manages region files with open handle tracking.
pub struct RegionManager {
    /// Base directory for region files (e.g., "world/region").
    base_path: PathBuf,
    /// Currently loaded regions, keyed by region position.
    regions: RwLock<FxHashMap<RegionPos, LoadedRegion>>,
}

/// A loaded region with reference counting.
struct LoadedRegion {
    /// The region data.
    data: RegionFile,
    /// Number of chunks currently loaded from this region.
    loaded_chunk_count: usize,
    /// Whether the region has been modified since last save.
    dirty: bool,
}

/// TODO: Needs
/// - Saving protochunks
/// - Fix block, properties and biomes to string.
/// - Investigate reducing ref count for non-dirty chunks.
impl RegionManager {
    /// Creates a new region manager.
    ///
    /// # Arguments
    /// * `base_path` - Directory where region files are stored.
    pub fn new(base_path: impl Into<PathBuf>) -> Self {
        Self {
            base_path: base_path.into(),
            regions: RwLock::new(FxHashMap::default()),
        }
    }

    /// Gets the file path for a region.
    fn region_path(&self, pos: RegionPos) -> PathBuf {
        self.base_path.join(pos.filename())
    }

    /// Loads a region from disk, or creates a new one if it doesn't exist.
    fn load_region(&self, pos: RegionPos) -> io::Result<RegionFile> {
        let path = self.region_path(pos);

        if !path.exists() {
            return Ok(RegionFile::new());
        }

        let file = File::open(&path)?;
        let mut reader = BufReader::new(file);

        // Read and verify magic
        let mut magic = [0u8; 4];
        reader.read_exact(&mut magic)?;
        if magic != REGION_MAGIC {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid region file magic",
            ));
        }

        // Read compressed data
        let mut compressed = Vec::new();
        reader.read_to_end(&mut compressed)?;

        // Decompress with zstd
        let data = zstd::decode_all(&compressed[..])?;

        let region: RegionFile = wincode::deserialize(&data)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;

        // Version check
        if region.version > FORMAT_VERSION {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "Region file version {} is newer than supported version {}",
                    region.version, FORMAT_VERSION
                ),
            ));
        }

        Ok(region)
    }

    /// Saves a region to disk atomically.
    fn save_region(&self, pos: RegionPos, region: &RegionFile) -> io::Result<()> {
        fs::create_dir_all(&self.base_path)?;

        let path = self.region_path(pos);
        let temp_path = path.with_extension("srg.tmp");

        let file = File::create(&temp_path)?;
        let mut writer = BufWriter::new(file);

        // Write magic
        writer.write_all(&REGION_MAGIC)?;

        // Serialize with wincode
        let data = wincode::serialize(region)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;

        // Compress with zstd (level 3 is a good balance of speed/ratio)
        let compressed = zstd::encode_all(&data[..], 3)?;

        writer.write_all(&compressed)?;
        writer.flush()?;
        drop(writer);

        // Atomic rename
        fs::rename(&temp_path, &path)?;

        Ok(())
    }

    /// Explicitly acquires a region, loading it from disk if needed.
    ///
    /// This is useful for batch operations where you want to pre-load a region
    /// before loading multiple chunks from it. Each call increments the reference
    /// count, so you must call `release_region` a matching number of times.
    ///
    /// For simple single-chunk loads, prefer `load_chunk` which handles this automatically.
    pub fn acquire_region(&self, chunk_pos: ChunkPos) -> io::Result<()> {
        let region_pos = RegionPos::from_chunk(chunk_pos.0.x, chunk_pos.0.y);

        let mut regions = self.regions.write();

        if let Some(loaded) = regions.get_mut(&region_pos) {
            loaded.loaded_chunk_count += 1;
            return Ok(());
        }

        // Load from disk
        let data = self.load_region(region_pos)?;
        regions.insert(
            region_pos,
            LoadedRegion {
                data,
                loaded_chunk_count: 1,
                dirty: false,
            },
        );

        Ok(())
    }

    /// Decrements a region's reference count and optionally saves/unloads it.
    ///
    /// When the reference count reaches zero, the region is saved (if dirty) and
    /// unloaded from memory.
    pub fn release_region(&self, chunk_pos: ChunkPos) -> io::Result<()> {
        let region_pos = RegionPos::from_chunk(chunk_pos.0.x, chunk_pos.0.y);

        let mut regions = self.regions.write();

        let should_unload = if let Some(loaded) = regions.get_mut(&region_pos) {
            loaded.loaded_chunk_count = loaded.loaded_chunk_count.saturating_sub(1);
            loaded.loaded_chunk_count == 0
        } else {
            return Ok(());
        };

        if should_unload {
            if let Some(loaded) = regions.remove(&region_pos) {
                if loaded.dirty {
                    // Rebuild tables and save
                    let optimized = self.rebuild_tables(loaded.data);
                    self.save_region(region_pos, &optimized)?;
                }
            }
        }

        Ok(())
    }

    /// Saves a chunk to the appropriate region.
    ///
    /// If the chunk is already tracked (via `load_chunk`), this just updates the data.
    /// Otherwise, this does NOT increment the reference count - the region will be
    /// saved and unloaded after `flush_all` or when another chunk from this region
    /// is released.
    ///
    /// For chunks that will remain loaded in memory, use `load_chunk` first or
    /// call `acquire_region` to ensure the region stays loaded.
    pub fn save_chunk(&self, chunk: &LevelChunk, status: ChunkStatus) -> io::Result<()> {
        let region_pos = RegionPos::from_chunk(chunk.pos.0.x, chunk.pos.0.y);
        let (local_x, local_z) = RegionPos::local_chunk_pos(chunk.pos.0.x, chunk.pos.0.y);
        let index = RegionFile::chunk_index(local_x, local_z);

        let mut regions = self.regions.write();

        let loaded = regions.entry(region_pos).or_insert_with(|| LoadedRegion {
            data: self
                .load_region(region_pos)
                .unwrap_or_else(|_| RegionFile::new()),
            loaded_chunk_count: 0,
            dirty: false,
        });

        // Convert chunk to persistent format
        let persistent = self.chunk_to_persistent(chunk, status, &mut loaded.data);
        loaded.data.chunks[index] = Some(persistent);
        loaded.dirty = true;

        Ok(())
    }

    /// Loads a chunk from the appropriate region.
    ///
    /// Automatically acquires the region if not already loaded. The region's reference
    /// count is incremented, so you must call `release_chunk` when done with the chunk.
    ///
    /// Returns `Ok(None)` if the chunk doesn't exist on disk.
    pub fn load_chunk(&self, pos: ChunkPos) -> io::Result<Option<(LevelChunk, ChunkStatus)>> {
        let region_pos = RegionPos::from_chunk(pos.0.x, pos.0.y);
        let (local_x, local_z) = RegionPos::local_chunk_pos(pos.0.x, pos.0.y);
        let index = RegionFile::chunk_index(local_x, local_z);

        let mut regions = self.regions.write();

        // Track if we just loaded this region (for cleanup on missing chunk)
        let was_already_loaded = regions.contains_key(&region_pos);

        // Get or load the region
        let loaded = if let Some(loaded) = regions.get_mut(&region_pos) {
            loaded
        } else {
            // Try to load from disk
            let data = match self.load_region(region_pos) {
                Ok(data) => data,
                Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(None),
                Err(e) => return Err(e),
            };
            regions.insert(
                region_pos,
                LoadedRegion {
                    data,
                    loaded_chunk_count: 0,
                    dirty: false,
                },
            );
            regions.get_mut(&region_pos).expect("just inserted")
        };

        // Check if chunk exists
        let chunk_exists = loaded.data.chunks[index].is_some();
        if !chunk_exists {
            // If we just loaded this region and the chunk doesn't exist,
            // remove the region to avoid memory leak (unless it has other refs)
            if !was_already_loaded && loaded.loaded_chunk_count == 0 && !loaded.dirty {
                regions.remove(&region_pos);
            }
            return Ok(None);
        }

        // Increment ref count since we're returning a chunk from this region
        loaded.loaded_chunk_count += 1;

        // SAFETY: We just checked chunk_exists above
        let persistent = loaded.data.chunks[index].as_ref().expect("checked above");
        let chunk = self.persistent_to_chunk(persistent, pos, &loaded.data);
        Ok(Some((chunk, persistent.status)))
    }

    /// Releases a chunk, decrementing the region's reference count.
    ///
    /// When all chunks from a region are released, the region is saved (if dirty)
    /// and unloaded from memory.
    pub fn release_chunk(&self, pos: ChunkPos) -> io::Result<()> {
        self.release_region(pos)
    }

    /// Flushes all dirty regions to disk.
    pub fn flush_all(&self) -> io::Result<()> {
        let mut regions = self.regions.write();

        for (pos, loaded) in regions.iter_mut() {
            if loaded.dirty {
                let optimized = self.rebuild_tables(std::mem::take(&mut loaded.data));
                self.save_region(*pos, &optimized)?;
                loaded.data = optimized;
                loaded.dirty = false;
            }
        }

        Ok(())
    }

    /// Converts a runtime chunk to persistent format.
    fn chunk_to_persistent(
        &self,
        chunk: &LevelChunk,
        status: ChunkStatus,
        region: &mut RegionFile,
    ) -> PersistentChunk {
        let sections = chunk
            .sections
            .sections
            .iter()
            .map(|section| self.section_to_persistent(section, region))
            .collect();

        PersistentChunk {
            status,
            last_modified: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs() as u32)
                .unwrap_or(0),
            sections,
            block_entities: Vec::new(), // TODO: Implement block entity serialization
        }
    }

    /// Converts a runtime section to persistent format.
    fn section_to_persistent(
        &self,
        section: &ChunkSection,
        region: &mut RegionFile,
    ) -> PersistentSection {
        let biomes = self.biomes_to_persistent(&section.biomes, region);

        match &section.states {
            PalettedContainer::Homogeneous(block_id) => {
                let block_idx = self.ensure_block_state(region, *block_id);
                PersistentSection::Homogeneous {
                    block_state: block_idx,
                    biomes,
                }
            }
            PalettedContainer::Heterogeneous(data) => {
                // Build local palette (indices into region's block_states)
                let palette: Vec<u32> = data
                    .palette
                    .iter()
                    .map(|(block_id, _)| self.ensure_block_state(region, *block_id))
                    .collect();

                // Pack block indices
                let bits = bits_for_palette_len(palette.len()).unwrap_or(4);
                let indices: Vec<u32> = data
                    .cube
                    .iter()
                    .flatten()
                    .flatten()
                    .map(|block_id| {
                        data.palette
                            .iter()
                            .position(|(v, _)| v == block_id)
                            .unwrap_or(0) as u32
                    })
                    .collect();

                let block_data = pack_indices(&indices, bits);

                PersistentSection::Heterogeneous {
                    palette,
                    bits_per_entry: bits,
                    block_data,
                    biomes,
                }
            }
        }
    }

    /// Converts runtime biome data to persistent format.
    fn biomes_to_persistent(
        &self,
        biomes: &PalettedContainer<u8, 4>,
        region: &mut RegionFile,
    ) -> PersistentBiomeData {
        match biomes {
            PalettedContainer::Homogeneous(biome_id) => {
                let biome_idx = self.ensure_biome(region, *biome_id);
                PersistentBiomeData::Homogeneous { biome: biome_idx }
            }
            PalettedContainer::Heterogeneous(data) => {
                let palette: Vec<u16> = data
                    .palette
                    .iter()
                    .map(|(biome_id, _)| self.ensure_biome(region, *biome_id))
                    .collect();

                let bits = bits_for_palette_len(palette.len()).unwrap_or(1);
                let indices: Vec<u32> = data
                    .cube
                    .iter()
                    .flatten()
                    .flatten()
                    .map(|biome_id| {
                        data.palette
                            .iter()
                            .position(|(v, _)| v == biome_id)
                            .unwrap_or(0) as u32
                    })
                    .collect();

                let biome_data = pack_indices(&indices, bits);

                PersistentBiomeData::Heterogeneous {
                    palette,
                    bits_per_entry: bits,
                    biome_data,
                }
            }
        }
    }

    /// Ensures a block state exists in the region's table, returning its index.
    fn ensure_block_state(&self, region: &mut RegionFile, block_id: BlockStateId) -> u32 {
        // TODO: Convert BlockStateId to PersistentBlockState using registry
        // For now, store as numeric identifier
        let persistent = PersistentBlockState {
            name: Identifier::vanilla(format!("__block_state_{}", block_id.0)),
            properties: Vec::new(),
        };

        // Check if already exists
        if let Some(idx) = region.block_states.iter().position(|s| s == &persistent) {
            return idx as u32;
        }

        // Add new entry
        let idx = region.block_states.len();
        region.block_states.push(persistent);
        idx as u32
    }

    /// Ensures a biome exists in the region's table, returning its index.
    fn ensure_biome(&self, region: &mut RegionFile, biome_id: u8) -> u16 {
        // TODO: Convert biome ID to identifier using registry
        let identifier = Identifier::vanilla(format!("__biome_{}", biome_id));

        if let Some(idx) = region.biomes.iter().position(|b| b == &identifier) {
            return idx as u16;
        }

        let idx = region.biomes.len();
        region.biomes.push(identifier);
        idx as u16
    }

    /// Converts a persistent chunk to runtime format.
    fn persistent_to_chunk(
        &self,
        persistent: &PersistentChunk,
        pos: ChunkPos,
        region: &RegionFile,
    ) -> LevelChunk {
        let sections: Vec<ChunkSection> = persistent
            .sections
            .iter()
            .map(|section| self.persistent_to_section(section, region))
            .collect();

        LevelChunk {
            sections: Sections {
                sections: sections.into_boxed_slice(),
            },
            pos,
        }
    }

    /// Converts a persistent section to runtime format.
    fn persistent_to_section(
        &self,
        persistent: &PersistentSection,
        region: &RegionFile,
    ) -> ChunkSection {
        match persistent {
            PersistentSection::Homogeneous {
                block_state,
                biomes,
            } => {
                let block_id = self.resolve_block_state(region, *block_state);
                let biome_data = self.persistent_to_biomes(biomes, region);

                ChunkSection {
                    states: PalettedContainer::Homogeneous(block_id),
                    biomes: biome_data,
                }
            }
            PersistentSection::Heterogeneous {
                palette,
                bits_per_entry,
                block_data,
                biomes,
            } => {
                // Unpack indices
                let indices = unpack_indices(block_data, *bits_per_entry, BLOCKS_PER_SECTION);

                // Build runtime palette
                let runtime_palette: Vec<BlockStateId> = palette
                    .iter()
                    .map(|&idx| self.resolve_block_state(region, idx))
                    .collect();

                // Build cube
                let mut cube = Box::new([[[BlockStateId(0); 16]; 16]; 16]);
                for (i, &idx) in indices.iter().enumerate() {
                    let y = i / 256;
                    let z = (i / 16) % 16;
                    let x = i % 16;
                    cube[y][z][x] = runtime_palette
                        .get(idx as usize)
                        .copied()
                        .unwrap_or(BlockStateId(0));
                }

                let states = PalettedContainer::from_cube(cube);
                let biome_data = self.persistent_to_biomes(biomes, region);

                ChunkSection {
                    states,
                    biomes: biome_data,
                }
            }
        }
    }

    /// Converts persistent biome data to runtime format.
    fn persistent_to_biomes(
        &self,
        persistent: &PersistentBiomeData,
        region: &RegionFile,
    ) -> PalettedContainer<u8, 4> {
        match persistent {
            PersistentBiomeData::Homogeneous { biome } => {
                let biome_id = self.resolve_biome(region, *biome);
                PalettedContainer::Homogeneous(biome_id)
            }
            PersistentBiomeData::Heterogeneous {
                palette,
                bits_per_entry,
                biome_data,
            } => {
                let indices = unpack_indices(biome_data, *bits_per_entry, BIOMES_PER_SECTION);

                let runtime_palette: Vec<u8> = palette
                    .iter()
                    .map(|&idx| self.resolve_biome(region, idx))
                    .collect();

                let mut cube = Box::new([[[0u8; 4]; 4]; 4]);
                for (i, &idx) in indices.iter().enumerate() {
                    let y = i / 16;
                    let z = (i / 4) % 4;
                    let x = i % 4;
                    cube[y][z][x] = runtime_palette.get(idx as usize).copied().unwrap_or(0);
                }

                PalettedContainer::from_cube(cube)
            }
        }
    }

    /// Resolves a persistent block state index to a runtime BlockStateId.
    fn resolve_block_state(&self, region: &RegionFile, index: u32) -> BlockStateId {
        // TODO: Use registry to convert PersistentBlockState to BlockStateId
        // For now, parse the numeric identifier we stored
        if let Some(state) = region.block_states.get(index as usize) {
            if let Some(num) = state.name.path.strip_prefix("__block_state_") {
                if let Ok(id) = num.parse::<u16>() {
                    return BlockStateId(id);
                }
            }
        }
        BlockStateId(0) // Air fallback
    }

    /// Resolves a persistent biome index to a runtime biome ID.
    fn resolve_biome(&self, region: &RegionFile, index: u16) -> u8 {
        // TODO: Use registry to convert identifier to biome ID
        if let Some(biome) = region.biomes.get(index as usize) {
            if let Some(num) = biome.path.strip_prefix("__biome_") {
                if let Ok(id) = num.parse::<u8>() {
                    return id;
                }
            }
        }
        0 // Plains fallback
    }

    /// Rebuilds the block state and biome tables to remove unused entries.
    fn rebuild_tables(&self, mut region: RegionFile) -> RegionFile {
        // Collect all used block state and biome indices
        let mut used_block_states = vec![false; region.block_states.len()];
        let mut used_biomes = vec![false; region.biomes.len()];

        // Always keep air (index 0)
        if !used_block_states.is_empty() {
            used_block_states[0] = true;
        }

        for chunk in region.chunks.iter().flatten() {
            for section in &chunk.sections {
                match section {
                    PersistentSection::Homogeneous {
                        block_state,
                        biomes,
                    } => {
                        if let Some(used) = used_block_states.get_mut(*block_state as usize) {
                            *used = true;
                        }
                        self.mark_biomes_used(biomes, &mut used_biomes);
                    }
                    PersistentSection::Heterogeneous {
                        palette, biomes, ..
                    } => {
                        for &idx in palette {
                            if let Some(used) = used_block_states.get_mut(idx as usize) {
                                *used = true;
                            }
                        }
                        self.mark_biomes_used(biomes, &mut used_biomes);
                    }
                }
            }
        }

        // Build remapping tables
        let mut block_state_remap: Vec<u32> = vec![0; region.block_states.len()];
        let mut new_block_states = Vec::new();
        for (old_idx, &used) in used_block_states.iter().enumerate() {
            if used {
                block_state_remap[old_idx] = new_block_states.len() as u32;
                new_block_states.push(region.block_states[old_idx].clone());
            }
        }

        let mut biome_remap: Vec<u16> = vec![0; region.biomes.len()];
        let mut new_biomes = Vec::new();
        for (old_idx, &used) in used_biomes.iter().enumerate() {
            if used {
                biome_remap[old_idx] = new_biomes.len() as u16;
                new_biomes.push(region.biomes[old_idx].clone());
            }
        }

        // Remap all indices in chunks
        for chunk in region.chunks.iter_mut().flatten() {
            for section in &mut chunk.sections {
                match section {
                    PersistentSection::Homogeneous {
                        block_state,
                        biomes,
                    } => {
                        *block_state = block_state_remap[*block_state as usize];
                        self.remap_biomes(biomes, &biome_remap);
                    }
                    PersistentSection::Heterogeneous {
                        palette, biomes, ..
                    } => {
                        for idx in palette.iter_mut() {
                            *idx = block_state_remap[*idx as usize];
                        }
                        self.remap_biomes(biomes, &biome_remap);
                    }
                }
            }
        }

        region.block_states = new_block_states;
        region.biomes = new_biomes;
        region
    }

    fn mark_biomes_used(&self, biomes: &PersistentBiomeData, used: &mut [bool]) {
        match biomes {
            PersistentBiomeData::Homogeneous { biome } => {
                if let Some(u) = used.get_mut(*biome as usize) {
                    *u = true;
                }
            }
            PersistentBiomeData::Heterogeneous { palette, .. } => {
                for &idx in palette {
                    if let Some(u) = used.get_mut(idx as usize) {
                        *u = true;
                    }
                }
            }
        }
    }

    fn remap_biomes(&self, biomes: &mut PersistentBiomeData, remap: &[u16]) {
        match biomes {
            PersistentBiomeData::Homogeneous { biome } => {
                *biome = remap[*biome as usize];
            }
            PersistentBiomeData::Heterogeneous { palette, .. } => {
                for idx in palette.iter_mut() {
                    *idx = remap[*idx as usize];
                }
            }
        }
    }
}
