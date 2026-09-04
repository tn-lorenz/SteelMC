use glam::DVec3;
use rustc_hash::{FxHashMap, FxHashSet};
use small_map::FxSmallMap;
use std::sync::Arc;
use steel_core::world::World;
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
    pub(crate) min_players: i32,
    pub(crate) max_players: i32,
    // TODO
    pub(crate) lobbies: FxHashSet<Lobby>,

    /// How many ticks have elapsed since the creation of this game.
    elapsed_time: SyncMutex<i32>,
}

impl GameBase {
    #[must_use]
    pub fn new(min_players: i32, max_players: i32) -> Self {
        Self {
            players: FxHashSet::default(),
            state: GameState::Waiting,
            teams: FxSmallMap::default(),
            objectives: FxHashMap::default(),
            min_players,
            max_players,
            lobbies: Default::default(),
            elapsed_time: SyncMutex::new(0),
        }
    }

    pub fn base_tick(&mut self) {
        let mut elapsed_time = self.elapsed_time.lock();
        *elapsed_time += 1;
    }
}

pub trait Game {
    fn tick(&mut self);

    fn start(&mut self);
    fn stop(&mut self);

    /// Event for when a player joins the Game.
    fn on_player_join(&mut self, _player: Uuid) {}

    /// Event for when a player leaves the Game.
    fn on_player_leave(&mut self, _player: Uuid) {}

    /// Event for when a player is killed within that game.
    fn on_player_killed(&mut self, _player: Uuid) {}

    /// Event for when a player obtains an objective within that game.
    fn on_player_obtain_objective(&mut self, _player: Uuid, _objective: Objective) {}

    fn state(&self) -> GameState;

    fn add_player(&mut self, player: Uuid);
    fn remove_player(&mut self, player: Uuid);
    fn clear_players(&mut self);
}

/// We need Lobbies where players can wait for the game to start, for the hub and also for the actual game world.
pub struct Lobby {
    // TODO: `world_key: Identifier`,
    world: Arc<World>,
    spawn_strategy: SpawnStrategy,
    pub(crate) lobby_type: LobbyType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LobbyType {
    /// The server Hub.
    Hub,

    /// A separate game lobby (player hasn't joined the game yet). Useful for example, if a player clicks a mini-game inside a hub compass GUI
    /// and needs to be teleported to a separate region, without joining a game immediately.
    GameLobby,

    /// A separate waiting lobby, while already having joined the game
    GameWaitingLobby,

    /// Useful if the winner of a mini-game should be teleported somewhere after having won.
    GameFinishedLobby,
}

impl Lobby {
    pub fn new(world: Arc<World>, spawn_strategy: SpawnStrategy, lobby_type: LobbyType) -> Self {
        Self {
            world,
            spawn_strategy,
            lobby_type,
        }
    }

    /// Spawns a player in the Game world or lobby.
    pub fn spawn_player(&self, player: Uuid) {
        match self.spawn_strategy {
            SpawnStrategy::RandomAnywhere => {
                self.spawn_random_anywhere(player);
            }
            SpawnStrategy::RandomWithRadius { center, radius } => {
                self.spawn_random_radius(player, center, radius);
            }
            SpawnStrategy::Fixed { location } => {
                self.spawn_at(player, location);
            }
        }
    }

    fn spawn_random_anywhere(&self, player: Uuid) {
        todo!()
    }

    fn spawn_random_radius(&self, player: Uuid, center: Location, radius: f32) {
        todo!()
    }

    fn spawn_at(&self, player: Uuid, location: Location) {
        todo!()
    }
}

pub trait TimedGame: Game {
    fn start_timer(&mut self);
    fn stop_timer(&mut self);
    fn elapsed_as_secs_f32(&self) -> f32;
    fn elapsed_formatted_msg(&self) -> String;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameState {
    Waiting,
    Starting,
    InProgress,
    Stopping,
}

/// We use this to enum-dispatch what do to when trying to spawn players in a lobby/game world
pub enum SpawnStrategy {
    /// If players can spawn at random locations on the entire world.
    RandomAnywhere,

    /// If players can spawn anywhere withing a given radius.
    RandomWithRadius { center: Location, radius: f32 },

    /// If players should spawn at predefined coordinates on a predefined world.
    Fixed { location: Location },
}

impl SpawnStrategy {
    /// determines coordinates for teleportation
    #[must_use]
    pub fn pick_spawn_position(&self) -> Location {
        match *self {
            Self::Fixed { location } => location,
            Self::RandomWithRadius { center, radius } => {
                // TODO
                center
            }
            // TODO
            _ => Location::new(DVec3::new(0.0, 0.0, 0.0), 0.0, 0.0),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Location {
    pos: DVec3,
    yaw: f32,
    pitch: f32,
}

impl Location {
    pub fn new(pos: DVec3, yaw: f32, pitch: f32) -> Self {
        Self { pos, yaw, pitch }
    }
}
