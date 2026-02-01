//! Torch block implementations.
//!
//! Torches come in two forms:
//! - Standing torches (TorchBlock): placed on top of blocks
//! - Wall torches (WallTorchBlock): placed on the side of blocks
//!
//! Both break when their supporting block is removed.

use steel_registry::REGISTRY;
use steel_registry::blocks::BlockRef;
use steel_registry::blocks::block_state_ext::BlockStateExt;
use steel_registry::blocks::properties::{BlockStateProperties, Direction};
use steel_registry::blocks::shapes::SupportType;
use steel_registry::vanilla_blocks;
use steel_utils::{BlockPos, BlockStateId};

use crate::behavior::block::BlockBehaviour;
use crate::behavior::context::BlockPlaceContext;
use crate::world::World;

/// Behavior for standing torch blocks (torch, `soul_torch`, `copper_torch`).
///
/// Standing torches are placed on top of blocks and require center support
/// from the block below to survive.
pub struct TorchBlock {
    block: BlockRef,
}

impl TorchBlock {
    /// Creates a new standing torch block behavior.
    #[must_use]
    pub const fn new(block: BlockRef) -> Self {
        Self { block }
    }

    /// Checks if a torch can survive at the given position.
    /// Requires the block below to provide center support on its top face.
    pub fn can_survive(world: &World, pos: BlockPos) -> bool {
        let below_pos = pos.offset(0, -1, 0);
        let below_state = world.get_block_state(&below_pos);
        below_state.is_face_sturdy_for(Direction::Up, SupportType::Center)
    }
}

impl BlockBehaviour for TorchBlock {
    fn update_shape(
        &self,
        state: BlockStateId,
        world: &World,
        pos: BlockPos,
        direction: Direction,
        _neighbor_pos: BlockPos,
        _neighbor_state: BlockStateId,
    ) -> BlockStateId {
        // Standing torches break when the block below is removed
        if direction == Direction::Down && !Self::can_survive(world, pos) {
            return REGISTRY.blocks.get_default_state_id(vanilla_blocks::AIR);
        }
        state
    }

    fn get_state_for_placement(&self, context: &BlockPlaceContext<'_>) -> Option<BlockStateId> {
        // Check if we can place on the block below
        if !Self::can_survive(context.world, context.relative_pos) {
            return None;
        }

        Some(self.block.default_state())
    }
}

/// Behavior for wall torch blocks (`wall_torch`, `soul_wall_torch`, `copper_wall_torch`).
///
/// Wall torches are placed on the side of blocks and require a sturdy face
/// from the block they're attached to.
pub struct WallTorchBlock {
    block: BlockRef,
}

impl WallTorchBlock {
    /// Creates a new wall torch block behavior.
    #[must_use]
    pub const fn new(block: BlockRef) -> Self {
        Self { block }
    }

    /// Checks if a wall torch can survive at the given position with the given facing.
    /// Requires the block behind (opposite of facing) to provide a sturdy face.
    pub fn can_survive(world: &World, pos: BlockPos, facing: Direction) -> bool {
        let attach_direction = facing.opposite();
        let attach_pos = attach_direction.relative(&pos);
        let attach_state = world.get_block_state(&attach_pos);
        attach_state.is_face_sturdy(facing)
    }
}

impl BlockBehaviour for WallTorchBlock {
    fn update_shape(
        &self,
        state: BlockStateId,
        world: &World,
        pos: BlockPos,
        direction: Direction,
        _neighbor_pos: BlockPos,
        _neighbor_state: BlockStateId,
    ) -> BlockStateId {
        // Wall torches break when the block they're attached to is removed
        let facing: Direction = state.get_value(&BlockStateProperties::HORIZONTAL_FACING);
        let attach_direction = facing.opposite();

        if direction == attach_direction && !Self::can_survive(world, pos, facing) {
            return REGISTRY.blocks.get_default_state_id(vanilla_blocks::AIR);
        }
        state
    }

    fn get_state_for_placement(&self, context: &BlockPlaceContext<'_>) -> Option<BlockStateId> {
        let clicked_face = context.clicked_face;
        if clicked_face.is_horizontal() {
            let facing = clicked_face;
            if Self::can_survive(context.world, context.relative_pos, facing) {
                return Some(
                    self.block
                        .default_state()
                        .set_value(&BlockStateProperties::HORIZONTAL_FACING, facing),
                );
            }
        }

        for &facing in &[
            Direction::North,
            Direction::South,
            Direction::West,
            Direction::East,
        ] {
            if Self::can_survive(context.world, context.relative_pos, facing) {
                return Some(
                    self.block
                        .default_state()
                        .set_value(&BlockStateProperties::HORIZONTAL_FACING, facing),
                );
            }
        }

        None
    }
}

/// Behavior for redstone torch blocks (`redstone_torch`).
///
/// Redstone torches are placed on top of blocks and require center support
/// from the block below to survive. They have the same placement logic as
/// regular torches but also have a LIT property.
///
/// # TODO: Missing Functionality
/// - Redstone signal output (getSignal/getDirectSignal - power level 15 when lit)
/// - Respond to incoming redstone signals (neighborChanged - turn off when powered from below)
/// - Scheduled tick behavior for state changes (tick method with 2 tick delay)
/// - Burnout mechanics (isToggledTooFrequently - max 8 toggles in 60 ticks, 160 tick cooldown)
/// - Notify neighbors when placed/removed (onPlace/affectNeighborsAfterRemoval)
/// - Particle effects when lit (animateTick - client-side only)
pub struct RedstoneTorchBlock {
    block: BlockRef,
}

