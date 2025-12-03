//! This module contains the chunk saver.

use steel_utils::{ChunkPos, Identifier};
use wincode::{SchemaRead, SchemaWrite};

use crate::chunk::chunk_access::ChunkStatus;

#[derive(SchemaWrite, SchemaRead)]
pub struct PersistentBlock {
    block_name: Identifier,
    block_props: Vec<(String, String)>,
}

#[derive(SchemaWrite, SchemaRead)]
pub struct PersistentChunkSection {
    palette: Vec<PersistentBlock>,
    blocks: Box<[u16]>,
}

#[derive(SchemaWrite, SchemaRead)]
pub struct EncodedChunk {
    sections: Vec<PersistentChunkSection>,
    status: ChunkStatus,
}

pub struct ChunkSaver;
