//! Crafting recipe types (shaped and shapeless).

use steel_utils::Identifier;

use crate::{item_stack::ItemStack, items::ItemRef};

use super::ingredient::Ingredient;

/// Category for crafting recipes (used by recipe book).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CraftingCategory {
    Building,
    Redstone,
    Equipment,
    Misc,
}

impl CraftingCategory {
    /// Parses a category from a JSON string.
    #[must_use]
    pub fn from_str(s: &str) -> Self {
        match s {
            "building" => Self::Building,
            "redstone" => Self::Redstone,
            "equipment" => Self::Equipment,
            _ => Self::Misc,
        }
    }
}

/// The result of a crafting recipe.
#[derive(Debug, Clone)]
pub struct RecipeResult {
    pub item: ItemRef,
    pub count: i32,
}

impl RecipeResult {
    /// Creates an `ItemStack` from this result.
    #[must_use]
    pub fn to_item_stack(&self) -> ItemStack {
        ItemStack::with_count(self.item, self.count)
    }
}

/// A shaped crafting recipe with a specific pattern.
#[derive(Debug)]
pub struct ShapedRecipe {
    pub id: Identifier,
    pub category: CraftingCategory,
    pub width: usize,
    pub height: usize,
    /// Pattern ingredients in row-major order (width * height).
    pub pattern: Vec<Ingredient>,
    pub result: RecipeResult,
    pub show_notification: bool,
}

/// A shapeless crafting recipe where ingredient order doesn't matter.
#[derive(Debug)]
pub struct ShapelessRecipe {
    pub id: Identifier,
    pub category: CraftingCategory,
    pub ingredients: Vec<Ingredient>,
    pub result: RecipeResult,
}

/// Trait for crafting recipes.
pub trait CraftingRecipe: std::fmt::Debug + Send + Sync {
    /// Returns the recipe identifier.
    fn id(&self) -> &Identifier;

    /// Returns the recipe category.
    fn category(&self) -> CraftingCategory;

    /// Returns the result of this recipe.
    fn result(&self) -> &RecipeResult;

    /// Tests if the crafting input matches this recipe.
    fn matches(&self, input: &CraftingInput) -> bool;

    /// Assembles the result item stack.
    fn assemble(&self, input: &CraftingInput) -> ItemStack;

    /// Gets the remaining items after crafting (e.g., empty buckets).
    fn get_remaining_items(&self, input: &CraftingInput) -> Vec<ItemStack>;

    /// Returns true if this recipe fits in a 2x2 grid.
    fn fits_in_2x2(&self) -> bool;
}

/// Represents the current state of a crafting grid.
#[derive(Debug, Clone)]
pub struct CraftingInput {
    pub width: usize,
    pub height: usize,
    /// Items in row-major order (width * height).
    pub items: Vec<ItemStack>,
}

impl CraftingInput {
    /// Creates a new crafting input.
    #[must_use]
    pub fn new(width: usize, height: usize, items: Vec<ItemStack>) -> Self {
        debug_assert_eq!(items.len(), width * height);
        Self {
            width,
            height,
            items,
        }
    }

    /// Creates a 2x2 crafting input.
    #[must_use]
    pub fn new_2x2(slots: [ItemStack; 4]) -> Self {
        Self::new(2, 2, slots.to_vec())
    }

    /// Creates a 3x3 crafting input.
    #[must_use]
    pub fn new_3x3(slots: [ItemStack; 9]) -> Self {
        Self::new(3, 3, slots.to_vec())
    }

    /// Gets the item at the specified position.
    #[must_use]
    pub fn get(&self, x: usize, y: usize) -> &ItemStack {
        &self.items[y * self.width + x]
    }

    /// Returns the bounding box of non-empty items (min_x, min_y, max_x, max_y).
    /// Returns None if the grid is empty.
    #[must_use]
    pub fn get_bounds(&self) -> Option<(usize, usize, usize, usize)> {
        let mut min_x = self.width;
        let mut min_y = self.height;
        let mut max_x = 0;
        let mut max_y = 0;
        let mut found = false;

        for y in 0..self.height {
            for x in 0..self.width {
                if !self.get(x, y).is_empty() {
                    min_x = min_x.min(x);
                    min_y = min_y.min(y);
                    max_x = max_x.max(x);
                    max_y = max_y.max(y);
                    found = true;
                }
            }
        }

        if found {
            Some((min_x, min_y, max_x, max_y))
        } else {
            None
        }
    }