impl RedstoneTorchBlock {
    /// Creates a new redstone torch block behavior.
    #[must_use]
    pub const fn new(block: BlockRef) -> Self {
        Self { block }
    }

    /// Checks if a redstone torch can survive at the given position.
    /// Uses the same logic as regular torches.
    fn can_survive(world: &World, pos: BlockPos) -> bool {
        TorchBlock::can_survive(world, pos)
    }
}

impl BlockBehaviour for RedstoneTorchBlock {
    fn update_shape(
        &self,
        state: BlockStateId,
        world: &World,
        pos: BlockPos,
        direction: Direction,
        _neighbor_pos: BlockPos,
        _neighbor_state: BlockStateId,
    ) -> BlockStateId {
        // Redstone torches break when the block below is removed
        if direction == Direction::Down && !Self::can_survive(world, pos) {
            return REGISTRY.blocks.get_default_state_id(vanilla_blocks::AIR);
        }
        state
    }

    fn get_state_for_placement(&self, context: &BlockPlaceContext<'_>) -> Option<BlockStateId> {
        // Check if we can place on the block below
        if !Self::can_survive(context.world, context.relative_pos) {
            return None;
        }

        // Redstone torches are placed with LIT=true by default
        Some(
            self.block
                .default_state()
                .set_value(&BlockStateProperties::LIT, true),
        )
    }

    // TODO: Implement on_place to notify neighbors when torch is placed
    // TODO: Implement affect_neighbors_after_removal to notify neighbors when torch is removed
    // TODO: Implement handle_neighbor_changed to schedule tick when powered state changes
    // TODO: Implement random_tick or scheduled tick for state changes (LIT toggling)
    // TODO: Implement get_signal (return 15 when LIT and direction != UP)
    // TODO: Implement get_direct_signal (return 15 when LIT and direction == DOWN)
    // TODO: Implement is_signal_source (return true)
}

/// Behavior for redstone wall torch blocks (`redstone_wall_torch`).
///
/// Redstone wall torches are placed on the side of blocks and require a sturdy face
/// from the block they're attached to. They have the same placement logic as
/// regular wall torches but also have a LIT property.
///
/// # TODO: Missing Functionality
/// - Redstone signal output (getSignal/getDirectSignal - power level 15 when lit, based on facing)
/// - Respond to incoming redstone signals (hasNeighborSignal - check block behind torch)
/// - Scheduled tick behavior for state changes (tick method with 2 tick delay)
/// - Burnout mechanics (inherited from `RedstoneTorchBlock`)
/// - Notify neighbors when placed/removed
/// - Particle effects when lit (animateTick - client-side only)
pub struct RedstoneWallTorchBlock {
    block: BlockRef,
}

impl RedstoneWallTorchBlock {
    /// Creates a new redstone wall torch block behavior.
    #[must_use]
    pub const fn new(block: BlockRef) -> Self {
        Self { block }
    }

    /// Checks if a redstone wall torch can survive at the given position with the given facing.
    /// Uses the same logic as regular wall torches.
    fn can_survive(world: &World, pos: BlockPos, facing: Direction) -> bool {
        WallTorchBlock::can_survive(world, pos, facing)
    }
}

impl BlockBehaviour for RedstoneWallTorchBlock {
    fn update_shape(
        &self,
        state: BlockStateId,
        world: &World,
        pos: BlockPos,
        direction: Direction,
        _neighbor_pos: BlockPos,
        _neighbor_state: BlockStateId,
    ) -> BlockStateId {
        // Redstone wall torches break when the block they're attached to is removed
        let facing: Direction = state.get_value(&BlockStateProperties::HORIZONTAL_FACING);
        let attach_direction = facing.opposite();

        if direction == attach_direction && !Self::can_survive(world, pos, facing) {
            return REGISTRY.blocks.get_default_state_id(vanilla_blocks::AIR);
        }
        state
    }

    fn get_state_for_placement(&self, context: &BlockPlaceContext<'_>) -> Option<BlockStateId> {
        // Try to place on the clicked face first if it's horizontal
        let clicked_face = context.clicked_face;
        if clicked_face.is_horizontal() {
            // When clicking on a wall, the torch faces away from the wall
            let facing = clicked_face;
            if Self::can_survive(context.world, context.relative_pos, facing) {
                return Some(
                    self.block
                        .default_state()
                        .set_value(&BlockStateProperties::HORIZONTAL_FACING, facing)
                        .set_value(&BlockStateProperties::LIT, true),
                );
            }
        }

        // Try all horizontal directions based on where the player is looking
        for &facing in &[
            Direction::North,
            Direction::South,
            Direction::West,
            Direction::East,
        ] {
            if Self::can_survive(context.world, context.relative_pos, facing) {
                return Some(
                    self.block
                        .default_state()
                        .set_value(&BlockStateProperties::HORIZONTAL_FACING, facing)
                        .set_value(&BlockStateProperties::LIT, true),
                );
            }
        }

        None
    }

    // TODO: Implement on_place to notify neighbors when torch is placed
    // TODO: Implement affect_neighbors_after_removal to notify neighbors when torch is removed
    // TODO: Implement handle_neighbor_changed to schedule tick when powered state changes
    // TODO: Implement random_tick or scheduled tick for state changes (LIT toggling)
    // TODO: Implement get_signal (return 15 when LIT and direction != facing)
    // TODO: Implement get_direct_signal (based on facing direction)
    // TODO: Implement is_signal_source (return true)
    // TODO: Implement hasNeighborSignal to check power from block behind torch (opposite of facing)
}
