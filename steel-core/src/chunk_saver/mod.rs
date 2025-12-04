//! Chunk persistence module.
//!
//! This module handles saving and loading chunks to/from disk using a region-based
//! file format. Each region file contains a 32×32 grid of chunks.
//!
//! ## Format Overview
//!
//! Region files are loaded entirely into memory, modified, and saved atomically.
//! This simplifies the implementation and allows for better compression.
//!
//! ### Key Features
//! - Per-region block state and biome tables for string deduplication
//! - Power-of-2 bit packing for efficient storage (1, 2, 4, 8, 16 bits)
//! - Homogeneous section optimization (single block type = no bit array)
//! - zstd compression at the region level

mod bit_pack;
mod format;
mod region_manager;

pub use format::*;
pub use region_manager::*;
