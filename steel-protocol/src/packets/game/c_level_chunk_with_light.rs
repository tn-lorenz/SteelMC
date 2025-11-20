use std::io::{Result, Write};

use simdnbt::owned::{NbtCompound, NbtTag};
use steel_macros::ClientPacket;
use steel_registry::packets::play::C_LEVEL_CHUNK_WITH_LIGHT;
use steel_utils::{
    ChunkPos,
    codec::{BitSet, VarInt},
    serial::WriteTo,
};

#[derive(Debug, Clone)]
pub struct Heightmaps(pub Vec<(i32, Vec<u64>)>);

impl WriteTo for Heightmaps {
    fn write(&self, writer: &mut impl Write) -> Result<()> {
        VarInt(self.0.len() as i32).write(writer)?;
        for (key, value) in &self.0 {
            VarInt(*key).write(writer)?;
            value.write(writer)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct BlockEntityInfo {
    pub packed_xz: u8,
    pub y: i16,
    pub type_id: VarInt,
    pub data: Option<NbtCompound>,
}

impl WriteTo for BlockEntityInfo {
    fn write(&self, writer: &mut impl Write) -> Result<()> {
        self.packed_xz.write(writer)?;
        self.y.write(writer)?;
        self.type_id.write(writer)?;
        match &self.data {
            Some(nbt) => WriteTo::write(&NbtTag::Compound(nbt.clone()), writer),
            None => 0u8.write(writer),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ChunkPacketData {
    pub heightmaps: Heightmaps,
    pub data: Vec<u8>,
    pub block_entities: Vec<BlockEntityInfo>,
}

impl WriteTo for ChunkPacketData {
    fn write(&self, writer: &mut impl Write) -> Result<()> {
        self.heightmaps.write(writer)?;
        self.data.write(writer)?;
        self.block_entities.write(writer)?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct LightUpdatePacketData {
    pub sky_y_mask: BitSet,
    pub block_y_mask: BitSet,
    pub empty_sky_y_mask: BitSet,
    pub empty_block_y_mask: BitSet,
    pub sky_updates: Vec<Vec<u8>>,
    pub block_updates: Vec<Vec<u8>>,
}

impl WriteTo for LightUpdatePacketData {
    fn write(&self, writer: &mut impl Write) -> Result<()> {
        self.sky_y_mask.write(writer)?;
        self.block_y_mask.write(writer)?;
        self.empty_sky_y_mask.write(writer)?;
        self.empty_block_y_mask.write(writer)?;
        self.sky_updates.write(writer)?;
        self.block_updates.write(writer)?;
        Ok(())
    }
}

#[derive(ClientPacket, Debug, Clone)]
#[packet_id(Play = C_LEVEL_CHUNK_WITH_LIGHT)]
pub struct CLevelChunkWithLight {
    pub pos: ChunkPos,
    pub chunk_data: ChunkPacketData,
    pub light_data: LightUpdatePacketData,
}

impl WriteTo for CLevelChunkWithLight {
    fn write(&self, writer: &mut impl Write) -> Result<()> {
        self.pos.write(writer)?;
        self.chunk_data.write(writer)?;
        self.light_data.write(writer)?;
        Ok(())
    }
}
