use crate::{
    codec::var_int, packet_traits::PacketRead, ser::NetworkReadExt, utils::PacketReadError,
};
use bytes::{Buf, BufMut};
use steel_macros::PacketRead;

pub enum ClientIntent {
    STATUS = 1,
    LOGIN = 2,
    TRANSFER = 3,
}

impl PacketRead for ClientIntent {
    fn read_packet(data: &mut bytes::Bytes) -> Result<Self, PacketReadError> {
        Ok(match var_int::read(&mut data.reader())? {
            1 => ClientIntent::STATUS,
            2 => ClientIntent::LOGIN,
            3 => ClientIntent::TRANSFER,
            _ => {
                return Err(PacketReadError::MalformedValue(
                    "Invalid client intent".to_string(),
                ));
            }
        })
    }
}

#[derive(PacketRead)]
pub struct ClientIntentionPacket {
    #[read_as(as = "var_int")]
    pub protocol_version: i32,
    #[read_as(as = "string", bound = 255)]
    pub hostname: String,
    pub port: u16,
    pub intention: ClientIntent,
}
