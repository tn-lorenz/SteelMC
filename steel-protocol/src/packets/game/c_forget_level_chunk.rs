use steel_macros::{ClientPacket, WriteTo};

use steel_registry::packets::play::C_FORGET_LEVEL_CHUNK;

#[derive(ClientPacket, WriteTo)]
#[packet_id(Play = C_FORGET_LEVEL_CHUNK)]
pub struct CForgetLevelChunk {
    #[write(as = "var_int")]
    pub z: i32,
    #[write(as = "var_int")]
    pub x: i32,
}
