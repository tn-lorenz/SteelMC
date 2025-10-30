use scc::HashMap;
use steel_protocol::packets::common::c_custom_payload_packet::CCustomPayloadPacket;
use steel_protocol::packets::common::{
    s_client_information_packet::SClientInformationPacket,
    s_custom_payload_packet::SCustomPayloadPacket,
};
use steel_protocol::packets::configuration::c_finish_configuration_packet::CFinishConfigurationPacket;
use steel_protocol::packets::configuration::c_registry_data_packet::{
    CRegistryDataPacket, RegistryEntry,
};
use steel_protocol::packets::configuration::c_select_known_packs::CSelectKnownPacks;
use steel_protocol::packets::configuration::s_finish_configuration_packet::SFinishConfigurationPacket;
use steel_protocol::packets::configuration::s_select_known_packs::SSelectKnownPacks;
use steel_protocol::packets::shared_implementation::KnownPack;
use steel_protocol::utils::ConnectionProtocol;
use steel_registry::{
    BANNER_PATTERN_REGISTRY, BIOMES_REGISTRY, CAT_VARIANT_REGISTRY, CHAT_TYPE_REGISTRY,
    CHICKEN_VARIANT_REGISTRY, COW_VARIANT_REGISTRY, DAMAGE_TYPE_REGISTRY, DIMENSION_TYPE_REGISTRY,
    FROG_VARIANT_REGISTRY, INSTRUMENT_REGISTRY, JUKEBOX_SONG_REGISTRY, PAINTING_VARIANT_REGISTRY,
    PIG_VARIANT_REGISTRY, TRIM_MATERIAL_REGISTRY, TRIM_PATTERN_REGISTRY,
    WOLF_SOUND_VARIANT_REGISTRY, WOLF_VARIANT_REGISTRY,
};
use steel_utils::ResourceLocation;
use steel_utils::text::TextComponent;

use crate::network::java_tcp_client::JavaTcpClient;

pub async fn handle_custom_payload(_tcp_client: &JavaTcpClient, packet: &SCustomPayloadPacket) {
    println!("Custom payload packet: {:?}", packet);
}

pub async fn handle_client_information(
    _tcp_client: &JavaTcpClient,
    packet: &SClientInformationPacket,
) {
    println!("Client information packet: {:?}", packet);
}

const BRAND_PAYLOAD: &[u8; 5] = b"Steel";

pub async fn start_configuration(tcp_client: &JavaTcpClient) {
    tcp_client
        .send_packet_now(CCustomPayloadPacket::new(
            ResourceLocation::vanilla_static("brand"),
            Box::new(*BRAND_PAYLOAD),
        ))
        .await;

    tcp_client
        .send_packet_now(CSelectKnownPacks::new(vec![KnownPack::new(
            "minecraft".to_string(),
            "core".to_string(),
            "1.21.10".to_string(),
        )]))
        .await;
}

