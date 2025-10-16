use steel_registry::{
    behaviour::BlockBehaviourProperties,
    blocks::{Block, BlockRegistry, offset},
    generated::vanilla_blocks,
    properties::{BlockStateProperties, RedstoneSide},
};

#[tokio::main]
async fn main() {
    let mut registry = BlockRegistry::new();

    let start = tokio::time::Instant::now();
    vanilla_blocks::register_blocks(&mut registry);
    println!("Time taken: {:?}", start.elapsed());
    registry.freeze();

    let block = registry.by_name("redstone_wire").unwrap();
    let default_state_id = registry.get_default_state_id(block);
    println!(
        "default_state_id: {:?}",
        registry.get_properties(default_state_id)
    );
}
