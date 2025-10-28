use std::collections::HashMap;
use steel_utils::ResourceLocation;

use crate::RegistryExt;

/// Represents a full cat variant definition from a data pack JSON file.
#[derive(Debug)]
pub struct CatVariant {
    pub key: ResourceLocation,
    pub asset_id: ResourceLocation,
    pub spawn_conditions: &'static [SpawnConditionEntry],
}

/// A single entry in the list of spawn conditions.
#[derive(Debug)]
pub struct SpawnConditionEntry {
    pub priority: i32,
    pub condition: Option<SpawnCondition>,
}

/// Defines various spawn conditions for cat variants.
#[derive(Debug)]
pub enum SpawnCondition {
    Structure { structures: &'static str },
    MoonBrightness { min: Option<f32>, max: Option<f32> },
    Biome { biomes: &'static str },
}

pub type CatVariantRef = &'static CatVariant;

pub struct CatVariantRegistry {
    cat_variants: HashMap<ResourceLocation, CatVariantRef>,
    allows_registering: bool,
}

impl CatVariantRegistry {
    pub fn new() -> Self {
        Self {
            cat_variants: HashMap::new(),
            allows_registering: true,
        }
    }

    pub fn register(&mut self, cat_variant: CatVariantRef) {
        if !self.allows_registering {
            panic!("Cannot register cat variants after the registry has been frozen");
        }

        self.cat_variants
            .insert(cat_variant.key.clone(), cat_variant);
    }
}

impl RegistryExt for CatVariantRegistry {
    fn freeze(&mut self) {
        self.allows_registering = false;
    }
}
