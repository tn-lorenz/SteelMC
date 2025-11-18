//! `ChunkGenerationTask` handles the generation process for chunks.
use std::{
    future::Future,
    pin::Pin,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
};

use futures::future::join_all;
use steel_utils::{ChunkPos, math::Vector2};

use crate::chunk::{
    chunk_access::ChunkStatus,
    chunk_holder::ChunkHolder,
    chunk_map::ChunkMap,
    chunk_pyramid::{GENERATION_PYRAMID, LOADING_PYRAMID},
};

/// A pre-filled 2D cache of elements, efficient for async creation.
pub struct StaticCache2D<T> {
    min_x: i32,
    min_z: i32,
    size: i32,
    /// Cache stored in row-major order (Z-then-X).
    cache: Vec<T>,
}

impl<T> StaticCache2D<T> {
    /// Creates a `StaticCache2D` by concurrently populating it via a factory.
    #[allow(clippy::missing_panics_doc)]
    pub async fn create<F, Fut>(center_x: i32, center_z: i32, radius: i32, mut factory: F) -> Self
    where
        F: FnMut(i32, i32) -> Fut,
        Fut: Future<Output = T>,
    {
        let size = radius * 2 + 1;
        let min_x = center_x - radius;
        let min_z = center_z - radius;
        let cap = usize::try_from(size * size).expect("Cache size negative");
        let mut futures = Vec::with_capacity(cap);

        for z_offset in 0..size {
            for x_offset in 0..size {
                futures.push(factory(min_x + x_offset, min_z + z_offset));
            }
        }

        Self {
            min_x,
            min_z,
            size,
            cache: join_all(futures).await,
        }
    }

    /// Gets a reference to an element by world coordinates.
    ///
    /// # Panics
    /// Panics if coordinates are out of bounds.
    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub fn get(&self, x: i32, z: i32) -> &T {
        let rel_x = x - self.min_x;
        let rel_z = z - self.min_z;

        if rel_x >= 0 && rel_x < self.size && rel_z >= 0 && rel_z < self.size {
            let index = usize::try_from(rel_z * self.size + rel_x).expect("Index error");
            &self.cache[index]
        } else {
            panic!(
                "Out of bounds: ({x}, {z}) vs [({}, {}) to ({}, {})]",
                self.min_x,
                self.min_z,
                self.min_x + self.size - 1,
                self.min_z + self.size - 1
            );
        }
    }
}

/// A pinned future representing a neighbor's readiness.
pub type NeighborReady = Pin<Box<dyn Future<Output = Option<()>> + Send + Sync>>;

/// A task responsible for driving a chunk to a target status.
pub struct ChunkGenerationTask {
    /// The chunk map associated with this task.
    pub chunk_map: Arc<ChunkMap>,
    /// The chunk position.
    pub pos: ChunkPos,
    /// The target generation status.
    pub target_status: ChunkStatus,
    /// The status scheduled for generation. Protected by a mutex for safe concurrent access.
    pub scheduled_status: parking_lot::Mutex<Option<ChunkStatus>>,
    /// Flag indicating if the task is cancelled.
    pub marked_for_cancel: AtomicBool,
    /// Futures for neighbors. Protected by a mutex.
    pub neighbor_ready: parking_lot::Mutex<Vec<NeighborReady>>,
    /// Cache of required chunks.
    pub cache: Arc<StaticCache2D<Arc<ChunkHolder>>>,
    /// Whether generation is required for this task.
    pub needs_generation: AtomicBool,
}

