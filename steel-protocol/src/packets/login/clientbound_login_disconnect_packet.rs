use std::io::Write;

use steel_macros::PacketWrite;
use steel_utils::text::{TextComponent, TextComponentBase};

use crate::{packet_traits::PacketWrite, ser::NetworkWriteExt, utils::PacketWriteError};

#[derive(PacketWrite)]
pub struct ClientboundLoginDisconnectPacket {
    #[write_as(as = "json")]
    pub reason: TextComponentBase,
}

impl ClientboundLoginDisconnectPacket {
    pub fn new(reason: TextComponentBase) -> Self {
        Self { reason }
    }
}
