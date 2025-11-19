use std::collections::{BTreeMap, HashMap, VecDeque};
use steel_utils::ChunkPos;

/// A standard max level for chunks that are unloaded.
pub const MAX_LEVEL: u8 = 66;

/// Tracks chunk levels based on propagation.
pub struct ChunkTracker {
    /// Map of chunk positions to their current levels.
    levels: BTreeMap<ChunkPos, u8>,
    /// Queue of chunks to update, keyed by level.
    queue: BTreeMap<u8, VecDeque<ChunkPos>>,
    /// Map of chunks currently in the queue to their queued level.
    /// Used to avoid duplicates and handle priority updates.
    computed_levels: HashMap<ChunkPos, u8>,
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
            levels: BTreeMap::new(),
            queue: BTreeMap::new(),
            computed_levels: HashMap::new(),
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
        get_ticket_level: impl Fn(ChunkPos) -> u8,
    ) {
        let current_level = self.get_level(pos);

        if new_ticket_level < current_level {
            // Case 1: Improvement. Just queue it.
            self.enqueue(pos, new_ticket_level);
        } else if new_ticket_level > current_level {
            // Case 2: Degradation. Re-evaluate.
            self.compute_level(pos, &get_ticket_level, true);
        }
    }

    fn enqueue(&mut self, pos: ChunkPos, level: u8) {
        if let Some(&old_level) = self.computed_levels.get(&pos)
            && old_level <= level
        {
            return;
        }
        self.computed_levels.insert(pos, level);
        self.queue.entry(level).or_default().push_back(pos);
    }

    /// Re-evaluates a chunk's level based on tickets and neighbors.
    fn compute_level<F>(&mut self, pos: ChunkPos, get_ticket_level: &F, force_reset: bool)
    where
        F: Fn(ChunkPos) -> u8,
    {
        let old_level = self.get_level(pos);
        let mut best_level = get_ticket_level(pos);

        let neighbors = [
            ChunkPos::new(pos.0.x + 1, pos.0.y),
            ChunkPos::new(pos.0.x - 1, pos.0.y),
            ChunkPos::new(pos.0.x, pos.0.y + 1),
            ChunkPos::new(pos.0.x, pos.0.y - 1),
        ];

        for neighbor in neighbors {
            let n_level = self.get_level(neighbor);
            if n_level < MAX_LEVEL {
                best_level = best_level.min(n_level + 1);
            }
        }

        if best_level == old_level && !force_reset {
            return;
        }

        if best_level < old_level {
            // Improvement
            self.enqueue(pos, best_level);
        } else if best_level > old_level {
            // Degradation: set new level, then recurse to neighbors that depended on us.
            self.levels.insert(pos, best_level);

            for neighbor in neighbors {
                let n_level = self.get_level(neighbor);
                if n_level == old_level + 1 {
                    // Dependent neighbor needs re-evaluation.
                    self.compute_level(neighbor, get_ticket_level, true);
                }
            }
        }
    }

    /// Processes all pending updates in the queue.
    ///
    /// # Panics
    /// Panics if the queue state is inconsistent (key found but removal failed).
    pub fn process_all_updates(
        &mut self,
        _get_ticket_level: impl Fn(ChunkPos) -> u8,
    ) -> Vec<(ChunkPos, u8, u8)> {
        let mut changes = Vec::new();

        loop {
            // Process levels in increasing order.
            let entry = self.queue.keys().next().copied();
            let Some(level) = entry else { break };

            let mut chunks = self.queue.remove(&level).expect("Queue entry must exist");

            while let Some(pos) = chunks.pop_front() {
                // Check if this entry is stale or if we have a better one queued.
                match self.computed_levels.get(&pos) {
                    Some(&computed) if computed != level => continue, // Stale
                    Some(_) => {
                        self.computed_levels.remove(&pos);
                    } // Valid, consume
                    None => continue, // Should not happen if logic is correct, but safe to skip
                }

                let current_level = self.get_level(pos);

                if level >= current_level {
                    continue;
                }

                self.levels.insert(pos, level);
                changes.push((pos, current_level, level));

                if level < MAX_LEVEL {
                    let next = level + 1;
                    let neighbors = [
                        ChunkPos::new(pos.0.x + 1, pos.0.y),
                        ChunkPos::new(pos.0.x - 1, pos.0.y),
                        ChunkPos::new(pos.0.x, pos.0.y + 1),
                        ChunkPos::new(pos.0.x, pos.0.y - 1),
                    ];

                    for neighbor in neighbors {
                        if self.get_level(neighbor) > next {
                            self.enqueue(neighbor, next);
                        }
                    }
                }
            }
        }

        changes
    }
}
