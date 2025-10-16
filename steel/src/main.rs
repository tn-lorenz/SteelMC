use steel_registry::{
    behaviour::BlockBehaviourProperties,
    blocks::{Block, BlockRegistry, offset},
    generated::vanilla_blocks,
    properties::{BlockStateProperties, RedstoneSide},
};

pub const REDSTONE_WIRE: Block = Block::new(
    "redstone_wire2",
    BlockBehaviourProperties::new(),
    &[
        &BlockStateProperties::EAST_REDSTONE,
        &BlockStateProperties::NORTH_REDSTONE,
        &BlockStateProperties::POWER,
        &BlockStateProperties::ATTACHED,
    ],
)
.with_default_state(offset!(
    BlockStateProperties::EAST_REDSTONE => RedstoneSide::Up,
    BlockStateProperties::NORTH_REDSTONE => RedstoneSide::None,
    BlockStateProperties::POWER => 10,
    BlockStateProperties::ATTACHED => BlockStateProperties::ATTACHED.index_of(false)
));

#[tokio::main]
async fn main() {
    let mut registry = BlockRegistry::new();

    let start = tokio::time::Instant::now();
    vanilla_blocks::register_blocks(&mut registry);
    registry.register(&REDSTONE_WIRE);
    println!("Time taken: {:?}", start.elapsed());
    registry.freeze();

    let block = registry.by_name("redstone_wire2").unwrap();
    let default_state_id = registry.get_default_state_id(block);
    println!(
        "default_state_id: {:?}",
        registry.get_properties(default_state_id)
    );
}
