//! This module contains the chunk saver.

use bitcode::{Decode, Encode};
use steel_utils::{ChunkPos, Identifier};

use crate::chunk::chunk_access::ChunkStatus;

pub struct PersistentBlock {
    block_name: Identifier,
    block_props: Vec<(String, String)>,
}

pub struct PersistentChunkSection {
    palette: Vec<PersistentBlock>,
    blocks: Box<[u16]>,
}

pub struct EncodedChunk {
    sections: Vec<PersistentChunkSection>,
    status: ChunkStatus,
}

pub struct ChunkSaver;
