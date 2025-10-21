use std::io::Write;

use crate::{
    ser::{NetworkReadExt, NetworkWriteExt},
    utils::{PacketReadError, PacketWriteError},
};
use bytes::{Buf, Bytes};

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

pub trait PacketWrite {
    fn write_packet(&self, writer: &mut impl Write) -> Result<(), PacketWriteError>;
}

impl PacketWrite for i32 {
    fn write_packet(&self, writer: &mut impl Write) -> Result<(), PacketWriteError> {
        writer.write_i32_be(*self)?;
        Ok(())
    }
}

impl PacketWrite for u16 {
    fn write_packet(&self, writer: &mut impl Write) -> Result<(), PacketWriteError> {
        writer.write_u16_be(*self)?;
        Ok(())
    }
}

impl PacketWrite for String {
    fn write_packet(&self, writer: &mut impl Write) -> Result<(), PacketWriteError> {
        writer.write_string(self)?;
        Ok(())
    }
}
