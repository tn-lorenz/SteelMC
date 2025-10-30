use std::{
    collections::HashMap,
    io::{Result, Write},
};
use steel_macros::{CBoundPacket, PacketWrite};
use steel_registry::packets::clientbound::config::CLIENTBOUND_UPDATE_TAGS;
use steel_registry::packets::clientbound::play::CLIENTBOUND_UPDATE_TAGS as PLAY_CLIENTBOUND_UPDATE_TAGS;
use steel_utils::ResourceLocation;

use crate::codec::VarInt;

#[derive(CBoundPacket, PacketWrite)]
#[packet_id(
    CONFIGURATION = "CLIENTBOUND_UPDATE_TAGS",
    PLAY = "PLAY_CLIENTBOUND_UPDATE_TAGS"
)]
pub struct CUpdateTagsPacket {
    pub tags: HashMap<ResourceLocation, HashMap<ResourceLocation, Vec<VarInt>>>,
}

impl CUpdateTagsPacket {
    pub fn new(tags: HashMap<ResourceLocation, HashMap<ResourceLocation, Vec<VarInt>>>) -> Self {
        Self { tags }
    }
}
