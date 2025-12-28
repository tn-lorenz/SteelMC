use std::sync::Arc;

use steel_utils::locks::SyncMutex;

use crate::inventory::container::{Container, ContainerType};

pub mod container;
pub mod equipment;
pub mod inventory_menu;
pub mod menu;
pub mod slot;

pub type SyncContainer = Arc<SyncMutex<ContainerType>>;
