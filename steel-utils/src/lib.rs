use tokio::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

use crate::math::{vector2::Vector2, vector3::Vector3};

pub mod math;

// A wrapper that allows us to do debug checks on the lock. To prevent deadlocks.
#[derive(Debug)]
pub struct SteelRwLock<T> {
    inner: RwLock<T>,
}

impl<T> SteelRwLock<T> {
    pub fn new(inner: T) -> Self {
        Self {
            inner: RwLock::new(inner),
        }
    }

    pub async fn read(&self) -> RwLockReadGuard<'_, T> {
        self.inner.read().await
    }

    pub async fn write(&self) -> RwLockWriteGuard<'_, T> {
        self.inner.write().await
    }
}

// Wrapper types making it harder to accidentaly use the wrong underlying type.

// A raw block state id. Using the registry this id can be derived into a block and it's current properties.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockStateId(pub u16);

// A chunk position.
pub struct ChunkPos(pub Vector2<i32>);

// A block position.
pub struct BlockPos(pub Vector3<i32>);
