//! Wir machen einfach mal ein Spiel mit vier teams.
//! Spieler können nur gegnerische Spieler vom Spielfeld kicken und müssen gleichzeitig darauf achten, nicht herunterzufallen.

use small_map::FxSmallMap;
use uuid::Uuid;

use crate::api::{Game, GameBase, GameState, Team};

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
            base: GameBase::new(),
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

    fn on_player_join(&mut self, _player: Uuid) {}

    fn on_player_leave(&mut self, _player: Uuid) {}

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
