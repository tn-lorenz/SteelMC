//! Chunk ticket management for tracking chunk levels and propagation.
#![allow(missing_docs)]

use rustc_hash::FxHashMap;
use smallvec::SmallVec;
use steel_utils::ChunkPos;

use crate::chunk::{chunk_access::ChunkStatus, chunk_pyramid::GENERATION_PYRAMID};

/// The maximum view distance for players.
pub const MAX_VIEW_DISTANCE: u8 = 32;
const RADIUS_AROUND_FULL_CHUNK: u8 = GENERATION_PYRAMID
    .get_step_to(ChunkStatus::Full)
    .accumulated_dependencies
    .get_radius_of(ChunkStatus::Empty) as u8;
const MAX_LEVEL: u8 = MAX_VIEW_DISTANCE + RADIUS_AROUND_FULL_CHUNK;

#[must_use]
pub fn is_full(level: u8) -> bool {
    level <= MAX_VIEW_DISTANCE
}

#[must_use]
pub fn generation_status(level: Option<u8>) -> Option<ChunkStatus> {
    match level {
        None => None,
        Some(level) => {
            if is_full(level) {
                Some(ChunkStatus::Full)
            } else {
                let distance = (level - MAX_VIEW_DISTANCE) as usize;
                // Fallback to None if distance is out of bounds (simulating Vanilla logic)
                GENERATION_PYRAMID
                    .get_step_to(ChunkStatus::Full)
                    .accumulated_dependencies
                    .get(distance)
            }
        }
    }
}

const NUM_BUCKETS: usize = MAX_LEVEL as usize + 1;

/// Up to 4 tickets stored inline per position.
type TicketLevels = SmallVec<[u8; 4]>;

/// A level change for a chunk position.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LevelChange {
    pub pos: ChunkPos,
    /// `Some(level)` if level changed or added, `None` if removed.
    pub new_level: Option<u8>,
}

/// Chunk ticket propagation using Dial's algorithm.
/// Lower levels = higher priority. Multiple tickets per position supported.
#[derive(Debug)]
pub struct ChunkTicketManager {
    tickets: FxHashMap<ChunkPos, TicketLevels>,
    levels: FxHashMap<ChunkPos, u8>,
    dirty: bool,
    buckets: [Vec<ChunkPos>; NUM_BUCKETS],
    /// Tracks changes from the last `run_all_updates()` call.
    changes: Vec<LevelChange>,
}

impl Default for ChunkTicketManager {
    fn default() -> Self {
        Self::new()
    }
}

impl ChunkTicketManager {
    #[must_use]
    pub fn new() -> Self {
        Self {
            tickets: FxHashMap::default(),
            levels: FxHashMap::default(),
            dirty: false,
            buckets: std::array::from_fn(|_| Vec::new()),
            changes: Vec::new(),
        }
    }

    /// Adds a ticket. Multiple tickets can exist at the same position.
    pub fn add_ticket(&mut self, pos: ChunkPos, level: u8) {
        if level > MAX_LEVEL {
            return;
        }
        self.tickets.entry(pos).or_default().push(level);
        self.dirty = true;
    }

    /// Removes one ticket matching (pos, level). Returns true if found.
    pub fn remove_ticket(&mut self, pos: ChunkPos, level: u8) -> bool {
        if let Some(levels) = self.tickets.get_mut(&pos)
            && let Some(idx) = levels.iter().position(|&l| l == level)
        {
            levels.swap_remove(idx);
            self.dirty = true;
            if levels.is_empty() {
                self.tickets.remove(&pos);
            }
            return true;
        }
        false
    }

    /// Removes all tickets at position.
    pub fn remove_all_tickets_at(&mut self, pos: ChunkPos) -> Option<TicketLevels> {
        let removed = self.tickets.remove(&pos);
        if removed.is_some() {
            self.dirty = true;
        }
        removed
    }

    /// Returns the minimum ticket level at position.
    #[must_use]
    pub fn get_ticket(&self, pos: ChunkPos) -> Option<u8> {
        self.tickets.get(&pos).and_then(|l| l.iter().min().copied())
    }

    #[must_use]
    pub fn get_tickets_at(&self, pos: ChunkPos) -> Option<&[u8]> {
        self.tickets.get(&pos).map(smallvec::SmallVec::as_slice)
    }

