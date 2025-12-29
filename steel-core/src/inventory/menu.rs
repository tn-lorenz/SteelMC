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

use steel_protocol::packets::game::{
    CContainerSetContent, CContainerSetSlot, CSetCursorItem, ClickType, HashedStack,
};
use steel_registry::{REGISTRY, item_stack::ItemStack, menu_type::MenuType};

use crate::{
    inventory::slot::{Slot, SlotType},
    player::networking::JavaConnection,
};
use std::sync::Arc;

/// Represents the server's perception of what the client knows about a slot.
///
/// This can be either:
/// - A full ItemStack (when we've sent the item to the client)
/// - A HashedStack (when we've received a hash from the client)
/// - Unknown (initial state, always needs sync)
#[derive(Debug, Clone)]
pub enum RemoteSlot {
    /// We don't know what the client has (initial state).
    Unknown,
    /// We know the exact ItemStack the client should have.
    Known(ItemStack),
    /// We received a hash from the client and verified it matches.
    Hashed(HashedStack),
}

impl Default for RemoteSlot {
    fn default() -> Self {
        Self::Unknown
    }
}

impl RemoteSlot {
    /// Creates an unknown remote slot.
    pub fn unknown() -> Self {
        Self::Unknown
    }

    /// Forces the remote slot to a known ItemStack state.
    /// Called when we send an item to the client.
    pub fn force(&mut self, item: &ItemStack) {
        *self = Self::Known(item.clone());
    }

    /// Receives a hashed stack from the client.
    /// Called when the client sends us their perception.
    pub fn receive(&mut self, hash: HashedStack) {
        *self = Self::Hashed(hash);
    }

    /// Checks if the remote slot matches the local ItemStack.
    pub fn matches(&self, local: &ItemStack) -> bool {
        match self {
            Self::Unknown => false,
            Self::Known(remote) => ItemStack::matches(remote, local),
            Self::Hashed(hash) => hashed_stack_matches(hash, local),
        }
    }
}

/// Checks if a hashed stack matches the given ItemStack.
fn hashed_stack_matches(hash: &HashedStack, item: &ItemStack) -> bool {
    match hash {
        HashedStack::Empty => item.is_empty(),
        HashedStack::Item {
            item_id,
            count,
            components: _,
        } => {
            if item.is_empty() {
                return false;
            }
            // Check item type and count match
            // TODO: Component hash verification would go here
            let local_id = *REGISTRY.items.get_id(item.item) as i32;
            local_id == *item_id && item.count == *count
        }
    }
}

/// Shared behavior and state for all menu types.
pub struct MenuBehavior {
    /// The slots in this menu.
    pub slots: Vec<SlotType>,
    /// Cloned itemstacks from the actual slots (updated each sync).
    pub last_slots: Vec<ItemStack>,
    /// The client's perception of the itemstacks.
    pub remote_slots: Vec<RemoteSlot>,
    /// The item being carried by the cursor.
    pub carried: ItemStack,
    /// The client's perception of the carried item.
    pub remote_carried: RemoteSlot,
    /// The container ID (0 for player inventory).
    pub container_id: u8,
    /// Incremented every time the server and client mismatch.
    pub state_id: u32,
    /// None for player inventory. Some for all other menus.
    pub menu_type: Option<MenuType>,
    /// When true, remote updates are suppressed (during click handling).
    suppress_remote_updates: bool,
}

impl MenuBehavior {
    /// Creates a new menu behavior with the given slots.
    pub fn new(slots: Vec<SlotType>, container_id: u8, menu_type: Option<MenuType>) -> Self {
        let slot_count = slots.len();
        Self {
            slots,
            last_slots: vec![ItemStack::empty(); slot_count],
            remote_slots: vec![RemoteSlot::Unknown; slot_count],
            carried: ItemStack::empty(),
            remote_carried: RemoteSlot::Unknown,
            container_id,
            state_id: 0,
            menu_type,
            suppress_remote_updates: false,
        }
    }

