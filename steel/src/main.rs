use steel_registry::{
    behaviour::BlockBehaviourProperties,
    blocks::{Block, BlockRegistry},
    properties::{BlockStateProperties, Direction},
};
use steel_utils::BlockStateId;

pub const TEST_BLOCK: Block = Block::new(
    "test",
    BlockBehaviourProperties::new().jump_factor(5.0),
    &[
        &BlockStateProperties::ATTACHED,
        &BlockStateProperties::FACING,
    ],
);

#[tokio::main]
async fn main() {
    let mut registry = BlockRegistry::new();
    registry.register(&TEST_BLOCK);

    registry.freeze();

    println!("{:?}", registry.by_id(0));
    let mut block_state = registry.set_property(
        BlockStateId(0),
        &BlockStateProperties::FACING,
        Direction::Up,
    );

    block_state = registry.set_property(block_state, &BlockStateProperties::ATTACHED, false);

    println!(
        "Attached: {:#?}, Facing: {:#?}, state id: {:#?}",
        registry.get_property(block_state, &BlockStateProperties::ATTACHED),
        registry.get_property(block_state, &BlockStateProperties::FACING),
        block_state.0,
    );
}
