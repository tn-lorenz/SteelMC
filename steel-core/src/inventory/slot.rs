//! Slot abstraction for inventory access.

use std::sync::Arc;

use enum_dispatch::enum_dispatch;
use steel_registry::data_components::vanilla_components::EquippableSlot;
use steel_registry::item_stack::ItemStack;
use steel_utils::locks::SyncMutex;

use crate::inventory::SyncContainer;
use crate::inventory::container::Container;
use crate::inventory::crafting::{CraftingContainer, ResultContainer};
use crate::inventory::recipe_manager;

/// A synchronized crafting container.
pub type SyncCraftingContainer = Arc<SyncMutex<CraftingContainer>>;

/// A synchronized result container.
pub type SyncResultContainer = Arc<SyncMutex<ResultContainer>>;

/// A slot is a view into a single position in a container.
/// Slots handle locking the container and provide access to items.
#[enum_dispatch]
pub trait Slot {
    /// Executes a function with a reference to the item in this slot.
    fn with_item<R>(&self, f: impl FnOnce(&ItemStack) -> R) -> R;

    /// Executes a function with a mutable reference to the item in this slot.
    fn with_item_mut<R>(&self, f: impl FnOnce(&mut ItemStack) -> R) -> R;

    /// Sets the item in this slot.
    fn set_item(&self, stack: ItemStack);

    /// Returns true if this slot has an item.
    fn has_item(&self) -> bool {
        self.with_item(|item| !item.is_empty())
    }

    /// Returns true if the given item can be placed in this slot.
    fn may_place(&self, _stack: &ItemStack) -> bool {
        true
    }

    /// Returns true if items can be picked up from this slot.
    fn may_pickup(&self) -> bool {
        true
    }

    /// Returns true if partial removal is allowed from this slot.
    ///
    /// For normal slots: `may_pickup() && may_place(current_item)`
    /// For result slots: `false` (must take the full stack)
    ///
    /// Based on Java's `Slot.allowModification`.
    fn allow_modification(&self) -> bool {
        self.may_pickup() && self.with_item(|item| self.may_place(item))
    }

    /// Returns the maximum stack size for this slot.
    fn get_max_stack_size(&self) -> i32 {
        99
    }

    /// Returns the maximum stack size for a specific item in this slot.
    fn get_max_stack_size_for_item(&self, stack: &ItemStack) -> i32 {
        self.get_max_stack_size().min(stack.max_stack_size())
    }

    /// Removes up to `amount` items from this slot and returns them.
    fn remove(&self, amount: i32) -> ItemStack {
        self.with_item_mut(|item| {
            if item.is_empty() || amount <= 0 {
                return ItemStack::empty();
            }

            let take_count = amount.min(item.count());
            let mut taken = item.clone();
            taken.set_count(take_count);

            let remaining = item.count() - take_count;
            if remaining <= 0 {
                *item = ItemStack::empty();
            } else {
                item.set_count(remaining);
            }

            taken
        })
    }

    /// Tries to remove items from this slot with validation.
    ///
    /// Returns `Some(items)` if removal succeeded, `None` otherwise.
    /// If `allow_modification()` is false and `max_amount < item.count`,
    /// returns `None` (forcing full stack pickup for result slots).
    ///
    /// Based on Java's `Slot.tryRemove`.
    fn try_remove(&self, amount: i32, max_amount: i32) -> Option<ItemStack> {
        if !self.may_pickup() {
            return None;
        }

        let item_count = self.with_item(steel_registry::item_stack::ItemStack::count);

        // If modification not allowed (e.g., result slots), must take full stack
        if !self.allow_modification() && max_amount < item_count {
            return None;
        }

        let take_amount = amount.min(max_amount);
        let result = self.remove(take_amount);
        if result.is_empty() {
            return None;
        }

        Some(result)
    }

    /// Called when an item is taken from this slot.
    /// Returns any remainder items that couldn't be placed back (e.g., crafting remainders).
    fn on_take(&self, _stack: &ItemStack) -> Option<ItemStack> {
        None
    }

    /// Marks the slot's container as changed.
    fn set_changed(&self);

    /// Returns the container slot index.
    fn get_container_slot(&self) -> usize;

    /// Returns true if this is a "fake" slot (like crafting result).
    /// Fake slots don't persist items and are virtual views.
    fn is_fake(&self) -> bool {
        false
    }
}

