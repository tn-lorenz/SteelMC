use rustc_hash::{FxHashMap, FxHashSet};
use std::{cmp::min, collections::VecDeque};
use steel_utils::ChunkPos;

/// A standard max level for chunks that are unloaded.
pub const MAX_LEVEL: u8 = 66;

/// Tracks chunk levels based on propagation.
pub struct ChunkTracker {
    /// Map of chunk positions to their current levels.
    levels: FxHashMap<ChunkPos, u8>,
    /// Priority queue: array of queues, one per priority level (0 to `MAX_LEVEL`).
    /// Lower priority values are processed first.
    priority_queue: Vec<VecDeque<ChunkPos>>,
    /// Set of chunks currently in the queue (for deduplication).
    in_queue: FxHashSet<ChunkPos>,
    /// Computed levels - the "desired" level for each chunk in the queue.
    computed_levels: FxHashMap<ChunkPos, u8>,
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
        // Initialize priority queue with one VecDeque per priority level
        let mut priority_queue = Vec::with_capacity((MAX_LEVEL + 1) as usize);
        for _ in 0..=MAX_LEVEL {
            priority_queue.push(VecDeque::new());
        }

        Self {
            levels: FxHashMap::default(),
            priority_queue,
            in_queue: FxHashSet::default(),
            computed_levels: FxHashMap::default(),
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
        new_ticket_level: u8,
        _get_ticket_level: impl Fn(ChunkPos) -> u8,
    ) {
        let current_level = self.get_level(pos);

        // Compute best level from new ticket level AND neighbors
        let mut best_level = new_ticket_level;

        if best_level > 0 {
            let neighbors = [
                ChunkPos::new(pos.0.x + 1, pos.0.y),
                ChunkPos::new(pos.0.x - 1, pos.0.y),
                ChunkPos::new(pos.0.x, pos.0.y + 1),
                ChunkPos::new(pos.0.x, pos.0.y - 1),
            ];

            for neighbor in neighbors {
                let neighbor_level = self.get_level(neighbor);
                let propagated = min(neighbor_level + 1, MAX_LEVEL);
                best_level = min(best_level, propagated);
            }
        }

        let computed_level = best_level;

        // Calculate priority: min(current_level, computed_level, MAX_LEVEL)
        let priority = min(min(current_level, computed_level), MAX_LEVEL);

        // Enqueue at the calculated priority level
        self.enqueue(pos, priority, computed_level);
    }

    /// Enqueues a chunk at the specified priority level.
    /// If the chunk is already queued, updates its priority and computed level if needed.
    fn enqueue(&mut self, pos: ChunkPos, priority: u8, computed_level: u8) {
        let priority = priority as usize;

        if let Some(&old_computed) = self.computed_levels.get(&pos) {
            // Chunk is already in queue - check if we need to update priority
            if old_computed != computed_level {
                let old_priority = min(min(self.get_level(pos), old_computed), MAX_LEVEL) as usize;

                if old_priority != priority {
                    // Remove from old priority queue and add to new one
                    if let Some(idx) = self.priority_queue[old_priority]
                        .iter()
                        .position(|&p| p == pos)
                    {
                        self.priority_queue[old_priority].remove(idx);
                    }
                    self.priority_queue[priority].push_back(pos);
                }

                // Update computed level
                self.computed_levels.insert(pos, computed_level);
            }
        } else {
            // Not in queue yet - add it
            self.in_queue.insert(pos);
            self.priority_queue[priority].push_back(pos);
            self.computed_levels.insert(pos, computed_level);
        }
    }

    /// Dequeues the next chunk from the lowest priority level.
    fn dequeue(&mut self) -> Option<ChunkPos> {
        for queue in &mut self.priority_queue {
            if let Some(pos) = queue.pop_front() {
                self.in_queue.remove(&pos);
                return Some(pos);
            }
        }
        None
    }

