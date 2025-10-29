use std::collections::HashMap;
use steel_utils::ResourceLocation;
use steel_utils::text::TextComponent;

use crate::RegistryExt;

/// Represents a musical instrument definition from a data pack JSON file,
/// primarily used for Goat Horns.
#[derive(Debug)]
pub struct Instrument {
    pub key: ResourceLocation,
    pub sound_event: ResourceLocation,
    pub use_duration: f32,
    pub range: f32,
    pub description: TextComponent,
}

pub type InstrumentRef = &'static Instrument;

pub struct InstrumentRegistry {
    instruments: HashMap<ResourceLocation, InstrumentRef>,
    allows_registering: bool,
}

impl InstrumentRegistry {
    pub fn new() -> Self {
        Self {
            instruments: HashMap::new(),
            allows_registering: true,
        }
    }

    pub fn register(&mut self, instrument: InstrumentRef) {
        if !self.allows_registering {
            panic!("Cannot register instruments after the registry has been frozen");
        }

        self.instruments.insert(instrument.key.clone(), instrument);
    }
}

impl RegistryExt for InstrumentRegistry {
    fn freeze(&mut self) {
        self.allows_registering = false;
    }
}
