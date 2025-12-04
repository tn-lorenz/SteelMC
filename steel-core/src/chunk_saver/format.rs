//! Data structures for the chunk persistence format.
//!
//! ## Design
//!
//! The format uses per-region tables for block states and biomes to deduplicate
//! strings. Sections store indices into these tables rather than full identifiers.
//!
//! Block data uses power-of-2 bit packing (1, 2, 4, 8, 16 bits) to avoid entries
//! spanning u64 boundaries.

use steel_utils::Identifier;
use wincode::{SchemaRead, SchemaWrite};

use crate::chunk::chunk_access::ChunkStatus;

/// Magic bytes for region file identification: "STLR" (Steel Region)
pub const REGION_MAGIC: [u8; 4] = *b"STLR";

/// Current format version. Increment when making breaking changes.
pub const FORMAT_VERSION: u16 = 1;

/// Number of chunks per region side (32×32 = 1024 chunks per region).
pub const REGION_SIZE: usize = 32;

/// Total chunks in a region.
pub const CHUNKS_PER_REGION: usize = REGION_SIZE * REGION_SIZE;

/// Number of blocks per section side (16×16×16 = 4096 blocks per section).
pub const SECTION_SIZE: usize = 16;

/// Total blocks in a section.
pub const BLOCKS_PER_SECTION: usize = SECTION_SIZE * SECTION_SIZE * SECTION_SIZE;

/// Number of biome cells per section side (4×4×4 = 64 biomes per section).
pub const BIOME_SIZE: usize = 4;

/// Total biome cells in a section.
pub const BIOMES_PER_SECTION: usize = BIOME_SIZE * BIOME_SIZE * BIOME_SIZE;

/// A region file containing a 32×32 grid of chunks.
///
/// Region files are loaded entirely into memory, modified, and saved atomically.
#[derive(SchemaWrite, SchemaRead)]
pub struct RegionFile {
    /// Format version for migrations.
    pub version: u16,

    /// Block states used in this region.
    /// Index 0 is always air (`minecraft:air` with no properties).
    pub block_states: Vec<PersistentBlockState>,

    /// Biome identifiers used in this region.
    /// Stored as full identifiers (e.g., "minecraft:plains").
    pub biomes: Vec<Identifier>,

    /// 32×32 chunks. `None` = chunk never generated.
    /// Index = local_z * 32 + local_x
    pub chunks: Box<[Option<PersistentChunk>; CHUNKS_PER_REGION]>,
}

impl RegionFile {
    /// Creates a new empty region file.
    #[must_use]
    pub fn new() -> Self {
        // Air is always index 0
        let air = PersistentBlockState {
            name: Identifier::vanilla_static("air"),
            properties: Vec::new(),
        };

        Self {
            version: FORMAT_VERSION,
            block_states: vec![air],
            biomes: Vec::new(),
            chunks: Box::new(std::array::from_fn(|_| None)),
        }
    }

    /// Gets the local index for a chunk position within this region.
    #[must_use]
    pub const fn chunk_index(local_x: usize, local_z: usize) -> usize {
        debug_assert!(local_x < REGION_SIZE);
        debug_assert!(local_z < REGION_SIZE);
        local_z * REGION_SIZE + local_x
    }

    /// Converts a chunk index back to local coordinates.
    #[must_use]
    pub const fn index_to_local(index: usize) -> (usize, usize) {
        debug_assert!(index < CHUNKS_PER_REGION);
        (index % REGION_SIZE, index / REGION_SIZE)
    }
}

impl Default for RegionFile {
    fn default() -> Self {
        Self::new()
    }
}

/// A block state with its identifier and properties.
#[derive(SchemaWrite, SchemaRead, Clone, PartialEq, Eq, Hash, Debug)]
pub struct PersistentBlockState {
    /// Block identifier (e.g., "minecraft:oak_stairs").
    pub name: Identifier,
    /// Block properties as key-value pairs (e.g., [("facing", "north")]).
    pub properties: Vec<(String, String)>,
}

/// A persistent chunk containing sections and metadata.
#[derive(SchemaWrite, SchemaRead)]
pub struct PersistentChunk {
    /// Generation status of this chunk.
    pub status: ChunkStatus,
    /// Unix timestamp of last modification.
    pub last_modified: u32,
    /// Vertical sections (typically 24 for -64 to 319).
    pub sections: Vec<PersistentSection>,
    /// Block entities (chests, signs, etc.). Currently placeholder.
    pub block_entities: Vec<PersistentBlockEntity>,
}

