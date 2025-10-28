use std::collections::HashMap;
use steel_utils::ResourceLocation;

use crate::RegistryExt;

/// Represents an armor trim material definition from the data packs.
#[derive(Debug)]
pub struct TrimMaterial {
    pub key: ResourceLocation,
    pub asset_name: String,
    pub description: StyledTextComponent,
    pub override_armor_assets: HashMap<ResourceLocation, String>,
}

/// Represents a translatable text component that can also include styling.
#[derive(Debug)]
pub struct StyledTextComponent {
    pub translate: String,
    pub color: Option<String>,
}

pub type TrimMaterialRef = &'static TrimMaterial;

pub struct TrimMaterialRegistry {
    trim_materials: HashMap<ResourceLocation, TrimMaterialRef>,
    allows_registering: bool,
}

impl TrimMaterialRegistry {
    pub fn new() -> Self {
        Self {
            trim_materials: HashMap::new(),
            allows_registering: true,
        }
    }

    pub fn register(&mut self, trim_material: TrimMaterialRef, key: ResourceLocation) {
        if !self.allows_registering {
            panic!("Cannot register trim materials after the registry has been frozen");
        }

        self.trim_materials.insert(key, trim_material);
    }
}

impl RegistryExt for TrimMaterialRegistry {
    fn freeze(&mut self) {
        self.allows_registering = false;
    }
}
