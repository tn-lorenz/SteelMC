use std::sync::Arc;

use steel_utils::locks::SyncMutex;

use crate::inventory::container::ContainerType;

pub mod container;
pub mod crafting;
pub mod equipment;
pub mod inventory_menu;
pub mod menu;
pub mod recipe_manager;
pub mod slot;

pub type SyncContainer = Arc<SyncMutex<ContainerType>>;
