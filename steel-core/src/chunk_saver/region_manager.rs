//! Region file manager with open handle tracking.
//!
//! Keeps region files open in memory while any chunk from that region is loaded,
//! avoiding repeated disk I/O for nearby chunk operations.

use std::{
    fs::{self, File},
    io::{self, BufReader, BufWriter, Read, Write},
    path::PathBuf,
    sync::Arc,
};

use parking_lot::RwLock;
use rustc_hash::FxHashMap;
use steel_registry::Registry;
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
    /// Registry for block state and biome conversions.
    registry: Arc<Registry>,
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

/// Builder for creating a persistent chunk with its own palettes.
struct ChunkBuilder<'a> {
    block_states: Vec<PersistentBlockState>,
    biomes: Vec<Identifier>,
    registry: &'a Registry,
}

impl<'a> ChunkBuilder<'a> {
    fn new(registry: &'a Registry) -> Self {
        Self {
            block_states: Vec::new(),
            biomes: Vec::new(),
            registry,
        }
    }

    /// Ensures a block state exists in the chunk's palette, returning its index.
    fn ensure_block_state(&mut self, block_id: BlockStateId) -> u16 {
        // Get block and properties from registry
        let block = self
            .registry
            .blocks
            .by_state_id(block_id)
            .expect("Invalid block state ID");
        let properties = self.registry.blocks.get_properties(block_id);

        let persistent = PersistentBlockState {
            name: block.key.clone(),
            properties: properties
                .into_iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect(),
        };

        // Check if already exists
        if let Some(idx) = self.block_states.iter().position(|s| s == &persistent) {
            return idx as u16;
        }

        // Add new entry
        let idx = self.block_states.len();
        self.block_states.push(persistent);
        idx as u16
    }

    /// Ensures a biome exists in the chunk's palette, returning its index.
    fn ensure_biome(&mut self, biome_id: u8) -> u16 {
        // Get biome identifier from registry
        let biome = self
            .registry
            .biomes
            .by_id(biome_id as usize)
            .expect("Invalid biome ID");
        let identifier = biome.key.clone();

        if let Some(idx) = self.biomes.iter().position(|b| b == &identifier) {
            return idx as u16;
        }

        let idx = self.biomes.len();
        self.biomes.push(identifier);
        idx as u16
    }
}

/// TODO: Needs
/// - Saving protochunks
impl RegionManager {
    /// Creates a new region manager.
    ///
    /// # Arguments
    /// * `base_path` - Directory where region files are stored.
    /// * `registry` - The registry for block state and biome conversions.
    pub fn new(base_path: impl Into<PathBuf>, registry: Arc<Registry>) -> Self {
        Self {
            base_path: base_path.into(),
            regions: RwLock::new(FxHashMap::default()),
            registry,
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

        if should_unload
            && let Some(loaded) = regions.remove(&region_pos)
            && loaded.dirty
        {
            self.save_region(region_pos, &loaded.data)?;
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
        let persistent = self.chunk_to_persistent(chunk, status);
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
    #[allow(clippy::missing_panics_doc)]
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
        let chunk = self.persistent_to_chunk(persistent, pos);
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
                self.save_region(*pos, &loaded.data)?;
                loaded.dirty = false;
            }
        }

        Ok(())
    }

