//! A proto chunk is a chunk that is still being generated.
use steel_utils::ChunkPos;

use crate::chunk::section::Sections;

/// A chunk that is still being generated.
#[derive(Debug)]
pub struct ProtoChunk {
    /// The sections of the chunk.
    pub sections: Sections,
    /// The position of the chunk.
    pub pos: ChunkPos,
    /// Whether the chunk has been modified since last save.
    /// Proto chunks start dirty since they're being generated.
    pub dirty: bool,
}

impl ProtoChunk {
    /// Creates a new proto chunk at the given position with empty sections.
    #[must_use]
    pub fn new(sections: Sections, pos: ChunkPos) -> Self {
        Self {
            sections,
            pos,
            dirty: true, // New chunks are always dirty
        }
    }

    /// Creates a proto chunk that was loaded from disk.
    #[must_use]
    pub fn from_disk(sections: Sections, pos: ChunkPos) -> Self {
        Self {
            sections,
            pos,
            dirty: false,
        }
    }
}
