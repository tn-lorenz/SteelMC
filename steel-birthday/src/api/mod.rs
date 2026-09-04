use rustc_hash::{FxHashMap, FxHashSet};
use small_map::FxSmallMap;
use steel_utils::locks::SyncMutex;
use uuid::Uuid;

pub struct Team {
    id: Uuid,
    name: String,
    players: FxHashSet<Uuid>,
}

impl Team {
    #[must_use]
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            players: FxHashSet::default(),
        }
    }

    #[must_use]
    pub fn id(&self) -> Uuid {
        self.id
    }

    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[must_use]
    pub fn players(&self) -> &FxHashSet<Uuid> {
        &self.players
    }
}

/// A mini-game may have multiple objectives. The `ids` set is used to identify which players have completed this objective.
pub struct Objective {
    ids: FxHashSet<Uuid>,
    name: String,
    description: String,
}

pub struct GameBase {
    pub(crate) players: FxHashSet<Uuid>,
    pub(crate) state: GameState,
    pub(crate) teams: FxSmallMap<8, Uuid, Team>,
    pub(crate) objectives: FxHashMap<Uuid, Objective>,

    /// How many ticks have elapsed since the creation of this game.
    elapsed_time: SyncMutex<i32>,
}

impl GameBase {
    #[must_use]
    pub fn new() -> Self {
        Self {
            players: FxHashSet::default(),
            state: GameState::Waiting,
            teams: FxSmallMap::default(),
            objectives: FxHashMap::default(),
            elapsed_time: SyncMutex::new(0),
        }
    }

    pub fn base_tick(&mut self) {
        let mut elapsed_time = self.elapsed_time.lock();
        *elapsed_time += 1;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameState {
    Waiting,
    Starting,
    InProgress,
    Stopping,
}

pub trait Game {
    fn tick(&mut self);

    fn start(&mut self);
    fn stop(&mut self);

    /// To, for example, broadcast a message if a player joined.
    fn on_player_join(&mut self, player: Uuid);
    fn on_player_leave(&mut self, player: Uuid);

    fn state(&self) -> GameState;

    fn add_player(&mut self, player: Uuid);
    fn remove_player(&mut self, player: Uuid);
    fn clear_players(&mut self);
}

pub trait TimedGame: Game {
    fn start_timer(&mut self);
    fn stop_timer(&mut self);
}