    /// Converts a runtime chunk to persistent format.
    fn chunk_to_persistent(&self, chunk: &LevelChunk, status: ChunkStatus) -> PersistentChunk {
        let mut builder = ChunkBuilder::new(&self.registry);

        let sections = chunk
            .sections
            .sections
            .iter()
            .map(|section| Self::section_to_persistent(section, &mut builder))
            .collect();

        PersistentChunk {
            status,
            last_modified: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs() as u32)
                .unwrap_or(0),
            block_states: builder.block_states,
            biomes: builder.biomes,
            sections,
            block_entities: Vec::new(), // TODO: Implement block entity serialization
        }
    }

    /// Converts a runtime section to persistent format.
    fn section_to_persistent(
        section: &ChunkSection,
        builder: &mut ChunkBuilder,
    ) -> PersistentSection {
        let biomes = Self::biomes_to_persistent(&section.biomes, builder);

        match &section.states {
            PalettedContainer::Homogeneous(block_id) => {
                let block_idx = builder.ensure_block_state(*block_id);
                PersistentSection::Homogeneous {
                    block_state: block_idx,
                    biomes,
                }
            }
            PalettedContainer::Heterogeneous(data) => {
                // Build section-local palette (indices into chunk's block_states)
                let palette: Vec<u16> = data
                    .palette
                    .iter()
                    .map(|(block_id, _)| builder.ensure_block_state(*block_id))
                    .collect();

                // Pack block indices (indices into section-local palette)
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
        biomes: &PalettedContainer<u8, 4>,
        builder: &mut ChunkBuilder,
    ) -> PersistentBiomeData {
        match biomes {
            PalettedContainer::Homogeneous(biome_id) => {
                let biome_idx = builder.ensure_biome(*biome_id);
                PersistentBiomeData::Homogeneous { biome: biome_idx }
            }
            PalettedContainer::Heterogeneous(data) => {
                // Build section-local palette (indices into chunk's biomes)
                let palette: Vec<u16> = data
                    .palette
                    .iter()
                    .map(|(biome_id, _)| builder.ensure_biome(*biome_id))
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

    /// Converts a persistent chunk to runtime format.
    fn persistent_to_chunk(&self, persistent: &PersistentChunk, pos: ChunkPos) -> LevelChunk {
        let sections: Vec<ChunkSection> = persistent
            .sections
            .iter()
            .map(|section| self.persistent_to_section(section, persistent))
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
        chunk: &PersistentChunk,
    ) -> ChunkSection {
        match persistent {
            PersistentSection::Homogeneous {
                block_state,
                biomes,
            } => {
                let block_id = self.resolve_block_state(chunk, *block_state);
                let biome_data = self.persistent_to_biomes(biomes, chunk);

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
                // Unpack indices (into section-local palette)
                let indices = unpack_indices(block_data, *bits_per_entry, BLOCKS_PER_SECTION);

                // Build runtime palette by resolving section-local -> chunk -> runtime
                let runtime_palette: Vec<BlockStateId> = palette
                    .iter()
                    .map(|&idx| self.resolve_block_state(chunk, idx))
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
                let biome_data = self.persistent_to_biomes(biomes, chunk);

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
        chunk: &PersistentChunk,
    ) -> PalettedContainer<u8, 4> {
        match persistent {
            PersistentBiomeData::Homogeneous { biome } => {
                let biome_id = self.resolve_biome(chunk, *biome);
                PalettedContainer::Homogeneous(biome_id)
            }
            PersistentBiomeData::Heterogeneous {
                palette,
                bits_per_entry,
                biome_data,
            } => {
                let indices = unpack_indices(biome_data, *bits_per_entry, BIOMES_PER_SECTION);

                // Resolve section-local palette -> chunk palette -> runtime
                let runtime_palette: Vec<u8> = palette
                    .iter()
                    .map(|&idx| self.resolve_biome(chunk, idx))
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

    /// Resolves a chunk palette index to a runtime `BlockStateId`.
    fn resolve_block_state(&self, chunk: &PersistentChunk, index: u16) -> BlockStateId {
        if let Some(state) = chunk.block_states.get(index as usize) {
            // Convert properties to the format expected by the registry
            let properties: Vec<(&str, &str)> = state
                .properties
                .iter()
                .map(|(k, v)| (k.as_str(), v.as_str()))
                .collect();

            if let Some(state_id) = self
                .registry
                .blocks
                .state_id_from_properties(&state.name, &properties)
            {
                return state_id;
            }
        }
        BlockStateId(0) // Air fallback
    }

    /// Resolves a chunk palette index to a runtime biome ID.
    fn resolve_biome(&self, chunk: &PersistentChunk, index: u16) -> u8 {
        if let Some(biome_key) = chunk.biomes.get(index as usize)
            && let Some(id) = self.registry.biomes.id_from_key(biome_key)
        {
            return id as u8;
        }
        0 // Plains fallback
    }
}
