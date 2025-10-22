use steel_macros::{PacketWrite, packet};
use steel_registry::packets::clientbound::login;
use steel_utils::text::TextComponentBase;

use crate::packet_traits::PacketWrite;

#[derive(PacketWrite, Clone)]
pub struct ClientboundDisconnectPacket {
    pub reason: TextComponentBase,
}

impl ClientboundDisconnectPacket {
    pub fn new(reason: TextComponentBase) -> Self {
        Self { reason }
    }
}
