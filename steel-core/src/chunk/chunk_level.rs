use crate::chunk::chunk_access::ChunkStatus;
use crate::chunk::chunk_pyramid::GENERATION_PYRAMID;
use crate::chunk::chunk_tracker::MAX_LEVEL;

/// Utilities for converting between chunk levels and statuses.
pub struct ChunkLevel;

impl ChunkLevel {
    pub const FULL_CHUNK_LEVEL: u8 = 33;
    const BLOCK_TICKING_LEVEL: u8 = 32;
    const ENTITY_TICKING_LEVEL: u8 = 31;

    const RADIUS_AROUND_FULL_CHUNK: u8 = 11;
    pub const MAX_LEVEL: u8 = Self::FULL_CHUNK_LEVEL + Self::RADIUS_AROUND_FULL_CHUNK;

    /// Returns the generation status for the given level.
    #[must_use]
    pub fn generation_status(level: u8) -> Option<ChunkStatus> {
        if level >= MAX_LEVEL {
            None
        } else if level <= Self::FULL_CHUNK_LEVEL {
            Some(ChunkStatus::Full)
        } else {
            let distance = (level - Self::FULL_CHUNK_LEVEL) as usize;
            // Fallback to None if distance is out of bounds (simulating Vanilla logic)
            GENERATION_PYRAMID
                .get_step_to(ChunkStatus::Full)
                .accumulated_dependencies
                .get(distance)
        }
    }

    /// Returns the full status for the given level.
    #[must_use]
    pub fn full_status(level: u8) -> Option<ChunkStatus> {
        Self::generation_status(level)
    }
}