    /// Returns the current state ID.
    pub fn get_state_id(&self) -> u32 {
        self.state_id
    }

    /// Suppresses remote updates during click handling.
    /// Call this before processing a click.
    pub fn suppress_remote_updates(&mut self) {
        self.suppress_remote_updates = true;
    }

    /// Resumes remote updates after click handling.
    /// Call this after processing a click.
    pub fn resume_remote_updates(&mut self) {
        self.suppress_remote_updates = false;
    }

    /// Returns true if a slot index is valid for this menu.
    pub fn is_valid_slot_index(&self, slot: i16) -> bool {
        slot == -999 || (slot >= 0 && (slot as usize) < self.slots.len())
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
        !self.remote_slots[index].matches(&self.last_slots[index])
    }

    /// Marks a slot as synced (updates remote perception).
    pub fn mark_slot_synced(&mut self, index: usize) {
        if index < self.last_slots.len() && index < self.remote_slots.len() {
            self.remote_slots[index].force(&self.last_slots[index]);
        }
    }

    /// Checks if carried item needs sync.
    pub fn carried_needs_sync(&self) -> bool {
        !self.remote_carried.matches(&self.carried)
    }

    /// Marks carried as synced.
    pub fn mark_carried_synced(&mut self) {
        self.remote_carried.force(&self.carried);
    }

    /// Sends all slots and carried item to the client (full sync).
    /// This is called when:
    /// - A menu is first opened
    /// - The client requests a full refresh
    /// - After certain operations that may have desynced the client
    pub fn send_all_data_to_remote(&mut self, connection: &Arc<JavaConnection>) {
        // First, update last_slots from actual slot contents
        self.update_last_slots();

        // Send full container content
        let packet = CContainerSetContent {
            container_id: self.container_id as i32,
            state_id: self.state_id as i32,
            items: self.last_slots.clone(),
            carried_item: self.carried.clone(),
        };

        connection.send_packet(packet);

        // Mark all slots and carried as synced
        for i in 0..self.last_slots.len() {
            self.remote_slots[i].force(&self.last_slots[i]);
        }
        self.remote_carried.force(&self.carried);
    }

    /// Broadcasts changes to the client (incremental sync).
    /// This is called every tick to sync only changed slots.
    ///
    /// Based on Java's AbstractContainerMenu::broadcastChanges.
    pub fn broadcast_changes(&mut self, connection: &Arc<JavaConnection>) {
        // Update last_slots from actual slot contents
        self.update_last_slots();

        // Track if we need to increment state_id
        let mut has_changes = false;

        // Check each slot for changes
        for i in 0..self.last_slots.len() {
            if self.slot_needs_sync(i) {
                has_changes = true;
                self.synchronize_slot_to_remote(i, connection);
            }
        }

        // Check carried item
        if self.carried_needs_sync() {
            has_changes = true;
            self.synchronize_carried_to_remote(connection);
        }

        // Increment state_id if we had any changes
        if has_changes {
            self.increment_state_id();
        }
    }

    /// Sends a single slot update to the client.
    /// Based on Java's AbstractContainerMenu::synchronizeSlotToRemote.
    fn synchronize_slot_to_remote(&mut self, slot: usize, connection: &Arc<JavaConnection>) {
        if self.suppress_remote_updates || slot >= self.last_slots.len() {
            return;
        }

        let item = &self.last_slots[slot];

        let packet = CContainerSetSlot {
            container_id: self.container_id as i32,
            state_id: self.state_id as i32,
            slot: slot as i16,
            item_stack: item.clone(),
        };

        connection.send_packet(packet);
        self.mark_slot_synced(slot);
    }

