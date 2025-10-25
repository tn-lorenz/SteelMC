use steel_macros::PacketRead;
use steel_utils::ResourceLocation;

#[derive(PacketRead, Clone, Debug)]
pub struct SCustomPayloadPacket {
    pub resource_location: ResourceLocation,
    #[read_as(as = "vec")]
    pub payload: Vec<u8>,
}
