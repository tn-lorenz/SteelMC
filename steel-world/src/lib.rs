use std::sync::Arc;

use steel_utils::{SteelRwLock, math::vector2::Vector2};

use crate::section::ChunkSections;

pub mod section;

#[derive(Debug)]
pub struct ChunkData {
    pub sections: ChunkSections,
}

pub struct Level {
    pub chunks: papaya::HashMap<Vector2<i32>, Arc<SteelRwLock<ChunkData>>>,
}
