//! Player inventory management.

use std::sync::{Arc, Weak};

use steel_registry::item_stack::ItemStack;
use steel_utils::locks::SyncMutex;

use crate::{
    inventory::{container::Container, equipment::EntityEquipment},
    player::Player,
};

pub struct PlayerInventory {
    slots: [ItemStack; Self::MAIN_SIZE],
    // TODO: Do we need to make this a weak reference?
    entity_equipment: Arc<SyncMutex<EntityEquipment>>,
    player: Weak<Player>,
    selected_slot: u8,
}

impl PlayerInventory {
    pub const MAIN_SIZE: usize = 36;

    pub fn new(entity_equipment: Arc<SyncMutex<EntityEquipment>>, player: Weak<Player>) -> Self {
        Self {
            slots: std::array::from_fn(|_| ItemStack::empty()),
            entity_equipment,
            player,
            selected_slot: 0,
        }
    }

    pub fn is_hotbar_slot(slot: u8) -> bool {
        slot >= 0 && slot < 9
    }

    pub fn set_selected_slot(&mut self, slot: u8) {
        if Self::is_hotbar_slot(slot) {
            self.selected_slot = slot;
        } else {
            panic!("Invalid hotbar slot")
        }
    }

    pub fn get_selected_item(&self) -> &ItemStack {
        &self.slots[self.selected_slot as usize]
    }

    pub fn set_selected_item(&mut self, item: ItemStack) {
        self.slots[self.selected_slot as usize] = item;
    }
}

impl Container for PlayerInventory {}
