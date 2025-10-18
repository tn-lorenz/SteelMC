use steel_registry::{
    blocks::blocks::BlockRegistry,
    data_components::{DataComponentMap, DataComponentRegistry, vanilla_components},
    generated::vanilla_blocks,
};
use steel_utils::ResourceLocation;

#[tokio::main]
async fn main() {
    let mut registry = BlockRegistry::new();

    let start = tokio::time::Instant::now();
    vanilla_blocks::register_blocks(&mut registry);
    println!("Time taken: {:?}", start.elapsed());
    registry.freeze();

    let mut data_component_registry = DataComponentRegistry::new();
    vanilla_components::register_vanilla_data_components(&mut data_component_registry);
    data_component_registry.freeze();

    let mut data_component_map = DataComponentMap::new();

    data_component_map.set(vanilla_components::MAX_STACK_SIZE, Some(64));
    data_component_map.set(vanilla_components::UNBREAKABLE, Some(()));
    println!(
        "Max stack size: {}",
        data_component_map
            .get(vanilla_components::MAX_STACK_SIZE)
            .unwrap()
    );

    data_component_map.set(vanilla_components::UNBREAKABLE, None);

    println!(
        "Unbreakable: {}",
        data_component_map.has(vanilla_components::UNBREAKABLE)
    );

    println!(
        "Reg id of max stack size: {}",
        data_component_registry
            .get_id(vanilla_components::MAX_DAMAGE)
            .unwrap()
    );
}
