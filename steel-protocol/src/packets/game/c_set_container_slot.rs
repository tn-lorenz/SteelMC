//! Clientbound packet to set a single slot in a container.

use steel_macros::{ClientPacket, WriteTo};
use steel_registry::packets::play::C_CONTAINER_SET_SLOT;

use super::item_stack::RawItemStack;

/// Sent by the server to update a single slot in a container.
#[derive(ClientPacket, WriteTo, Clone, Debug)]
#[packet_id(Play = C_CONTAINER_SET_SLOT)]
pub struct CSetContainerSlot {
    /// The container ID.
    pub container_id: i8,
    /// State ID, incremented on mismatch.
    #[write(as = VarInt)]
    pub state_id: i32,
    /// The slot index to update.
    pub slot: i16,
    /// The new item in this slot.
    pub item: RawItemStack,
}

impl CSetContainerSlot {
    pub fn new(container_id: i8, state_id: i32, slot: i16, item: RawItemStack) -> Self {
        Self {
            container_id,
            state_id,
            slot,
            item,
        }
    }
}
