use std::io::{Error, Read, Write};

use async_compression::{Level, tokio::write::ZlibEncoder};
use bytes::Bytes;
use serde::Deserialize;
use tokio::io::AsyncWriteExt;

use crate::{
    codec::VarInt,
    packets::clientbound::CBoundPacket,
    utils::{MAX_PACKET_DATA_SIZE, MAX_PACKET_SIZE, PacketError},
};

const DEFAULT_BOUND: usize = i32::MAX as _;

// These are the network read/write traits
pub trait PacketRead: ReadFrom {
    fn read_packet(data: &mut impl Read) -> Result<Self, PacketError> {
        Self::read(data).map_err(PacketError::from)
    }
}
pub trait PacketWrite: WriteTo {
    fn write_packet(&self, writer: &mut impl Write) -> Result<(), PacketError> {
        self.write(writer).map_err(PacketError::from)
    }
}

// These are the general read/write traits with io::error
// We dont use Write/Read because it conflicts with std::io::Read/Write
pub trait ReadFrom: Sized {
    fn read(data: &mut impl Read) -> Result<Self, Error>;
}
pub trait WriteTo {
    fn write(&self, writer: &mut impl Write) -> Result<(), Error>;
}

pub trait PrefixedRead: Sized {
    fn read_prefixed_bound<P: TryInto<usize> + ReadFrom>(
        data: &mut impl Read,
        bound: usize,
    ) -> Result<Self, Error>;

    fn read_prefixed<P: TryInto<usize> + ReadFrom>(data: &mut impl Read) -> Result<Self, Error> {
        Self::read_prefixed_bound::<P>(data, DEFAULT_BOUND)
    }
}

pub trait PrefixedWrite {
    fn write_prefixed_bound<P: TryFrom<usize> + WriteTo>(
        &self,
        writer: &mut impl Write,
        bound: usize,
    ) -> Result<(), Error>;

    fn write_prefixed<P: TryFrom<usize> + WriteTo>(
        &self,
        writer: &mut impl Write,
    ) -> Result<(), Error> {
        self.write_prefixed_bound::<P>(writer, DEFAULT_BOUND)
    }
}

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct CompressionInfo {
    /// The compression threshold used when compression is enabled.
    pub threshold: usize,
    /// A value between `0..9`.
    /// `1` = Optimize for the best speed of encoding.
    /// `9` = Optimize for the size of data being encoded.
    pub level: i32,
}

impl Default for CompressionInfo {
    fn default() -> Self {
        Self {
            threshold: 256,
            level: 4,
        }
    }
}

/// Represents an encoded clientbound packet, optionally applying compression based on threshold and level.
///
/// # Packet Size Limits
/// - Maximum packet size: 2097151 bytes (2^21 - 1, max 3-byte VarInt)
/// - Maximum uncompressed size for compressed packets: 8388608 bytes (2^23)
/// - Length field must not exceed 3 bytes
///
/// # Packet Encoding Format
///
/// **Without Compression:**
/// ```text
/// [Length: VarInt]     Length of (Packet ID + Data)
/// [Packet ID: VarInt]  Protocol ID from packet report
/// [Data: Byte Array]   Packet payload
/// ```
///
/// **With Compression (size >= threshold):**
/// ```text
/// [Length: VarInt]     Length of (Data Length + compressed data)
/// [Data Length: VarInt] Length of uncompressed (Packet ID + Data)
/// [Compressed Data]    zlib compressed (Packet ID + Data)
/// ```
///
/// **With Compression (size < threshold):**
/// ```text
/// [Length: VarInt]     Length of (Data Length + uncompressed data)
/// [Data Length: VarInt] 0 to indicate uncompressed
/// [Packet ID: VarInt]  Protocol ID from packet report
/// [Data: Byte Array]   Uncompressed packet payload
/// ```
///
/// Compression is only applied when:
/// 1. Compression is enabled via Set Compression packet
/// 2. The uncompressed data length meets/exceeds the threshold
/// 3. The threshold is non-negative
pub struct EncodedPacket {
    pub encoded_data: Bytes,
}

impl EncodedPacket {
    async fn from_packet_data_no_compression(packet_data: Bytes) -> Result<Self, PacketError> {
        let mut buf: Vec<u8> = Vec::new();
        let data_len = packet_data.len();

        let complete_len = VarInt::written_size(data_len as _) + data_len;
        if complete_len > MAX_PACKET_SIZE {
            return Err(PacketError::TooLong(complete_len));
        }

        VarInt(data_len as _).write(&mut buf)?;
        std::io::Write::write_all(&mut buf, &packet_data)
            .map_err(|e| PacketError::EncryptionFailed(e.to_string()))?;

        Ok(Self {
            encoded_data: buf.into(),
        })
    }

    async fn from_packet_data(
        packet_data: Bytes,
        threshold: usize,
        level: i32,
    ) -> Result<Self, PacketError> {
        let mut buf: Vec<u8> = Vec::new();
        let data_len = packet_data.len();
        // We dont need any more size check to convert to i32 as MAX_PACKET_DATA_SIZE < i32::MAX
        if data_len > MAX_PACKET_DATA_SIZE {
            return Err(PacketError::TooLong(data_len));
        }

        if data_len >= threshold {
            let mut compressed_buf = Vec::new();
            let mut compressor =
                ZlibEncoder::with_quality(&mut compressed_buf, Level::Precise(level));
            compressor
                .write_all(&packet_data)
                .await
                .map_err(|e| PacketError::CompressionFailed(e.to_string()))?;
            compressor
                .flush()
                .await
                .map_err(|e| PacketError::CompressionFailed(e.to_string()))?;

            let full_packet_len: i32 = (VarInt::written_size(data_len as _) + compressed_buf.len())
                .try_into()
                .map_err(|_| PacketError::TooLong(data_len))?;

            let complete_len = VarInt::written_size(full_packet_len) + full_packet_len as usize;
            if complete_len > MAX_PACKET_SIZE {
                return Err(PacketError::TooLong(complete_len));
            }

            VarInt(full_packet_len).write(&mut buf)?;
            VarInt(data_len as _).write(&mut buf)?;

            std::io::Write::write_all(&mut buf, &compressed_buf)
                .map_err(|e| PacketError::CompressionFailed(e.to_string()))?;
        } else {
            // Pushed before data:
            // Length of (Data Length) + length of compressed (Packet ID + Data)
            // 0 to indicate uncompressed

            let full_packet_len = data_len as i32;

            let complete_serialization_len =
                VarInt::written_size(full_packet_len) + full_packet_len as usize;
            if complete_serialization_len > MAX_PACKET_SIZE {
                return Err(PacketError::TooLong(complete_serialization_len));
            }

            VarInt(full_packet_len).write(&mut buf)?;
            VarInt(0).write(&mut buf)?;

            std::io::Write::write_all(&mut buf, &packet_data)
                .map_err(|e| PacketError::EncryptionFailed(e.to_string()))?;
        }

        Ok(Self {
            encoded_data: buf.into(),
        })
    }

    pub async fn from_packet(
        packet: &CBoundPacket,
        compression_info: Option<CompressionInfo>,
    ) -> Result<Self, PacketError> {
        let mut buf = Vec::new();
        let packet_id = packet.get_id();
        VarInt(packet_id).write(&mut buf)?;
        packet.write_packet(&mut buf)?;
        if let Some(compression_info) = compression_info {
            Self::from_packet_data(
                buf.into(),
                compression_info.threshold,
                compression_info.level,
            )
            .await
        } else {
            Self::from_packet_data_no_compression(buf.into()).await
        }
    }
}
