//! Container trait for anything that holds items.

use enum_dispatch::enum_dispatch;
use steel_registry::item_stack::ItemStack;

use crate::player::player_inventory::PlayerInventory;

/// Something that contains items.
/// I also use container interchangeably with inventory as they mean approximately the same thing.
/// But inventory could also refer to the player's inventory.
/// Example: PlayerInventory, Chest, Temporary Crafting Table
#[enum_dispatch]
pub trait Container {
    /// Returns the number of slots in this container.
    fn get_container_size(&self) -> usize;

    /// Returns true if all slots in this container are empty.
    fn is_empty(&self) -> bool {
        for i in 0..self.get_container_size() {
            let empty = self.with_item(i, |item| item.is_empty());
            if !empty {
                return false;
            }
        }
        true
    }

    /// Executes a function with a reference to the item in the specified slot.
    fn with_item<R>(&self, slot: usize, f: impl FnOnce(&ItemStack) -> R) -> R;

    /// Executes a function with a mutable reference to the item in the specified slot.
    fn with_item_mut<R>(&mut self, slot: usize, f: impl FnOnce(&mut ItemStack) -> R) -> R;

    /// Sets the item in the specified slot.
    fn set_item(&mut self, slot: usize, stack: ItemStack);

    /// Removes up to `count` items from the specified slot and returns them.
    fn remove_item(&mut self, slot: usize, count: i32) -> ItemStack {
        self.with_item_mut(slot, |item| {
            if item.is_empty() || count <= 0 {
                return ItemStack::empty();
            }

            let take_count = count.min(item.count());
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

    /// Removes the item from the specified slot without triggering updates.
    fn remove_item_no_update(&mut self, slot: usize) -> ItemStack {
        self.with_item_mut(slot, |item| std::mem::take(item))
    }

    /// Returns the maximum stack size for this container.
    fn get_max_stack_size(&self) -> i32 {
        99
    }

    /// Marks this container as changed (dirty) for saving/syncing.
    fn set_changed(&mut self);

    /// Returns true if the specified item can be placed in the specified slot.
    fn can_place_item(&self, _slot: usize, _stack: &ItemStack) -> bool {
        true
    }

    /// Returns true if the specified item can be taken from the specified slot.
    fn can_take_item(&self, _slot: usize, _stack: &ItemStack) -> bool {
        true
    }

    /// Clears all items from this container.
    fn clear_content(&mut self) {
        for i in 0..self.get_container_size() {
            self.set_item(i, ItemStack::empty());
        }
    }
}

#[enum_dispatch(Container)]
pub enum ContainerType {
    PlayerInventory(PlayerInventory),
}