    /// Returns the dimensions of the bounding box of non-empty items.
    #[must_use]
    pub fn get_content_size(&self) -> (usize, usize) {
        match self.get_bounds() {
            Some((min_x, min_y, max_x, max_y)) => (max_x - min_x + 1, max_y - min_y + 1),
            None => (0, 0),
        }
    }

    /// Returns the number of non-empty items.
    #[must_use]
    pub fn count_non_empty(&self) -> usize {
        self.items.iter().filter(|s| !s.is_empty()).count()
    }
}

impl ShapedRecipe {
    /// Tests if the crafting input matches this recipe at the given offset.
    fn matches_at(
        &self,
        input: &CraftingInput,
        offset_x: usize,
        offset_y: usize,
        mirrored: bool,
    ) -> bool {
        for y in 0..self.height {
            for x in 0..self.width {
                let pattern_x = if mirrored { self.width - 1 - x } else { x };
                let ingredient = &self.pattern[y * self.width + pattern_x];
                let input_item = input.get(offset_x + x, offset_y + y);

                if !ingredient.test(input_item) {
                    return false;
                }
            }
        }
        true
    }
}

impl CraftingRecipe for ShapedRecipe {
    fn id(&self) -> &Identifier {
        &self.id
    }

    fn category(&self) -> CraftingCategory {
        self.category
    }

    fn result(&self) -> &RecipeResult {
        &self.result
    }

    fn matches(&self, input: &CraftingInput) -> bool {
        // Get the bounding box of non-empty items in input
        let Some((min_x, min_y, max_x, max_y)) = input.get_bounds() else {
            // Empty grid doesn't match any shaped recipe
            return false;
        };

        let input_width = max_x - min_x + 1;
        let input_height = max_y - min_y + 1;

        // Check if dimensions match
        if input_width != self.width || input_height != self.height {
            return false;
        }

        // Try normal orientation
        if self.matches_at(input, min_x, min_y, false) {
            return true;
        }

        // Try mirrored
        if self.matches_at(input, min_x, min_y, true) {
            return true;
        }

        false
    }

    fn assemble(&self, _input: &CraftingInput) -> ItemStack {
        self.result.to_item_stack()
    }

    fn get_remaining_items(&self, input: &CraftingInput) -> Vec<ItemStack> {
        input
            .items
            .iter()
            .map(|stack| {
                if stack.is_empty() {
                    ItemStack::empty()
                } else {
                    stack.item.get_crafting_remainder()
                }
            })
            .collect()
    }

    fn fits_in_2x2(&self) -> bool {
        self.width <= 2 && self.height <= 2
    }
}

impl CraftingRecipe for ShapelessRecipe {
    fn id(&self) -> &Identifier {
        &self.id
    }

    fn category(&self) -> CraftingCategory {
        self.category
    }

    fn result(&self) -> &RecipeResult {
        &self.result
    }

    fn matches(&self, input: &CraftingInput) -> bool {
        let non_empty: Vec<&ItemStack> = input.items.iter().filter(|s| !s.is_empty()).collect();

        // Must have same number of items as ingredients
        if non_empty.len() != self.ingredients.len() {
            return false;
        }

        // Try to match each ingredient to an input item
        let mut used = vec![false; non_empty.len()];

        for ingredient in &self.ingredients {
            let mut found = false;
            for (i, item) in non_empty.iter().enumerate() {
                if !used[i] && ingredient.test(item) {
                    used[i] = true;
                    found = true;
                    break;
                }
            }
            if !found {
                return false;
            }
        }

        true
    }

    fn assemble(&self, _input: &CraftingInput) -> ItemStack {
        self.result.to_item_stack()
    }

    fn get_remaining_items(&self, input: &CraftingInput) -> Vec<ItemStack> {
        input
            .items
            .iter()
            .map(|stack| {
                if stack.is_empty() {
                    ItemStack::empty()
                } else {
                    stack.item.get_crafting_remainder()
                }
            })
            .collect()
    }

    fn fits_in_2x2(&self) -> bool {
        self.ingredients.len() <= 4
    }
}
