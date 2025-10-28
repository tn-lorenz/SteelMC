use std::collections::HashMap;
use steel_utils::ResourceLocation;

use crate::RegistryExt;

/// Represents a full cow variant definition from a data pack JSON file.
#[derive(Debug)]
pub struct CowVariant {
    pub key: ResourceLocation,
    pub asset_id: ResourceLocation,
    pub model: CowModelType,
    pub spawn_conditions: &'static [SpawnConditionEntry],
}

/// The model type for the cow, which can affect its shape.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CowModelType {
    Normal,
    Cold,
    Warm,
}

impl Default for CowModelType {
    fn default() -> Self {
        CowModelType::Normal
    }
}

/// A single entry in the list of spawn conditions.
#[derive(Debug)]
pub struct SpawnConditionEntry {
    pub priority: i32,
    pub condition: Option<BiomeCondition>,
}

/// Defines a condition based on a biome or list of biomes.
#[derive(Debug)]
pub struct BiomeCondition {
    pub condition_type: &'static str,
    pub biomes: &'static str,
}

pub type CowVariantRef = &'static CowVariant;

pub struct CowVariantRegistry {
    cow_variants: HashMap<ResourceLocation, CowVariantRef>,
    allows_registering: bool,
}

impl CowVariantRegistry {
    pub fn new() -> Self {
        Self {
            cow_variants: HashMap::new(),
            allows_registering: true,
        }
    }

    pub fn register(&mut self, cow_variant: CowVariantRef) {
        if !self.allows_registering {
            panic!("Cannot register cow variants after the registry has been frozen");
        }

        self.cow_variants
            .insert(cow_variant.key.clone(), cow_variant);
    }
}

impl RegistryExt for CowVariantRegistry {
    fn freeze(&mut self) {
        self.allows_registering = false;
    }
}
