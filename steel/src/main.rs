use steel_registry::{
    blocks::BlockRegistry,
    generated::vanilla_blocks,
    properties::{Axis, BlockStateProperties},
};
use steel_utils::BlockStateId;

#[tokio::main]
async fn main() {
    let mut registry = BlockRegistry::new();

    let start = tokio::time::Instant::now();
    vanilla_blocks::register_blocks(&mut registry);
    println!("Time taken: {:?}", start.elapsed());
    registry.freeze();

    let block = registry.by_name("redstone_wire");
    println!("{:#?}", block);
}
