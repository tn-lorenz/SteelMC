use tokio::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

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
