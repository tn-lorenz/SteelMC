use crate::{
    ser::NetworkReadExt,
    utils::{PacketReadError, PacketWriteError},
};
use bytes::{Buf, Bytes};

pub trait PacketWrite {
    fn write_packet(&self) -> Result<Bytes, PacketWriteError>;
}

pub trait PacketRead {
    fn read_packet(data: &mut Bytes) -> Result<Self, PacketReadError>
    where
        Self: Sized;
}

impl PacketRead for i32 {
    fn read_packet(data: &mut bytes::Bytes) -> Result<Self, PacketReadError> {
        Ok(data.reader().get_i32_be()?)
    }
}

impl PacketRead for u16 {
    fn read_packet(data: &mut bytes::Bytes) -> Result<Self, PacketReadError> {
        Ok(data.reader().get_u16_be()?)
    }
}