    /// Processes all pending updates in the queue.
    pub fn process_all_updates(
        &mut self,
        get_ticket_level: impl Fn(ChunkPos) -> u8,
    ) -> Vec<(ChunkPos, u8, u8)> {
        let mut changes = Vec::new();

        while let Some(pos) = self.dequeue() {
            let current_level = self.get_level(pos);
            let computed_level = self.computed_levels.remove(&pos).unwrap_or(MAX_LEVEL);

            if computed_level < current_level {
                // Level is decreasing - update and propagate decrease to neighbors
                self.levels.insert(pos, computed_level);
                changes.push((pos, current_level, computed_level));
                self.check_neighbors_after_update(pos, computed_level, true, &get_ticket_level);
            } else if computed_level > current_level {
                // Level is increasing - first set to MAX, then propagate
                self.levels.insert(pos, MAX_LEVEL);
                changes.push((pos, current_level, MAX_LEVEL));

                // Re-enqueue if not yet at desired level
                if computed_level != MAX_LEVEL {
                    let priority = min(MAX_LEVEL, computed_level);
                    self.enqueue(pos, priority, computed_level);
                }

                self.check_neighbors_after_update(pos, current_level, false, &get_ticket_level);
            }
        }

        changes
    }

    /// Checks and updates neighbors after a level change.
    fn check_neighbors_after_update(
        &mut self,
        pos: ChunkPos,
        level: u8,
        only_decrease: bool,
        get_ticket_level: &impl Fn(ChunkPos) -> u8,
    ) {
        // Skip neighbor updates if only decreasing and level is near max
        // (optimization from Java implementation)
        if only_decrease && level >= MAX_LEVEL - 1 {
            return;
        }

        let neighbors = [
            ChunkPos::new(pos.0.x + 1, pos.0.y),
            ChunkPos::new(pos.0.x - 1, pos.0.y),
            ChunkPos::new(pos.0.x, pos.0.y + 1),
            ChunkPos::new(pos.0.x, pos.0.y - 1),
        ];

        for neighbor in neighbors {
            self.check_neighbor(pos, neighbor, level, only_decrease, get_ticket_level);
        }
    }

    /// Checks a specific neighbor and enqueues it if needed.
    fn check_neighbor(
        &mut self,
        from: ChunkPos,
        to: ChunkPos,
        from_level: u8,
        only_decrease: bool,
        get_ticket_level: &impl Fn(ChunkPos) -> u8,
    ) {
        let to_level = self.get_level(to);
        let propagated_level = min(from_level + 1, MAX_LEVEL);

        // Check against currently computed level in queue if present, otherwise current level
        let stored_computed = self.computed_levels.get(&to).copied();
        let target_level = stored_computed.unwrap_or(to_level);

        let computed_level = if only_decrease {
            // When only decreasing, just propagate the level
            min(target_level, propagated_level)
        } else if propagated_level == target_level {
            // When increasing, if 'to' (or its pending update) derived its level from 'from',
            // we must recompute 'to's level ignoring 'from' (since 'from' increased).
            self.compute_level(to, from, MAX_LEVEL, get_ticket_level)
        } else {
            // 'to' has a better source or is otherwise unaffected
            return;
        };

        if computed_level != target_level {
            let priority = min(min(to_level, computed_level), MAX_LEVEL);
            self.enqueue(to, priority, computed_level);
        } else if stored_computed.is_some() && computed_level == to_level {
            // If we computed the same as current level, and it was in queue, we should remove it?
            // Java does: this.priorityQueue.dequeue(to, oldPriority, this.levelCount);
            // this.computedLevels.remove(to);
            // But our enqueue handles updates. If we want to remove, we don't have a remove method exposed easily here.
            // However, leaving it in queue with level == current level is harmless, just redundant processing.
            // But wait, if computed_level == to_level, process_all_updates will dequeue it.
            // computed (X) == current (X).
            // It does nothing.
            // So it effectively removes it.
        }
    }

