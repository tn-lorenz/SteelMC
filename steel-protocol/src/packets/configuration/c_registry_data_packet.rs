use simdnbt::owned::NbtTag;
use steel_macros::{CBoundPacket, PacketWrite};
use steel_registry::packets::clientbound::config::CLIENTBOUND_REGISTRY_DATA;
use steel_utils::ResourceLocation;

#[derive(Clone, Debug, PacketWrite)]
pub struct RegistryEntry {
    pub id: ResourceLocation,
    #[write_as(as = "option")]
    pub data: Option<NbtTag>,
}

#[derive(PacketWrite, CBoundPacket, Clone, Debug)]
#[packet_id(CONFIGURATION = "CLIENTBOUND_REGISTRY_DATA")]
pub struct CRegistryDataPacket {
    pub registry: ResourceLocation,
    #[write_as(as = "vec")]
    pub entries: Vec<RegistryEntry>,
}

impl CRegistryDataPacket {
    pub fn new(registry: ResourceLocation, entries: Vec<RegistryEntry>) -> Self {
        Self { registry, entries }
    }
}
