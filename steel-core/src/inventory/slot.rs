use std::sync::Arc;

use enum_dispatch::enum_dispatch;
use steel_utils::locks::SyncMutex;

use crate::inventory::{SyncContainer, container::Container};

pub trait Slot {}

pub struct NormalSlot {
    inventory: SyncContainer,
    index: usize,
}

impl Slot for NormalSlot {}

#[enum_dispatch(Slot)]
pub enum SlotType {
    Normal(NormalSlot),
}
