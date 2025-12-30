//! Crafting containers and related types.
//!
//! This module provides the `CraftingContainer` for the crafting grid,
//! `ResultContainer` for crafting output, and `CraftingInput` for recipe matching.

use steel_registry::item_stack::ItemStack;

use crate::inventory::container::Container;

/// A container for crafting grid items.
///
/// This container holds items in a crafting grid pattern (2x2 for player inventory,
/// 3x3 for crafting table). It notifies a callback when the grid contents change,
/// allowing the crafting result to be recalculated.
pub struct CraftingContainer {
    width: usize,
    height: usize,
    items: Vec<ItemStack>,
}

impl CraftingContainer {
    /// Creates a new crafting container with the given dimensions.
    #[must_use]
    pub fn new(width: usize, height: usize) -> Self {
        let size = width * height;
        Self {
            width,
            height,
            items: vec![ItemStack::empty(); size],
        }
    }

    /// Returns the width of the crafting grid.
    #[must_use]
    pub fn width(&self) -> usize {
        self.width
    }

    /// Returns the height of the crafting grid.
    #[must_use]
    pub fn height(&self) -> usize {
        self.height
    }

    /// Creates a `CraftingInput` representing the current state of the grid.
    /// This is used for recipe matching.
    #[must_use]
    pub fn as_input(&self) -> CraftingInput {
        CraftingInput {
            width: self.width,
            height: self.height,
            items: self.items.clone(),
        }
    }

    /// Returns a reference to the items in the grid.
    #[must_use]
    pub fn items(&self) -> &[ItemStack] {
        &self.items
    }
}

impl Container for CraftingContainer {
    fn get_container_size(&self) -> usize {
        self.items.len()
    }

    fn with_item<R>(&self, slot: usize, f: impl FnOnce(&ItemStack) -> R) -> R {
        f(&self.items[slot])
    }

    fn with_item_mut<R>(&mut self, slot: usize, f: impl FnOnce(&mut ItemStack) -> R) -> R {
        f(&mut self.items[slot])
    }

    fn set_item(&mut self, slot: usize, stack: ItemStack) {
        self.items[slot] = stack;
    }

    fn set_changed(&mut self) {
        // Crafting container doesn't track dirty state itself;
        // the menu handles recipe recalculation on changes.
    }
}

/// A simple container for holding a single crafting result.
pub struct ResultContainer {
    result: ItemStack,
}

impl ResultContainer {
    /// Creates a new empty result container.
    #[must_use]
    pub fn new() -> Self {
        Self {
            result: ItemStack::empty(),
        }
    }
}

impl Default for ResultContainer {
    fn default() -> Self {
        Self::new()
    }
}

impl Container for ResultContainer {
    fn get_container_size(&self) -> usize {
        1
    }

    fn with_item<R>(&self, _slot: usize, f: impl FnOnce(&ItemStack) -> R) -> R {
        f(&self.result)
    }

    fn with_item_mut<R>(&mut self, _slot: usize, f: impl FnOnce(&mut ItemStack) -> R) -> R {
        f(&mut self.result)
    }

    fn set_item(&mut self, _slot: usize, stack: ItemStack) {
        self.result = stack;
    }

    /// Removes items from the result container.
    ///
    /// Unlike normal containers, this **always takes the entire stack**
    /// regardless of the `count` parameter. This matches Java's
    /// `ResultContainer.removeItem()` behavior which uses `takeItem()`.
    ///
    /// This ensures that right-clicking on a crafting result takes the
    /// full crafted item, not half of it.
    fn remove_item(&mut self, _slot: usize, _count: i32) -> ItemStack {
        std::mem::take(&mut self.result)
    }

    fn set_changed(&mut self) {
        // Result container doesn't track dirty state.
    }
}

/// An immutable snapshot of a crafting grid for recipe matching.
///
/// This is created from a `CraftingContainer` and used to test recipes
/// without holding a borrow on the container.
#[derive(Clone)]
pub struct CraftingInput {
    width: usize,
    height: usize,
    items: Vec<ItemStack>,
}

impl CraftingInput {
    /// Returns the width of the crafting grid.
    #[must_use]
    pub fn width(&self) -> usize {
        self.width
    }

    /// Returns the height of the crafting grid.
    #[must_use]
    pub fn height(&self) -> usize {
        self.height
    }

    /// Returns the item at the given position.
    #[must_use]
    pub fn get_item(&self, x: usize, y: usize) -> &ItemStack {
        &self.items[y * self.width + x]
    }

    /// Returns the total number of slots.
    #[must_use]
    pub fn size(&self) -> usize {
        self.items.len()
    }

    /// Returns a reference to all items.
    #[must_use]
    pub fn items(&self) -> &[ItemStack] {
        &self.items
    }

    /// Returns true if all slots are empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.items.iter().all(ItemStack::is_empty)
    }

    /// Returns the number of non-empty slots.
    #[must_use]
    pub fn ingredient_count(&self) -> usize {
        self.items.iter().filter(|s| !s.is_empty()).count()
    }

    /// Computes the bounding box of non-empty items.
    /// Returns (`start_x`, `start_y`, width, height) or None if empty.
    #[must_use]
    pub fn bounding_box(&self) -> Option<(usize, usize, usize, usize)> {
        let mut min_x = self.width;
        let mut max_x = 0;
        let mut min_y = self.height;
        let mut max_y = 0;

        for y in 0..self.height {
            for x in 0..self.width {
                if !self.get_item(x, y).is_empty() {
                    min_x = min_x.min(x);
                    max_x = max_x.max(x);
                    min_y = min_y.min(y);
                    max_y = max_y.max(y);
                }
            }
        }

        if min_x > max_x || min_y > max_y {
            None
        } else {
            Some((min_x, min_y, max_x - min_x + 1, max_y - min_y + 1))
        }
    }
}
