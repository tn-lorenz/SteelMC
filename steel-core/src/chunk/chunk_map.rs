use std::sync::Arc;
use steel_utils::ChunkPos;
use tokio::sync::Mutex;
use tokio_util::task::TaskTracker;

use crate::chunk::chunk_holder::ChunkHolder;
use crate::chunk::{
    chunk_access::ChunkStatus, chunk_generation_task::ChunkGenerationTask,
    chunk_generator::SimpleChunkGenerator, chunk_pyramid::GENERATION_PYRAMID,
    chunk_tracker::MAX_LEVEL, distance_manager::DistanceManager,
    world_gen_context::WorldGenContext,
};

/// A map of chunks managing their state, loading, and generation.
pub struct ChunkMap {
    /// Map of active chunks.
    pub chunks: scc::HashMap<ChunkPos, Arc<ChunkHolder>>,
    /// Map of chunks currently being unloaded.
    pub unloading_chunks: scc::HashMap<ChunkPos, Arc<ChunkHolder>>,
    /// Queue of pending generation tasks.
    pub pending_generation_tasks: Mutex<Vec<Arc<ChunkGenerationTask>>>,
    /// Tracker for background generation tasks.
    pub task_tracker: TaskTracker,
    /// Manager for chunk distances and tickets.
    pub distance_manager: Mutex<DistanceManager>,
    /// The world generation context.
    pub world_gen_context: Arc<WorldGenContext>,
}

impl Default for ChunkMap {
    fn default() -> Self {
        Self::new()
    }
}

impl ChunkMap {
    /// Creates a new chunk map.
    #[must_use]
    pub fn new() -> Self {
        Self {
            chunks: scc::HashMap::new(),
            unloading_chunks: scc::HashMap::new(),
            pending_generation_tasks: Mutex::new(Vec::new()),
            task_tracker: TaskTracker::new(),
            distance_manager: Mutex::new(DistanceManager::new()),
            world_gen_context: Arc::new(WorldGenContext {
                generator: Arc::new(SimpleChunkGenerator),
            }),
        }
    }

    /// Schedules a new generation task.
    pub async fn schedule_generation_task(
        self: &Arc<Self>,
        target_status: ChunkStatus,
        pos: ChunkPos,
    ) -> Arc<ChunkGenerationTask> {
        let task = Arc::new(ChunkGenerationTask::new(pos, target_status, self.clone()).await);
        self.pending_generation_tasks
            .lock()
            .await
            .push(task.clone());
        task
    }

    /// Runs queued generation tasks.
    pub async fn run_generation_tasks(&self) {
        let mut pending = self.pending_generation_tasks.lock().await;
        log::info!("Running {} generation tasks", pending.len());
        for task in pending.drain(..) {
            self.task_tracker.spawn(async move {
                task.run().await;
            });
        }
    }

    /// Updates scheduling for a chunk based on its new level.
    /// Returns the chunk holder if it is active.
    pub async fn update_chunk_level(
        self: &Arc<Self>,
        pos: ChunkPos,
        new_level: u8,
    ) -> Option<Arc<ChunkHolder>> {
        // Recover from unloading if possible, else create new holder.
        let chunk_holder = if let Some(entry) = self.unloading_chunks.remove_async(&pos).await {
            let holder = entry.1;
            let _ = self.chunks.insert_async(pos, holder.clone()).await;
            holder
        } else {
            if let Some(holder) = self.chunks.get_async(&pos).await {
                holder.get().clone()
            } else {
                if new_level > MAX_LEVEL {
                    return None;
                }
                let holder = Arc::new(ChunkHolder::new(pos, new_level));
                let _ = self.chunks.insert_async(pos, holder.clone()).await;
                holder
            }
        };

        *chunk_holder.ticket_level.lock().await = new_level;

        if new_level > MAX_LEVEL {
            log::info!("Unloading chunk at {pos:?}");
            if let Some((_, holder)) = self.chunks.remove_async(&pos).await {
                let _ = self.unloading_chunks.insert_async(pos, holder).await;
            }
            None
        } else {
            Some(chunk_holder)
        }
    }

    /// Processes chunk updates.
    pub async fn tick(self: &Arc<Self>) {
        let changes = self.distance_manager.lock().await.run_updates();

        let mut updates_to_schedule = Vec::new();

        for (pos, _, new_level) in changes {
            if let Some(holder) = self.update_chunk_level(pos, new_level).await {
                updates_to_schedule.push((holder, new_level));
            }
        }

        let mut futures = Vec::new();
        for (chunk_holder, new_level) in updates_to_schedule {
            // Use the generation pyramid to determine the target status for the given level.
            let target_status = if new_level > MAX_LEVEL {
                None
            } else if new_level <= 33 {
                Some(ChunkStatus::Full)
            } else {
                let distance = (new_level - 33) as usize;
                // Fallback to None if distance is out of bounds (simulating Vanilla logic)
                GENERATION_PYRAMID
                    .get_step_to(ChunkStatus::Full)
                    .accumulated_dependencies
                    .get(distance)
            };

            if let Some(status) = target_status {
                let chunk_holder_clone = chunk_holder.clone();
                let map_clone = self.clone();
                futures.push(async move {
                    let _ = chunk_holder_clone
                        .schedule_chunk_generation_task(status, map_clone)
                        .await;
                });
            }
        }

        futures::future::join_all(futures).await;

        self.run_generation_tasks().await;
    }
}
