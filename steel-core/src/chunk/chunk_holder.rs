//! `ChunkHolder` manages chunk state and asynchronous generation tasks.
use std::pin::Pin;
use std::sync::Arc;

use futures::{Future, future};
use replace_with::replace_with_or_abort;
use steel_utils::ChunkPos;
use tokio::sync::{Mutex, watch};

use crate::chunk::chunk_generation_task::{NeighborReady, StaticCache2D};
use crate::{
    ChunkMap,
    chunk::{
        chunk_access::{ChunkAccess, ChunkStatus},
        chunk_generation_task::ChunkGenerationTask,
        chunk_pyramid::ChunkStep,
        level_chunk::LevelChunk,
    },
};

/// A tuple containing the chunk status and the chunk access.
pub type ChunkStageHolder = (ChunkStatus, ChunkAccess);

/// The result of a chunk operation.
pub enum ChunkResult {
    /// The chunk is not loaded.
    Unloaded,
    /// The chunk operation failed.
    Failed,
    /// The chunk operation succeeded.
    Ok(ChunkStageHolder),
}

/// Holds a chunk in a watch channel, allowing for concurrent access and state tracking.
pub struct ChunkHolder {
    chunk_access: watch::Receiver<ChunkResult>,
    sender: watch::Sender<ChunkResult>,
    generation_task: Mutex<Option<Arc<ChunkGenerationTask>>>,
    pos: ChunkPos,
    /// The current ticket level of the chunk.
    pub ticket_level: Mutex<u8>,
}

impl ChunkHolder {
    /// Creates a new chunk holder.
    #[must_use]
    pub fn new(pos: ChunkPos, ticket_level: u8) -> Self {
        let (sender, receiver) = watch::channel(ChunkResult::Unloaded);
        Self {
            chunk_access: receiver,
            sender,
            generation_task: Mutex::new(None),
            pos,
            ticket_level: Mutex::new(ticket_level),
        }
    }

    /// Returns a future that completes when the chunk reaches the given status or is cancelled.
    #[allow(clippy::missing_panics_doc)]
    pub async fn schedule_chunk_generation_task(
        &self,
        status: ChunkStatus,
        chunk_map: Arc<ChunkMap>,
    ) -> Pin<Box<dyn Future<Output = Option<()>> + Send + '_>> {
        if self.with_chunk(status, |_| ()).is_some() {
            return Box::pin(future::ready(Some(())));
        }

        let task = self.generation_task.lock().await;

        #[allow(clippy::unwrap_used)]
        if task.is_none() || status > task.as_ref().unwrap().target_status {
            drop(task);
            self.reschedule_chunk_task(status, chunk_map).await;
        }

        Box::pin(self.await_chunk_and_then(status, |_| ()))
    }

    /// Reschedules the chunk task to the given status.
    pub async fn reschedule_chunk_task(&self, status: ChunkStatus, chunk_map: Arc<ChunkMap>) {
        let new_task = chunk_map.schedule_generation_task(status, self.pos).await;
        let mut old_task_guard = self.generation_task.lock().await;

        let old_task = old_task_guard.replace(new_task);
        drop(old_task_guard);

        if let Some(old_task) = old_task {
            old_task.mark_for_cancel();
        }
    }

    /// Gets mutable access to the chunk if it has reached the given status.
    pub fn with_chunk_mut<F, R>(&self, status: ChunkStatus, f: F) -> Option<R>
    where
        F: FnOnce(&mut ChunkAccess) -> R,
    {
        let mut return_value: Option<R> = None;
        self.sender.send_modify(|chunk| match chunk {
            ChunkResult::Ok((s, chunk)) if status <= *s => {
                return_value = Some(f(chunk));
            }
            _ => {}
        });
        return_value
    }

    /// Gets access to the chunk if it has reached the given status.
    pub fn with_chunk<F, R>(&self, status: ChunkStatus, f: F) -> Option<R>
    where
        F: FnOnce(&ChunkAccess) -> R,
    {
        match &*self.chunk_access.borrow() {
            ChunkResult::Ok((s, chunk)) if status <= *s => Some(f(chunk)),
            _ => None,
        }
    }

    /// Gets access to the chunk if it is full.
    pub fn with_full_chunk<F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(&LevelChunk) -> R,
    {
        match &*self.chunk_access.borrow() {
            ChunkResult::Ok((ChunkStatus::Full, ChunkAccess::Full(chunk))) => Some(f(chunk)),
            _ => None,
        }
    }

    /// Waits until the chunk has reached the given status, then calls the function.
    pub fn await_chunk_and_then<F, R>(
        &self,
        status: ChunkStatus,
        f: F,
    ) -> impl Future<Output = Option<R>>
    where
        F: FnOnce(&ChunkAccess) -> R,
    {
        let mut subscriber = self.sender.subscribe();
        async move {
            loop {
                {
                    let chunk_access = subscriber.borrow_and_update();
                    match &*chunk_access {
                        ChunkResult::Ok((s, chunk)) if status <= *s => {
                            return Some(f(chunk));
                        }
                        ChunkResult::Failed => return None,
                        _ => {}
                    }
                }

                if subscriber.changed().await.is_err() {
                    log::error!("Failed to wait for chunk access");
                    return None;
                }
            }
        }
    }

    /// Waits until this chunk has reached the Full status.
    pub async fn await_full_and_then<F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(&ChunkAccess) -> R,
    {
        self.await_chunk_and_then(ChunkStatus::Full, f).await
    }

    /// Waits until this chunk has reached the Full status.
    pub async fn await_full(&self) -> Option<()> {
        self.await_full_and_then(|_| ()).await
    }

    /// Gets the persisted status of the chunk.
    pub fn persisted_status(&self) -> ChunkStatus {
        match &*self.chunk_access.borrow() {
            ChunkResult::Ok((s, _)) => *s,
            _ => ChunkStatus::Empty,
        }
    }

    /// Applies a step to the chunk.
    pub fn apply_step(
        &self,
        step: Arc<ChunkStep>,
        _chunk_map: Arc<ChunkMap>,
        _cache: Arc<StaticCache2D<Arc<ChunkHolder>>>,
    ) -> NeighborReady {
        let target_status = step.target_status;
        let sender = self.sender.clone();

        Box::pin(async move {
            // Simulate work placeholder
            sender.send_modify(|chunk| match chunk {
                ChunkResult::Ok((s, _)) if *s >= target_status => {}
                ChunkResult::Ok((s, _)) => *s = target_status,
                ChunkResult::Unloaded | ChunkResult::Failed => {}
            });

            Some(())
        })
    }

    /// Upgrades the chunk to a full chunk.
    ///
    /// # Panics
    /// Panics if the chunk is not at `ProtoChunk` stage or completed.
    pub fn upgrade_to_full(&self) {
        self.sender.send_modify(|chunk| {
            replace_with_or_abort(chunk, |chunk| match chunk {
                ChunkResult::Ok((_, ChunkAccess::Proto(proto_chunk))) => ChunkResult::Ok((
                    ChunkStatus::Full,
                    ChunkAccess::Full(LevelChunk::from_proto(proto_chunk)),
                )),
                _ => panic!("Cannot upgrade chunk: not at ProtoChunk status"),
            });
        });
    }
}
