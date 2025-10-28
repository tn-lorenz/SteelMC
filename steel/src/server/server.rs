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
        let mut registry = Registry::new_vanilla();
        registry.freeze();

        Server {
            key_store: KeyStore::new(),
            registry: Arc::new(registry),
        }
    }
}
