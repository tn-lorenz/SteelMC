use std::collections::HashMap;
use steel_utils::ResourceLocation;

use crate::RegistryExt;

/// Represents a banner pattern definition from a data pack JSON file.
#[derive(Debug)]
pub struct BannerPattern {
    pub key: ResourceLocation,
    pub asset_id: ResourceLocation,
    pub translation_key: &'static str,
}

pub type BannerPatternRef = &'static BannerPattern;

pub struct BannerPatternRegistry {
    banner_patterns: HashMap<ResourceLocation, BannerPatternRef>,
    allows_registering: bool,
}

impl BannerPatternRegistry {
    pub fn new() -> Self {
        Self {
            banner_patterns: HashMap::new(),
            allows_registering: true,
        }
    }

    pub fn register(&mut self, banner_pattern: BannerPatternRef) {
        if !self.allows_registering {
            panic!("Cannot register banner patterns after the registry has been frozen");
        }

        self.banner_patterns
            .insert(banner_pattern.key.clone(), banner_pattern);
    }
}

impl RegistryExt for BannerPatternRegistry {
    fn freeze(&mut self) {
        self.allows_registering = false;
    }
}
