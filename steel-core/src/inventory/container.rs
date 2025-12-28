use enum_dispatch::enum_dispatch;

use crate::player::player_inventory::PlayerInventory;

/// Something that contains items.
/// I also use container interchangeably with inventory as they mean approximately the same thing. But inventory could also refer to the player's inventory.
/// Example: PlayerInventory, Chest, Temporary Crafting Table
pub trait Container {}

#[enum_dispatch(Container)]
pub enum ContainerType {
    PlayerInventory(PlayerInventory),
}