    /// Iterator over (position, `min_level`) for all ticket sources.
    pub fn tickets(&self) -> impl Iterator<Item = (ChunkPos, u8)> + '_ {
        self.tickets
            .iter()
            .filter_map(|(&pos, levels)| levels.iter().min().map(|&level| (pos, level)))
    }

    #[must_use]
    pub fn ticket_count(&self) -> usize {
        self.tickets.values().map(smallvec::SmallVec::len).sum()
    }

    #[must_use]
    pub fn ticket_position_count(&self) -> usize {
        self.tickets.len()
    }

    /// Propagates all tickets using Dial's algorithm. Only runs if dirty.
    /// Returns a slice of changes (added/updated/removed levels).
    pub fn run_all_updates(&mut self) -> &[LevelChange] {
        self.changes.clear();

        if !self.dirty {
            return &self.changes;
        }

        // Swap out old levels to compare against later
        let old_levels = std::mem::take(&mut self.levels);

        // Seed buckets with ticket sources
        for (&pos, levels) in &self.tickets {
            if let Some(&min_level) = levels.iter().min() {
                self.buckets[min_level as usize].push(pos);
            }
        }

        self.dirty = false;

        // Process buckets low to high
        for current_level in 0..NUM_BUCKETS {
            let current_level = current_level as u8;
            let mut i = 0;

            while i < self.buckets[current_level as usize].len() {
                let current_pos = self.buckets[current_level as usize][i];
                i += 1;

                // Skip if already has equal or better level
                if self
                    .levels
                    .get(&current_pos)
                    .is_some_and(|&e| e <= current_level)
                {
                    continue;
                }

                self.levels.insert(current_pos, current_level);

                let next_level = current_level + 1;
                if next_level <= MAX_LEVEL {
                    for neighbor in current_pos.neighbors() {
                        let dominated =
                            self.levels.get(&neighbor).is_some_and(|&e| e <= next_level);

                        if !dominated {
                            self.buckets[next_level as usize].push(neighbor);
                        }
                    }
                }
            }

            self.buckets[current_level as usize].clear();
        }

        // Find changed/added levels
        for (&pos, &new_level) in &self.levels {
            match old_levels.get(&pos) {
                Some(&old_level) if old_level == new_level => {} // No change
                _ => self.changes.push(LevelChange {
                    pos,
                    new_level: Some(new_level),
                }),
            }
        }

        // Find removed levels
        for &pos in old_levels.keys() {
            if !self.levels.contains_key(&pos) {
                self.changes.push(LevelChange {
                    pos,
                    new_level: None,
                });
            }
        }

        &self.changes
    }

    /// Returns the propagated level at position. Call `run_all_updates()` first.
    #[must_use]
    pub fn get_level(&self, pos: ChunkPos) -> Option<u8> {
        self.levels.get(&pos).copied()
    }

    #[allow(dead_code)]
    #[must_use]
    fn is_dirty(&self) -> bool {
        self.dirty
    }

    #[allow(dead_code)]
    fn clear(&mut self) {
        self.tickets.clear();
        self.levels.clear();
        self.dirty = false;
        self.changes.clear();
    }

    pub fn iter_levels(&self) -> impl Iterator<Item = (ChunkPos, u8)> + '_ {
        self.levels.iter().map(|(&pos, &level)| (pos, level))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_ticket_propagation() {
        let mut manager = ChunkTicketManager::new();
        manager.add_ticket(ChunkPos::new(0, 0), 0);
        manager.run_all_updates();

        assert_eq!(manager.get_level(ChunkPos::new(0, 0)), Some(0));
        assert_eq!(manager.get_level(ChunkPos::new(-1, -1)), Some(1));
        assert_eq!(manager.get_level(ChunkPos::new(0, -1)), Some(1));
        assert_eq!(manager.get_level(ChunkPos::new(1, 0)), Some(1));
        assert_eq!(manager.get_level(ChunkPos::new(-2, -2)), Some(2));
    }

    #[test]
    fn test_deferred_updates() {
        let mut manager = ChunkTicketManager::new();
        manager.add_ticket(ChunkPos::new(0, 0), 0);

        assert!(manager.is_dirty());
        assert_eq!(manager.get_level(ChunkPos::new(0, 0)), None);

        manager.run_all_updates();
        assert!(!manager.is_dirty());
        assert_eq!(manager.get_level(ChunkPos::new(0, 0)), Some(0));
    }

    #[test]
    fn test_multiple_tickets_same_position() {
        let mut manager = ChunkTicketManager::new();
        manager.add_ticket(ChunkPos::new(0, 0), 2);
        manager.add_ticket(ChunkPos::new(0, 0), 0);
        manager.add_ticket(ChunkPos::new(0, 0), 1);
        manager.run_all_updates();

        assert_eq!(manager.get_ticket(ChunkPos::new(0, 0)), Some(0));
        assert_eq!(manager.get_level(ChunkPos::new(0, 0)), Some(0));
    }

    #[test]
    fn test_overlapping_propagation() {
        let mut manager = ChunkTicketManager::new();
        manager.add_ticket(ChunkPos::new(0, 0), 0);
        manager.add_ticket(ChunkPos::new(3, 0), 0);
        manager.run_all_updates();

        assert_eq!(manager.get_level(ChunkPos::new(1, 0)), Some(1));
        assert_eq!(manager.get_level(ChunkPos::new(2, 0)), Some(1));
    }

    #[test]
    fn test_remove_ticket() {
        let mut manager = ChunkTicketManager::new();
        manager.add_ticket(ChunkPos::new(0, 0), 0);
        manager.add_ticket(ChunkPos::new(5, 0), 0);
        manager.run_all_updates();

        assert_eq!(manager.get_level(ChunkPos::new(0, 0)), Some(0));
        assert_eq!(manager.get_level(ChunkPos::new(5, 0)), Some(0));

        assert!(manager.remove_ticket(ChunkPos::new(0, 0), 0));
        manager.run_all_updates();

        assert_eq!(manager.get_level(ChunkPos::new(0, 0)), Some(5));
        assert_eq!(manager.get_level(ChunkPos::new(5, 0)), Some(0));
    }

    #[test]
    fn test_remove_all_tickets_at_position() {
        let mut manager = ChunkTicketManager::new();
        manager.add_ticket(ChunkPos::new(0, 0), 0);
        manager.run_all_updates();

        manager.remove_ticket(ChunkPos::new(0, 0), 0);
        manager.run_all_updates();

        assert_eq!(manager.get_level(ChunkPos::new(0, 0)), None);
    }

    #[test]
    fn test_multiple_tickets_same_position_with_removal() {
        let mut manager = ChunkTicketManager::new();
        manager.add_ticket(ChunkPos::new(0, 0), 0);
        manager.add_ticket(ChunkPos::new(0, 0), 2);
        manager.add_ticket(ChunkPos::new(0, 0), 1);
        manager.run_all_updates();

        assert_eq!(manager.get_ticket(ChunkPos::new(0, 0)), Some(0));
        assert_eq!(manager.ticket_count(), 3);

        manager.remove_ticket(ChunkPos::new(0, 0), 0);
        manager.run_all_updates();
        assert_eq!(manager.get_ticket(ChunkPos::new(0, 0)), Some(1));

        manager.remove_ticket(ChunkPos::new(0, 0), 1);
        manager.run_all_updates();
        assert_eq!(manager.get_ticket(ChunkPos::new(0, 0)), Some(2));
    }

    #[test]
    fn test_duplicate_tickets_same_level() {
        let mut manager = ChunkTicketManager::new();
        manager.add_ticket(ChunkPos::new(0, 0), 0);
        manager.add_ticket(ChunkPos::new(0, 0), 0);
        manager.run_all_updates();

        assert_eq!(manager.ticket_count(), 2);

        manager.remove_ticket(ChunkPos::new(0, 0), 0);
        manager.run_all_updates();
        assert_eq!(manager.ticket_count(), 1);
        assert_eq!(manager.get_level(ChunkPos::new(0, 0)), Some(0));

        manager.remove_ticket(ChunkPos::new(0, 0), 0);
        manager.run_all_updates();
        assert_eq!(manager.ticket_count(), 0);
        assert_eq!(manager.get_level(ChunkPos::new(0, 0)), None);
    }

    #[test]
    fn test_no_recalculation_when_clean() {
        let mut manager = ChunkTicketManager::new();
        manager.add_ticket(ChunkPos::new(0, 0), 0);
        manager.run_all_updates();

        assert!(!manager.is_dirty());
        manager.run_all_updates();
        assert!(!manager.is_dirty());
    }
}
