//! The player's inventory menu.
//!
//! Slot layout (46 total):
//! - Slot 0: Crafting result
//! - Slots 1-4: 2x2 crafting grid
//! - Slots 5-8: Armor (head, chest, legs, feet)
//! - Slots 9-35: Main inventory (27 slots)
//! - Slots 36-44: Hotbar (9 slots)
//! - Slot 45: Offhand

use steel_registry::data_components::vanilla_components::EquippableSlot;
use steel_registry::item_stack::ItemStack;

use crate::inventory::{
    SyncContainer,
    menu::{Menu, MenuBehavior},
    slot::{ArmorSlot, NormalSlot, Slot, SlotType},
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
        // Order matches Java's SLOT_IDS: HEAD, CHEST, LEGS, FEET
        menu_slots.push(SlotType::Armor(ArmorSlot::new(
            inventory.clone(),
            39,
            EquippableSlot::Head,
        ))); // Head
        menu_slots.push(SlotType::Armor(ArmorSlot::new(
            inventory.clone(),
            38,
            EquippableSlot::Chest,
        ))); // Chest
        menu_slots.push(SlotType::Armor(ArmorSlot::new(
            inventory.clone(),
            37,
            EquippableSlot::Legs,
        ))); // Legs
        menu_slots.push(SlotType::Armor(ArmorSlot::new(
            inventory.clone(),
            36,
            EquippableSlot::Feet,
        ))); // Feet

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

    /// Returns true if the given slot index is in the hotbar or offhand.
    /// Based on Java's `InventoryMenu::isHotbarSlot`.
    #[must_use] 
    pub fn is_hotbar_slot(slot: usize) -> bool {
        (slots::HOTBAR_SLOT_START..slots::HOTBAR_SLOT_END).contains(&slot)
            || slot == slots::OFFHAND_SLOT
    }

    /// Returns true if the given slot index is an armor slot.
    #[must_use] 
    pub fn is_armor_slot(slot: usize) -> bool {
        (slots::ARMOR_SLOT_START..slots::ARMOR_SLOT_END).contains(&slot)
    }

    /// Returns true if the given slot index is in the main inventory.
    #[must_use] 
    pub fn is_inventory_slot(slot: usize) -> bool {
        (slots::INV_SLOT_START..slots::INV_SLOT_END).contains(&slot)
    }

    /// Helper method to move items between inventory and hotbar.
    fn move_between_inventory_and_hotbar(
        &mut self,
        slot_index: usize,
        stack: &mut ItemStack,
    ) -> bool {
        if (slots::INV_SLOT_START..slots::INV_SLOT_END).contains(&slot_index) {
            // Main inventory -> hotbar (36-45)
            self.behavior.move_item_stack_to(
                stack,
                slots::HOTBAR_SLOT_START,
                slots::HOTBAR_SLOT_END,
                false,
            )
        } else if (slots::HOTBAR_SLOT_START..slots::HOTBAR_SLOT_END).contains(&slot_index) {
            // Hotbar -> main inventory (9-36)
            self.behavior.move_item_stack_to(
                stack,
                slots::INV_SLOT_START,
                slots::INV_SLOT_END,
                false,
            )
        } else if slot_index == slots::OFFHAND_SLOT {
            // Offhand -> inventory (9-45)
            self.behavior.move_item_stack_to(
                stack,
                slots::INV_SLOT_START,
                slots::OFFHAND_SLOT,
                false,
            )
        } else {
            // Default: try to move to inventory
            self.behavior.move_item_stack_to(
                stack,
                slots::INV_SLOT_START,
                slots::OFFHAND_SLOT,
                false,
            )
        }
    }
}

impl Menu for InventoryMenu {
    fn behavior(&self) -> &MenuBehavior {
        &self.behavior
    }

    fn behavior_mut(&mut self) -> &mut MenuBehavior {
        &mut self.behavior
    }

