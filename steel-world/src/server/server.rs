use crate::player::player::Player;

pub trait WorldServer {
    fn add_player(&self, player: Player);
}
