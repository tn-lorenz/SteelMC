use enum_dispatch::enum_dispatch;

use crate::player::player_inventory::PlayerInventory;

/// Something that contains items
/// Example: PlayerInventory, Chest, Temporary Crafting Table
pub trait Container {}

#[enum_dispatch(Container)]
pub enum ContainerType {
    PlayerInventory(PlayerInventory),
}
