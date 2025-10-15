use steel_registry::{
    behaviour::BlockBehaviourProperties,
    blocks::{Block, BlockRegistry},
    properties::{self, BlockStateProperties},
};

#[tokio::main]
async fn main() {
    println!("{:?}", BlockStateProperties::HORIZONTAL_AXIS);
}
