use steel_utils::{BlockPos, BlockStateId, types::UpdateFlags};

use crate::blocks::BlockRef;
use crate::blocks::properties::Direction;

pub trait RegistryPlayer {}

pub trait RegistryWorld {
    /// Gets the block state at the given position.
    fn get_block_state(&self, pos: &BlockPos) -> BlockStateId;

    /// Sets a block at the given position.
    /// Returns `true` if the block was successfully set, `false` otherwise.
    fn set_block(&self, pos: BlockPos, block_state: BlockStateId, flags: UpdateFlags) -> bool;

    /// Returns whether the block position is within valid world bounds.
    fn is_in_valid_bounds(&self, block_pos: &BlockPos) -> bool;

    /// Called when a neighbor's shape changes, to update this block's state.
    ///
    /// This is the Rust equivalent of vanilla's `LevelAccessor.neighborShapeChanged()`.
    /// It triggers `BlockBehaviour::update_shape()` on the block at `pos`.
    ///
    /// # Arguments
    /// * `direction` - Direction from the neighbor TO this block
    /// * `pos` - Position of the block to update
    /// * `neighbor_pos` - Position of the neighbor that changed
    /// * `neighbor_state` - New state of the neighbor
    /// * `flags` - Update flags for propagation
    /// * `update_limit` - Recursion limit to prevent infinite loops
    fn neighbor_shape_changed(
        &self,
        direction: Direction,
        pos: BlockPos,
        neighbor_pos: BlockPos,
        neighbor_state: BlockStateId,
        flags: UpdateFlags,
        update_limit: i32,
    );

    /// Notifies a block that one of its neighbors changed.
    ///
    /// This is the Rust equivalent of vanilla's `Level.neighborChanged()`.
    /// Used by redstone components, doors, and other blocks that react to neighbor changes.
    ///
    /// # Arguments
    /// * `pos` - Position of the block to notify
    /// * `source_block` - The block type that changed
    /// * `moved_by_piston` - Whether the change was caused by a piston
    fn neighbor_changed(&self, pos: BlockPos, source_block: BlockRef, moved_by_piston: bool);
}

pub trait RegistryServer {}
