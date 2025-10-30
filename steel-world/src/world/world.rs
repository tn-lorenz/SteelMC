use scc::{HashIndex, HashMap};
use steel_utils::{ChunkPos, locks::SteelRwLock};

use crate::{ChunkData, player::player::Player};

pub struct World {
    pub loaded_chunks: HashIndex<ChunkPos, SteelRwLock<ChunkData>>,
    pub players: HashMap<uuid::Uuid, SteelRwLock<Player>>,
}

impl World {
    pub fn new() -> Self {
        Self {
            loaded_chunks: HashIndex::new(),
            players: HashMap::new(),
        }
    }

    pub fn add_player(&self, player: Player) {
        self.players
            .insert_sync(player.game_profile.id, SteelRwLock::new(player))
            .unwrap();
    }
}
