use std::sync::Arc;
use std::sync::atomic::Ordering;
use std::time::Duration;
use std::u8;

use parking_lot::Mutex as ParkingMutex;
use rayon::iter::{IntoParallelRefIterator, ParallelBridge, ParallelIterator};
use rayon::{ThreadPool, ThreadPoolBuilder};
use rustc_hash::{FxBuildHasher, FxHashMap};
use steel_protocol::packets::game::CSetChunkCenter;
use steel_registry::blocks::BlockRegistry;
use steel_registry::vanilla_blocks;
use steel_utils::ChunkPos;
use tokio::runtime::Runtime;
use tokio_util::task::TaskTracker;

use crate::chunk::chunk_holder::ChunkHolder;
use crate::chunk::chunk_ticket_manager::{
    ChunkTicketManager, LevelChange, MAX_VIEW_DISTANCE, is_full,
};
use crate::chunk::player_chunk_view::PlayerChunkView;
use crate::chunk::{
    chunk_access::ChunkStatus, chunk_generation_task::ChunkGenerationTask,
    flat_chunk_generator::FlatChunkGenerator, world_gen_context::WorldGenContext,
};
use crate::config::STEEL_CONFIG;
use crate::player::Player;

const PROCESS_CHANGES_WARN_THRESHOLD: usize = 1_000;
const PROCESS_CHANGES_WARN_MIN_DURATION: Duration = Duration::from_micros(500);
const SLOW_TASK_WARN_THRESHOLD: Duration = Duration::from_micros(250);
/// A map of chunks managing their state, loading, and generation.
pub struct ChunkMap {
    /// Map of active chunks.
    pub chunks: scc::HashMap<ChunkPos, Arc<ChunkHolder>, FxBuildHasher>,
    /// Map of chunks currently being unloaded.
    pub unloading_chunks: ParkingMutex<FxHashMap<ChunkPos, Arc<ChunkHolder>>>,
    /// Queue of pending generation tasks.
    pub pending_generation_tasks: ParkingMutex<Vec<Arc<ChunkGenerationTask>>>,
    /// Tracker for background generation tasks.
    pub task_tracker: TaskTracker,
    /// Manager for chunk distances and tickets.
    pub chunk_tickets: ParkingMutex<ChunkTicketManager>,
    /// The world generation context.
    pub world_gen_context: Arc<WorldGenContext>,
    /// The thread pool to use for generation.
    pub thread_pool: Arc<ThreadPool>,
    /// The runtime to use for chunk tasks.
    pub chunk_runtime: Arc<Runtime>,
}

impl ChunkMap {
    /// Creates a new chunk map.
    #[must_use]
    pub fn new(block_registry: &BlockRegistry, chunk_runtime: Arc<Runtime>) -> Self {
        Self {
            chunks: scc::HashMap::with_capacity_and_hasher(1000, FxBuildHasher),
            unloading_chunks: ParkingMutex::new(FxHashMap::with_capacity_and_hasher(
                1000,
                FxBuildHasher,
            )),
            pending_generation_tasks: ParkingMutex::new(Vec::new()),
            task_tracker: TaskTracker::new(),
            chunk_tickets: ParkingMutex::new(ChunkTicketManager::new()),
            world_gen_context: Arc::new(WorldGenContext {
                generator: Arc::new(FlatChunkGenerator::new(
                    block_registry.get_default_state_id(vanilla_blocks::BEDROCK), // Bedrock
                    block_registry.get_default_state_id(vanilla_blocks::DIRT),    // Dirt
                    block_registry.get_default_state_id(vanilla_blocks::GRASS_BLOCK), // Grass Block
                )),
            }),
            thread_pool: Arc::new(ThreadPoolBuilder::new().build().unwrap()),
            chunk_runtime,
        }
    }

    /// Schedules a new generation task.
    #[inline]
    pub(crate) fn schedule_generation_task_b(
        self: &Arc<Self>,
        target_status: ChunkStatus,
        pos: ChunkPos,
    ) -> Arc<ChunkGenerationTask> {
        let start = tokio::time::Instant::now();
        let task = Arc::new(ChunkGenerationTask::new(
            pos,
            target_status,
            self.clone(),
            self.thread_pool.clone(),
        ));
        if start.elapsed() >= Duration::from_millis(1) {
            log::warn!("schedule_generation_task_b took: {:?}", start.elapsed());
        }
        self.pending_generation_tasks.lock().push(task.clone());
        task
    }

