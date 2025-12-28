//! Serverbound packet for container clicks.

use steel_macros::{ReadFrom, ServerPacket};

use super::item_stack::HashedStack;

/// Click type for container interactions.
#[derive(ReadFrom, Clone, Copy, Debug, PartialEq, Eq)]
#[read(as = VarInt)]
#[repr(i32)]
pub enum ClickType {
    /// Normal left/right click.
    Pickup = 0,
    /// Shift + left/right click.
    QuickMove = 1,
    /// Number keys (1-9) or offhand (F).
    Swap = 2,
    /// Middle click (creative clone).
    Clone = 3,
    /// Q key to drop items.
    Throw = 4,
    /// Click and drag to distribute items.
    QuickCraft = 5,
    /// Double-click to collect items.
    PickupAll = 6,
}

/// A slot change sent by the client (slot index -> hashed item).
#[derive(ReadFrom, Clone, Debug)]
pub struct SlotChange {
    pub slot: i16,
    pub item: HashedStack,
}

/// Sent by the client when they click in a container.
#[derive(ServerPacket, ReadFrom, Clone, Debug)]
pub struct SContainerClick {
    /// The container ID.
    pub container_id: u8,
    /// State ID for synchronization.
    #[read(as = VarInt)]
    pub state_id: i32,
    /// The slot that was clicked (-999 for outside window).
    pub slot: i16,
    /// The button used (0 = left, 1 = right, etc.).
    pub button: i8,
    /// The type of click action.
    pub mode: ClickType,
    /// Slots that the client thinks changed (hashed).
    #[read(as = Prefixed(VarInt))]
    pub changed_slots: Vec<SlotChange>,
    /// The item the client thinks is on the cursor after the action (hashed).
    pub carried: HashedStack,
}