/// A normal slot that references a container and index.
pub struct NormalSlot {
    container: SyncContainer,
    index: usize,
}

impl NormalSlot {
    /// Creates a new normal slot.
    pub fn new(container: SyncContainer, index: usize) -> Self {
        Self { container, index }
    }
}

impl Slot for NormalSlot {
    fn with_item<R>(&self, f: impl FnOnce(&ItemStack) -> R) -> R {
        self.container.lock().with_item(self.index, f)
    }

    fn with_item_mut<R>(&self, f: impl FnOnce(&mut ItemStack) -> R) -> R {
        self.container.lock().with_item_mut(self.index, f)
    }

    fn set_item(&self, stack: ItemStack) {
        self.container.lock().set_item(self.index, stack);
    }

    fn set_changed(&self) {
        self.container.lock().set_changed();
    }

    fn get_container_slot(&self) -> usize {
        self.index
    }
}

/// An armor slot that only accepts items equippable in the corresponding slot.
///
/// Based on Java's `ArmorSlot` class.
pub struct ArmorSlot {
    container: SyncContainer,
    index: usize,
    /// The equipment slot this armor slot accepts.
    slot: EquippableSlot,
}

impl ArmorSlot {
    /// Creates a new armor slot.
    pub fn new(container: SyncContainer, index: usize, slot: EquippableSlot) -> Self {
        Self {
            container,
            index,
            slot,
        }
    }

    /// Returns the equipment slot this armor slot accepts.
    #[must_use]
    pub fn equipment_slot(&self) -> EquippableSlot {
        self.slot
    }
}

impl Slot for ArmorSlot {
    fn with_item<R>(&self, f: impl FnOnce(&ItemStack) -> R) -> R {
        self.container.lock().with_item(self.index, f)
    }

    fn with_item_mut<R>(&self, f: impl FnOnce(&mut ItemStack) -> R) -> R {
        self.container.lock().with_item_mut(self.index, f)
    }

    fn set_item(&self, stack: ItemStack) {
        self.container.lock().set_item(self.index, stack);
    }

    fn may_place(&self, stack: &ItemStack) -> bool {
        stack.is_equippable_in_slot(self.slot)
    }

    fn get_max_stack_size(&self) -> i32 {
        1
    }

    fn set_changed(&self) {
        self.container.lock().set_changed();
    }

    fn get_container_slot(&self) -> usize {
        self.index
    }
}

/// A slot in a crafting grid.
///
/// This slot holds items placed in the crafting grid and triggers
/// recipe recalculation when changed.
pub struct CraftingGridSlot {
    container: SyncCraftingContainer,
    result_container: SyncResultContainer,
    index: usize,
}

impl CraftingGridSlot {
    /// Creates a new crafting grid slot.
    pub fn new(
        container: SyncCraftingContainer,
        result_container: SyncResultContainer,
        index: usize,
    ) -> Self {
        Self {
            container,
            result_container,
            index,
        }
    }

    /// Updates the crafting result based on current grid contents.
    fn update_result(&self) {
        let crafting = self.container.lock();
        let mut result = self.result_container.lock();
        recipe_manager::slot_changed_crafting_grid(&crafting, &mut *result, true);
    }
}

impl Slot for CraftingGridSlot {
    fn with_item<R>(&self, f: impl FnOnce(&ItemStack) -> R) -> R {
        self.container.lock().with_item(self.index, f)
    }

    fn with_item_mut<R>(&self, f: impl FnOnce(&mut ItemStack) -> R) -> R {
        self.container.lock().with_item_mut(self.index, f)
    }

    fn set_item(&self, stack: ItemStack) {
        self.container.lock().set_item(self.index, stack);
        // Update crafting result when grid contents change
        self.update_result();
    }

    fn set_changed(&self) {
        self.container.lock().set_changed();
        // Update crafting result when grid contents change
        self.update_result();
    }

    fn get_container_slot(&self) -> usize {
        self.index
    }
}

/// A slot that displays the crafting result.
///
/// This slot shows what can be crafted from the current grid contents.
/// When an item is taken from this slot, it consumes ingredients from the grid
/// and handles crafting remainders (e.g., buckets from milk buckets).
pub struct CraftingResultSlot {
    result_container: SyncResultContainer,
    crafting_container: SyncCraftingContainer,
}

