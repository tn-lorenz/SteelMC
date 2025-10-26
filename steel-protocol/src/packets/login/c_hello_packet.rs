use steel_macros::{CBoundPacket, PacketWrite};
use steel_registry::packets::clientbound::login::CLIENTBOUND_HELLO;

#[derive(PacketWrite, CBoundPacket, Clone, Debug)]
#[packet_id(LOGIN = "CLIENTBOUND_HELLO")]
pub struct CHelloPacket {
    #[write_as(as = "string", bound = 20)]
    pub server_id: String,
    #[write_as(as = "vec")]
    pub public_key: Box<[u8]>,
    #[write_as(as = "vec")]
    pub challenge: [u8; 4],
    pub should_authenticate: bool,
}

impl CHelloPacket {
    pub fn new(
        server_id: String,
        public_key: Box<[u8]>,
        challenge: [u8; 4],
        should_authenticate: bool,
    ) -> Self {
        Self {
            server_id,
            public_key,
            challenge,
            should_authenticate,
        }
    }
}
