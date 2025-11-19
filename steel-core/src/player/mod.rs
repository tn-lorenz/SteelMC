//! This module contains all things player-related.
pub mod chunk_sender;
mod game_profile;
/// This module contains the networking implementation for the player.
pub mod networking;

use std::sync::Arc;

pub use game_profile::GameProfile;
use parking_lot::Mutex;

use steel_protocol::packets::common::SCustomPayload;
use steel_utils::{ChunkPos, math::Vector3};

use crate::{
    chunk::chunk_tracking_view::ChunkTrackingView,
    player::{chunk_sender::ChunkSender, networking::JavaConnection},
    world::World,
};

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

    /// The player's position.
    pub position: Mutex<Vector3<f64>>,
    /// The last chunk position of the player.
    pub last_chunk_pos: Mutex<ChunkPos>,
    /// The last chunk tracking view of the player.
    pub last_tracking_view: Mutex<Option<ChunkTrackingView>>,
    /// The chunk sender for the player.
    pub chunk_sender: Mutex<ChunkSender>,
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
            position: Mutex::new(Vector3::default()),
            last_chunk_pos: Mutex::new(ChunkPos::new(0, 0)),
            last_tracking_view: Mutex::new(None),
            chunk_sender: Mutex::new(ChunkSender::default()),
        }
    }

    /// Ticks the player.
    pub fn tick(&self) {
        if !self.client_loaded {
            //return;
        }

        let current_pos = *self.position.lock();
        #[allow(clippy::cast_possible_truncation)]
        let chunk_x = (current_pos.x as i32) >> 4;
        #[allow(clippy::cast_possible_truncation)]
        let chunk_z = (current_pos.z as i32) >> 4;
        let chunk_pos = ChunkPos::new(chunk_x, chunk_z);

        *self.last_chunk_pos.lock() = chunk_pos;

        self.world.chunk_map.update_player_status(self);

        self.chunk_sender
            .lock()
            .send_next_chunks(&self.connection, &self.world, chunk_pos);

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
