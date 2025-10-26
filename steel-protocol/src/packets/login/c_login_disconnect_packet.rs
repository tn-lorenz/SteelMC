use steel_macros::{CBoundPacket, PacketWrite};
use steel_registry::packets::clientbound::login::CLIENTBOUND_LOGIN_DISCONNECT;
use steel_utils::text::TextComponentBase;

#[derive(PacketWrite, CBoundPacket, Clone, Debug)]
#[packet_id(LOGIN = "CLIENTBOUND_LOGIN_DISCONNECT")]
pub struct CLoginDisconnectPacket {
    #[write_as(as = "json")]
    pub reason: TextComponentBase,
}

impl CLoginDisconnectPacket {
    pub fn new(reason: TextComponentBase) -> Self {
        Self { reason }
    }
}
