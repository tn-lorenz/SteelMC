use rustc_hash::{FxHashMap, FxHashSet};
use std::{cmp::min, collections::VecDeque};
use steel_utils::ChunkPos;

/// A standard max level for chunks that are unloaded.
pub const MAX_LEVEL: u8 = 66;

/// Tracks chunk levels based on propagation.
pub struct ChunkTracker {
    /// Map of chunk positions to their current levels.
    levels: FxHashMap<ChunkPos, u8>,
    /// Queue of chunks to update.
    queue: VecDeque<ChunkPos>,
    /// Set of chunks currently in the queue (for deduplication).
    in_queue: FxHashSet<ChunkPos>,
    /// Pending changes from direct updates.
    pending_changes: Vec<(ChunkPos, u8, u8)>,
}

impl Default for ChunkTracker {
    fn default() -> Self {
        Self::new()
    }
}

impl ChunkTracker {
    /// Creates a new chunk tracker.
    #[must_use]
    pub fn new() -> Self {
        Self {
            levels: FxHashMap::default(),
            queue: VecDeque::new(),
            in_queue: FxHashSet::default(),
            pending_changes: Vec::new(),
        }
    }

    /// Gets the level of a chunk. Returns `MAX_LEVEL` if untracked.
    #[must_use]
    pub fn get_level(&self, pos: ChunkPos) -> u8 {
        *self.levels.get(&pos).unwrap_or(&MAX_LEVEL)
    }

    /// Updates the level of a chunk from a source (ticket).
    pub fn update(
        &mut self,
        pos: ChunkPos,
        _new_ticket_level: u8,
        _get_ticket_level: impl Fn(ChunkPos) -> u8,
    ) {
        // We don't need to use `new_ticket_level` directly or recurse.
        // We just mark this chunk as needing an update.
        // The process loop will check `get_ticket_level`.
        if !self.in_queue.contains(&pos) {
            self.in_queue.insert(pos);
            self.queue.push_back(pos);
        }
    }

    /// Processes all pending updates in the queue.
    pub fn process_all_updates(
        &mut self,
        get_ticket_level: impl Fn(ChunkPos) -> u8,
    ) -> Vec<(ChunkPos, u8, u8)> {
        let mut changes = std::mem::take(&mut self.pending_changes);

        while let Some(pos) = self.queue.pop_front() {
            self.in_queue.remove(&pos);

            let old_level = self.get_level(pos);
            let ticket_level = get_ticket_level(pos);

            // Check neighbors to find the best level from them.
            let neighbors = [
                ChunkPos::new(pos.0.x + 1, pos.0.y),
                ChunkPos::new(pos.0.x - 1, pos.0.y),
                ChunkPos::new(pos.0.x, pos.0.y + 1),
                ChunkPos::new(pos.0.x, pos.0.y - 1),
            ];

            let mut best_neighbor = MAX_LEVEL;
            for n in neighbors {
                let n_level = self.get_level(n);
                if n_level < MAX_LEVEL {
                    best_neighbor = min(best_neighbor, n_level);
                }
            }

            // Calculate new level based on source (ticket) and neighbors.
            // Note: Propagation adds 1 to neighbor level.
            let propagated_level = if best_neighbor == MAX_LEVEL {
                MAX_LEVEL
            } else {
                best_neighbor + 1
            };

            let new_level = min(ticket_level, propagated_level);

            if new_level != old_level {
                self.levels.insert(pos, new_level);
                changes.push((pos, old_level, new_level));

                // If our level changed, our neighbors might need to update.
                // (Either we improved so they might improve, OR we degraded so they might degrade).
                for n in neighbors {
                    if !self.in_queue.contains(&n) {
                        self.in_queue.insert(n);
                        self.queue.push_back(n);
                    }
                }

                // If we degraded, we might need to re-evaluate ourselves if our new level relies on a neighbor
                // that depended on us (circular dependency breaker).
                // Actually, simple iterative updates handle this eventually, but enqueuing self again
                // is sometimes needed if we are not stable?
                // With standard Bellman-Ford/Dijkstra on this grid, simple neighbor enqueue is usually sufficient.
            }
        }

        changes
    }
}
