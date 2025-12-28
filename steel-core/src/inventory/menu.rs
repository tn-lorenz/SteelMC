//! A menu can be considered everything that's shown on the screen.
//! It consists of slots, slots consist of a view into a single inventory and position.
//! When you have a chest open for example a chest menu is shown, consisting of the chests slots and the players inventory slots.
//!
//! A menu is always the middle man between the server and the client.
//! This means that when the player doesn't have any menus open it actually has, it always has it's own inventory menu open.
//!
//! A menu holds 3 important structures:
//! - All slots for that menu
//! - All slots as cloned itemstacks
//! - The clients perception of the itemstacks
//!
//! This makes it so every time we run a sync (once per tick) we update the cloned itemstacks.
//! This in turn makes it so we can compare it with the clients perception of the itemstacks.
//! And if there are mismatches we can send the correct itemstacks to the client.
//!
//! The client also sends the itemstacks it thinks it has on interaction, so this makes it so we only update the client if they mismatch.

use steel_registry::item_stack::ItemStack;

use crate::inventory::slot::SlotType;

pub struct MenuBehavior {
    pub slots: Vec<SlotType>,
    pub cloned_itemstacks: Vec<ItemStack>,
    pub remote_itemstacks: Vec<ItemStack>,
}

pub trait Menu {}
