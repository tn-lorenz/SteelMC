//! Barrel block behavior implementation.
//!
//! Opens a 27-slot container menu when right-clicked.

use steel_registry::blocks::BlockRef;
use steel_registry::blocks::block_state_ext::BlockStateExt;
use steel_registry::blocks::properties::BlockStateProperties;
use steel_registry::vanilla_block_entity_types;
use steel_utils::text::TextComponent;
use steel_utils::{BlockPos, BlockStateId, translations};

use crate::behavior::block::BlockBehaviour;
use crate::behavior::context::{BlockHitResult, BlockPlaceContext, InteractionResult};
use crate::block_entity::{BLOCK_ENTITIES, SharedBlockEntity};
use crate::inventory::chest_menu::ChestMenuProvider;
use crate::inventory::lock::ContainerRef;
use crate::player::Player;
use crate::world::World;

/// Behavior for barrel blocks.
///
/// Barrels are container block entities with 27 slots (3x9 grid).
/// They use the same menu as chests but cannot form double containers.
pub struct BarrelBlock {
    block: BlockRef,
}

impl BarrelBlock {
    /// Creates a new barrel block behavior.
    #[must_use]
    pub const fn new(block: BlockRef) -> Self {
        Self { block }
    }
}

impl BlockBehaviour for BarrelBlock {
    fn get_state_for_placement(&self, context: &BlockPlaceContext<'_>) -> Option<BlockStateId> {
        // Barrel faces opposite to the player's look direction.
        // For now, we use horizontal_direction since BlockPlaceContext doesn't have pitch.
        // This means barrels will always face horizontally when placed.
        // TODO: Add pitch to BlockPlaceContext for proper 6-direction facing.
        let facing = context.horizontal_direction.opposite();

        Some(
            self.block
                .default_state()
                .set_value(&BlockStateProperties::FACING, facing),
        )
    }

    fn use_without_item(
        &self,
        _state: BlockStateId,
        world: &World,
        pos: BlockPos,
        player: &Player,
        _hit_result: &BlockHitResult,
    ) -> InteractionResult {
        // Get the block entity
        let Some(block_entity) = world.get_block_entity(&pos) else {
            return InteractionResult::Pass;
        };

        // Create a container reference from the block entity
        let Some(container_ref) = ContainerRef::from_block_entity(block_entity) else {
            return InteractionResult::Pass;
        };

        // Open the chest menu (3 rows for barrel)
        player.open_menu(&ChestMenuProvider::three_rows(
            player.inventory.clone(),
            container_ref,
            TextComponent::new().translate(translations::CONTAINER_BARREL.msg()),
        ));

        // TODO: Award stat OPEN_BARREL
        // TODO: Anger nearby piglins (PiglinAi.angerNearbyPiglins)

        InteractionResult::Success
    }

    fn has_block_entity(&self) -> bool {
        true
    }

    fn new_block_entity(
        &self,
        level: std::sync::Weak<World>,
        pos: BlockPos,
        state: BlockStateId,
    ) -> Option<SharedBlockEntity> {
        BLOCK_ENTITIES.create(vanilla_block_entity_types::BARREL, level, pos, state)
    }
}
