//! Clientbound packet to set the entire contents of a container.

use steel_macros::{ClientPacket, WriteTo};
use steel_registry::packets::play::C_CONTAINER_SET_CONTENT;

use super::item_stack::RawItemStack;

/// Sent by the server to set the entire contents of a container.
#[derive(ClientPacket, WriteTo, Clone, Debug)]
#[packet_id(Play = C_CONTAINER_SET_CONTENT)]
pub struct CSetContainerContent {
    /// The container ID (0 for player inventory).
    pub container_id: u8,
    /// State ID, incremented on mismatch.
    #[write(as = VarInt)]
    pub state_id: i32,
    /// All items in the container.
    #[write(as = Prefixed(VarInt))]
    pub items: Vec<RawItemStack>,
    /// The item being carried by the cursor.
    pub carried: RawItemStack,
}

impl CSetContainerContent {
    pub fn new(
        container_id: u8,
        state_id: i32,
        items: Vec<RawItemStack>,
        carried: RawItemStack,
    ) -> Self {
        Self {
            container_id,
            state_id,
            items,
            carried,
        }
    }
}
