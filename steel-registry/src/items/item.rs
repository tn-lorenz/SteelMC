use steel_utils::{BlockPos, math::Vector3, types::InteractionHand};

use crate::{
    blocks::{BlockRef, properties::Direction},
    compat_traits::{RegistryPlayer, RegistryWorld},
    item_stack::ItemStack,
};

pub enum InteractionResult {
    Success,
    Fail,
    Pass,
    TryEmptyHandInteraction,
}

pub struct BlockPlaceContext {
    pub relative_pos: BlockPos,
    pub replace_clicked_block: bool,
}

pub struct BlockHitResult {
    pub location: Vector3<f64>,
    pub direction: Direction,
    pub block_pos: BlockPos,
    pub miss: bool,
    pub inside: bool,
    pub world_border_hit: bool,
}

pub struct UseOnContext<'a> {
    pub player: &'a dyn RegistryPlayer,
    pub hand: InteractionHand,
    pub hit_result: BlockHitResult,
    pub world: &'a dyn RegistryWorld,
    pub item_stack: ItemStack,
}

/// Trait defining item behavior (use, placement, etc.)
pub trait ItemBehavior: Send + Sync {
    fn use_on(&self, use_on_context: &UseOnContext) -> InteractionResult;
}

/// Default item behavior - does nothing special
pub struct DefaultItemBehavior;

impl ItemBehavior for DefaultItemBehavior {
    fn use_on(&self, _use_on_context: &UseOnContext) -> InteractionResult {
        InteractionResult::Pass
    }
}

/// Behavior for items that place blocks
pub struct BlockItemBehavior {
    pub block: BlockRef,
}

impl ItemBehavior for BlockItemBehavior {
    fn use_on(&self, _use_on_context: &UseOnContext) -> InteractionResult {
        // TODO: Implement block placement logic
        InteractionResult::Pass
    }
}

impl BlockItemBehavior {
    #[must_use]
    pub const fn new(block: BlockRef) -> Self {
        Self { block }
    }

    #[allow(dead_code)]
    fn place(&self, _place_context: &BlockPlaceContext) {
        // TODO: Implement placement logic
    }
}
