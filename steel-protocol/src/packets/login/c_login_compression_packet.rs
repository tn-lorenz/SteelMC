use steel_macros::PacketWrite;

#[derive(PacketWrite, Clone, Debug)]
pub struct CLoginCompressionPacket {
    #[write_as(as = "var_int")]
    pub threshold: i32,
}

impl CLoginCompressionPacket {
    pub fn new(threshold: i32) -> Self {
        Self { threshold }
    }
}
