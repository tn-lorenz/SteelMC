use std::collections::HashMap;
use steel_utils::ResourceLocation;

use crate::RegistryExt;

/// Represents a full frog variant definition from a data pack JSON file.
#[derive(Debug)]
pub struct FrogVariant {
    pub key: ResourceLocation,
    pub asset_id: ResourceLocation,
    pub spawn_conditions: &'static [SpawnConditionEntry],
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

pub type FrogVariantRef = &'static FrogVariant;

pub struct FrogVariantRegistry {
    frog_variants: HashMap<ResourceLocation, FrogVariantRef>,
    allows_registering: bool,
}

impl FrogVariantRegistry {
    pub fn new() -> Self {
        Self {
            frog_variants: HashMap::new(),
            allows_registering: true,
        }
    }

    pub fn register(&mut self, frog_variant: FrogVariantRef) {
        if !self.allows_registering {
            panic!("Cannot register frog variants after the registry has been frozen");
        }

        self.frog_variants
            .insert(frog_variant.key.clone(), frog_variant);
    }
}

impl RegistryExt for FrogVariantRegistry {
    fn freeze(&mut self) {
        self.allows_registering = false;
    }
}
