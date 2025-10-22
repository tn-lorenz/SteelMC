use steel_macros::PacketRead;

use crate::packet_traits::PacketRead;

#[derive(PacketRead, Clone, Debug)]
pub struct ServerboundStatusRequestPacket {}

impl ServerboundStatusRequestPacket {
    pub fn new() -> Self {
        Self {}
    }
}
