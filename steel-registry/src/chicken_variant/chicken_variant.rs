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
    chicken_variants_by_id: Vec<ChickenVariantRef>,
    chicken_variants_by_key: HashMap<ResourceLocation, usize>,
    allows_registering: bool,
}

impl ChickenVariantRegistry {
    pub fn new() -> Self {
        Self {
            chicken_variants_by_id: Vec::new(),
            chicken_variants_by_key: HashMap::new(),
            allows_registering: true,
        }
    }

    pub fn register(&mut self, chicken_variant: ChickenVariantRef) -> usize {
        if !self.allows_registering {
            panic!("Cannot register chicken variants after the registry has been frozen");
        }

        let id = self.chicken_variants_by_id.len();
        self.chicken_variants_by_key
            .insert(chicken_variant.key.clone(), id);
        self.chicken_variants_by_id.push(chicken_variant);
        id
    }

    pub fn by_id(&self, id: usize) -> Option<ChickenVariantRef> {
        self.chicken_variants_by_id.get(id).copied()
    }

    pub fn get_id(&self, chicken_variant: ChickenVariantRef) -> &usize {
        self.chicken_variants_by_key
            .get(&chicken_variant.key)
            .expect("Chicken variant not found")
    }

    pub fn by_key(&self, key: &ResourceLocation) -> Option<ChickenVariantRef> {
        self.chicken_variants_by_key
            .get(key)
            .and_then(|id| self.by_id(*id))
    }

    pub fn iter(&self) -> impl Iterator<Item = (usize, ChickenVariantRef)> + '_ {
        self.chicken_variants_by_id
            .iter()
            .enumerate()
            .map(|(id, &variant)| (id, variant))
    }

    pub fn len(&self) -> usize {
        self.chicken_variants_by_id.len()
    }

    pub fn is_empty(&self) -> bool {
        self.chicken_variants_by_id.is_empty()
    }
}

impl RegistryExt for ChickenVariantRegistry {
    fn freeze(&mut self) {
        self.allows_registering = false;
    }
}
