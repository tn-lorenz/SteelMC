use steel_registry::{
    behaviour::BlockBehaviourProperties,
    blocks::{Block, BlockRegistry},
    properties::BlockStateProperties,
};

#[tokio::main]
async fn main() {
    let mut registry = BlockRegistry::new();
    registry.register(Box::leak(Box::new(Block::new(
        "test",
        BlockBehaviourProperties::default(),
        &[
            &BlockStateProperties::ATTACHED,
            &BlockStateProperties::FACING,
        ],
    ))));

    registry.freeze();

    println!("{:?}", registry.by_id(0));
    println!("{:#?}", registry.state_to_block_lookup.len());
}
