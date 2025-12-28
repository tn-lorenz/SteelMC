//! Serverbound packet for closing a container.

use steel_macros::{ReadFrom, ServerPacket};

/// Sent by the client when they close a container window.
#[derive(ServerPacket, ReadFrom, Clone, Debug)]
pub struct SContainerClose {
    /// The container ID being closed.
    pub container_id: u8,
}
