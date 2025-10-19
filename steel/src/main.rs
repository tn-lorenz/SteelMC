use std::sync::LazyLock;

use steel_registry::{
    blocks::blocks::BlockRegistry,
    data_components::{DataComponentMap, DataComponentRegistry, vanilla_components},
    generated::vanilla_blocks,
    items::items::{Item, ItemRegistry},
};
use steel_utils::ResourceLocation;

static TEST_ITEM: LazyLock<Item> = LazyLock::new(|| Item {
    key: ResourceLocation::vanilla_static("test_item"),
    components: DataComponentMap::common_item_components()
        .builder_set(vanilla_components::MAX_STACK_SIZE, Some(64)),
});

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

    let mut item_registry = ItemRegistry::new();
    item_registry.register(&TEST_ITEM);
    item_registry.freeze();

    println!("Test item id: {:?}", item_registry.by_id(0));
}
