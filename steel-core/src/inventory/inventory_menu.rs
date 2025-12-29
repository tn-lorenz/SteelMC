//! The player's inventory menu.
//!
//! Slot layout (46 total):
//! - Slot 0: Crafting result
//! - Slots 1-4: 2x2 crafting grid
//! - Slots 5-8: Armor (head, chest, legs, feet)
//! - Slots 9-35: Main inventory (27 slots)
//! - Slots 36-44: Hotbar (9 slots)
//! - Slot 45: Offhand

use steel_registry::item_stack::ItemStack;

use crate::inventory::{
    SyncContainer,
    menu::{Menu, MenuBehavior},
    slot::{NormalSlot, SlotType},
};

/// Slot indices for the inventory menu.
pub mod slots {
    pub const RESULT_SLOT: usize = 0;
    pub const CRAFT_SLOT_START: usize = 1;
    pub const CRAFT_SLOT_END: usize = 5;
    pub const ARMOR_SLOT_START: usize = 5;
    pub const ARMOR_SLOT_END: usize = 9;
    pub const INV_SLOT_START: usize = 9;
    pub const INV_SLOT_END: usize = 36;
    pub const HOTBAR_SLOT_START: usize = 36;
    pub const HOTBAR_SLOT_END: usize = 45;
    pub const OFFHAND_SLOT: usize = 45;
    pub const TOTAL_SLOTS: usize = 46;
}

/// The player's inventory menu.
/// This is always open when no other menu is open.
pub struct InventoryMenu {
    behavior: MenuBehavior,
}

impl InventoryMenu {
    /// Container ID for the player inventory (always 0).
    pub const CONTAINER_ID: u8 = 0;

    /// Creates a new inventory menu for a player.
    ///
    /// The inventory container should contain:
    /// - Slots 0-35: Main inventory (hotbar 0-8, main 9-35)
    /// - Slots 36-39: Armor (feet, legs, chest, head)
    /// - Slot 40: Offhand
    pub fn new(inventory: SyncContainer) -> Self {
        let mut menu_slots = Vec::with_capacity(slots::TOTAL_SLOTS);

        // Slot 0: Crafting result (placeholder - not backed by real storage yet)
        // For now, we'll use a dummy slot that maps to slot 0 of inventory
        // TODO: Implement proper crafting result slot
        menu_slots.push(SlotType::Normal(NormalSlot::new(inventory.clone(), 0)));

        // Slots 1-4: Crafting grid (placeholder)
        // TODO: Implement proper crafting grid slots
        for i in 0..4 {
            menu_slots.push(SlotType::Normal(NormalSlot::new(inventory.clone(), i)));
        }

        // Slots 5-8: Armor (head, chest, legs, feet)
        // Maps to inventory slots 39, 38, 37, 36
        menu_slots.push(SlotType::Normal(NormalSlot::new(inventory.clone(), 39))); // Head
        menu_slots.push(SlotType::Normal(NormalSlot::new(inventory.clone(), 38))); // Chest
        menu_slots.push(SlotType::Normal(NormalSlot::new(inventory.clone(), 37))); // Legs
        menu_slots.push(SlotType::Normal(NormalSlot::new(inventory.clone(), 36))); // Feet

        // Slots 9-35: Main inventory (27 slots)
        // Maps to inventory slots 9-35
        for i in 9..36 {
            menu_slots.push(SlotType::Normal(NormalSlot::new(inventory.clone(), i)));
        }

        // Slots 36-44: Hotbar (9 slots)
        // Maps to inventory slots 0-8
        for i in 0..9 {
            menu_slots.push(SlotType::Normal(NormalSlot::new(inventory.clone(), i)));
        }

        // Slot 45: Offhand
        // Maps to inventory slot 40
        menu_slots.push(SlotType::Normal(NormalSlot::new(inventory.clone(), 40)));

        Self {
            behavior: MenuBehavior::new(menu_slots, Self::CONTAINER_ID, None),
        }
    }

    /// Returns true if the given slot index is in the hotbar.
    pub fn is_hotbar_slot(slot: usize) -> bool {
        slot >= slots::HOTBAR_SLOT_START && slot < slots::HOTBAR_SLOT_END
    }

    /// Returns true if the given slot index is an armor slot.
    pub fn is_armor_slot(slot: usize) -> bool {
        slot >= slots::ARMOR_SLOT_START && slot < slots::ARMOR_SLOT_END
    }

    /// Returns true if the given slot index is in the main inventory.
    pub fn is_inventory_slot(slot: usize) -> bool {
        slot >= slots::INV_SLOT_START && slot < slots::INV_SLOT_END
    }
}

impl Menu for InventoryMenu {
    fn behavior(&self) -> &MenuBehavior {
        &self.behavior
    }

    fn behavior_mut(&mut self) -> &mut MenuBehavior {
        &mut self.behavior
    }

    /*
    fn clicked(&mut self, slot: i16, button: i8, click_type: ClickType) {
        // TODO: Implement click handling
        // For now, just log the click
        log::trace!(
            "InventoryMenu clicked: slot={}, button={}, click_type={:?}",
            slot,
            button,
            click_type
        );
    } */

    fn quick_move_stack(&mut self, _slot_index: usize) -> ItemStack {
        // TODO: Implement shift-click logic
        // For now, return empty (no movement)
        ItemStack::empty()
    }
}
