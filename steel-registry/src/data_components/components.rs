use std::{collections::HashMap, marker::PhantomData};

use steel_utils::ResourceLocation;

//TODO: Implement codecs, also one for persistent storage and one for network.
pub struct DataComponentType<T> {
    pub key: ResourceLocation,
    _phantom: PhantomData<T>,
}

impl<T> DataComponentType<T> {
    pub const fn new(key: ResourceLocation) -> Self {
        Self {
            key,
            _phantom: PhantomData,
        }
    }
}

pub struct DataComponentRegistry {
    components_by_key: HashMap<ResourceLocation, usize>,
    allows_registering: bool,
}

impl DataComponentRegistry {
    pub fn new() -> Self {
        Self {
            components_by_key: HashMap::new(),
            allows_registering: true,
        }
    }

    pub fn freeze(&mut self) {
        self.allows_registering = false;
    }

    pub fn register<T: 'static>(&mut self, component: &'static DataComponentType<T>) {
        if !self.allows_registering {
            panic!("Cannot register data components after the registry has been frozen");
        }

        let id = self.components_by_key.len();
        self.components_by_key.insert(component.key.clone(), id);
    }

    pub fn get_id<T: 'static>(&self, component: &DataComponentType<T>) -> Option<usize> {
        self.components_by_key.get(&component.key).copied()
    }
}

pub struct DataComponentMap {
    map: HashMap<ResourceLocation, Box<dyn std::any::Any>>,
}

impl DataComponentMap {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn set<T: 'static>(&mut self, component: &DataComponentType<T>, data: Option<T>) {
        if let Some(data) = data {
            self.map.insert(component.key.clone(), Box::new(data));
        } else {
            self.map.remove(&component.key);
        }
    }

    pub fn get<T: 'static>(&self, component: &DataComponentType<T>) -> Option<&T> {
        self.map
            .get(&component.key)
            .and_then(|data| data.downcast_ref::<T>())
    }

    pub fn has<T: 'static>(&self, component: &DataComponentType<T>) -> bool {
        self.map.contains_key(&component.key)
    }
}
