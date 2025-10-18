use steel_registry::{blocks::blocks::BlockRegistry, generated::vanilla_blocks};
use steel_utils::ResourceLocation;

#[tokio::main]
async fn main() {
    let mut registry = BlockRegistry::new();

    let start = tokio::time::Instant::now();
    vanilla_blocks::register_blocks(&mut registry);
    println!("Time taken: {:?}", start.elapsed());
    registry.freeze();

    let block = registry
        .by_key(&ResourceLocation::vanilla("oak_log".to_string()))
        .unwrap();

    println!("block: {:#?}", block);
}
