use steel_utils::locks::SteelRwLock;

use crate::chunk::{proto_chunk::ProtoChunk, section::Sections};

#[derive(Debug)]
pub struct LevelChunk {
    pub sections: SteelRwLock<Sections>,
}

impl LevelChunk {
    pub fn from_proto(proto_chunk: ProtoChunk) -> Self {
        Self {
            sections: proto_chunk.sections,
        }
    }
}
