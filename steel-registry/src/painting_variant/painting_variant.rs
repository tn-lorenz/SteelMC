use std::collections::HashMap;
use steel_utils::ResourceLocation;

use crate::RegistryExt;

/// Represents a painting variant definition from a data pack JSON file.
#[derive(Debug)]
pub struct PaintingVariant {
    pub key: ResourceLocation,
    pub width: i32,
    pub height: i32,
    pub asset_id: ResourceLocation,
    pub title: Option<TextComponent>,
    pub author: Option<TextComponent>,
}

/// Represents a text component with color and translation.
#[derive(Debug)]
pub struct TextComponent {
    pub translate: &'static str,
    pub color: Option<&'static str>,
}

pub type PaintingVariantRef = &'static PaintingVariant;

pub struct PaintingVariantRegistry {
    painting_variants: HashMap<ResourceLocation, PaintingVariantRef>,
    allows_registering: bool,
}

impl PaintingVariantRegistry {
    pub fn new() -> Self {
        Self {
            painting_variants: HashMap::new(),
            allows_registering: true,
        }
    }

    pub fn register(&mut self, painting_variant: PaintingVariantRef) {
        if !self.allows_registering {
            panic!("Cannot register painting variants after the registry has been frozen");
        }

        self.painting_variants
            .insert(painting_variant.key.clone(), painting_variant);
    }
}

impl RegistryExt for PaintingVariantRegistry {
    fn freeze(&mut self) {
        self.allows_registering = false;
    }
}
