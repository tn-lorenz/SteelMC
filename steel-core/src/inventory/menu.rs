//! A menu can be considered everything that's shown on the screen.
//! It consists of slots, slots consist of a view into a single inventory and position.
//! When you have a chest open for example a chest menu is shown, consisting of the chests slots and the players inventory slots.
//!
//! A menu is always the middle man between the server and the client.
//! This means that when the player doesn't have any menus open it actually has, it always has it's own inventory menu open.
//!
//! A menu holds 3 important structures:
//! - All slots for that menu
//! - All slots as cloned itemstacks
//! - The clients perception of the itemstacks
//!
//! This makes it so every time we run a sync (once per tick) we update the cloned itemstacks.
//! This in turn makes it so we can compare it with the clients perception of the itemstacks.
//! And if there are mismatches we can send the correct itemstacks to the client.
//!
//! The client also sends the itemstacks it thinks it has on interaction, so this makes it so we only update the client if they mismatch.

use steel_registry::{item_stack::ItemStack, menu_type::MenuType};

use crate::inventory::slot::{Slot, SlotType};

/// Shared behavior and state for all menu types.
pub struct MenuBehavior {
    /// The slots in this menu.
    pub slots: Vec<SlotType>,
    /// Cloned itemstacks from the actual slots (updated each sync).
    pub last_slots: Vec<ItemStack>,
    /// The client's perception of the itemstacks.
    pub remote_slots: Vec<ItemStack>,
    /// The item being carried by the cursor.
    pub carried: ItemStack,
    /// The client's perception of the carried item.
    pub remote_carried: ItemStack,
    /// The container ID (0 for player inventory).
    pub container_id: u8,
    /// Incremented every time the server and client mismatch.
    pub state_id: u32,
    /// None for player inventory. Some for all other menus.
    pub menu_type: Option<MenuType>,
}

impl MenuBehavior {
    /// Creates a new menu behavior with the given slots.
    pub fn new(slots: Vec<SlotType>, container_id: u8, menu_type: Option<MenuType>) -> Self {
        let slot_count = slots.len();
        Self {
            slots,
            last_slots: vec![ItemStack::empty(); slot_count],
            remote_slots: vec![ItemStack::empty(); slot_count],
            carried: ItemStack::empty(),
            remote_carried: ItemStack::empty(),
            container_id,
            state_id: 0,
            menu_type,
        }
    }

    /// Returns the number of slots in this menu.
    pub fn slot_count(&self) -> usize {
        self.slots.len()
    }

    /// Gets a reference to a slot by index.
    pub fn get_slot(&self, index: usize) -> Option<&SlotType> {
        self.slots.get(index)
    }

    /// Gets the carried item (cursor).
    pub fn get_carried(&self) -> &ItemStack {
        &self.carried
    }

    /// Sets the carried item (cursor).
    pub fn set_carried(&mut self, item: ItemStack) {
        self.carried = item;
    }

    /// Increments and returns the new state ID.
    pub fn increment_state_id(&mut self) -> u32 {
        self.state_id = self.state_id.wrapping_add(1) & 0x7FFF; // Keep it within 15 bits
        self.state_id
    }

    /// Updates last_slots from actual slot contents.
    /// Call this once per tick before comparing with remote_slots.
    pub fn update_last_slots(&mut self) {
        for (i, slot) in self.slots.iter().enumerate() {
            self.last_slots[i] = slot.with_item(|item| item.clone());
        }
    }

    /// Checks if a slot has changed compared to remote perception.
    /// Returns true if slot needs to be synced to client.
    pub fn slot_needs_sync(&self, index: usize) -> bool {
        if index >= self.last_slots.len() || index >= self.remote_slots.len() {
            return false;
        }
        !ItemStack::matches(&self.last_slots[index], &self.remote_slots[index])
    }

    /// Marks a slot as synced (updates remote perception).
    pub fn mark_slot_synced(&mut self, index: usize) {
        if index < self.last_slots.len() && index < self.remote_slots.len() {
            self.remote_slots[index] = self.last_slots[index].clone();
        }
    }

    /// Checks if carried item needs sync.
    pub fn carried_needs_sync(&self) -> bool {
        !ItemStack::matches(&self.carried, &self.remote_carried)
    }

    /// Marks carried as synced.
    pub fn mark_carried_synced(&mut self) {
        self.remote_carried = self.carried.clone();
    }
}

/// Trait for menu implementations.
pub trait Menu {
    /// Returns a reference to the menu behavior.
    fn behavior(&self) -> &MenuBehavior;

    /// Returns a mutable reference to the menu behavior.
    fn behavior_mut(&mut self) -> &mut MenuBehavior;

    /// Handles a click action in this menu.
    //fn clicked(&mut self, slot: i16, button: i8, click_type: ClickType);

    /// Handles shift-click (quick move) for a slot.
    /// Returns the resulting item stack (empty if fully moved).
    fn quick_move_stack(&mut self, slot_index: usize) -> ItemStack;

    /// Returns true if this menu is still valid for the player.
    fn still_valid(&self) -> bool {
        true
    }
}
