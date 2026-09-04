//! Let's make a game with four teams, for the sake of example.
//! Players can hit only those who are not on their own team. Each player needs to be aware of not falling down. (The ground beneath them crumbles...)

use small_map::FxSmallMap;
use uuid::Uuid;

use crate::api::{Game, GameBase, GameState, LobbyType, Objective, Team};

pub struct TnTRunGame {
    base: GameBase,
    teams: FxSmallMap<8, Uuid, Team>,
    floor_level: i32,
}

impl TnTRunGame {
    #[must_use]
    pub fn new() -> Self {
        let mut teams = FxSmallMap::default();

        for name in ["Steel", "Bronze", "Titanium", "Chrome"] {
            let team = Team::new(name);

            teams.insert(team.id(), team);
        }

        Self {
            base: GameBase::new(2, 32),
            teams,
            floor_level: 64,
        }
    }
}

impl Game for TnTRunGame {
    fn tick(&mut self) {
        if self.base.state != GameState::InProgress {
            return;
        }

        self.base.base_tick();
    }

    fn start(&mut self) {
        self.base.state = GameState::InProgress;
    }

    fn stop(&mut self) {
        self.base.state = GameState::Stopping;
    }

    fn on_player_join(&mut self, player: Uuid) {
        for lobby in &self.base.lobbies {
            if lobby.lobby_type == LobbyType::GameWaitingLobby {
                lobby.spawn_player(player);
            }
        }
    }

    fn on_player_leave(&mut self, _player: Uuid) {}

    fn on_player_obtain_objective(&mut self, _player: Uuid, _objective: Objective) {}

    fn state(&self) -> GameState {
        self.base.state
    }

    fn add_player(&mut self, player: Uuid) {
        self.base.players.insert(player);
        self.on_player_join(player);
    }

    fn remove_player(&mut self, player: Uuid) {
        self.base.players.remove(&player);
        self.on_player_leave(player);
    }

    fn clear_players(&mut self) {
        self.base.players.clear();
    }
}
