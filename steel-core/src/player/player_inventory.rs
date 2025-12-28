//! Player inventory management.

use std::sync::{Arc, Weak};

use steel_registry::item_stack::ItemStack;
use steel_utils::locks::SyncMutex;

use crate::{
    inventory::{
        container::Container,
        equipment::{EntityEquipment, EquipmentSlot},
    },
    player::Player,
};

/// Maps inventory slot indices (36+) to equipment slots.
/// Slots 36-39: Armor (feet, legs, chest, head)
/// Slot 40: Offhand
/// Slot 41: Body armor (for animals, not used for players)
/// Slot 42: Saddle (for animals, not used for players)
fn slot_to_equipment(slot: usize) -> Option<EquipmentSlot> {
    match slot {
        36 => Some(EquipmentSlot::Feet),
        37 => Some(EquipmentSlot::Legs),
        38 => Some(EquipmentSlot::Chest),
        39 => Some(EquipmentSlot::Head),
        40 => Some(EquipmentSlot::OffHand),
        41 => Some(EquipmentSlot::Body),
        42 => Some(EquipmentSlot::Saddle),
        _ => None,
    }
}

pub struct PlayerInventory {
    /// The 36 main inventory slots (0-8 hotbar, 9-35 main).
    items: [ItemStack; Self::INVENTORY_SIZE],
    /// Entity equipment (armor, hands).
    equipment: Arc<SyncMutex<EntityEquipment>>,
    /// Weak reference to the player.
    player: Weak<Player>,
    /// Currently selected hotbar slot (0-8).
    selected: u8,
    /// Counter incremented on every change.
    times_changed: u32,
}

impl PlayerInventory {
    /// Number of main inventory slots.
    pub const INVENTORY_SIZE: usize = 36;
    /// Number of hotbar slots.
    pub const SELECTION_SIZE: usize = 9;
    /// Slot index for offhand.
    pub const SLOT_OFFHAND: usize = 40;

    pub fn new(equipment: Arc<SyncMutex<EntityEquipment>>, player: Weak<Player>) -> Self {
        Self {
            items: std::array::from_fn(|_| ItemStack::empty()),
            equipment,
            player,
            selected: 0,
            times_changed: 0,
        }
    }

    pub fn is_hotbar_slot(slot: usize) -> bool {
        slot < Self::SELECTION_SIZE
    }

    pub fn get_selected_slot(&self) -> u8 {
        self.selected
    }

    pub fn set_selected_slot(&mut self, slot: u8) {
        if Self::is_hotbar_slot(slot as usize) {
            self.selected = slot;
        } else {
            panic!("Invalid hotbar slot: {}", slot);
        }
    }

    pub fn with_selected_item<R>(&self, f: impl FnOnce(&ItemStack) -> R) -> R {
        f(&self.items[self.selected as usize])
    }

    pub fn with_selected_item_mut<R>(&mut self, f: impl FnOnce(&mut ItemStack) -> R) -> R {
        let result = f(&mut self.items[self.selected as usize]);
        self.set_changed();
        result
    }

    /// Returns the number of times this inventory has been modified.
    pub fn get_times_changed(&self) -> u32 {
        self.times_changed
    }

    /// Returns the non-equipment items (main 36 slots).
    pub fn get_items(&self) -> &[ItemStack; Self::INVENTORY_SIZE] {
        &self.items
    }

    /// Finds the first empty slot in the inventory, or -1 if full.
    pub fn get_free_slot(&self) -> i32 {
        for i in 0..self.items.len() {
            if self.items[i].is_empty() {
                return i as i32;
            }
        }
        -1
    }
}

impl Container for PlayerInventory {
    fn get_container_size(&self) -> usize {
        // 36 main slots + 7 equipment slots (feet, legs, chest, head, offhand, body, saddle)
        Self::INVENTORY_SIZE + 7
    }

    fn with_item<R>(&self, slot: usize, f: impl FnOnce(&ItemStack) -> R) -> R {
        if slot < Self::INVENTORY_SIZE {
            f(&self.items[slot])
        } else if let Some(eq_slot) = slot_to_equipment(slot) {
            let equipment = self.equipment.lock();
            f(equipment.get_ref(eq_slot))
        } else {
            f(&ItemStack::empty())
        }
    }

    fn with_item_mut<R>(&mut self, slot: usize, f: impl FnOnce(&mut ItemStack) -> R) -> R {
        if slot < Self::INVENTORY_SIZE {
            f(&mut self.items[slot])
        } else if let Some(eq_slot) = slot_to_equipment(slot) {
            let mut equipment = self.equipment.lock();
            f(equipment.get_mut(eq_slot))
        } else {
            // Invalid slot, provide a temporary empty stack
            let mut empty = ItemStack::empty();
            f(&mut empty)
        }
    }

    fn set_item(&mut self, slot: usize, stack: ItemStack) {
        if slot < Self::INVENTORY_SIZE {
            self.items[slot] = stack;
        } else if let Some(eq_slot) = slot_to_equipment(slot) {
            self.equipment.lock().set(eq_slot, stack);
        }
        self.set_changed();
    }

    fn is_empty(&self) -> bool {
        for item in &self.items {
            if !item.is_empty() {
                return false;
            }
        }

        let equipment = self.equipment.lock();
        for slot in EquipmentSlot::ALL {
            if !equipment.get_ref(slot).is_empty() {
                return false;
            }
        }

        true
    }

    fn set_changed(&mut self) {
        self.times_changed = self.times_changed.wrapping_add(1);
    }

    fn clear_content(&mut self) {
        for item in &mut self.items {
            *item = ItemStack::empty();
        }
        self.equipment.lock().clear();
        self.set_changed();
    }
}
