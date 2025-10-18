use steel_utils::ResourceLocation;

use crate::data_components::{DataComponentRegistry, DataComponentType};

pub const MAX_STACK_SIZE: &'static DataComponentType<i32> =
    &DataComponentType::new(ResourceLocation::vanilla_static("max_stack_size"));

pub const MAX_DAMAGE: &'static DataComponentType<i32> =
    &DataComponentType::new(ResourceLocation::vanilla_static("max_damage"));

pub const DAMAGE: &'static DataComponentType<i32> =
    &DataComponentType::new(ResourceLocation::vanilla_static("damage"));

pub const UNBREAKABLE: &'static DataComponentType<()> =
    &DataComponentType::new(ResourceLocation::vanilla_static("unbreakable"));

pub fn register_vanilla_data_components(registry: &mut DataComponentRegistry) {
    registry.register(MAX_STACK_SIZE);
    registry.register(MAX_DAMAGE);
    registry.register(DAMAGE);
    registry.register(UNBREAKABLE);
}
