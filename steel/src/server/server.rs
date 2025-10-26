use std::sync::Arc;

use steel_registry::{
    Registry,
    blocks::blocks::BlockRegistry,
    data_components::{DataComponentRegistry, vanilla_components},
    items::items::ItemRegistry,
    vanilla_blocks, vanilla_items,
};

use crate::server::key_store::KeyStore;

pub struct Server {
    pub key_store: KeyStore,
    pub registry: Arc<Registry>,
}

impl Default for Server {
    fn default() -> Self {
        Self::new()
    }
}

impl Server {
    pub fn new() -> Self {
        let mut block_registry = BlockRegistry::new();
        vanilla_blocks::register_blocks(&mut block_registry);
        block_registry.freeze();

        let mut data_component_registry = DataComponentRegistry::new();
        vanilla_components::register_vanilla_data_components(&mut data_component_registry);
        data_component_registry.freeze();

        let mut item_registry = ItemRegistry::new();
        vanilla_items::register_items(&mut item_registry);
        item_registry.freeze();

        Server {
            key_store: KeyStore::new(),
            registry: Arc::new(Registry {
                blocks: block_registry,
                data_components: data_component_registry,
                items: item_registry,
            }),
        }
    }
}