    /// Runs queued generation tasks.
    pub fn run_generation_tasks_b(&self) {
        let mut pending = self.pending_generation_tasks.lock();
        if pending.is_empty() {
            return;
        }
        //log::info!("Running {} generation tasks", pending.len());
        pending.drain(..).par_bridge().for_each(|task| {
            self.task_tracker
                .spawn_on(async move { task.run().await }, self.chunk_runtime.handle());
        });
    }

    /// Updates scheduling for a chunk based on its new level.
    /// Returns the chunk holder if it is active.
    #[inline]
    pub fn update_chunk_level(
        self: &Arc<Self>,
        pos: &ChunkPos,
        new_level: Option<u8>,
    ) -> Option<Arc<ChunkHolder>> {
        // Recover from unloading if possible, else create new holder.
        let chunk_holder =
            if let Some(holder) = self.chunks.read_sync(pos, |_, holder| holder.clone()) {
                holder
            } else {
                new_level?;

                if let Some(entry) = self.unloading_chunks.lock().remove(pos) {
                    let _ = self.chunks.insert_sync(*pos, entry.clone());
                    entry
                } else {
                    let holder = Arc::new(ChunkHolder::new(*pos, new_level.unwrap()));
                    let _ = self.chunks.insert_sync(*pos, holder.clone());
                    holder
                }
            };

        if let Some(level) = new_level {
            let old = chunk_holder.ticket_level.swap(level, Ordering::Relaxed);
            if old != level {
                chunk_holder.update_highest_allowed_status(level);
            }
            Some(chunk_holder)
        } else {
            //log::info!("Unloading chunk at {pos:?}");
            chunk_holder.force_fail();

            // Check for two cause we are also holding a reference to the chunk
            if let Some((_, holder)) = self
                .chunks
                .remove_if_sync(pos, |chunk| Arc::strong_count(chunk) == 2)
            {
                let _ = self.unloading_chunks.lock().insert(*pos, holder);
            } else {
                chunk_holder.ticket_level.store(u8::MAX, Ordering::Relaxed);
                chunk_holder.update_highest_allowed_status(u8::MAX);
            }
            None
        }
    }

    /// Processes chunk updates.
    pub fn tick_b(self: &Arc<Self>, tick_count: u64) {
        let start = tokio::time::Instant::now();

        {
            let mut ct = self.chunk_tickets.lock();

            let updates_start = tokio::time::Instant::now();
            // Only process chunks that actually changed
            let changes: Vec<LevelChange> = ct.run_all_updates().to_vec();
            let updates_elapsed = updates_start.elapsed();

            let holder_creation_start = tokio::time::Instant::now();
            let holders_to_schedule: Vec<_> = changes
                .iter()
                .filter_map(|change| {
                    self.update_chunk_level(&change.pos, change.new_level)
                        .map(|holder| (holder, change.new_level))
                })
                .collect();
            let holder_creation_elapsed = holder_creation_start.elapsed();

            let schedule_start = tokio::time::Instant::now();
            holders_to_schedule.par_iter().for_each(|(holder, level)| {
                if let Some(level) = level {
                    if is_full(*level) {
                        holder.schedule_chunk_generation_task_b(ChunkStatus::Full, self);
                    }
                }
            });
            let schedule_elapsed = schedule_start.elapsed();

            if updates_elapsed >= SLOW_TASK_WARN_THRESHOLD {
                log::warn!("chunk_tickets run_updates slow: {updates_elapsed:?}");
            }
            if holder_creation_elapsed >= SLOW_TASK_WARN_THRESHOLD {
                log::warn!("holder_creation slow: {holder_creation_elapsed:?}");
            }
            if schedule_elapsed >= SLOW_TASK_WARN_THRESHOLD {
                log::warn!("schedule slow: {schedule_elapsed:?}");
            }
        };

        let start_gen = tokio::time::Instant::now();
        self.run_generation_tasks_b();
        let gen_elapsed = start_gen.elapsed();
        if gen_elapsed >= SLOW_TASK_WARN_THRESHOLD {
            log::warn!("run_generation_tasks_b slow: {gen_elapsed:?}");
        }

        let start_unload = tokio::time::Instant::now();
        self.process_unloads();
        let unload_elapsed = start_unload.elapsed();
        if unload_elapsed >= SLOW_TASK_WARN_THRESHOLD {
            log::warn!("process_unloads slow: {unload_elapsed:?}");
        }

        let tick_elapsed = start.elapsed();
        if tick_elapsed >= SLOW_TASK_WARN_THRESHOLD {
            log::warn!("Tick_b slow: total {tick_elapsed:?}");
        }

        if tick_count.is_multiple_of(100) {
            log::debug!(
                "Chunk map entries: {}, unloading chunks: {}",
                self.chunks.len(),
                self.unloading_chunks.lock().len()
            );
        }
    }

