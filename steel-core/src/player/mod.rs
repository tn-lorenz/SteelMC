//! This module contains all things player-related.
pub mod chunk_sender;
mod game_profile;
/// This module contains the networking implementation for the player.
pub mod networking;

use std::sync::Arc;

pub use game_profile::GameProfile;

use steel_protocol::packets::common::SCustomPayload;

use crate::{player::networking::JavaConnection, world::World};

/// A struct representing a player.
pub struct Player {
    /// The player's game profile.
    pub gameprofile: GameProfile,
    /// The player's connection.
    pub connection: Arc<JavaConnection>,

    /// The world the player is in.
    pub world: Arc<World>,

    /// Whether the player has finished loading the client.
    pub client_loaded: bool,
}

impl Player {
    /// Creates a new player.
    pub fn new(
        gameprofile: GameProfile,
        connection: Arc<JavaConnection>,
        world: Arc<World>,
    ) -> Self {
        Self {
            gameprofile,
            connection,

            world,
            client_loaded: false,
        }
    }

    /// Ticks the player.
    pub fn tick(&self) {
        if !self.client_loaded {
        }
        // TODO: Implement player ticking logic here
        // This will include:
        // - Checking if the player is alive
        // - Handling movement
        // - Updating inventory
        // - Handling food/health regeneration
        // - Managing game mode specific logic
        // - Updating advancements
        // - Handling falling
    }

    /// Handles a custom payload packet.
    pub fn handle_custom_payload(&self, packet: SCustomPayload) {
        log::info!("Hello from the other side! {packet:?}");
    }

    /// Handles the end of a client tick.
    pub fn handle_client_tick_end(&self) {
        log::info!("Hello from the other side!");
    }
}
