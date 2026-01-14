//! Block behavior implementations for vanilla blocks.
//!
//! The actual behavior registration is auto-generated from classes.json.
//! See `src/generated/behaviors.rs` for the generated registration code.

mod fence_block;
mod rotated_pillar_block;

pub use fence_block::FenceBlock;
pub use rotated_pillar_block::RotatedPillarBlock;
