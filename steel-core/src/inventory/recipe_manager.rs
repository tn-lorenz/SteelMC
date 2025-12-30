//! Recipe matching and crafting grid management.
//!
//! This module provides functions to match crafting grid contents against
//! registered recipes and update the result slot accordingly.

use steel_registry::{
    REGISTRY,
    item_stack::ItemStack,
    recipe::{CraftingRecipe, PositionedCraftingInput},
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
    let input = crafting.as_input();

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
    let input = crafting.as_input();

    if is_2x2 {
        REGISTRY.recipes.find_crafting_recipe_2x2(&input)
    } else {
        REGISTRY.recipes.find_crafting_recipe(&input)
    }
}

/// Gets the remaining items (crafting remainders) for a recipe.
///
/// This queries the recipe for its remaining items, which may include
/// items like empty buckets when using milk buckets in a recipe.
///
/// # Arguments
/// * `crafting` - The crafting container
/// * `is_2x2` - Whether this is a 2x2 crafting grid
///
/// # Returns
/// A vector of remaining items for each slot in the positioned input,
/// along with the positioned input for mapping back to grid slots.
/// Returns None if no recipe matches.
#[must_use]
pub fn get_remaining_items(
    crafting: &CraftingContainer,
    is_2x2: bool,
) -> Option<(Vec<ItemStack>, PositionedCraftingInput)> {
    let positioned = crafting.as_positioned_input();
    let recipe = find_recipe(crafting, is_2x2)?;

    // Get remainders from the recipe using the positioned (trimmed) input
    let remainders = recipe.get_remaining_items(&positioned.input);

    Some((remainders, positioned))
}
