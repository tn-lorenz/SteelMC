use steel_macros::{CBoundPacket, PacketWrite};
use steel_registry::packets::clientbound::config::CLIENTBOUND_FINISH_CONFIGURATION;

#[derive(PacketWrite, CBoundPacket, Clone, Debug)]
#[packet_id(CONFIGURATION = "CLIENTBOUND_FINISH_CONFIGURATION")]
pub struct CFinishConfigurationPacket {}

impl CFinishConfigurationPacket {
    pub fn new() -> Self {
        Self {}
    }
}
