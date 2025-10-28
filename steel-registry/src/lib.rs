#![feature(
    const_trait_impl,
    const_slice_make_iter,
    const_cmp,
    derive_const,
    core_intrinsics
)]
#![allow(internal_features)]

use std::path::Path;

use crate::{
    blocks::blocks::BlockRegistry,
    data_components::{DataComponentRegistry, vanilla_components},
    items::items::ItemRegistry,
};
use include_dir::{Dir, include_dir};
pub mod blocks;
pub mod data_components;
pub mod items;

pub static MINECRAFT_CORE_DIR: Dir =
    include_dir!("$CARGO_MANIFEST_DIR/build_assets/builtin_datapacks");

//#[rustfmt::skip]
#[path = "generated/vanilla_blocks.rs"]
pub mod vanilla_blocks;

//#[rustfmt::skip]
#[path = "generated/vanilla_items.rs"]
pub mod vanilla_items;

//#[rustfmt::skip]
#[path = "generated/packets.rs"]
pub mod packets;

pub trait RegistryExt {
    fn freeze(&mut self);
}

pub struct Registry {
    pub blocks: BlockRegistry,
    pub items: ItemRegistry,
    pub data_components: DataComponentRegistry,
}

impl Registry {
    pub fn new_vanilla() -> Self {
        let mut block_registry = BlockRegistry::new();
        vanilla_blocks::register_blocks(&mut block_registry);

        let mut data_component_registry = DataComponentRegistry::new();
        vanilla_components::register_vanilla_data_components(&mut data_component_registry);

        let mut item_registry = ItemRegistry::new();
        vanilla_items::register_items(&mut item_registry);

        Self {
            blocks: block_registry,
            data_components: data_component_registry,
            items: item_registry,
        }
    }

    pub fn freeze(&mut self) {
        self.blocks.freeze();
        self.data_components.freeze();
        self.items.freeze();
    }
}
