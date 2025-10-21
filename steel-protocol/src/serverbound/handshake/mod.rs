use std::io::Write;

use crate::{
    codec::var_int,
    packet_traits::{PacketRead, PacketWrite},
    ser::NetworkWriteExt,
    utils::{PacketReadError, PacketWriteError},
};
use bytes::Buf;
use steel_macros::{PacketRead, PacketWrite};

#[derive(Clone, Copy, PartialEq, Eq)]
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

impl PacketWrite for ClientIntent {
    fn write_packet(&self, writer: &mut impl Write) -> Result<(), PacketWriteError> {
        writer.write_var_int(*self as i32)?;
        Ok(())
    }
}
#[derive(PacketRead, PacketWrite)]
pub struct ClientIntentionPacket {
    #[read_as(as = "var_int")]
    #[write_as(as = "var_int")]
    pub protocol_version: i32,
    #[read_as(as = "string", bound = 255)]
    pub hostname: String,
    pub port: u16,
    pub intention: ClientIntent,
}
