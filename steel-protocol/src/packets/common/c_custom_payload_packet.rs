use steel_macros::PacketWrite;
use steel_utils::ResourceLocation;

#[derive(PacketWrite, Clone, Debug)]
pub struct CCustomPayloadPacket {
    pub resource_location: ResourceLocation,
    #[write_as(as = "vec")]
    pub payload: Vec<u8>,
}

impl CCustomPayloadPacket {
    pub fn new(resource_location: ResourceLocation, payload: Vec<u8>) -> Self {
        Self {
            resource_location,
            payload,
        }
    }
}
