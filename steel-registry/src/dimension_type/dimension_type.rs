use std::collections::HashMap;
use steel_utils::ResourceLocation;

use crate::RegistryExt;

/// Represents a full dimension type definition from a data pack JSON file.
#[derive(Debug)]
pub struct DimensionType {
    pub key: ResourceLocation,
    pub fixed_time: Option<i64>,
    pub has_skylight: bool,
    pub has_ceiling: bool,
    pub ultrawarm: bool,
    pub natural: bool,
    pub coordinate_scale: f64,
    pub bed_works: bool,
    pub respawn_anchor_works: bool,
    pub min_y: i32,
    pub height: i32,
    pub logical_height: i32,
    pub infiniburn: &'static str,
    pub effects: ResourceLocation,
    pub ambient_light: f32,
    pub cloud_height: Option<i32>,
    pub piglin_safe: bool,
    pub has_raids: bool,
    pub monster_spawn_light_level: MonsterSpawnLightLevel,
    pub monster_spawn_block_light_limit: i32,
}

/// Represents the complex structure for monster spawn light level.
#[derive(Debug)]
pub enum MonsterSpawnLightLevel {
    Simple(i32),
    Complex {
        distribution_type: &'static str,
        min_inclusive: i32,
        max_inclusive: i32,
    },
}

pub type DimensionTypeRef = &'static DimensionType;

pub struct DimensionTypeRegistry {
    dimension_types: HashMap<ResourceLocation, DimensionTypeRef>,
    allows_registering: bool,
}

impl DimensionTypeRegistry {
    pub fn new() -> Self {
        Self {
            dimension_types: HashMap::new(),
            allows_registering: true,
        }
    }

    pub fn register(&mut self, dimension_type: DimensionTypeRef) {
        if !self.allows_registering {
            panic!("Cannot register dimension types after the registry has been frozen");
        }

        self.dimension_types
            .insert(dimension_type.key.clone(), dimension_type);
    }
}

impl RegistryExt for DimensionTypeRegistry {
    fn freeze(&mut self) {
        self.allows_registering = false;
    }
}
