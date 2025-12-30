//! Container trait for anything that holds items.

use enum_dispatch::enum_dispatch;
use steel_registry::item_stack::ItemStack;

use crate::player::player_inventory::PlayerInventory;

/// Default distance buffer for container interaction range checks.
pub const DEFAULT_DISTANCE_BUFFER: f32 = 4.0;

/// Something that contains items.
/// I also use container interchangeably with inventory as they mean approximately the same thing.
/// But inventory could also refer to the player's inventory.
/// Example: `PlayerInventory`, Chest, Temporary Crafting Table
#[enum_dispatch]
pub trait Container {
    /// Returns the number of slots in this container.
    fn get_container_size(&self) -> usize;

    /// Returns true if all slots in this container are empty.
    fn is_empty(&self) -> bool {
        for i in 0..self.get_container_size() {
            let empty = self.with_item(i, steel_registry::item_stack::ItemStack::is_empty);
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
        self.with_item_mut(slot, std::mem::take)
    }

    /// Returns the maximum stack size for this container.
    fn get_max_stack_size(&self) -> i32 {
        99
    }

    /// Returns the maximum stack size for a specific item in this container.
    ///
    /// Takes the minimum of the container's max stack size and the item's max stack size.
    /// Based on Java's `Container.getMaxStackSize(ItemStack)`.
    fn get_max_stack_size_for_item(&self, item: &ItemStack) -> i32 {
        self.get_max_stack_size().min(item.max_stack_size())
    }

    /// Marks this container as changed (dirty) for saving/syncing.
    fn set_changed(&mut self);

    /// Returns true if the player can still interact with this container.
    ///
    /// This is used to validate that:
    /// - The container still exists (e.g., chest block hasn't been destroyed)
    /// - The player is within interaction range
    /// - Any other conditions for valid interaction
    ///
    /// The default implementation always returns true (e.g., for player inventory).
    /// Block-based containers should override this to check block existence and distance.
    ///
    /// Based on Java's `Container.stillValid(Player)`.
    fn still_valid(&self) -> bool {
        true
    }

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

    /// Tries to add an item to the container.
    ///
    /// First tries to stack with existing matching items, then tries empty slots.
    /// Returns true if the entire stack was added, false if some or all couldn't fit.
    /// The passed stack is modified to contain any remaining items.
    ///
    /// Based on Java's `Inventory.add(ItemStack)`.
    fn add(&mut self, stack: &mut ItemStack) -> bool {
        if stack.is_empty() {
            return true;
        }

        let size = self.get_container_size();

        // First pass: try to stack with existing items
        if stack.is_stackable() {
            for slot in 0..size {
                if stack.is_empty() {
                    return true;
                }
                let can_stack = self.with_item(slot, |existing| {
                    !existing.is_empty() && ItemStack::is_same_item_same_components(existing, stack)
                });
                if can_stack {
                    let max_size = self.get_max_stack_size_for_item(stack);
                    self.with_item_mut(slot, |existing| {
                        let space = max_size - existing.count();
                        if space > 0 {
                            let to_add = stack.count().min(space);
                            existing.grow(to_add);
                            stack.shrink(to_add);
                        }
                    });
                }
            }
        }

        // Second pass: try empty slots
        for slot in 0..size {
            if stack.is_empty() {
                return true;
            }
            let is_empty = self.with_item(slot, ItemStack::is_empty);
            if is_empty && self.can_place_item(slot, stack) {
                let max_size = self.get_max_stack_size_for_item(stack);
                let to_place = stack.count().min(max_size);
                let mut placed = stack.clone();
                placed.set_count(to_place);
                self.set_item(slot, placed);
                stack.shrink(to_place);
            }
        }

        stack.is_empty()
    }
}

/// Enum of all container types that implement the Container trait.
///
/// This enum uses `enum_dispatch` to efficiently delegate Container trait methods
/// to the appropriate container type implementation.
#[enum_dispatch(Container)]
pub enum ContainerType {
    /// Player inventory container.
    PlayerInventory(PlayerInventory),
}
