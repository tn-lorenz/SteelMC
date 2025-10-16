use std::collections::HashMap;

use steel_utils::BlockStateId;

use crate::{
    behaviour::BlockBehaviourProperties,
    properties::{DynProperty, Property},
};

#[derive(Debug)]
pub struct Block {
    pub name: &'static str,
    pub behaviour: BlockBehaviourProperties,
    pub properties: &'static [&'static dyn DynProperty],
}

impl Block {
    pub const fn new(
        name: &'static str,
        behaviour: BlockBehaviourProperties,
        properties: &'static [&'static dyn DynProperty],
    ) -> Self {
        Self {
            name,
            behaviour,
            properties,
        }
    }
}

pub type BlockRef = &'static Block;

// The central registry for all blocks.
pub struct BlockRegistry {
    blocks_by_id: Vec<BlockRef>,
    blocks_by_name: HashMap<&'static str, usize>,
    allows_registering: bool,
    pub state_to_block_lookup: Vec<BlockRef>,
    /// Maps state IDs to block IDs (parallel to state_to_block_lookup for O(1) lookup)
    state_to_block_id: Vec<usize>,
    /// Maps block IDs to their base state ID
    block_to_base_state: Vec<u16>,
    /// The next state ID to be allocated
    next_state_id: u16,
}

impl BlockRegistry {
    // Creates a new, empty registry.
    pub fn new() -> Self {
        Self {
            blocks_by_id: Vec::new(),
            blocks_by_name: HashMap::new(),
            allows_registering: true,
            state_to_block_lookup: Vec::new(),
            state_to_block_id: Vec::new(),
            block_to_base_state: Vec::new(),
            next_state_id: 0,
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
        let base_state_id = self.next_state_id;

        self.blocks_by_name.insert(block.name, id);
        self.blocks_by_id.push(&block);
        self.block_to_base_state.push(base_state_id);

        let mut state_count = 1;
        for property in block.properties {
            state_count *= property.get_possible_values().len();
        }

        for _ in 0..state_count {
            self.state_to_block_lookup.push(block);
            self.state_to_block_id.push(id);
        }

        self.next_state_id += state_count as u16;

        id
    }

    // Retrieves a block by its ID.
    pub fn by_id(&self, id: usize) -> Option<BlockRef> {
        self.blocks_by_id.get(id).map(|b| *b)
    }

    pub fn by_state_id(&self, state_id: BlockStateId) -> Option<BlockRef> {
        self.state_to_block_lookup
            .get(state_id.0 as usize)
            .map(|b| *b)
    }

    // Retrieves a block by its name.
    pub fn by_name(&self, name: &str) -> Option<BlockRef> {
        self.blocks_by_name.get(name).and_then(|id| self.by_id(*id))
    }

    // Panics if that property isn't supposed to be on this block.
    pub fn get_property<T, P: Property<T>>(&self, id: BlockStateId, property: &P) -> T {
        let block = self.by_state_id(id).expect("Invalid state ID");

        // Find the property index in the block's property list
        let property_index = block
            .properties
            .iter()
            .position(|prop| prop.get_name() == property.as_dyn().get_name())
            .expect("Property not found on this block");

        // Get the base state ID for this block (O(1) lookup)
        let block_id = self.state_to_block_id[id.0 as usize];
        let base_state_id = self.block_to_base_state[block_id];

        // Calculate the relative state index
        let relative_index = id.0 - base_state_id;

        // Decode the property indices from the relative state index
        let mut index = relative_index;
        let mut property_value_index = 0;

        for (i, prop) in block.properties.iter().enumerate() {
            let count = prop.get_possible_values().len() as u16;
            let current_index = (index % count) as usize;

            if i == property_index {
                property_value_index = current_index;
            }

            index /= count;
        }

        // Convert the index back to the actual value
        property.value_from_index(property_value_index)
    }

    // Panics if that property isn't supposed to be on this block.
    pub fn set_property<T, P: Property<T>>(
        &self,
        id: BlockStateId,
        property: &P,
        value: T,
    ) -> BlockStateId {
        let block = self.by_state_id(id).expect("Invalid state ID");

        // Find the property index in the block's property list
        let property_index = block
            .properties
            .iter()
            .position(|prop| prop.get_name() == property.as_dyn().get_name())
            .expect(&format!(
                "Property {} not found on block {}",
                property.as_dyn().get_name(),
                block.name
            ));

        // Get the base state ID for this block (O(1) lookup)
        let block_id = self.state_to_block_id[id.0 as usize];
        let base_state_id = self.block_to_base_state[block_id];

        // Calculate the relative state index
        let relative_index = id.0 - base_state_id;

        // Decode all property indices from the relative state index
        let mut index = relative_index;
        let mut property_indices = Vec::with_capacity(block.properties.len());

        for prop in block.properties.iter() {
            let count = prop.get_possible_values().len() as u16;
            property_indices.push((index % count) as usize);
            index /= count;
        }

        // Update the specific property's index
        let new_value_index = property.get_internal_index(&value);
        property_indices[property_index] = new_value_index;

        // Re-encode the property indices back to a state ID
        let (new_relative_index, _) = property_indices.iter().zip(block.properties.iter()).fold(
            (0u16, 1u16),
            |(current_index, multiplier), (&value_idx, prop)| {
                let count = prop.get_possible_values().len() as u16;
                (
                    current_index + (value_idx as u16) * multiplier,
                    multiplier * count,
                )
            },
        );

        BlockStateId(base_state_id + new_relative_index)
    }
}
