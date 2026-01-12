use steel_utils::{BlockStateId, math::Axis};

use crate::{
    blocks::block_state_ext::BlockStateExt,
    blocks::{
        BlockRef,
        behaviour::BlockBehaviour,
        properties::{BlockStateProperties, EnumProperty},
    },
    items::item::BlockPlaceContext,
};

pub struct RotatedPillarBlock {
    block: BlockRef,
}

impl RotatedPillarBlock {
    pub const AXIS: EnumProperty<Axis> = BlockStateProperties::AXIS;

    pub const fn new(block: BlockRef) -> Self {
        Self { block }
    }
}

impl BlockBehaviour for RotatedPillarBlock {
    fn get_state_for_placement(&self, context: &BlockPlaceContext<'_>) -> Option<BlockStateId> {
        Some(
            self.block
                .default_state()
                .set_value(&Self::AXIS, context.clicked_face.get_axis()),
        )
    }
}
