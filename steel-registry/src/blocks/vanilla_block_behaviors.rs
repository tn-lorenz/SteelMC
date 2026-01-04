//! Manual block behavior assignments for vanilla blocks.
//!
//! This module is for custom block behaviors that are too complex to auto-generate.
//! Add custom behaviors here for blocks like fences, doors, redstone components, etc.

use crate::blocks::BlockRegistry;

// Example: Custom behavior for fence blocks
// pub static FENCE_BEHAVIOR: FenceBehavior = FenceBehavior;

/// Assigns custom block behaviors that cannot be auto-generated.
/// This is called after the auto-generated `assign_block_behaviors`.
pub fn assign_custom_block_behaviors(_registry: &mut BlockRegistry) {
    // Example usage:
    // registry.set_behavior(vanilla_blocks::OAK_FENCE, &FENCE_BEHAVIOR);

    // TODO: Add custom behavior assignments here
    // For now, all blocks use DefaultBlockBehaviour
}
