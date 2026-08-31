//! Synchronous player simulation-distance tracking.

use std::mem;

use rustc_hash::{FxBuildHasher, FxHashMap};
use steel_utils::ChunkPos;

use crate::chunk::chunk_ticket_manager::ChunkTicketLevel;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct PlayerSimulationLevelChange {
    pub(crate) pos: ChunkPos,
    pub(crate) new_level: Option<ChunkTicketLevel>,
}

/// Tracks the simulation-distance contribution from player-occupied chunks.
///
/// Vanilla keeps player loading and player simulation in separate distance
/// trackers. Player simulation updates run synchronously before each world's
/// entity phase, while loading updates may continue through Steel's background
/// chunk scheduler.
#[derive(Debug, Default)]
pub(crate) struct PlayerSimulationTracker {
    players_per_chunk: FxHashMap<ChunkPos, usize>,
    levels: FxHashMap<ChunkPos, ChunkTicketLevel>,
    simulation_distance: Option<u8>,
    dirty: bool,
}

impl PlayerSimulationTracker {
    pub(crate) fn is_inactive(&self) -> bool {
        !self.dirty && self.players_per_chunk.is_empty() && self.levels.is_empty()
    }

    pub(crate) fn add_player(&mut self, pos: ChunkPos) {
        let player_count = self.players_per_chunk.entry(pos).or_default();
        assert_ne!(
            *player_count,
            usize::MAX,
            "player simulation source count exhausted"
        );
        if *player_count == 0 {
            self.dirty = true;
        }
        *player_count += 1;
    }

    pub(crate) fn remove_player(&mut self, pos: ChunkPos) {
        let remove_source = {
            let Some(player_count) = self.players_per_chunk.get_mut(&pos) else {
                panic!("player simulation source missing at {pos:?}");
            };
            assert_ne!(*player_count, 0, "player simulation source count is zero");
            *player_count -= 1;
            *player_count == 0
        };

        if remove_source {
            self.players_per_chunk.remove(&pos);
            self.dirty = true;
        }
    }

    pub(crate) fn move_player(&mut self, old_pos: ChunkPos, new_pos: ChunkPos) {
        if old_pos == new_pos {
            return;
        }
        self.remove_player(old_pos);
        self.add_player(new_pos);
    }

    pub(crate) fn run_all_updates(
        &mut self,
        simulation_distance: u8,
    ) -> Vec<PlayerSimulationLevelChange> {
        if self.simulation_distance != Some(simulation_distance) {
            self.simulation_distance = Some(simulation_distance);
            self.dirty = true;
        }
        if !self.dirty {
            return Vec::new();
        }

        let old_capacity = self.levels.capacity();
        let old_levels = mem::replace(
            &mut self.levels,
            FxHashMap::with_capacity_and_hasher(old_capacity, FxBuildHasher),
        );
        self.dirty = false;

        let source_level = ChunkTicketLevel::for_entity_ticking_radius(simulation_distance);
        let radius = i32::from(source_level.distance_to_block_ticking());
        for source_pos in self.players_per_chunk.keys() {
            for dz in -radius..=radius {
                for dx in -radius..=radius {
                    let distance = dx.abs().max(dz.abs()) as u8;
                    let Some(level) = source_level.with_distance(distance) else {
                        continue;
                    };
                    let pos = ChunkPos::new(source_pos.0.x + dx, source_pos.0.y + dz);
                    self.levels
                        .entry(pos)
                        .and_modify(|current| *current = (*current).min(level))
                        .or_insert(level);
                }
            }
        }

        let mut changes = Vec::new();
        for (&pos, &new_level) in &self.levels {
            if old_levels.get(&pos) != Some(&new_level) {
                changes.push(PlayerSimulationLevelChange {
                    pos,
                    new_level: Some(new_level),
                });
            }
        }
        for &pos in old_levels.keys() {
            if !self.levels.contains_key(&pos) {
                changes.push(PlayerSimulationLevelChange {
                    pos,
                    new_level: None,
                });
            }
        }
        changes
    }

    pub(crate) fn get_level(&self, pos: ChunkPos) -> Option<ChunkTicketLevel> {
        self.levels.get(&pos).copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simulation_distance_keeps_vanilla_block_ticking_outer_ring() {
        let center = ChunkPos::new(0, 0);
        let mut tracker = PlayerSimulationTracker::default();
        tracker.add_player(center);
        tracker.run_all_updates(1);

        assert!(
            tracker
                .get_level(ChunkPos::new(1, 0))
                .is_some_and(ChunkTicketLevel::is_entity_ticking)
        );
        assert_eq!(
            tracker.get_level(ChunkPos::new(2, 0)),
            Some(ChunkTicketLevel::BLOCK_TICKING_CHUNK)
        );
        assert_eq!(tracker.get_level(ChunkPos::new(3, 0)), None);
    }

    #[test]
    fn players_in_one_chunk_share_one_simulation_source() {
        let center = ChunkPos::new(4, -3);
        let mut tracker = PlayerSimulationTracker::default();
        tracker.add_player(center);
        tracker.add_player(center);
        tracker.run_all_updates(2);

        tracker.remove_player(center);
        assert_eq!(tracker.run_all_updates(2).len(), 0);
        assert!(
            tracker
                .get_level(center)
                .is_some_and(ChunkTicketLevel::is_entity_ticking)
        );

        tracker.remove_player(center);
        let changes = tracker.run_all_updates(2);
        assert!(
            changes
                .iter()
                .any(|change| change.pos == center && change.new_level.is_none())
        );
        assert_eq!(tracker.get_level(center), None);
    }

    #[test]
    fn overlapping_player_sources_survive_one_player_removal() {
        let first = ChunkPos::new(0, 0);
        let second = ChunkPos::new(2, 0);
        let overlap = ChunkPos::new(1, 0);
        let mut tracker = PlayerSimulationTracker::default();
        tracker.add_player(first);
        tracker.add_player(second);
        tracker.run_all_updates(1);
        assert!(
            tracker
                .get_level(overlap)
                .is_some_and(ChunkTicketLevel::is_entity_ticking)
        );

        tracker.remove_player(first);
        tracker.run_all_updates(1);
        assert!(
            tracker
                .get_level(overlap)
                .is_some_and(ChunkTicketLevel::is_entity_ticking)
        );
    }
}