    /// Handles shift-click (quick move) for a slot.
    /// Based on Java's `InventoryMenu::quickMoveStack`.
    fn quick_move_stack(&mut self, slot_index: usize) -> ItemStack {
        if slot_index >= self.behavior.slots.len() {
            return ItemStack::empty();
        }

        // Get the current item from the slot (avoiding holding a borrow)
        let stack = self.behavior.slots[slot_index].with_item(std::clone::Clone::clone);
        if stack.is_empty() {
            return ItemStack::empty();
        }

        let clicked = stack.clone();
        let mut stack_mut = stack;

        // Determine target range based on which slot was clicked
        // This matches the Java implementation in InventoryMenu::quickMoveStack
        let moved = if slot_index == slots::RESULT_SLOT {
            // Crafting result -> inventory (9-45), prefer to fill existing stacks first
            self.behavior.move_item_stack_to(
                &mut stack_mut,
                slots::INV_SLOT_START,
                slots::OFFHAND_SLOT,
                true,
            )
        } else if (slots::CRAFT_SLOT_START..slots::CRAFT_SLOT_END).contains(&slot_index) {
            // Crafting grid -> inventory (9-45)
            self.behavior.move_item_stack_to(
                &mut stack_mut,
                slots::INV_SLOT_START,
                slots::OFFHAND_SLOT,
                false,
            )
        } else if (slots::ARMOR_SLOT_START..slots::ARMOR_SLOT_END).contains(&slot_index) {
            // Armor -> inventory (9-45)
            self.behavior.move_item_stack_to(
                &mut stack_mut,
                slots::INV_SLOT_START,
                slots::OFFHAND_SLOT,
                false,
            )
        } else {
            // Item is in inventory/hotbar - try to equip it first
            let equippable_slot = clicked.get_equippable_slot();

            // Try to move to armor slot if it's armor
            if let Some(eq_slot) = equippable_slot {
                if eq_slot.is_humanoid_armor() {
                    // Calculate the target armor slot index based on the equipment slot
                    // Java: 8 - eqSlot.getIndex() where HEAD=0, CHEST=1, LEGS=2, FEET=3
                    let armor_slot_index = match eq_slot {
                        EquippableSlot::Head => slots::ARMOR_SLOT_START, // 5
                        EquippableSlot::Chest => slots::ARMOR_SLOT_START + 1, // 6
                        EquippableSlot::Legs => slots::ARMOR_SLOT_START + 2, // 7
                        EquippableSlot::Feet => slots::ARMOR_SLOT_START + 3, // 8
                        _ => unreachable!(),
                    };

                    // Only try to move if the target armor slot is empty
                    if self.behavior.slots[armor_slot_index].has_item() {
                        // Armor slot occupied, move between inventory/hotbar
                        self.move_between_inventory_and_hotbar(slot_index, &mut stack_mut)
                    } else {
                        self.behavior.move_item_stack_to(
                            &mut stack_mut,
                            armor_slot_index,
                            armor_slot_index + 1,
                            false,
                        )
                    }
                } else if eq_slot == EquippableSlot::Offhand {
                    // Try to move to offhand slot if empty
                    if self.behavior.slots[slots::OFFHAND_SLOT].has_item() {
                        self.move_between_inventory_and_hotbar(slot_index, &mut stack_mut)
                    } else {
                        self.behavior.move_item_stack_to(
                            &mut stack_mut,
                            slots::OFFHAND_SLOT,
                            slots::OFFHAND_SLOT + 1,
                            false,
                        )
                    }
                } else {
                    self.move_between_inventory_and_hotbar(slot_index, &mut stack_mut)
                }
            } else {
                self.move_between_inventory_and_hotbar(slot_index, &mut stack_mut)
            }
        };

        if !moved {
            return ItemStack::empty();
        }

        // Update the source slot with the remaining items
        self.behavior.slots[slot_index].set_item(stack_mut.clone());

        // Check if unchanged
        if stack_mut.count == clicked.count {
            return ItemStack::empty();
        }

        self.behavior.slots[slot_index].set_changed();
        clicked
    }

    /// Returns true if the item can be taken from the slot during pickup all.
    /// Prevents taking from the crafting result slot.
    fn can_take_item_for_pick_all(&self, _carried: &ItemStack, slot_index: usize) -> bool {
        // Can't pickup-all from the crafting result slot
        slot_index != slots::RESULT_SLOT
    }
}
