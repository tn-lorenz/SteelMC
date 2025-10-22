use steel_macros::PacketRead;

use crate::packet_traits::PacketRead;

#[derive(PacketRead, Clone, Debug)]
pub struct ServerboundPingRequestPacket {
    pub time: i64,
}

impl ServerboundPingRequestPacket {
    pub fn new(time: i64) -> Self {
        Self { time }
    }
}
