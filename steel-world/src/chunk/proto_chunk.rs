use steel_utils::locks::SteelRwLock;

use crate::chunk::section::Sections;

// A chunk represeting a chunk that is generating
#[derive(Debug)]
pub struct ProtoChunk {
    pub sections: SteelRwLock<Sections>,
}
