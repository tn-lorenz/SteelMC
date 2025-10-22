
use steel_registry::{
    blocks::blocks::BlockRegistry,
    data_components::{DataComponentRegistry, vanilla_components},
    items::items::ItemRegistry,
    vanilla_blocks, vanilla_items,
};
use steel_utils::ResourceLocation;

#[tokio::main]
async fn main() {
    let mut registry = BlockRegistry::new();

    let start = tokio::time::Instant::now();
    vanilla_blocks::register_blocks(&mut registry);
    registry.freeze();
    println!("Block registry loaded in: {:?}", start.elapsed());

    let mut data_component_registry = DataComponentRegistry::new();
    vanilla_components::register_vanilla_data_components(&mut data_component_registry);
    data_component_registry.freeze();

    let mut item_registry = ItemRegistry::new();
    let start = tokio::time::Instant::now();
    vanilla_items::register_items(&mut item_registry);
    item_registry.freeze();
    println!("Item registry loaded in: {:?}", start.elapsed());

    println!(
        "Test item id: {:?}",
        item_registry.by_key(&ResourceLocation::vanilla_static("stone"))
    );
}
