use std::collections::HashMap;

use crate::properties::BlockProperties;

pub struct Block {
    pub name: &'static str,
    pub properties: BlockProperties,
}

impl Block {
    pub fn new(name: &'static str, properties: BlockProperties) -> Self {
        Self { name, properties }
    }
}

pub type BlockRef = &'static Block;

// The central registry for all blocks.
pub struct BlockRegistry {
    blocks_by_id: Vec<BlockRef>,
    blocks_by_name: HashMap<&'static str, usize>,
    allows_registering: bool,
}

impl BlockRegistry {
    // Creates a new, empty registry.
    pub fn new() -> Self {
        Self {
            blocks_by_id: Vec::new(),
            blocks_by_name: HashMap::new(),
            allows_registering: true,
        }
    }

    // Prevents the registry from registering new blocks.
    pub fn freeze(&mut self) {
        self.allows_registering = false;
    }

    // Registers a new block.
    pub fn register(&mut self, block: BlockRef) -> usize {
        if !self.allows_registering {
            panic!("Cannot register blocks after the registry has been frozen");
        }

        let id = self.blocks_by_id.len();
        self.blocks_by_name.insert(block.name, id);
        self.blocks_by_id.push(&block);
        id
    }

    // Retrieves a block by its ID.
    pub fn by_id(&self, id: usize) -> Option<BlockRef> {
        self.blocks_by_id.get(id).map(|b| *b)
    }

    // Retrieves a block by its name.
    pub fn by_name(&self, name: &str) -> Option<BlockRef> {
        self.blocks_by_name.get(name).and_then(|id| self.by_id(*id))
    }
}
