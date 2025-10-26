use steel_macros::{CBoundPacket, PacketWrite};
use steel_registry::packets::clientbound::config::CLIENTBOUND_CUSTOM_PAYLOAD;
use steel_registry::packets::clientbound::play::CLIENTBOUND_CUSTOM_PAYLOAD as PLAY_CLIENTBOUND_CUSTOM_PAYLOAD;
use steel_utils::ResourceLocation;

#[derive(PacketWrite, CBoundPacket, Clone, Debug)]
#[packet_id(
    CONFIGURATION = "CLIENTBOUND_CUSTOM_PAYLOAD",
    PLAY = "PLAY_CLIENTBOUND_CUSTOM_PAYLOAD"
)]
pub struct CCustomPayloadPacket {
    pub resource_location: ResourceLocation,
    #[write_as(as = "vec")]
    pub payload: Box<[u8]>,
}

impl CCustomPayloadPacket {
    pub fn new(resource_location: ResourceLocation, payload: Box<[u8]>) -> Self {
        Self {
            resource_location,
            payload,
        }
    }
}
