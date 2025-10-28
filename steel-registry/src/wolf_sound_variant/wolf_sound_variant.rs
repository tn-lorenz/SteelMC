use std::collections::HashMap;
use steel_utils::ResourceLocation;

use crate::RegistryExt;

/// Represents a set of sounds for a wolf variant from a data pack JSON file.
#[derive(Debug)]
pub struct WolfSoundVariant {
    pub key: ResourceLocation,
    pub ambient_sound: ResourceLocation,
    pub death_sound: ResourceLocation,
    pub growl_sound: ResourceLocation,
    pub hurt_sound: ResourceLocation,
    pub pant_sound: ResourceLocation,
    pub whine_sound: ResourceLocation,
}

pub type WolfSoundVariantRef = &'static WolfSoundVariant;

pub struct WolfSoundVariantRegistry {
    wolf_sound_variants: HashMap<ResourceLocation, WolfSoundVariantRef>,
    allows_registering: bool,
}

impl WolfSoundVariantRegistry {
    pub fn new() -> Self {
        Self {
            wolf_sound_variants: HashMap::new(),
            allows_registering: true,
        }
    }

    pub fn register(&mut self, wolf_sound_variant: WolfSoundVariantRef) {
        if !self.allows_registering {
            panic!("Cannot register wolf sound variants after the registry has been frozen");
        }

        self.wolf_sound_variants
            .insert(wolf_sound_variant.key.clone(), wolf_sound_variant);
    }
}

impl RegistryExt for WolfSoundVariantRegistry {
    fn freeze(&mut self) {
        self.allows_registering = false;
    }
}
