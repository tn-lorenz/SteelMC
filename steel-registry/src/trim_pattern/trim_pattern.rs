use std::collections::HashMap;
use steel_utils::ResourceLocation;

use crate::RegistryExt;

/// Represents an armor trim pattern definition from the data packs.
#[derive(Debug)]
pub struct TrimPattern {
    pub key: ResourceLocation,
    pub asset_id: ResourceLocation,
    pub description: TextComponent,
    pub decal: bool,
}

/// A simplified representation of a translatable text component.
#[derive(Debug)]
pub struct TextComponent {
    pub translate: &'static str,
}

pub type TrimPatternRef = &'static TrimPattern;

pub struct TrimPatternRegistry {
    trim_patterns: HashMap<ResourceLocation, TrimPatternRef>,
    allows_registering: bool,
}

impl TrimPatternRegistry {
    pub fn new() -> Self {
        Self {
            trim_patterns: HashMap::new(),
            allows_registering: true,
        }
    }

    pub fn register(&mut self, trim_pattern: TrimPatternRef) {
        if !self.allows_registering {
            panic!("Cannot register trim patterns after the registry has been frozen");
        }

        self.trim_patterns
            .insert(trim_pattern.key.clone(), trim_pattern);
    }
}

impl RegistryExt for TrimPatternRegistry {
    fn freeze(&mut self) {
        self.allows_registering = false;
    }
}
