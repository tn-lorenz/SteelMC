use std::io::Write;

use steel_utils::text::TextComponentBase;

use crate::{packet_traits::PacketWrite, utils::PacketWriteError};

impl PacketWrite for TextComponentBase {
    fn write_packet(&self, _writer: &mut impl Write) -> Result<(), PacketWriteError> {
        //TODO: Implement
        todo!()
    }
}
