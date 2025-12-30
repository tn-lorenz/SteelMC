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
    pub fn parse_json(s: &str) -> Self {
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
    /// An empty crafting input.
    pub const EMPTY: CraftingInput = CraftingInput {
        width: 0,
        height: 0,
        items: Vec::new(),
    };

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

    /// Creates a positioned crafting input from this grid.
    ///
    /// The positioned input contains a trimmed version of the grid (only the
    /// bounding box of non-empty items) along with the offset from the original
    /// grid origin.
    #[must_use]
    pub fn as_positioned(&self) -> PositionedCraftingInput {
        if self.width == 0 || self.height == 0 {
            return PositionedCraftingInput::EMPTY;
        }

        let Some((min_x, min_y, max_x, max_y)) = self.get_bounds() else {
            return PositionedCraftingInput::EMPTY;
        };

        let new_width = max_x - min_x + 1;
        let new_height = max_y - min_y + 1;

        // If the bounds match the original grid, no need to create a new input
        if new_width == self.width && new_height == self.height {
            return PositionedCraftingInput {
                input: self.clone(),
                left: min_x,
                top: min_y,
            };
        }

        // Create a trimmed input containing only the bounding box
        let mut new_items = Vec::with_capacity(new_width * new_height);
        for y in 0..new_height {
            for x in 0..new_width {
                let index = (x + min_x) + (y + min_y) * self.width;
                new_items.push(self.items[index].clone());
            }
        }

        PositionedCraftingInput {
            input: CraftingInput::new(new_width, new_height, new_items),
            left: min_x,
            top: min_y,
        }
    }
}

/// A crafting input with position information.
///
/// This represents a trimmed crafting grid (containing only the bounding box
/// of non-empty items) along with the offset from the original grid origin.
/// This is used when consuming ingredients to correctly map recipe slots back
/// to the original crafting grid slots.
#[derive(Debug, Clone)]
pub struct PositionedCraftingInput {
    /// The trimmed crafting input.
    pub input: CraftingInput,
    /// The X offset from the original grid origin.
    pub left: usize,
    /// The Y offset from the original grid origin.
    pub top: usize,
}

impl PositionedCraftingInput {
    /// An empty positioned crafting input.
    pub const EMPTY: PositionedCraftingInput = PositionedCraftingInput {
        input: CraftingInput::EMPTY,
        left: 0,
        top: 0,
    };

    /// Converts a position in the trimmed input back to the original grid slot index.
    ///
    /// # Arguments
    /// * `x` - X position in the trimmed input (0 to input.width-1)
    /// * `y` - Y position in the trimmed input (0 to input.height-1)
    /// * `grid_width` - Width of the original crafting grid
    ///
    /// # Returns
    /// The slot index in the original crafting grid.
    #[must_use]
    pub fn to_grid_slot(&self, x: usize, y: usize, grid_width: usize) -> usize {
        (x + self.left) + (y + self.top) * grid_width
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
