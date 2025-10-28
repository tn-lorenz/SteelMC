use std::sync::Arc;

use steel_registry::{
    Registry,
    blocks::blocks::BlockRegistry,
    data_components::{DataComponentRegistry, vanilla_components},
    items::items::ItemRegistry,
    vanilla_blocks, vanilla_items,
};
use steel_utils::ResourceLocation;
use tokio::time::Instant;

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
        let start = Instant::now();
        let mut registry = Registry::new_vanilla();
        registry.freeze();
        log::info!("Vanilla registry loaded in {:?}", start.elapsed());

        Server {
            key_store: KeyStore::new(),
            registry: Arc::new(registry),
        }
    }
}
