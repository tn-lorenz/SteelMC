use std::io::{self, Read};

use steel_utils::{BlockPos, math::Vector3, serial::ReadFrom, types::InteractionHand};

use crate::{
    blocks::{BlockRef, properties::Direction},
    compat_traits::{RegistryPlayer, RegistryWorld},
    item_stack::ItemStack,
};

pub use crate::blocks::behaviour::BlockPlaceContext;

pub enum InteractionResult {
    Success,
    Fail,
    Pass,
    TryEmptyHandInteraction,
}

#[derive(Debug, Clone)]
pub struct BlockHitResult {
    pub location: Vector3<f64>,
    pub direction: Direction,
    pub block_pos: BlockPos,
    pub miss: bool,
    pub inside: bool,
    pub world_border_hit: bool,
}

impl ReadFrom for BlockHitResult {
    fn read(data: &mut impl Read) -> io::Result<Self> {
        let block_pos = BlockPos::read(data)?;
        let direction = Direction::read(data)?;
        let x = f32::read(data)?;
        let y = f32::read(data)?;
        let z = f32::read(data)?;
        let inside = bool::read(data)?;
        let world_border_hit = bool::read(data)?;

        Ok(BlockHitResult {
            location: Vector3::new(f64::from(x), f64::from(y), f64::from(z)),
            direction,
            block_pos,
            miss: false,
            inside,
            world_border_hit,
        })
    }
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
