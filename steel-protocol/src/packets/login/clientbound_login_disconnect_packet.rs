use steel_macros::PacketWrite;
use steel_utils::text::TextComponentBase;

use crate::{packet_traits::PacketWrite, ser::NetworkWriteExt};

#[derive(PacketWrite, Clone, Debug)]
pub struct ClientboundLoginDisconnectPacket {
    #[write_as(as = "json")]
    pub reason: TextComponentBase,
}

impl ClientboundLoginDisconnectPacket {
    pub fn new(reason: TextComponentBase) -> Self {
        Self { reason }
    }
}
