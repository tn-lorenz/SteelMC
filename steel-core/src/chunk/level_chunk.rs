//! This module contains the `LevelChunk` struct, which is a chunk that is ready to be sent to the client.
use std::{
    io::Cursor,
    sync::{Arc, atomic::AtomicBool},
};

use steel_protocol::packets::game::{
    ChunkPacketData, HeightmapType, Heightmaps, LightUpdatePacketData,
};
use steel_utils::{ChunkPos, codec::BitSet};

use crate::chunk::{proto_chunk::ProtoChunk, section::Sections};

/// A chunk that is ready to be sent to the client.
#[derive(Debug, Clone)]
pub struct LevelChunk {
    /// The sections of the chunk.
    pub sections: Sections,
    /// The position of the chunk.
    pub pos: ChunkPos,
    /// Whether the chunk has been modified since last save.
    pub dirty: Arc<AtomicBool>,
}

impl LevelChunk {
    /// Creates a new `LevelChunk` from a `ProtoChunk`.
    #[must_use]
    pub fn from_proto(proto_chunk: ProtoChunk) -> Self {
        Self {
            sections: proto_chunk.sections,
            pos: proto_chunk.pos,
            dirty: proto_chunk.dirty.clone(),
        }
    }

    /// Creates a new `LevelChunk` that was loaded from disk (not dirty).
    #[must_use]
    pub fn from_disk(sections: Sections, pos: ChunkPos) -> Self {
        Self {
            sections,
            pos,
            dirty: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Extracts the chunk data for sending to the client.
    #[must_use]
    pub fn extract_chunk_data(&self) -> ChunkPacketData {
        let data = Vec::new();

        let mut cursor = Cursor::new(data);
        for section in &self.sections.sections {
            section.read().write(&mut cursor);
        }

        ChunkPacketData {
            heightmaps: Heightmaps {
                heightmaps: vec![
                    (HeightmapType::WorldSurface, vec![0; 37]),
                    (HeightmapType::MotionBlocking, vec![0; 37]),
                    (HeightmapType::MotionBlockingNoLeaves, vec![0; 37]),
                ],
            },
            data: cursor.into_inner(),
            block_entities: Vec::new(),
        }
    }

    /// Extracts the light data for sending to the client.
    #[must_use]
    pub fn extract_light_data(&self) -> LightUpdatePacketData {
        let section_count = self.sections.sections.len();
        let mut sky_y_mask = BitSet(vec![0; section_count.div_ceil(64)].into_boxed_slice());
        let mut block_y_mask = BitSet(vec![0; section_count.div_ceil(64)].into_boxed_slice());
        let empty_sky_y_mask = BitSet(vec![0; section_count.div_ceil(64)].into_boxed_slice());
        let empty_block_y_mask = BitSet(vec![0; section_count.div_ceil(64)].into_boxed_slice());

        let mut sky_updates = Vec::new();
        let mut block_updates = Vec::new();

        for i in 0..section_count {
            sky_y_mask.set(i, true);
            block_y_mask.set(i, true);
            sky_updates.push(vec![0xFF; 2048]);
            block_updates.push(vec![0xFF; 2048]);
        }

        LightUpdatePacketData {
            sky_y_mask,
            block_y_mask,
            empty_sky_y_mask,
            empty_block_y_mask,
            sky_updates,
            block_updates,
        }
    }
}
