use steel_macros::{ClientPacket, WriteTo};
use steel_registry::packets::play::C_SET_CHUNK_CACHE_CENTER;

#[derive(ClientPacket, WriteTo)]
#[packet_id(Play = C_SET_CHUNK_CACHE_CENTER)]
pub struct CSetChunkCenter {
    #[write(as = "var_int")]
    pub x: i32,
    #[write(as = "var_int")]
    pub y: i32,
}
