use rustc_hash::FxHashMap;
use steel_utils::ChunkPos;

use crate::chunk::{
    chunk_tracker::{ChunkTracker, MAX_LEVEL},
    ticket::{Ticket, TicketStorage, TicketType},
};

/// Manages chunk distances and tickets.
pub struct DistanceManager {
    /// Storage for all chunk tickets.
    pub ticket_storage: TicketStorage,
    /// Tracker for propagating chunk levels.
    pub tracker: ChunkTracker,
    /// Reference counts for simulation tickets (center chunks).
    sim_ref_counts: FxHashMap<ChunkPos, usize>,
    /// Reference counts for loading tickets (view distance).
    loading_ref_counts: FxHashMap<ChunkPos, usize>,
}

impl Default for DistanceManager {
    fn default() -> Self {
        Self::new()
    }
}

impl DistanceManager {
    /// Creates a new distance manager.
    #[must_use]
    pub fn new() -> Self {
        Self {
            ticket_storage: TicketStorage::new(),
            tracker: ChunkTracker::new(),
            sim_ref_counts: FxHashMap::default(),
            loading_ref_counts: FxHashMap::default(),
        }
    }

    /// Adds a ticket for a specific chunk.
    pub fn add_ticket(&mut self, pos: ChunkPos, ticket: Ticket) {
        self.ticket_storage.add_ticket(pos, ticket);
        self.update_chunk_tracker(pos);
    }

    /// Removes a ticket from a specific chunk.
    pub fn remove_ticket(&mut self, pos: ChunkPos, ticket: &Ticket) {
        self.ticket_storage.remove_ticket(pos, ticket);
        self.update_chunk_tracker(pos);
    }

    /// Adds a player's tickets (simulation and view distance).
    pub fn add_player(&mut self, pos: ChunkPos, view_distance: u8, simulation_distance: u8) {
        // 1. Simulation Ticket (Center Chunk only)
        // Level = 31 - sim_distance (limits entity ticking range)
        let should_add_sim = {
            let sim_count = self.sim_ref_counts.entry(pos).or_default();
            let should = *sim_count == 0;
            *sim_count += 1;
            should
        };

        if should_add_sim {
            let level = 31_u8.saturating_sub(simulation_distance);
            self.add_ticket(
                pos,
                Ticket {
                    ticket_type: TicketType::PlayerSimulation,
                    level,
                    expiration: None,
                },
            );
        }

        // 2. Loading Tickets (Square Radius = view_distance)
        // Level = 31 (Entity Ticking) - ensures chunks are loaded
        let r = view_distance as i32;
        for x in -r..=r {
            for z in -r..=r {
                let chunk_pos = ChunkPos::new(pos.0.x + x, pos.0.y + z);
                let should_add_load = {
                    let load_count = self.loading_ref_counts.entry(chunk_pos).or_default();
                    let should = *load_count == 0;
                    *load_count += 1;
                    should
                };

                if should_add_load {
                    self.add_ticket(
                        chunk_pos,
                        Ticket {
                            ticket_type: TicketType::Player,
                            level: 31,
                            expiration: None,
                        },
                    );
                }
            }
        }
    }

    /// Removes a player's tickets.
    pub fn remove_player(&mut self, pos: ChunkPos, view_distance: u8, simulation_distance: u8) {
        // 1. Remove Simulation Ticket
        let should_remove_sim = {
            if let Some(sim_count) = self.sim_ref_counts.get_mut(&pos) {
                *sim_count -= 1;
                if *sim_count == 0 {
                    self.sim_ref_counts.remove(&pos);
                    true
                } else {
                    false
                }
            } else {
                false
            }
        };

        if should_remove_sim {
            let level = 31_u8.saturating_sub(simulation_distance);
            let ticket = Ticket {
                ticket_type: TicketType::PlayerSimulation,
                level,
                expiration: None,
            };
            self.remove_ticket(pos, &ticket);
        }

        // 2. Remove Loading Tickets
        let r = view_distance as i32;
        for x in -r..=r {
            for z in -r..=r {
                let chunk_pos = ChunkPos::new(pos.0.x + x, pos.0.y + z);
                let should_remove_load = {
                    if let Some(load_count) = self.loading_ref_counts.get_mut(&chunk_pos) {
                        *load_count -= 1;
                        if *load_count == 0 {
                            self.loading_ref_counts.remove(&chunk_pos);
                            true
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                };

                if should_remove_load {
                    let ticket = Ticket {
                        ticket_type: TicketType::Player,
                        level: 31,
                        expiration: None,
                    };
                    self.remove_ticket(chunk_pos, &ticket);
                }
            }
        }
    }

    #[inline]
    fn update_chunk_tracker(&mut self, pos: ChunkPos) {
        let ticket_level = ticket_level_or_max(&self.ticket_storage, pos);
        let ticket_storage = &self.ticket_storage;
        self.tracker.update(pos, ticket_level, |p| {
            ticket_level_or_max(ticket_storage, p)
        });
    }

    /// Runs pending updates and returns a deduplicated map of chunk level changes.
    ///
    /// Like Java's `LoadingChunkTracker`, uses a `HashSet`-like structure to collect
    /// chunks that need updating. Each chunk appears at most once with its final level.
    pub fn run_updates(&mut self) -> FxHashMap<ChunkPos, u8> {
        let ticket_storage = &self.ticket_storage;
        let mut chunks_to_update: FxHashMap<ChunkPos, u8> = FxHashMap::default();

        self.tracker.process_all_updates(
            |p| ticket_level_or_max(ticket_storage, p),
            |pos, new_level| {
                // Like Java's chunksToUpdateFutures.add(chunk), this deduplicates.
                // The last level written wins (which is the final level after all updates).
                chunks_to_update.insert(pos, new_level);
            },
        );

        chunks_to_update
    }

    /// Purges expired tickets and updates the tracker.
    pub fn purge_tickets(&mut self, current_tick: u64) {
        let changed_chunks = self.ticket_storage.purge_expired(current_tick);
        for pos in changed_chunks {
            self.update_chunk_tracker(pos);
        }
    }
}

#[inline]
fn ticket_level_or_max(storage: &TicketStorage, pos: ChunkPos) -> u8 {
    storage
        .get_cached_level(pos)
        .or_else(|| storage.get_level(pos))
        .unwrap_or(MAX_LEVEL + 1)
}
