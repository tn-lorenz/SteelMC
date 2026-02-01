//! Standing and wall block item behavior implementation.
//!
//! This handles items like torches that place different block variants
//! depending on whether they're placed on top of a block (standing) or
//! on the side of a block (wall).

use steel_registry::REGISTRY;
use steel_registry::blocks::BlockRef;
use steel_registry::blocks::block_state_ext::BlockStateExt;
use steel_registry::blocks::properties::Direction;
use steel_utils::types::UpdateFlags;

use crate::behavior::context::{BlockPlaceContext, InteractionResult, UseOnContext};
use crate::behavior::{BLOCK_BEHAVIORS, ItemBehavior};

/// Behavior for items that place either a standing or wall variant of a block.
///
/// Used for torches (torch/wall_torch), soul torches, copper torches, etc.
/// When placed on top of a block, places the standing variant.
/// When placed on the side of a block, places the wall variant.
pub struct StandingAndWallBlockItem {
    /// The block to place when on top of another block (e.g., `torch`).
    pub standing_block: BlockRef,
    /// The block to place when on the side of another block (e.g., `wall_torch`).
    pub wall_block: BlockRef,
}

impl StandingAndWallBlockItem {
    /// Creates a new standing and wall block item behavior.
    #[must_use]
    pub const fn new(standing_block: BlockRef, wall_block: BlockRef) -> Self {
        Self {
            standing_block,
            wall_block,
        }
    }

    /// Determines which block variant to use based on placement context.
    /// Returns the appropriate block and its placement state.
    fn get_placement_block_and_state(
        &self,
        place_context: &BlockPlaceContext<'_>,
    ) -> Option<(BlockRef, steel_utils::BlockStateId)> {
        let block_behaviors = &*BLOCK_BEHAVIORS;

        // If clicking on top of a block (facing up), try standing variant first
        if place_context.clicked_face == Direction::Up {
            let behavior = block_behaviors.get_behavior(self.standing_block);
            if let Some(state) = behavior.get_state_for_placement(place_context) {
                return Some((self.standing_block, state));
            }
        }

        // If clicking on the side of a block, or standing failed, try wall variant
        if place_context.clicked_face.is_horizontal() || place_context.clicked_face == Direction::Up
        {
            let behavior = block_behaviors.get_behavior(self.wall_block);
            if let Some(state) = behavior.get_state_for_placement(place_context) {
                return Some((self.wall_block, state));
            }
        }

        // If clicking on the bottom face, or wall variant also failed, try standing as fallback
        let behavior = block_behaviors.get_behavior(self.standing_block);
        if let Some(state) = behavior.get_state_for_placement(place_context) {
            return Some((self.standing_block, state));
        }

        None
    }
}

impl ItemBehavior for StandingAndWallBlockItem {
    fn use_on(&self, context: &mut UseOnContext) -> InteractionResult {
        let clicked_pos = context.hit_result.block_pos;
        let clicked_state = context.world.get_block_state(&clicked_pos);

        // Get the clicked block to check if it's replaceable
        let clicked_block = REGISTRY.blocks.by_state_id(clicked_state);
        let clicked_replaceable = clicked_block.is_some_and(|b| b.config.replaceable);

        // Determine placement position: replace clicked block if replaceable,
        // otherwise place adjacent to the clicked face
        let (place_pos, replace_clicked) = if clicked_replaceable {
            (clicked_pos, true)
        } else {
            (context.hit_result.direction.relative(&clicked_pos), false)
        };

        // Check if placement position is within world bounds
        if !context.world.is_in_valid_bounds(&place_pos) {
            return InteractionResult::Fail;
        }

        // Check if the placement position already has a non-replaceable block
        let existing_state = context.world.get_block_state(&place_pos);
        let existing_block = REGISTRY.blocks.by_state_id(existing_state);
        let existing_replaceable = existing_block.is_some_and(|b| b.config.replaceable);

        if !existing_replaceable {
            return InteractionResult::Fail;
        }

        // Get player rotation for placement context
        let (yaw, pitch) = context.player.rotation.load();

        let place_context = BlockPlaceContext {
            clicked_pos,
            clicked_face: context.hit_result.direction,
            click_location: context.hit_result.location,
            inside: context.hit_result.inside,
            relative_pos: place_pos,
            replace_clicked,
            horizontal_direction: Direction::from_yaw(yaw),
            rotation: yaw,
            pitch,
            world: context.world,
        };

        // Get the appropriate block variant and state
        let Some((block, new_state)) = self.get_placement_block_and_state(&place_context) else {
            return InteractionResult::Fail;
        };

        // Check if the block placement would intersect with any entity
        let collision_shape = new_state.get_collision_shape();
        if !context.world.is_unobstructed(collision_shape, &place_pos) {
            return InteractionResult::Fail;
        }

        // Place the block
        if !context
            .world
            .set_block(place_pos, new_state, UpdateFlags::UPDATE_ALL_IMMEDIATE)
        {
            return InteractionResult::Fail;
        }

        // Play place sound (exclude the placing player, they hear it client-side)
        let sound_type = &block.config.sound_type;
        context.world.play_block_sound(
            sound_type.place_sound,
            place_pos,
            sound_type.volume,
            sound_type.pitch,
            Some(context.player.id),
        );

        // Consume one item from the stack
        context.item_stack.shrink(1);

        InteractionResult::Success
    }
}
