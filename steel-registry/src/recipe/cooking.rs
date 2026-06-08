//! Cooking recipe types.

use steel_utils::Identifier;

use crate::{item_stack::ItemStack, items::ItemRef};

use super::{Ingredient, RecipeResult};

/// A furnace smelting recipe.
#[derive(Debug)]
pub struct SmeltingRecipe {
    pub id: Identifier,
    pub ingredient: Ingredient,
    pub result: RecipeResult,
    pub experience: f32,
    pub cooking_time: i32,
}

impl SmeltingRecipe {
    /// Returns whether this smelting recipe accepts `input`.
    #[must_use]
    pub fn matches(&self, input: &ItemStack) -> bool {
        self.ingredient.test(input)
    }

    /// Returns the result item type used by loot-table furnace smelting.
    #[must_use]
    pub fn result_item(&self) -> ItemRef {
        self.result.item
    }
}
