use std::collections::HashMap;
use steel_utils::ResourceLocation;

use crate::RegistryExt;

/// Represents a damage type definition from a data pack JSON file.
#[derive(Debug)]
pub struct DamageType {
    pub key: ResourceLocation,
    pub message_id: &'static str,
    pub scaling: DamageScaling,
    pub exhaustion: f32,
    pub effects: DamageEffects,
    pub death_message_type: DeathMessageType,
}

/// How the damage scales with difficulty.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DamageScaling {
    Always,
    WhenCausedByLivingNonPlayer,
    Never,
}

/// The sound effects played when an entity is damaged.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DamageEffects {
    Hurt,
    Thorns,
    Drowning,
    Burning,
    Poking,
    Freezing,
}

/// How the death message is formatted.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeathMessageType {
    Default,
    FallVariants,
    IntentionalGameDesign,
}

pub type DamageTypeRef = &'static DamageType;

pub struct DamageTypeRegistry {
    damage_types: HashMap<ResourceLocation, DamageTypeRef>,
    allows_registering: bool,
}

impl DamageTypeRegistry {
    pub fn new() -> Self {
        Self {
            damage_types: HashMap::new(),
            allows_registering: true,
        }
    }

    pub fn register(&mut self, damage_type: DamageTypeRef) {
        if !self.allows_registering {
            panic!("Cannot register damage types after the registry has been frozen");
        }

        self.damage_types
            .insert(damage_type.key.clone(), damage_type);
    }
}

impl RegistryExt for DamageTypeRegistry {
    fn freeze(&mut self) {
        self.allows_registering = false;
    }
}
