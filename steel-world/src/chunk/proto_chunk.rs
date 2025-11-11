use crate::chunk::section::Sections;

// A chunk representing a chunk that is generating
#[derive(Debug)]
pub struct ProtoChunk {
    pub sections: Sections,
}
