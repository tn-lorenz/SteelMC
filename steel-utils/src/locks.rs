use tokio::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

// A wrapper that allows us to do debug checks on the lock. To prevent deadlocks.
#[derive(Debug)]
pub struct SteelRwLock<T>(RwLock<T>);

impl<T> SteelRwLock<T> {
    pub fn new(inner: T) -> Self {
        Self(RwLock::new(inner))
    }

    pub async fn read(&self) -> RwLockReadGuard<'_, T> {
        self.0.read().await
    }

    pub async fn write(&self) -> RwLockWriteGuard<'_, T> {
        self.0.write().await
    }
}
