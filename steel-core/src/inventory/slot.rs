//! Slot abstraction for inventory access.

use enum_dispatch::enum_dispatch;
use steel_registry::item_stack::ItemStack;

use crate::inventory::SyncContainer;
use crate::inventory::container::Container;

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

#[enum_dispatch(Slot)]
pub enum SlotType {
    Normal(NormalSlot),
}
