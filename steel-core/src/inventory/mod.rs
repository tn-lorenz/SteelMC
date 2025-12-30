//! Inventory and container management system.
//!
//! This module provides the core inventory system including containers,
//! menus, crafting, equipment, and recipes.

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

/// Thread-safe container type wrapped in Arc<Mutex>.
pub type SyncContainer = Arc<SyncMutex<ContainerType>>;
