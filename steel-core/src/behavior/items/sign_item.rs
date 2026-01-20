//! Sign item behavior implementation.
//!
//! Places sign blocks and opens the sign editor after placement.
//! Handles both standing signs (on ground) and wall signs (on walls).

use std::cmp::Ordering;

use steel_registry::REGISTRY;
use steel_registry::blocks::BlockRef;
use steel_registry::blocks::block_state_ext::BlockStateExt;
use steel_registry::blocks::properties::Direction;
use steel_utils::BlockPos;
use steel_utils::types::UpdateFlags;

use crate::behavior::context::{BlockPlaceContext, InteractionResult, UseOnContext};
use crate::behavior::{BLOCK_BEHAVIORS, ItemBehavior};
use crate::world::World;

/// Behavior for sign items that place sign blocks and open the editor.
///
/// Sign items can place either standing signs (on ground) or wall signs (on walls),
/// depending on where the player clicks.
pub struct SignItemBehavior {
    /// The standing sign block (placed on ground).
    pub standing_block: BlockRef,
    /// The wall sign block (attached to walls).
    pub wall_block: BlockRef,
}

impl SignItemBehavior {
    /// Creates a new sign item behavior for the given sign blocks.
    #[must_use]
    pub const fn new(standing_block: BlockRef, wall_block: BlockRef) -> Self {
        Self {
            standing_block,
            wall_block,
        }
    }

    /// Tries to get a valid placement state, checking standing sign first if
    /// clicking from above, otherwise trying wall sign.
    fn get_placement_state(
        &self,
        context: &BlockPlaceContext<'_>,
        place_pos: &BlockPos,
    ) -> Option<steel_utils::BlockStateId> {
        let block_behaviors = &*BLOCK_BEHAVIORS;

        // Get nearest looking directions to determine placement preference
        let directions = get_nearest_looking_directions(context.rotation, context.clicked_face);

        for direction in directions {
            // Skip the opposite of down (which is up) - signs don't attach to ceilings
            // (unless they're hanging signs, which are handled separately)
            if direction == Direction::Up {
                continue;
            }

            let (block, can_place) = if direction == Direction::Down {
                // Try standing sign (placed on top of a block)
                let behavior = block_behaviors.get_behavior(self.standing_block);
                // Check if we can get a valid placement state
                behavior.get_state_for_placement(context)?;
                (
                    self.standing_block,
                    can_survive_standing(context.world, place_pos),
                )
            } else {
                // Try wall sign (attached to a wall)
                let behavior = block_behaviors.get_behavior(self.wall_block);
                if behavior.get_state_for_placement(context).is_some() {
                    (self.wall_block, true) // Wall sign placement already checks survival
                } else {
                    continue;
                }
            };

            if can_place {
                let behavior = block_behaviors.get_behavior(block);
                if let Some(state) = behavior.get_state_for_placement(context) {
                    // Check collision
                    let collision_shape = state.get_collision_shape();
                    if context.world.is_unobstructed(collision_shape, place_pos) {
                        return Some(state);
                    }
                }
            }
        }

        None
    }
}

/// Checks if a standing sign can survive at the given position.
fn can_survive_standing(world: &World, pos: &BlockPos) -> bool {
    let below_pos = BlockPos::new(pos.x(), pos.y() - 1, pos.z());
    let below_state = world.get_block_state(&below_pos);
    below_state.is_face_sturdy(Direction::Up)
}

/// Gets the nearest looking directions from the player's rotation.
fn get_nearest_looking_directions(rotation: f32, clicked_face: Direction) -> Vec<Direction> {
    let mut directions = Vec::with_capacity(5);

    // If clicked on top face, prioritize standing sign (down attachment)
    if clicked_face == Direction::Up {
        directions.push(Direction::Down);
    }

    // Add horizontal directions sorted by how closely they match player's rotation
    let all_horizontal = [
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ];

    let mut scored: Vec<(Direction, f32)> = all_horizontal
        .iter()
        .map(|&dir| {
            let dir_angle = dir.to_yaw();
            let diff = (rotation - dir_angle + 180.0).rem_euclid(360.0) - 180.0;
            (dir, diff.abs())
        })
        .collect();

    scored.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(Ordering::Equal));

    for (dir, _) in scored {
        directions.push(dir);
    }

    // If we didn't already add down, add it at the end
    if clicked_face != Direction::Up {
        directions.push(Direction::Down);
    }

    directions
}

