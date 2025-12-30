//! Slot abstraction for inventory access.

use std::sync::Arc;

use enum_dispatch::enum_dispatch;
use steel_registry::data_components::vanilla_components::EquippableSlot;
use steel_registry::item_stack::ItemStack;
use steel_utils::locks::SyncMutex;

use crate::inventory::SyncContainer;
use crate::inventory::container::Container;
use crate::inventory::crafting::{CraftingContainer, ResultContainer};

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

    /// Called when an item is taken from this slot.
    fn on_take(&self, _stack: &ItemStack) {}

    /// Marks the slot's container as changed.
    fn set_changed(&self);

    /// Returns the container slot index.
    fn get_container_slot(&self) -> usize;
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
    index: usize,
}

impl CraftingGridSlot {
    /// Creates a new crafting grid slot.
    pub fn new(container: SyncCraftingContainer, index: usize) -> Self {
        Self { container, index }
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
    }

    fn set_changed(&self) {
        self.container.lock().set_changed();
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

    fn set_changed(&self) {
        self.result_container.lock().set_changed();
    }

    fn get_container_slot(&self) -> usize {
        0
    }

    /// Called when an item is taken from the result slot.
    /// This consumes ingredients and handles remainders.
    fn on_take(&self, _stack: &ItemStack) {
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

                    // If there's a remainder and the slot is now empty, place it
                    if !remainder.is_empty() && item.is_empty() {
                        *item = remainder;
                    }
                    // TODO: If slot isn't empty but has remainder, add to player inventory
                }
            });
        }
    }
}

#[enum_dispatch(Slot)]
pub enum SlotType {
    Normal(NormalSlot),
    Armor(ArmorSlot),
    CraftingGrid(CraftingGridSlot),
    CraftingResult(CraftingResultSlot),
}