    /// Sends the carried item (cursor) to the client.
    /// Based on Java's AbstractContainerMenu::synchronizeCarriedToRemote.
    fn synchronize_carried_to_remote(&mut self, connection: &Arc<JavaConnection>) {
        if self.suppress_remote_updates {
            return;
        }

        let packet = CSetCursorItem {
            item_stack: self.carried.clone(),
        };

        connection.send_packet(packet);
        self.mark_carried_synced();
    }

    /// Sets a remote slot to a known ItemStack.
    /// Called when we know exactly what the client has (e.g., creative mode set).
    /// Based on Java's AbstractContainerMenu::setRemoteSlot.
    pub fn set_remote_slot_known(&mut self, slot: usize, item: &ItemStack) {
        if slot < self.remote_slots.len() {
            self.remote_slots[slot].force(item);
        }
    }

    /// Handles a remote slot update from the client.
    /// This is called when the client sends us their perception of a slot.
    /// Based on Java's AbstractContainerMenu::setRemoteSlotUnsafe.
    pub fn set_remote_slot(&mut self, slot: usize, hash: HashedStack) {
        if slot < self.remote_slots.len() {
            self.remote_slots[slot].receive(hash);
        }
    }

    /// Handles a remote carried update from the client.
    /// Based on Java's AbstractContainerMenu::setRemoteCarried.
    pub fn set_remote_carried(&mut self, hash: HashedStack) {
        self.remote_carried.receive(hash);
    }

    /// Broadcasts full state to client.
    /// This forces all slots to be synced, even if they match.
    /// Based on Java's AbstractContainerMenu::broadcastFullState.
    pub fn broadcast_full_state(&mut self, connection: &Arc<JavaConnection>) {
        self.update_last_slots();

        // Send all individual slots
        for i in 0..self.last_slots.len() {
            self.synchronize_slot_to_remote(i, connection);
        }

        // Send carried item
        self.synchronize_carried_to_remote(connection);

        // Increment state_id since we sent everything
        self.increment_state_id();
    }

    /// Handles a click action in this menu.
    /// Based on Java's AbstractContainerMenu::clicked and doClick.
    pub fn clicked(&mut self, slot_num: i16, button: i8, click_type: ClickType) {
        match click_type {
            ClickType::Pickup => self.do_pickup(slot_num, button),
            ClickType::QuickMove => self.do_quick_move(slot_num),
            ClickType::Swap => self.do_swap(slot_num, button),
            ClickType::Clone => self.do_clone(slot_num),
            ClickType::Throw => self.do_throw(slot_num, button),
            ClickType::QuickCraft => {
                // TODO: Implement quick craft (drag distribution)
                log::trace!("QuickCraft not yet implemented");
            }
            ClickType::PickupAll => self.do_pickup_all(slot_num, button),
        }
    }

