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

use steel_protocol::packets::game::HashedStack;
use steel_registry::{REGISTRY, item_stack::ItemStack, menu_type::MenuType};

use crate::inventory::slot::{Slot, SlotType};

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
