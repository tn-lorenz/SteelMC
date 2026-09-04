use rustc_hash::{FxHashMap, FxHashSet};
use small_map::FxSmallMap;
use steel_core::player::Player;
use uuid::Uuid;

struct Team {
    id: Uuid,
    name: String,
    players: Vec<Player>,
}

impl Team {
    fn new(id: Uuid, name: String, players: Vec<Player>) -> Self {
        Self { id, name, players }
    }

    fn id(&self) -> &Uuid {
        &self.id
    }

    fn name(&self) -> &String {
        &self.name
    }

    fn players(&self) -> &Vec<Player> {
        &self.players
    }
}

/// A mini-game may have multiple objectives. The HashSet `ids` is used to identify which players have completed this objective.
struct Objective {
    ids: FxHashSet<Uuid>,
    name: String,
    description: String,
}

struct Game {
    players: FxHashSet<Uuid>,
    teams: FxSmallMap<8, Uuid, Team>,
    state: GameState,
    objectives: FxHashMap<Uuid, Objective>,
}

pub enum GameState {
    Waiting,
    Starting,
    InProgress,
    Stopping,
}

pub trait BasicGame {
    fn new() -> Self;
    fn start(&self);
    fn stop(&self);
    fn add_player(&mut self, player: Player);
    fn remove_player(&mut self, player: Player);
    fn clear_players(&self);
}

pub trait TimedGame: BasicGame {
    fn start_timer(&self);
    fn stop_timer(&self);
}
