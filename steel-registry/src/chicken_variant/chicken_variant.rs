use std::collections::HashMap;
use steel_utils::ResourceLocation;

use crate::RegistryExt;

/// Represents a full chicken variant definition from a data pack JSON file.
#[derive(Debug)]
pub struct ChickenVariant {
    pub key: ResourceLocation,
    pub asset_id: ResourceLocation,
    pub model: ChickenModelType,
    pub spawn_conditions: &'static [SpawnConditionEntry],
}

/// The model type for the chicken, which can affect its shape.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChickenModelType {
    Normal,
    Cold,
}

impl Default for ChickenModelType {
    fn default() -> Self {
        ChickenModelType::Normal
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

pub type ChickenVariantRef = &'static ChickenVariant;

pub struct ChickenVariantRegistry {
    chicken_variants: HashMap<ResourceLocation, ChickenVariantRef>,
    allows_registering: bool,
}

impl ChickenVariantRegistry {
    pub fn new() -> Self {
        Self {
            chicken_variants: HashMap::new(),
            allows_registering: true,
        }
    }

    pub fn register(&mut self, chicken_variant: ChickenVariantRef) {
        if !self.allows_registering {
            panic!("Cannot register chicken variants after the registry has been frozen");
        }

        self.chicken_variants
            .insert(chicken_variant.key.clone(), chicken_variant);
    }
}

impl RegistryExt for ChickenVariantRegistry {
    fn freeze(&mut self) {
        self.allows_registering = false;
    }
}
