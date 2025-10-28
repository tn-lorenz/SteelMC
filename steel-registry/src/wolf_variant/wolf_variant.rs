use std::collections::HashMap;
use steel_utils::ResourceLocation;

use crate::RegistryExt;

/// Represents a full wolf variant definition from a data pack JSON file.
#[derive(Debug)]
pub struct WolfVariant {
    pub key: ResourceLocation,
    pub assets: WolfAssetInfo,
    pub spawn_conditions: &'static [SpawnConditionEntry],
}

/// Contains the texture resource locations for a wolf variant.
#[derive(Debug)]
pub struct WolfAssetInfo {
    pub wild: ResourceLocation,
    pub tame: ResourceLocation,
    pub angry: ResourceLocation,
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

pub type WolfVariantRef = &'static WolfVariant;

pub struct WolfVariantRegistry {
    wolf_variants: HashMap<ResourceLocation, WolfVariantRef>,
    allows_registering: bool,
}

impl WolfVariantRegistry {
    pub fn new() -> Self {
        Self {
            wolf_variants: HashMap::new(),
            allows_registering: true,
        }
    }

    pub fn register(&mut self, wolf_variant: WolfVariantRef) {
        if !self.allows_registering {
            panic!("Cannot register wolf variants after the registry has been frozen");
        }

        self.wolf_variants
            .insert(wolf_variant.key.clone(), wolf_variant);
    }
}

impl RegistryExt for WolfVariantRegistry {
    fn freeze(&mut self) {
        self.allows_registering = false;
    }
}