impl ChunkGenerationTask {
    /// Creates a new generation task.
    #[must_use]
    #[allow(clippy::unwrap_used, clippy::missing_panics_doc)]
    pub async fn new(pos: ChunkPos, target_status: ChunkStatus, chunk_map: Arc<ChunkMap>) -> Self {
        let worst_case_radius = i32::try_from(
            GENERATION_PYRAMID
                .get_step_to(target_status)
                .accumulated_dependencies
                .get_radius_of(ChunkStatus::Empty),
        )
        .unwrap();

        let cache = StaticCache2D::create(pos.0.x, pos.0.y, worst_case_radius, async |x, y| {
            chunk_map
                .chunks
                .get_async(&ChunkPos(Vector2::new(x, y)))
                .await
                .expect("Scheduled chunk required")
                .clone()
        })
        .await;

        Self {
            chunk_map,
            pos,
            target_status,
            scheduled_status: parking_lot::Mutex::new(None),
            marked_for_cancel: AtomicBool::new(false),
            neighbor_ready: parking_lot::Mutex::new(Vec::new()),
            cache: Arc::new(cache),
            needs_generation: AtomicBool::new(true),
        }
    }

    /// Marks the task for cancellation.
    pub fn mark_for_cancel(&self) {
        self.marked_for_cancel.store(true, Ordering::Relaxed);
    }

    /// Schedules a chunk for a specific layer.
    ///
    /// # Panics
    /// Panics if generation is required but not expected.
    pub fn schedule_chunk_in_layer(
        &self,
        status: ChunkStatus,
        needs_generation: bool,
        chunk_holder: &Arc<ChunkHolder>,
    ) -> bool {
        let persisted_status = chunk_holder.persisted_status();
        let generate = status > persisted_status;
        let pyramid = if generate {
            &*GENERATION_PYRAMID
        } else {
            &*LOADING_PYRAMID
        };

        assert!(
            !generate || needs_generation,
            "Generation required but not expected for chunk load"
        );

        let future = chunk_holder.apply_step(
            pyramid.get_step_to(status),
            self.chunk_map.clone(),
            self.cache.clone(),
        );

        self.neighbor_ready.lock().push(future);
        true
    }

    /// Schedules tasks for the current layer's neighbors.
    pub fn schedule_layer(&self, status: ChunkStatus, needs_generation: bool) {
        // TODO: Implement dynamic radius (getRadiusForLayer)
        let radius = 1;
        for x in (self.pos.0.x - radius)..=(self.pos.0.x + radius) {
            for y in (self.pos.0.y - radius)..=(self.pos.0.y + radius) {
                let chunk_holder = self.cache.get(x, y);
                if self.marked_for_cancel.load(Ordering::Relaxed)
                    || !self.schedule_chunk_in_layer(status, needs_generation, chunk_holder)
                {
                    return;
                }
            }
        }
    }

    /// Schedules the next layer of generation dependencies.
    ///
    /// # Panics
    /// Panics if the schedule is invalid.
    pub fn schedule_next_layer(&self) {
        let status_to_schedule;
        if self.scheduled_status.lock().is_none() {
            status_to_schedule = ChunkStatus::Empty;
            // TODO: check canLoadWithoutGeneration
        } else if !self.needs_generation.load(Ordering::Relaxed)
            && *self.scheduled_status.lock() == Some(ChunkStatus::Empty)
        {
            self.needs_generation.store(true, Ordering::Relaxed);
            status_to_schedule = ChunkStatus::Empty;
        } else {
            status_to_schedule = self
                .scheduled_status
                .lock()
                .expect("Scheduled status missing")
                .next()
                .expect("Next status missing");
        }

        self.schedule_layer(
            status_to_schedule,
            self.needs_generation.load(Ordering::Relaxed),
        );
        self.scheduled_status.lock().replace(status_to_schedule);
    }

    /// Runs the generation task loop.
    pub async fn run(self: Arc<Self>) {
        log::info!("Running generation task for {:?}", self.pos);
        self.wait_for_scheduled_layers().await;

        if self.marked_for_cancel.load(Ordering::Relaxed)
            || self.scheduled_status.lock().unwrap_or(ChunkStatus::Empty) == self.target_status
        {
            return;
        }

        self.schedule_next_layer();
    }

    /// Waits for all scheduled neighbor tasks to complete.
    pub async fn wait_for_scheduled_layers(&self) {
        loop {
            let future = self.neighbor_ready.lock().pop();
            if let Some(future) = future {
                if future.await.is_none() {
                    self.mark_for_cancel();
                }
            } else {
                break;
            }
        }
    }
}
