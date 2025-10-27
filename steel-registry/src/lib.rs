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
    blocks::blocks::BlockRegistry, data_components::DataComponentRegistry,
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

pub struct Registry {
    pub blocks: BlockRegistry,
    pub items: ItemRegistry,
    pub data_components: DataComponentRegistry,
}