pub async fn handle_select_known_packs(tcp_client: &JavaTcpClient, packet: &SSelectKnownPacks) {
    println!("Select known packs packet: {:?}", packet);
    let registry = &tcp_client.server.registry;

    let mut registry_data = Vec::new();

    //TODO: For non vanilla entries we need to encode the data into nbt

    registry_data.push((
        BIOMES_REGISTRY,
        registry
            .biomes
            .iter()
            .map(|(_, biome)| RegistryEntry::new(biome.key.clone(), None))
            .collect::<Vec<RegistryEntry>>(),
    ));
    registry_data.push((
        CHAT_TYPE_REGISTRY,
        registry
            .chat_types
            .iter()
            .map(|(_, chat_type)| RegistryEntry::new(chat_type.key.clone(), None))
            .collect::<Vec<RegistryEntry>>(),
    ));
    registry_data.push((
        TRIM_PATTERN_REGISTRY,
        registry
            .trim_patterns
            .iter()
            .map(|(_, trim_pattern)| RegistryEntry::new(trim_pattern.key.clone(), None))
            .collect::<Vec<RegistryEntry>>(),
    ));
    registry_data.push((
        TRIM_MATERIAL_REGISTRY,
        registry
            .trim_materials
            .iter()
            .map(|(_, trim_material)| RegistryEntry::new(trim_material.key.clone(), None))
            .collect::<Vec<RegistryEntry>>(),
    ));
    registry_data.push((
        WOLF_VARIANT_REGISTRY,
        registry
            .wolf_variants
            .iter()
            .map(|(_, wolf_variant)| RegistryEntry::new(wolf_variant.key.clone(), None))
            .collect::<Vec<RegistryEntry>>(),
    ));
    registry_data.push((
        WOLF_SOUND_VARIANT_REGISTRY,
        registry
            .wolf_sound_variants
            .iter()
            .map(|(_, wolf_sound_variant)| RegistryEntry::new(wolf_sound_variant.key.clone(), None))
            .collect::<Vec<RegistryEntry>>(),
    ));
    registry_data.push((
        PIG_VARIANT_REGISTRY,
        registry
            .pig_variants
            .iter()
            .map(|(_, pig_variant)| RegistryEntry::new(pig_variant.key.clone(), None))
            .collect::<Vec<RegistryEntry>>(),
    ));
    registry_data.push((
        FROG_VARIANT_REGISTRY,
        registry
            .frog_variants
            .iter()
            .map(|(_, frog_variant)| RegistryEntry::new(frog_variant.key.clone(), None))
            .collect::<Vec<RegistryEntry>>(),
    ));
    registry_data.push((
        CAT_VARIANT_REGISTRY,
        registry
            .cat_variants
            .iter()
            .map(|(_, cat_variant)| RegistryEntry::new(cat_variant.key.clone(), None))
            .collect::<Vec<RegistryEntry>>(),
    ));
    registry_data.push((
        COW_VARIANT_REGISTRY,
        registry
            .cow_variants
            .iter()
            .map(|(_, cow_variant)| RegistryEntry::new(cow_variant.key.clone(), None))
            .collect::<Vec<RegistryEntry>>(),
    ));
    registry_data.push((
        CHICKEN_VARIANT_REGISTRY,
        registry
            .chicken_variants
            .iter()
            .map(|(_, chicken_variant)| RegistryEntry::new(chicken_variant.key.clone(), None))
            .collect::<Vec<RegistryEntry>>(),
    ));
    registry_data.push((
        PAINTING_VARIANT_REGISTRY,
        registry
            .painting_variants
            .iter()
            .map(|(_, painting_variant)| RegistryEntry::new(painting_variant.key.clone(), None))
            .collect::<Vec<RegistryEntry>>(),
    ));
    registry_data.push((
        DIMENSION_TYPE_REGISTRY,
        registry
            .dimension_types
            .iter()
            .map(|(_, dimension_type)| RegistryEntry::new(dimension_type.key.clone(), None))
            .collect::<Vec<RegistryEntry>>(),
    ));
    registry_data.push((
        DAMAGE_TYPE_REGISTRY,
        registry
            .damage_types
            .iter()
            .map(|(_, damage_type)| RegistryEntry::new(damage_type.key.clone(), None))
            .collect::<Vec<RegistryEntry>>(),
    ));
    registry_data.push((
        BANNER_PATTERN_REGISTRY,
        registry
            .banner_patterns
            .iter()
            .map(|(_, banner_pattern)| RegistryEntry::new(banner_pattern.key.clone(), None))
            .collect::<Vec<RegistryEntry>>(),
    ));
    // TODO: Add enchantments when implemented in the registry
    /*
    registry_data.push((
        ResourceLocation::vanilla_static("enchantments"),
        registry
            .enchantments
            .iter()
            .map(|(_, enchantment)| RegistryEntry::new(enchantment.key.clone(), None))
            .collect::<Vec<RegistryEntry>>(),
    ));
    */
    registry_data.push((
        JUKEBOX_SONG_REGISTRY,
        registry
            .jukebox_songs
            .iter()
            .map(|(_, jukebox_song)| RegistryEntry::new(jukebox_song.key.clone(), None))
            .collect::<Vec<RegistryEntry>>(),
    ));
    registry_data.push((
        INSTRUMENT_REGISTRY,
        registry
            .instruments
            .iter()
            .map(|(_, instrument)| RegistryEntry::new(instrument.key.clone(), None))
            .collect::<Vec<RegistryEntry>>(),
    ));

    // Now send the CRegistryDataPacket for each registry as part of configuration
    for (registry_key, entries) in registry_data.into_iter() {
        tcp_client
            .send_packet_now(CRegistryDataPacket::new(registry_key, entries))
            .await;
    }

    //TODO: Send tags

    // Finish configuration with CFinishConfigurationPacket
    tcp_client
        .send_packet_now(CFinishConfigurationPacket::new())
        .await;
}

pub async fn handle_finish_configuration(
    tcp_client: &JavaTcpClient,
    packet: &SFinishConfigurationPacket,
) {
    tcp_client
        .connection_protocol
        .store(ConnectionProtocol::PLAY);
    println!("Finish configuration packet: {:?}", packet);
}