    /// Saves a chunk to disk.
    ///
    /// This function is currently a placeholder for the actual saving logic.
    pub async fn save_chunk(&self, chunk_holder: &Arc<ChunkHolder>) {
        let _pos = chunk_holder.get_pos();
        // Access the chunk to ensure it's loaded and ready for saving
        // We use ChunkStatus::StructureReferences as the minimum requirement, effectively checking if any data exists.
        let _saved = chunk_holder.try_chunk(ChunkStatus::StructureReferences);
        //TODO: Save the chunk to disk
    }

    /// Processes chunks that are pending unload.
    ///
    /// This method iterates over the chunks in the `unloading_chunks` map.
    /// If a chunk is only held by the map (strong count is 1), it is removed
    /// and a background task is spawned to save it.
    pub fn process_unloads(self: &Arc<Self>) {
        self.unloading_chunks.lock().retain(|_, holder| {
            // If the strong count is 1, it means only this map holds a reference to the chunk.
            // We can safely unload it.
            if Arc::strong_count(&*holder) == 1 {
                let holder_clone = holder.clone();
                let map_clone = self.clone();

                self.task_tracker.spawn(async move {
                    map_clone.save_chunk(&holder_clone).await;
                });
                // Remove from unloading_chunks.
                return false;
            }
            true
        });
    }

    /// Updates the player's status in the chunk map.
    pub fn update_player_status(&self, player: &Player) {
        let current_chunk_pos = *player.last_chunk_pos.lock();
        let view_distance = STEEL_CONFIG.view_distance;

        let new_view = PlayerChunkView::new(current_chunk_pos, view_distance);
        let mut last_view_guard = player.last_tracking_view.lock();

        if last_view_guard.as_ref() != Some(&new_view) {
            let mut chunk_tickets = self.chunk_tickets.lock();

            let connection = &player.connection;

            if let Some(last_view) = last_view_guard.as_ref() {
                if last_view.center != new_view.center
                    || last_view.view_distance != new_view.view_distance
                {
                    chunk_tickets.remove_ticket(
                        last_view.center,
                        MAX_VIEW_DISTANCE.saturating_sub(last_view.view_distance),
                    );
                    chunk_tickets.add_ticket(
                        new_view.center,
                        MAX_VIEW_DISTANCE.saturating_sub(new_view.view_distance),
                    );

                    connection.send_packet(CSetChunkCenter {
                        x: new_view.center.0.x,
                        y: new_view.center.0.y,
                    });
                }

                // We lock here to ensure we have unique access for the duration of the diff
                let mut chunk_sender = player.chunk_sender.lock();
                PlayerChunkView::difference(
                    last_view,
                    &new_view,
                    |pos, chunk_sender| {
                        chunk_sender.mark_chunk_pending_to_send(pos);
                    },
                    |pos, chunk_sender| {
                        chunk_sender.drop_chunk(connection, pos);
                    },
                    &mut chunk_sender,
                );
            } else {
                chunk_tickets.add_ticket(
                    new_view.center,
                    MAX_VIEW_DISTANCE.saturating_sub(new_view.view_distance),
                );

                let mut chunk_sender = player.chunk_sender.lock();
                new_view.for_each(|pos| {
                    chunk_sender.mark_chunk_pending_to_send(pos);
                });
            }

            *last_view_guard = Some(new_view);
        }
    }

    /// Removes a player from the chunk map.
    pub fn remove_player(&self, player: &Player) {
        // Okay to lock sync lock here cause it has low contention
        let mut last_view_guard = player.last_tracking_view.lock();
        if let Some(last_view) = last_view_guard.take() {
            drop(last_view_guard);
            let mut chunk_tickets = self.chunk_tickets.lock();
            chunk_tickets.remove_ticket(
                last_view.center,
                MAX_VIEW_DISTANCE.saturating_sub(last_view.view_distance),
            );
        }
    }
}