    /// Handles pickup click (left/right click to pick up or place items).
    fn do_pickup(&mut self, slot_num: i16, button: i8) {
        // Slot -999 means clicked outside the inventory (drop items)
        if slot_num == -999 {
            if !self.carried.is_empty() {
                if button == 0 {
                    // Left click outside - drop all carried items
                    // TODO: Actually drop the items into the world
                    log::debug!("Would drop all carried: {:?}", self.carried);
                    self.carried = ItemStack::empty();
                } else {
                    // Right click outside - drop one carried item
                    // TODO: Actually drop one item into the world
                    log::debug!("Would drop one carried");
                    let new_count = self.carried.count - 1;
                    if new_count <= 0 {
                        self.carried = ItemStack::empty();
                    } else {
                        self.carried.set_count(new_count);
                    }
                }
            }
            return;
        }

        let slot_index = slot_num as usize;
        if slot_index >= self.slots.len() {
            return;
        }

        let slot = &self.slots[slot_index];

        // Get the current item in the slot
        let slot_item = slot.with_item(|item| item.clone());
        let carried = std::mem::take(&mut self.carried);

        if slot_item.is_empty() {
            // Slot is empty - place carried items
            if !carried.is_empty() {
                let amount = if button == 0 { carried.count } else { 1 };
                let mut to_place = carried.clone();
                to_place.set_count(amount);

                let remaining = carried.count - amount;
                if remaining > 0 {
                    let mut new_carried = carried;
                    new_carried.set_count(remaining);
                    self.carried = new_carried;
                }

                slot.set_item(to_place);
            }
        } else if carried.is_empty() {
            // Carried is empty - pick up from slot
            let amount = if button == 0 {
                slot_item.count
            } else {
                (slot_item.count + 1) / 2
            };

            let taken = slot.remove(amount);
            self.carried = taken;
        } else if ItemStack::is_same_item_same_components(&slot_item, &carried) {
            // Same item type - try to stack
            if button == 0 {
                // Left click - add as many as possible to slot
                let max = slot.get_max_stack_size_for_item(&carried);
                let space = max - slot_item.count;
                let to_add = space.min(carried.count);

                if to_add > 0 {
                    slot.with_item_mut(|item| {
                        item.set_count(item.count + to_add);
                    });
                    let remaining = carried.count - to_add;
                    if remaining > 0 {
                        let mut new_carried = carried;
                        new_carried.set_count(remaining);
                        self.carried = new_carried;
                    }
                } else {
                    self.carried = carried;
                }
            } else {
                // Right click - add one to slot
                let max = slot.get_max_stack_size_for_item(&carried);
                if slot_item.count < max {
                    slot.with_item_mut(|item| {
                        item.set_count(item.count + 1);
                    });
                    let remaining = carried.count - 1;
                    if remaining > 0 {
                        let mut new_carried = carried;
                        new_carried.set_count(remaining);
                        self.carried = new_carried;
                    }
                } else {
                    self.carried = carried;
                }
            }
        } else {
            // Different items - swap
            if carried.count <= slot.get_max_stack_size_for_item(&carried) {
                slot.set_item(carried);
                self.carried = slot_item;
            } else {
                self.carried = carried;
            }
        }

        slot.set_changed();
    }

    /// Handles quick move (shift-click).
    fn do_quick_move(&mut self, slot_num: i16) {
        if slot_num < 0 {
            return;
        }
        // TODO: Delegate to Menu trait's quick_move_stack
        log::trace!("QuickMove slot {} not yet fully implemented", slot_num);
    }

    /// Handles swap (number keys to swap with hotbar).
    fn do_swap(&mut self, slot_num: i16, button: i8) {
        if slot_num < 0 {
            return;
        }
        // button is the hotbar slot (0-8) or 40 for offhand
        log::trace!(
            "Swap slot {} with hotbar {} not yet implemented",
            slot_num,
            button
        );
        // TODO: Implement hotbar swap
    }

    /// Handles clone (middle-click in creative).
    fn do_clone(&mut self, slot_num: i16) {
        if slot_num < 0 || self.carried.is_empty() == false {
            return;
        }
        // TODO: Check if player has infinite materials (creative mode)
        // For now, just log
        log::trace!("Clone slot {} not yet implemented", slot_num);
    }

    /// Handles throw (drop key).
    fn do_throw(&mut self, slot_num: i16, button: i8) {
        if slot_num < 0 {
            return;
        }

        let slot_index = slot_num as usize;
        if slot_index >= self.slots.len() {
            return;
        }

        let slot = &self.slots[slot_index];
        let amount = if button == 0 {
            1
        } else {
            slot.with_item(|i| i.count)
        };

        let dropped = slot.remove(amount);
        if !dropped.is_empty() {
            // TODO: Actually drop the items into the world
            log::debug!("Would drop {:?}", dropped);
        }
        slot.set_changed();
    }

    /// Handles pickup all (double-click).
    fn do_pickup_all(&mut self, slot_num: i16, _button: i8) {
        if slot_num < 0 || self.carried.is_empty() {
            return;
        }
        // TODO: Collect matching items from all slots
        log::trace!("PickupAll slot {} not yet implemented", slot_num);
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
