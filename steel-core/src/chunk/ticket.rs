//! `Ticket` module manages chunk tickets and their storage.
use std::cmp::Ordering;
use std::collections::BTreeMap;

use steel_utils::ChunkPos;

/// The type of a ticket.
///
/// Variants are ordered by priority (lowest value = highest priority).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TicketType {
    /// A ticket created by a player.
    Player,
    /// A forced ticket.
    Forced,
    /// A light update ticket.
    Light,
    /// A portal ticket.
    Portal,
    /// A ticket created after teleportation.
    PostTeleport,
    /// An unknown ticket type.
    Unknown,
}

/// A ticket that keeps a chunk loaded at a certain level.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ticket {
    /// The type of the ticket.
    pub ticket_type: TicketType,
    /// The level of the ticket.
    pub level: u8,
    /// Expiration time in ticks. `None` means never expires.
    pub expiration: Option<u64>,
}

impl PartialOrd for Ticket {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Ticket {
    fn cmp(&self, other: &Self) -> Ordering {
        // Lower level = Higher priority
        self.level
            .cmp(&other.level)
            .then_with(|| self.ticket_type.cmp(&other.ticket_type))
            .then_with(|| self.expiration.cmp(&other.expiration))
    }
}

/// Manages tickets for chunks.
pub struct TicketStorage {
    tickets: BTreeMap<ChunkPos, Vec<Ticket>>,
}

impl Default for TicketStorage {
    fn default() -> Self {
        Self::new()
    }
}

impl TicketStorage {
    /// Creates a new ticket storage.
    #[must_use]
    pub fn new() -> Self {
        Self {
            tickets: BTreeMap::new(),
        }
    }

    /// Adds a ticket.
    pub fn add_ticket(&mut self, pos: ChunkPos, ticket: Ticket) {
        self.tickets.entry(pos).or_default().push(ticket);
    }

    /// Removes a ticket.
    pub fn remove_ticket(&mut self, pos: ChunkPos, ticket: &Ticket) {
        if let Some(tickets) = self.tickets.get_mut(&pos) {
            if let Some(index) = tickets.iter().position(|t| t == ticket) {
                tickets.remove(index);
            }
            if tickets.is_empty() {
                self.tickets.remove(&pos);
            }
        }
    }

    /// Gets the lowest (best) level for a chunk.
    #[must_use] 
    pub fn get_level(&self, pos: ChunkPos) -> Option<u8> {
        self.tickets
            .get(&pos)
            .and_then(|tickets| tickets.iter().map(|t| t.level).min())
    }

    /// Purges expired tickets and returns chunks that changed levels.
    pub fn purge_expired(&mut self, current_tick: u64) -> Vec<ChunkPos> {
        let mut changed_chunks = Vec::new();
        let mut empty_entries = Vec::new();

        for (pos, tickets) in &mut self.tickets {
            let original_min = tickets.iter().map(|t| t.level).min();
            let len_before = tickets.len();

            tickets.retain(|t| t.expiration.is_none_or(|exp| exp > current_tick));

            if tickets.len() != len_before {
                let new_min = tickets.iter().map(|t| t.level).min();
                if original_min != new_min {
                    changed_chunks.push(*pos);
                }
            }

            if tickets.is_empty() {
                empty_entries.push(*pos);
            }
        }

        for pos in empty_entries {
            self.tickets.remove(&pos);
        }

        changed_chunks
    }
}
