//! Recipe matching and crafting grid management.
//!
//! This module provides functions to match crafting grid contents against
//! registered recipes and update the result slot accordingly.

use steel_registry::{
    REGISTRY,
    item_stack::ItemStack,
    recipe::{CraftingInput, CraftingRecipe},
};

use super::container::Container;
use super::crafting::CraftingContainer;

/// Called when a slot changes in the crafting grid.
/// Updates the result container with the matching recipe result.
///
/// # Arguments
/// * `crafting` - The crafting container to check
/// * `result` - The result container to update
/// * `is_2x2` - Whether this is a 2x2 crafting grid (player inventory)
pub fn slot_changed_crafting_grid<R: Container>(
    crafting: &CraftingContainer,
    result: &mut R,
    is_2x2: bool,
) {
    let input = create_crafting_input(crafting);

    let recipe = if is_2x2 {
        REGISTRY.recipes.find_crafting_recipe_2x2(&input)
    } else {
        REGISTRY.recipes.find_crafting_recipe(&input)
    };

    let result_stack = match recipe {
        Some(r) => r.assemble(&input),
        None => ItemStack::empty(),
    };

    result.set_item(0, result_stack);
}

/// Creates a `CraftingInput` from a `CraftingContainer`.
fn create_crafting_input(crafting: &CraftingContainer) -> CraftingInput {
    let items: Vec<ItemStack> = crafting.items().to_vec();
    CraftingInput::new(crafting.width(), crafting.height(), items)
}

/// Finds a matching recipe for the given crafting container.
///
/// # Arguments
/// * `crafting` - The crafting container to check
/// * `is_2x2` - Whether this is a 2x2 crafting grid
///
/// # Returns
/// The matching recipe, or None if no recipe matches.
#[must_use]
pub fn find_recipe(
    crafting: &CraftingContainer,
    is_2x2: bool,
) -> Option<&'static dyn CraftingRecipe> {
    let input = create_crafting_input(crafting);

    if is_2x2 {
        REGISTRY.recipes.find_crafting_recipe_2x2(&input)
    } else {
        REGISTRY.recipes.find_crafting_recipe(&input)
    }
}
