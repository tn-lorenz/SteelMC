//! Manual block behavior assignments for vanilla blocks.
//!
//! This module is for custom block behaviors that are too complex to auto-generate.
//! Add custom behaviors here for blocks like fences, doors, redstone components, etc.
pub mod fence_block;
pub mod rotated_pillar_block;

use crate::{
    blocks::{
        BlockRegistry,
        vanilla_behaviours::{fence_block::FenceBlock, rotated_pillar_block::RotatedPillarBlock},
    },
    vanilla_blocks,
};

// Rotated pillar blocks
static OAK_LOG: RotatedPillarBlock = RotatedPillarBlock::new(vanilla_blocks::OAK_LOG);

// Fence blocks
static OAK_FENCE: FenceBlock = FenceBlock::new(vanilla_blocks::OAK_FENCE);
static SPRUCE_FENCE: FenceBlock = FenceBlock::new(vanilla_blocks::SPRUCE_FENCE);
static BIRCH_FENCE: FenceBlock = FenceBlock::new(vanilla_blocks::BIRCH_FENCE);
static JUNGLE_FENCE: FenceBlock = FenceBlock::new(vanilla_blocks::JUNGLE_FENCE);
static ACACIA_FENCE: FenceBlock = FenceBlock::new(vanilla_blocks::ACACIA_FENCE);
static DARK_OAK_FENCE: FenceBlock = FenceBlock::new(vanilla_blocks::DARK_OAK_FENCE);
static PALE_OAK_FENCE: FenceBlock = FenceBlock::new(vanilla_blocks::PALE_OAK_FENCE);
static MANGROVE_FENCE: FenceBlock = FenceBlock::new(vanilla_blocks::MANGROVE_FENCE);
static CHERRY_FENCE: FenceBlock = FenceBlock::new(vanilla_blocks::CHERRY_FENCE);
static BAMBOO_FENCE: FenceBlock = FenceBlock::new(vanilla_blocks::BAMBOO_FENCE);
static CRIMSON_FENCE: FenceBlock = FenceBlock::new(vanilla_blocks::CRIMSON_FENCE);
static WARPED_FENCE: FenceBlock = FenceBlock::new(vanilla_blocks::WARPED_FENCE);
static NETHER_BRICK_FENCE: FenceBlock = FenceBlock::new(vanilla_blocks::NETHER_BRICK_FENCE);

pub fn assign_custom_block_behaviors(registry: &mut BlockRegistry) {
    // Rotated pillar blocks
    registry.set_behavior(vanilla_blocks::OAK_LOG, &OAK_LOG);

    // Fence blocks
    registry.set_behavior(vanilla_blocks::OAK_FENCE, &OAK_FENCE);
    registry.set_behavior(vanilla_blocks::SPRUCE_FENCE, &SPRUCE_FENCE);
    registry.set_behavior(vanilla_blocks::BIRCH_FENCE, &BIRCH_FENCE);
    registry.set_behavior(vanilla_blocks::JUNGLE_FENCE, &JUNGLE_FENCE);
    registry.set_behavior(vanilla_blocks::ACACIA_FENCE, &ACACIA_FENCE);
    registry.set_behavior(vanilla_blocks::DARK_OAK_FENCE, &DARK_OAK_FENCE);
    registry.set_behavior(vanilla_blocks::PALE_OAK_FENCE, &PALE_OAK_FENCE);
    registry.set_behavior(vanilla_blocks::MANGROVE_FENCE, &MANGROVE_FENCE);
    registry.set_behavior(vanilla_blocks::CHERRY_FENCE, &CHERRY_FENCE);
    registry.set_behavior(vanilla_blocks::BAMBOO_FENCE, &BAMBOO_FENCE);
    registry.set_behavior(vanilla_blocks::CRIMSON_FENCE, &CRIMSON_FENCE);
    registry.set_behavior(vanilla_blocks::WARPED_FENCE, &WARPED_FENCE);
    registry.set_behavior(vanilla_blocks::NETHER_BRICK_FENCE, &NETHER_BRICK_FENCE);
}
