use steel_macros::PacketWrite;

#[derive(PacketWrite, Clone, Debug)]
pub struct CHelloPacket {
    #[write_as(as = "string", bound = 20)]
    pub server_id: String,
    #[write_as(as = "vec")]
    pub public_key: Vec<u8>,
    #[write_as(as = "vec")]
    pub challenge: Vec<u8>,
    pub should_authenticate: bool,
}