impl CraftingResultSlot {
    /// Creates a new crafting result slot.
    pub fn new(
        result_container: SyncResultContainer,
        crafting_container: SyncCraftingContainer,
    ) -> Self {
        Self {
            result_container,
            crafting_container,
        }
    }

    /// Returns a reference to the crafting container.
    #[must_use]
    pub fn crafting_container(&self) -> &SyncCraftingContainer {
        &self.crafting_container
    }
}

impl Slot for CraftingResultSlot {
    fn with_item<R>(&self, f: impl FnOnce(&ItemStack) -> R) -> R {
        self.result_container.lock().with_item(0, f)
    }

    fn with_item_mut<R>(&self, f: impl FnOnce(&mut ItemStack) -> R) -> R {
        self.result_container.lock().with_item_mut(0, f)
    }

    fn set_item(&self, stack: ItemStack) {
        self.result_container.lock().set_item(0, stack);
    }

    /// Cannot place items directly in the result slot.
    fn may_place(&self, _stack: &ItemStack) -> bool {
        false
    }

    /// Removes items from the crafting result slot.
    ///
    /// Unlike normal slots, this **always takes the entire stack** regardless
    /// of the `amount` parameter. This matches Java's behavior where
    /// `ResultContainer.removeItem()` ignores the count and takes everything.
    ///
    /// This ensures right-clicking on crafting results takes the full item.
    fn remove(&self, _amount: i32) -> ItemStack {
        self.with_item_mut(std::mem::take)
    }

    fn set_changed(&self) {
        self.result_container.lock().set_changed();
    }

    fn get_container_slot(&self) -> usize {
        0
    }

    /// Called when an item is taken from the result slot.
    /// This consumes ingredients, handles remainders, and updates the result.
    ///
    /// Returns any remainder items that couldn't be placed in the crafting grid
    /// (these should be added to the player's inventory or dropped).
    fn on_take(&self, _stack: &ItemStack) -> Option<ItemStack> {
        let mut remainder_overflow: Option<ItemStack> = None;

        // Consume one of each ingredient in the crafting grid
        let mut crafting = self.crafting_container.lock();
        for i in 0..crafting.get_container_size() {
            crafting.with_item_mut(i, |item| {
                if !item.is_empty() {
                    // Get the remainder before consuming
                    let remainder = item.item().get_crafting_remainder();

                    // Consume one item
                    if item.count() > 1 {
                        item.set_count(item.count() - 1);
                    } else {
                        *item = ItemStack::empty();
                    }

                    // Handle remainder placement
                    if !remainder.is_empty() {
                        if item.is_empty() {
                            // Slot is empty, place remainder there
                            *item = remainder;
                        } else if ItemStack::is_same_item_same_components(item, &remainder) {
                            // Same item type, try to stack
                            let new_count = item.count() + remainder.count();
                            if new_count <= item.max_stack_size() {
                                item.set_count(new_count);
                            } else {
                                // Overflow - need to return to player
                                let overflow_count = new_count - item.max_stack_size();
                                item.set_count(item.max_stack_size());
                                let mut overflow = remainder.clone();
                                overflow.set_count(overflow_count);
                                // Accumulate overflow
                                if let Some(ref mut existing) = remainder_overflow {
                                    existing.grow(overflow_count);
                                } else {
                                    remainder_overflow = Some(overflow);
                                }
                            }
                        } else {
                            // Different item type - need to return to player
                            if let Some(ref mut existing) = remainder_overflow {
                                if ItemStack::is_same_item_same_components(existing, &remainder) {
                                    existing.grow(remainder.count());
                                } else {
                                    // Multiple different remainders - just add count
                                    // In practice this shouldn't happen often
                                    existing.grow(remainder.count());
                                }
                            } else {
                                remainder_overflow = Some(remainder);
                            }
                        }
                    }
                }
            });
        }

        // Update the crafting result based on remaining ingredients
        // This mimics Java's slotsChanged() callback from TransientCraftingContainer
        let mut result = self.result_container.lock();
        recipe_manager::slot_changed_crafting_grid(&crafting, &mut *result, true);

        remainder_overflow
    }

    /// Crafting result slots are "fake" - they don't persist items.
    fn is_fake(&self) -> bool {
        true
    }
}

#[enum_dispatch(Slot)]
pub enum SlotType {
    Normal(NormalSlot),
    Armor(ArmorSlot),
    CraftingGrid(CraftingGridSlot),
    CraftingResult(CraftingResultSlot),
}
