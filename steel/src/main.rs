use steel_registry::{blocks::blocks::BlockRegistry, generated::vanilla_blocks};

#[tokio::main]
async fn main() {
    let mut registry = BlockRegistry::new();

    let start = tokio::time::Instant::now();
    vanilla_blocks::register_blocks(&mut registry);
    println!("Time taken: {:?}", start.elapsed());
    registry.freeze();

    let block = registry.by_name("slime_block").unwrap();

    println!("behaviour: {:?}", block.behaviour);

    std::thread::sleep(std::time::Duration::from_secs(10));
}
