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

    /// Adds a player ticket (simulates player loading).
    pub fn add_player(&mut self, pos: ChunkPos, view_distance: u8) {
        // Level 31 is entity ticking.
        let level = 31_u8.saturating_sub(view_distance);
        self.add_ticket(
            pos,
            Ticket {
                ticket_type: TicketType::Player,
                level,
                expiration: None,
            },
        );
    }

    /// Removes a player ticket.
    pub fn remove_player(&mut self, pos: ChunkPos, view_distance: u8) {
        let level = 31_u8.saturating_sub(view_distance);
        let ticket = Ticket {
            ticket_type: TicketType::Player,
            level,
            expiration: None,
        };
        self.remove_ticket(pos, &ticket);
    }

    fn update_chunk_tracker(&mut self, pos: ChunkPos) {
        let ticket_level = self.ticket_storage.get_level(pos).unwrap_or(MAX_LEVEL + 1);
        self.tracker.update(pos, ticket_level, |p| {
            self.ticket_storage.get_level(p).unwrap_or(MAX_LEVEL + 1)
        });
    }

    /// Runs pending updates and returns a list of changes.
    pub fn run_updates(&mut self) -> Vec<(ChunkPos, u8, u8)> {
        self.tracker
            .process_all_updates(|p| self.ticket_storage.get_level(p).unwrap_or(MAX_LEVEL + 1))
    }
}