/// A 16×16×16 section of a chunk.
#[derive(SchemaWrite, SchemaRead)]
pub enum PersistentSection {
    /// All blocks are the same type.
    Homogeneous {
        /// Index into region's `block_states` table.
        block_state: u32,
        /// Biome data for this section.
        biomes: PersistentBiomeData,
    },
    /// Multiple block types present.
    Heterogeneous {
        /// Local palette: indices into region's `block_states` table.
        palette: Vec<u32>,
        /// Bits per entry (1, 2, 4, 8, or 16).
        bits_per_entry: u8,
        /// Packed block indices into local palette. 4096 entries.
        block_data: Box<[u64]>,
        /// Biome data for this section.
        biomes: PersistentBiomeData,
    },
}

/// Biome data for a section (4×4×4 = 64 cells).
#[derive(SchemaWrite, SchemaRead)]
pub enum PersistentBiomeData {
    /// All 64 biome cells are the same.
    Homogeneous {
        /// Index into region's `biomes` table.
        biome: u16,
    },
    /// Multiple biomes present.
    Heterogeneous {
        /// Local palette: indices into region's `biomes` table.
        palette: Vec<u16>,
        /// Bits per entry (1, 2, 4, or 8).
        bits_per_entry: u8,
        /// Packed biome indices into local palette. 64 entries.
        biome_data: Box<[u64]>,
    },
}

/// A block entity (tile entity) stored with a chunk.
///
/// TODO: Implement proper block entity serialization.
#[derive(SchemaWrite, SchemaRead)]
pub struct PersistentBlockEntity {
    /// Relative X position within chunk (0-15).
    pub x: u8,
    /// Absolute Y position (world height).
    pub y: i16,
    /// Relative Z position within chunk (0-15).
    pub z: u8,
    /// Block entity type identifier.
    pub entity_type: Identifier,
    /// Serialized NBT-like data. Format TBD.
    pub data: Vec<u8>,
}

/// Position of a region in region coordinates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RegionPos {
    /// Region X coordinate (chunk_x / 32).
    pub x: i32,
    /// Region Z coordinate (chunk_z / 32).
    pub z: i32,
}

impl RegionPos {
    /// Creates a new region position.
    #[must_use]
    pub const fn new(x: i32, z: i32) -> Self {
        Self { x, z }
    }

    /// Converts a chunk position to a region position.
    #[must_use]
    pub const fn from_chunk(chunk_x: i32, chunk_z: i32) -> Self {
        Self {
            x: chunk_x.div_euclid(REGION_SIZE as i32),
            z: chunk_z.div_euclid(REGION_SIZE as i32),
        }
    }

    /// Gets the local chunk coordinates within this region for a global chunk position.
    #[must_use]
    pub const fn local_chunk_pos(chunk_x: i32, chunk_z: i32) -> (usize, usize) {
        (
            chunk_x.rem_euclid(REGION_SIZE as i32) as usize,
            chunk_z.rem_euclid(REGION_SIZE as i32) as usize,
        )
    }

    /// Returns the filename for this region (e.g., "r.0.-1.srg").
    #[must_use]
    pub fn filename(&self) -> String {
        format!("r.{}.{}.srg", self.x, self.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_region_pos_from_chunk() {
        // Positive chunks
        assert_eq!(RegionPos::from_chunk(0, 0), RegionPos::new(0, 0));
        assert_eq!(RegionPos::from_chunk(31, 31), RegionPos::new(0, 0));
        assert_eq!(RegionPos::from_chunk(32, 32), RegionPos::new(1, 1));

        // Negative chunks
        assert_eq!(RegionPos::from_chunk(-1, -1), RegionPos::new(-1, -1));
        assert_eq!(RegionPos::from_chunk(-32, -32), RegionPos::new(-1, -1));
        assert_eq!(RegionPos::from_chunk(-33, -33), RegionPos::new(-2, -2));
    }

    #[test]
    fn test_local_chunk_pos() {
        assert_eq!(RegionPos::local_chunk_pos(0, 0), (0, 0));
        assert_eq!(RegionPos::local_chunk_pos(31, 31), (31, 31));
        assert_eq!(RegionPos::local_chunk_pos(32, 32), (0, 0));
        assert_eq!(RegionPos::local_chunk_pos(-1, -1), (31, 31));
        assert_eq!(RegionPos::local_chunk_pos(-32, -32), (0, 0));
    }

    #[test]
    fn test_chunk_index() {
        assert_eq!(RegionFile::chunk_index(0, 0), 0);
        assert_eq!(RegionFile::chunk_index(31, 0), 31);
        assert_eq!(RegionFile::chunk_index(0, 1), 32);
        assert_eq!(RegionFile::chunk_index(31, 31), 1023);
    }
}
