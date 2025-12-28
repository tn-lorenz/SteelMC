use crate::inventory::menu::Menu;

/// The player's inventory menu. This is the menu that is displayed when the player opens their inventory, including their equipment and items.
pub struct InventoryMenu {}

impl InventoryMenu {
    pub fn new() -> Self {
        InventoryMenu {}
    }
}

impl Menu for InventoryMenu {}
