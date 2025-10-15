use steel_registry::{
    blocks::{Block, BlockRegistry},
    properties::BlockProperties,
};

#[tokio::main]
async fn main() {
    let mut registry = BlockRegistry::new();

    registry.register(Box::leak(Box::new(Block::new(
        "stone",
        BlockProperties::new().strength(1.0, 3.0),
    ))));
}
