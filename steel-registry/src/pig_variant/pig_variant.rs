use std::collections::HashMap;
use steel_utils::ResourceLocation;

use crate::RegistryExt;

/// Represents a full pig variant definition from a data pack JSON file.
#[derive(Debug)]
pub struct PigVariant {
    pub key: ResourceLocation,
    pub asset_id: ResourceLocation,
    pub model: PigModelType,
    pub spawn_conditions: &'static [SpawnConditionEntry],
}

/// The model type for the pig, which can affect its shape.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PigModelType {
    Normal,
    Cold,
}

impl Default for PigModelType {
    fn default() -> Self {
        PigModelType::Normal
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

pub type PigVariantRef = &'static PigVariant;

pub struct PigVariantRegistry {
    pig_variants: HashMap<ResourceLocation, PigVariantRef>,
    allows_registering: bool,
}

impl PigVariantRegistry {
    pub fn new() -> Self {
        Self {
            pig_variants: HashMap::new(),
            allows_registering: true,
        }
    }

    pub fn register(&mut self, pig_variant: PigVariantRef) {
        if !self.allows_registering {
            panic!("Cannot register pig variants after the registry has been frozen");
        }

        self.pig_variants
            .insert(pig_variant.key.clone(), pig_variant);
    }
}

impl RegistryExt for PigVariantRegistry {
    fn freeze(&mut self) {
        self.allows_registering = false;
    }
}
