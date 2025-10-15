use std::fs;

use proc_macro2::TokenStream;
use quote::quote;
use serde::Deserialize;
use steel_utils::math::vector3::Vector3;

#[derive(Deserialize, Clone, Debug)]
pub struct FlammableStruct {
    pub spread_chance: u8,
    pub burn_chance: u8,
}

#[derive(Deserialize, Clone, Debug)]
pub struct BlockState {
    pub id: u16,
    pub state_flags: u16,
    pub side_flags: u8,
    pub instrument: String, // TODO: make this an enum
    pub luminance: u8,
    //pub piston_behavior: PistonBehavior,
    pub hardness: f32,
    pub collision_shapes: Vec<u16>,
    pub outline_shapes: Vec<u16>,
    pub opacity: Option<u8>,
    pub block_entity_type: Option<u16>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Block {
    pub id: u16,
    pub name: String,
    pub translation_key: String,
    pub hardness: f32,
    pub blast_resistance: f32,
    pub item_id: u16,
    pub flammable: Option<FlammableStruct>,
    //pub loot_table: Option<LootTableStruct>,
    pub slipperiness: f32,
    pub velocity_multiplier: f32,
    pub jump_velocity_multiplier: f32,
    pub properties: Vec<i32>,
    pub default_state_id: u16,
    pub states: Vec<BlockState>,
    //pub experience: Option<Experience>,
}

#[derive(Deserialize, Clone, Copy, Debug)]
pub struct CollisionShape {
    pub min: Vector3<f64>,
    pub max: Vector3<f64>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct BlockAssets {
    pub blocks: Vec<Block>,
    pub shapes: Vec<CollisionShape>,
    pub block_entity_types: Vec<String>,
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "type")]
pub enum GeneratedPropertyType {
    #[serde(rename = "boolean")]
    Boolean,
    #[serde(rename = "int")]
    Int { min: u8, max: u8 },
    #[serde(rename = "enum")]
    Enum { values: Vec<String> },
}

#[derive(Deserialize, Clone, Debug)]
pub struct GeneratedProperty {
    hash_key: i32,
    enum_name: String,
    serialized_name: String,
    #[serde(rename = "type")]
    #[serde(flatten)]
    property_type: GeneratedPropertyType,
}

pub(crate) fn build() -> TokenStream {
    let block_assets: BlockAssets =
        serde_json::from_str(&fs::read_to_string("build_assets/blocks.json").unwrap()).unwrap();

    let generated_properties: Vec<GeneratedProperty> =
        serde_json::from_str(&fs::read_to_string("build_assets/properties.json").unwrap())
            .expect("Failed to parse properties.json");

    let mut stream = TokenStream::new();

    stream.extend(quote! {
        pub fn register_blocks(registry: &mut BlockRegistry) {
            for block in block_assets.blocks {
                registry.register(&block);
            }
        }
    });

    stream
}