impl ItemBehavior for SignItemBehavior {
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
        let (yaw, _pitch) = context.player.rotation.load();

        let place_context = BlockPlaceContext {
            clicked_pos,
            clicked_face: context.hit_result.direction,
            click_location: context.hit_result.location,
            inside: context.hit_result.inside,
            relative_pos: place_pos,
            replace_clicked,
            horizontal_direction: Direction::from_yaw(yaw),
            rotation: yaw,
            world: context.world,
        };

        // Try to get a valid placement state (standing or wall)
        let Some(new_state) = self.get_placement_state(&place_context, &place_pos) else {
            return InteractionResult::Fail;
        };

        // Place the block
        if !context
            .world
            .set_block(place_pos, new_state, UpdateFlags::UPDATE_ALL_IMMEDIATE)
        {
            return InteractionResult::Fail;
        }

        // Consume one item from the stack
        context.item_stack.shrink(1);

        // Open the sign editor for the player (front text by default)
        context.player.open_sign_editor(place_pos, true);

        // TODO: Play place sound

        InteractionResult::Success
    }
}

/// Behavior for hanging sign items that place hanging sign blocks.
///
/// Hanging signs can be placed as ceiling hanging signs or wall hanging signs.
pub struct HangingSignItemBehavior {
    /// The ceiling hanging sign block.
    pub ceiling_block: BlockRef,
    /// The wall hanging sign block.
    pub wall_block: BlockRef,
}

impl HangingSignItemBehavior {
    /// Creates a new hanging sign item behavior.
    #[must_use]
    pub const fn new(ceiling_block: BlockRef, wall_block: BlockRef) -> Self {
        Self {
            ceiling_block,
            wall_block,
        }
    }
}

impl ItemBehavior for HangingSignItemBehavior {
    fn use_on(&self, context: &mut UseOnContext) -> InteractionResult {
        let clicked_pos = context.hit_result.block_pos;
        let clicked_state = context.world.get_block_state(&clicked_pos);

        // Get the clicked block to check if it's replaceable
        let clicked_block = REGISTRY.blocks.by_state_id(clicked_state);
        let clicked_replaceable = clicked_block.is_some_and(|b| b.config.replaceable);

        // Determine placement position
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
        let (yaw, _pitch) = context.player.rotation.load();

        let place_context = BlockPlaceContext {
            clicked_pos,
            clicked_face: context.hit_result.direction,
            click_location: context.hit_result.location,
            inside: context.hit_result.inside,
            relative_pos: place_pos,
            replace_clicked,
            horizontal_direction: Direction::from_yaw(yaw),
            rotation: yaw,
            world: context.world,
        };

        let block_behaviors = &*BLOCK_BEHAVIORS;

        // Try ceiling hanging sign first if clicked from below, otherwise try wall
        let blocks_to_try = if context.hit_result.direction == Direction::Down {
            vec![self.ceiling_block, self.wall_block]
        } else {
            vec![self.wall_block, self.ceiling_block]
        };

        let mut new_state = None;
        for block in blocks_to_try {
            let behavior = block_behaviors.get_behavior(block);
            if let Some(state) = behavior.get_state_for_placement(&place_context) {
                let collision_shape = state.get_collision_shape();
                if context.world.is_unobstructed(collision_shape, &place_pos) {
                    new_state = Some(state);
                    break;
                }
            }
        }

        let Some(state) = new_state else {
            return InteractionResult::Fail;
        };

        // Place the block
        if !context
            .world
            .set_block(place_pos, state, UpdateFlags::UPDATE_ALL_IMMEDIATE)
        {
            return InteractionResult::Fail;
        }

        // Consume one item from the stack
        context.item_stack.shrink(1);

        // Open the sign editor for the player (front text by default)
        context.player.open_sign_editor(place_pos, true);

        // TODO: Play place sound

        InteractionResult::Success
    }
}
