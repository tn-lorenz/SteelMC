use steel_registry::{
    blocks::BlockRegistry,
    generated::vanilla_blocks,
    properties::{BlockStateProperties, Direction},
};

#[tokio::main]
async fn main() {
    let mut registry = BlockRegistry::new();

    let start = tokio::time::Instant::now();
    vanilla_blocks::register_blocks(&mut registry);
    println!("Time taken: {:?}", start.elapsed());
    registry.freeze();

    println!(
        "{}",
        BlockStateProperties::HORIZONTAL_FACING.get_internal_index_const(&Direction::North)
    );

    let block = registry.by_name("blast_furnace").unwrap();
    let default_state_id = registry.get_default_state_id(block);

    println!(
        "default_state_id: {:?}",
        registry.get_properties(default_state_id)
    );
}
