use std::sync::Arc;

use scc::{HashIndex, hash_index::OccupiedEntry};
use steel_utils::{ChunkPos, SteelRwLock, math::vector2::Vector2};

use crate::section::ChunkSections;

pub mod section;

#[derive(Debug)]
pub struct ChunkData {
    pub sections: ChunkSections,
}

pub struct Level {
    pub chunks: HashIndex<ChunkPos, SteelRwLock<ChunkData>>,
}

impl Level {
    pub fn new() -> Self {
        Self {
            chunks: HashIndex::new(),
        }
    }
}
