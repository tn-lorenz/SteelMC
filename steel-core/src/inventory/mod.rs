//! Inventory and container management system.
//!
//! This module provides the core inventory system including containers,
//! menus, crafting, equipment, and recipes.

use std::sync::Arc;

use parking_lot::{ArcMutexGuard, RawMutex};
use steel_utils::locks::SyncMutex;

use crate::{
    inventory::{
        container::Container,
        crafting::{CraftingContainer, ResultContainer},
    },
    player::player_inventory::PlayerInventory,
};

pub mod container;
pub mod crafting;
pub mod equipment;
pub mod inventory_menu;
pub mod menu;
pub mod recipe_manager;
pub mod slot;

pub type SyncPlayerInv = Arc<SyncMutex<PlayerInventory>>;
pub type PluginContainer = Box<dyn Container + Send + Sync>;

pub enum LockedContainer {
    PlayerInventory(ArcMutexGuard<RawMutex, PlayerInventory>),
    CraftingContainer(ArcMutexGuard<RawMutex, CraftingContainer>),
    ResultContainer(ArcMutexGuard<RawMutex, ResultContainer>),
    Other(ArcMutexGuard<RawMutex, PluginContainer>),
}

#[derive(Clone)]
pub enum ContainerRef {
    PlayerInventory(SyncPlayerInv),
    CraftingContainer(Arc<SyncMutex<CraftingContainer>>),
    ResultContainer(Arc<SyncMutex<ResultContainer>>),
    Other(Arc<SyncMutex<PluginContainer>>),
}