    /// Computes the level for a node based on all neighbors and ticket level.
    fn compute_level(
        &self,
        pos: ChunkPos,
        known_parent: ChunkPos,
        known_level_from_parent: u8,
        get_ticket_level: &impl Fn(ChunkPos) -> u8,
    ) -> u8 {
        let ticket_level = get_ticket_level(pos);
        let mut best_level = min(ticket_level, known_level_from_parent);

        if best_level == 0 {
            return 0;
        }

        let neighbors = [
            ChunkPos::new(pos.0.x + 1, pos.0.y),
            ChunkPos::new(pos.0.x - 1, pos.0.y),
            ChunkPos::new(pos.0.x, pos.0.y + 1),
            ChunkPos::new(pos.0.x, pos.0.y - 1),
        ];

        for neighbor in neighbors {
            if neighbor != known_parent {
                let neighbor_level = self.get_level(neighbor);
                let propagated = min(neighbor_level + 1, MAX_LEVEL);
                best_level = min(best_level, propagated);

                if best_level == 0 {
                    return 0;
                }
            }
        }

        best_level
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ticket_propagation() {
        let mut tracker = ChunkTracker::new();
        let pos = ChunkPos::new(0, 0);

        // Add ticket
        tracker.update(pos, 31, |_| MAX_LEVEL);
        tracker.process_all_updates(|p| if p == pos { 31 } else { MAX_LEVEL });

        assert_eq!(tracker.get_level(pos), 31);
        assert_eq!(tracker.get_level(ChunkPos::new(1, 0)), 32);
    }

    #[test]
    fn test_ticket_removal() {
        let mut tracker = ChunkTracker::new();
        let pos = ChunkPos::new(0, 0);

        // Setup initial state
        tracker.update(pos, 31, |_| MAX_LEVEL);
        tracker.process_all_updates(|p| if p == pos { 31 } else { MAX_LEVEL });

        // Remove ticket
        tracker.update(pos, MAX_LEVEL, |_| MAX_LEVEL);
        tracker.process_all_updates(|_| MAX_LEVEL);

        assert_eq!(tracker.get_level(pos), MAX_LEVEL);
        assert_eq!(tracker.get_level(ChunkPos::new(1, 0)), MAX_LEVEL);
    }

    #[test]
    fn test_circular_dependency_unloading() {
        let mut tracker = ChunkTracker::new();
        let center = ChunkPos::new(0, 0);
        let neighbor = ChunkPos::new(1, 0);

        // Setup: Center has ticket 31. Neighbor has ticket 33 (weaker).
        // Center -> 31. Neighbor -> 32 (from center).
        tracker.update(center, 31, |_| MAX_LEVEL);
        tracker.update(neighbor, 33, |_| MAX_LEVEL);

        tracker.process_all_updates(|p| {
            if p == center {
                31
            } else if p == neighbor {
                33
            } else {
                MAX_LEVEL
            }
        });

        assert_eq!(tracker.get_level(center), 31);
        assert_eq!(tracker.get_level(neighbor), 32); // Propagated from center is better than 33

        // Remove center ticket. Neighbor ticket remains 33.
        // Center should become 34 (from neighbor 33). Neighbor becomes 33 (its ticket).
        tracker.update(center, MAX_LEVEL, |_| MAX_LEVEL);
        tracker.process_all_updates(|p| if p == neighbor { 33 } else { MAX_LEVEL });

        assert_eq!(tracker.get_level(neighbor), 33);
        assert_eq!(tracker.get_level(center), 34);
    }

    #[test]
    fn test_circular_dependency_full_unload() {
        let mut tracker = ChunkTracker::new();
        let center = ChunkPos::new(0, 0);
        let neighbor = ChunkPos::new(1, 0);

        // Setup: Center has ticket 31.
        tracker.update(center, 31, |_| MAX_LEVEL);
        tracker.process_all_updates(|p| if p == center { 31 } else { MAX_LEVEL });

        assert_eq!(tracker.get_level(center), 31);
        assert_eq!(tracker.get_level(neighbor), 32);

        // Remove ticket. Both should unload.
        tracker.update(center, MAX_LEVEL, |_| MAX_LEVEL);
        tracker.process_all_updates(|_| MAX_LEVEL);

        assert_eq!(tracker.get_level(center), MAX_LEVEL);
        assert_eq!(tracker.get_level(neighbor), MAX_LEVEL);
    }
}
