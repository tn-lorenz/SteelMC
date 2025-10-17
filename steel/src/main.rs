use steel_registry::{blocks::blocks::BlockRegistry, generated::vanilla_blocks};

#[tokio::main]
async fn main() {
    let mut registry = BlockRegistry::new();

    let start = tokio::time::Instant::now();
    vanilla_blocks::register_blocks(&mut registry);
    println!("Time taken: {:?}", start.elapsed());
    registry.freeze();

    let block = registry.by_name("waxed_copper_door").unwrap();
    let default_state_id = registry.get_default_state_id(block);

    println!(
        "default_state_id: {:?}",
        registry.get_properties(default_state_id)
    );

    std::thread::sleep(std::time::Duration::from_secs(10));
}
