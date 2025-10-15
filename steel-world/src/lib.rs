use std::sync::Arc;

use steel_utils::{SteelRwLock, math::vector2::Vector2};

use crate::section::ChunkSections;

pub mod section;

// A raw block state id. Using the registry this id can be derived into a block and it's current properties.
pub struct BlockStateId(pub u16);

#[derive(Debug)]
pub struct ChunkData {
    pub sections: ChunkSections,
}

pub struct Level {
    pub chunks: papaya::HashMap<Vector2<i32>, Arc<SteelRwLock<ChunkData>>>,
}
