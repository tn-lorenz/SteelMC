//! This module contains the `ChunkGenerator` trait, which is used to generate chunks.
use std::ops::{Deref, DerefMut};

use crate::chunk::chunk_access::ChunkAccess;
use enum_dispatch::enum_dispatch;
use parking_lot::{RwLock as ParkingRwLock, RwLockWriteGuard};

/// A guard that provides access to a chunk while holding the lock.
pub struct ChunkGuard<'a> {
    mutex: &'a ParkingRwLock<Option<ChunkAccess>>,
    guard: Option<RwLockWriteGuard<'a, Option<ChunkAccess>>>,
}

impl<'a> ChunkGuard<'a> {
    /// Creates a new `ChunkGuard` that holds the write lock.
    pub fn new(mutex: &'a ParkingRwLock<Option<ChunkAccess>>) -> Self {
        Self {
            mutex,
            guard: Some(mutex.write()),
        }
    }

    /// Temporarily drops the lock, runs the provided closure, and re-acquires the lock immediately after.
    pub fn yield_lock<F, R>(&mut self, func: F) -> R
    where
        F: FnOnce() -> R,
    {
        self.guard = None;

        let result = func();

        self.guard = Some(self.mutex.write());

        result
    }
}

impl Deref for ChunkGuard<'_> {
    type Target = ChunkAccess;

    #[inline]
    #[allow(clippy::unwrap_used)]
    fn deref(&self) -> &Self::Target {
        // SAFETY: It needs to contain a mutex guard to be dereferenceable and the chunk access is guaranteed to be there by it's creator.
        self.guard
            .as_deref()
            .unwrap()
            .as_ref()
            .expect("Chunk should be loaded")
    }
}

impl DerefMut for ChunkGuard<'_> {
    #[inline]
    #[allow(clippy::unwrap_used)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: It needs to contain a mutex guard to be dereferenceable and the chunk access is guaranteed to be there by it's creator.
        self.guard
            .as_deref_mut()
            .unwrap()
            .as_mut()
            .expect("Chunk should be loaded")
    }
}

/// A trait for generating chunks.
#[enum_dispatch]
pub trait ChunkGenerator: Send + Sync {
    /// Creates the structures in a chunk.
    fn create_structures(&self, chunk_guard: &mut ChunkGuard);

    /// Creates the biomes in a chunk.
    fn create_biomes(&self, chunk_guard: &mut ChunkGuard);

    /// Fills the chunk with noise.
    fn fill_from_noise(&self, chunk_guard: &mut ChunkGuard);

    /// Builds the surface of the chunk.
    fn build_surface(&self, chunk_guard: &mut ChunkGuard);

    /// Applies carvers to the chunk.
    fn apply_carvers(&self, chunk_guard: &mut ChunkGuard);

    /// Applies biome decorations to the chunk.
    fn apply_biome_decorations(&self, chunk_guard: &mut ChunkGuard);
}

/// A simple chunk generator that does nothing.
pub struct SimpleChunkGenerator;

impl ChunkGenerator for SimpleChunkGenerator {
    fn create_structures(&self, _chunk_guard: &mut ChunkGuard) {}
    fn create_biomes(&self, _chunk_guard: &mut ChunkGuard) {}
    fn fill_from_noise(&self, _chunk_guard: &mut ChunkGuard) {}
    fn build_surface(&self, _chunk_guard: &mut ChunkGuard) {}
    fn apply_carvers(&self, _chunk_guard: &mut ChunkGuard) {}
    fn apply_biome_decorations(&self, _chunk_guard: &mut ChunkGuard) {}
}
