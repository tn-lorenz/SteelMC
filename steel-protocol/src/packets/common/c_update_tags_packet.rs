use std::{
    collections::HashMap,
    io::{Result, Write},
};
use steel_macros::CBoundPacket;
use steel_registry::packets::clientbound::config::CLIENTBOUND_UPDATE_TAGS;
use steel_registry::packets::clientbound::play::CLIENTBOUND_UPDATE_TAGS as PLAY_CLIENTBOUND_UPDATE_TAGS;
use steel_utils::ResourceLocation;

use crate::{
    codec::VarInt,
    packet_traits::{PacketWrite, WriteTo},
};

#[derive(CBoundPacket, Clone, Debug)]
#[packet_id(
    CONFIGURATION = "CLIENTBOUND_UPDATE_TAGS",
    PLAY = "PLAY_CLIENTBOUND_UPDATE_TAGS"
)]
pub struct CUpdateTagsPacket {
    pub tags: HashMap<ResourceLocation, Vec<i32>>,
}

impl PacketWrite for CUpdateTagsPacket {}

impl WriteTo for CUpdateTagsPacket {
    fn write(&self, writer: &mut impl Write) -> Result<()> {
        // Make the Vec<i32> into a Vec<VarInt>
        let mapped_tags: HashMap<ResourceLocation, Vec<VarInt>> = self
            .tags
            .iter()
            .map(|(k, v)| (k.clone(), v.iter().map(|v| VarInt(*v)).collect::<Vec<_>>()))
            .collect();

        mapped_tags.write(writer)?;
        Ok(())
    }
}
impl CUpdateTagsPacket {
    pub fn new(tags: HashMap<ResourceLocation, Vec<i32>>) -> Self {
        Self { tags }
    }
}
