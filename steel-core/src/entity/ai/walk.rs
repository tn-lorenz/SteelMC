//! Vanilla walk path-type classification.

use steel_registry::blocks::block_state_ext::BlockStateExt as _;
use steel_registry::blocks::properties::BlockStateProperties;
use steel_registry::vanilla_block_tags::BlockTag;
use steel_registry::vanilla_blocks;
use steel_utils::BlockPos;

use crate::behavior::BlockStateBehaviorExt as _;
use crate::entity::ai::path::{PathComputationType, PathType, PathfindingContext};
use crate::fluid::FluidStateExt as _;
use crate::world::LevelReader;

pub struct WalkPathEvaluator;

impl WalkPathEvaluator {
    #[must_use]
    pub fn path_type(context: &mut PathfindingContext<'_>, x: i32, y: i32, z: i32) -> PathType {
        Self::path_type_static(context, BlockPos::new(x, y, z))
    }

    #[must_use]
    pub fn path_type_static(context: &mut PathfindingContext<'_>, pos: BlockPos) -> PathType {
        let x = pos.x();
        let y = pos.y();
        let z = pos.z();
        let block_path_type = context.get_path_type_from_state(x, y, z);
        if block_path_type != PathType::Open || y < context.level().min_y() + 1 {
            return block_path_type;
        }

        match context.get_path_type_from_state(x, y - 1, z) {
            PathType::Open | PathType::Water | PathType::Lava | PathType::Walkable => {
                PathType::Open
            }
            PathType::Fire => PathType::Fire,
            PathType::Damaging => PathType::Damaging,
            PathType::StickyHoney => PathType::StickyHoney,
            PathType::PowderSnow => PathType::OnTopOfPowderSnow,
            PathType::DamageCautious => PathType::DamageCautious,
            PathType::Trapdoor => PathType::OnTopOfTrapdoor,
            _ => Self::check_neighbour_blocks(context, x, y, z, PathType::Walkable),
        }
    }

    #[must_use]
    pub fn check_neighbour_blocks(
        context: &mut PathfindingContext<'_>,
        x: i32,
        y: i32,
        z: i32,
        block_path_type: PathType,
    ) -> PathType {
        for dx in -1..=1 {
            for dy in -1..=1 {
                for dz in -1..=1 {
                    if dx == 0 && dz == 0 {
                        continue;
                    }

                    match context.get_path_type_from_state(x + dx, y + dy, z + dz) {
                        PathType::Damaging => return PathType::DamagingInNeighbor,
                        PathType::Fire | PathType::Lava => return PathType::FireInNeighbor,
                        PathType::Water => return PathType::WaterBorder,
                        PathType::DamageCautious => return PathType::DamageCautious,
                        _ => {}
                    }
                }
            }
        }

        block_path_type
    }

    #[must_use]
    pub fn path_type_from_state(level: &dyn LevelReader, pos: BlockPos) -> PathType {
        let block_state = level.get_block_state(pos);
        let block = block_state.get_block();
        if block_state.is_air() {
            return PathType::Open;
        }

        if block.has_tag(&BlockTag::TRAPDOORS)
            || block == &vanilla_blocks::LILY_PAD
            || block == &vanilla_blocks::BIG_DRIPLEAF
        {
            return PathType::Trapdoor;
        }

        if block == &vanilla_blocks::POWDER_SNOW {
            return PathType::PowderSnow;
        }

        if block == &vanilla_blocks::CACTUS || block == &vanilla_blocks::SWEET_BERRY_BUSH {
            return PathType::Damaging;
        }

        if block == &vanilla_blocks::HONEY_BLOCK {
            return PathType::StickyHoney;
        }

        if block == &vanilla_blocks::COCOA {
            return PathType::Cocoa;
        }

        if block == &vanilla_blocks::WITHER_ROSE || block == &vanilla_blocks::POINTED_DRIPSTONE {
            return PathType::DamageCautious;
        }

        let fluid_state = block_state.get_fluid_state();
        if fluid_state.is_lava() {
            return PathType::Lava;
        }

        if Self::is_burning_block(block_state) {
            return PathType::Fire;
        }

        if block.has_tag(&BlockTag::DOORS) {
            return if block_state
                .try_get_value(&BlockStateProperties::OPEN)
                .unwrap_or(false)
            {
                PathType::DoorOpen
            } else if block.has_tag(&BlockTag::MOB_INTERACTABLE_DOORS) {
                PathType::DoorWoodClosed
            } else {
                PathType::DoorIronClosed
            };
        }

        if block.has_tag(&BlockTag::RAILS) {
            return PathType::Rail;
        }

        if block.has_tag(&BlockTag::LEAVES) {
            return PathType::Leaves;
        }

        if block.has_tag(&BlockTag::FENCES)
            || block.has_tag(&BlockTag::WALLS)
            || block.has_tag(&BlockTag::FENCE_GATES)
                && !block_state
                    .try_get_value(&BlockStateProperties::OPEN)
                    .unwrap_or(false)
        {
            return PathType::Fence;
        }

        if !block_state.is_pathfindable(PathComputationType::Land) {
            return PathType::Blocked;
        }

        if fluid_state.is_water() {
            PathType::Water
        } else {
            PathType::Open
        }
    }

    #[must_use]
    pub fn is_burning_block(block_state: steel_utils::BlockStateId) -> bool {
        let block = block_state.get_block();
        block.has_tag(&BlockTag::FIRE)
            || block == &vanilla_blocks::LAVA
            || block == &vanilla_blocks::MAGMA_BLOCK
            || block == &vanilla_blocks::LAVA_CAULDRON
            || block.has_tag(&BlockTag::CAMPFIRES)
                && block_state
                    .try_get_value(&BlockStateProperties::LIT)
                    .unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use steel_registry::blocks::block_state_ext::BlockStateExt as _;
    use steel_registry::blocks::properties::BlockStateProperties;
    use steel_registry::{REGISTRY, test_support::init_test_registry, vanilla_blocks};
    use steel_utils::{BlockPos, BlockStateId};

    use super::WalkPathEvaluator;
    use crate::behavior::{BlockStateBehaviorExt as _, init_behaviors};
    use crate::entity::ai::path::{PathComputationType, PathType, PathfindingContext};
    use crate::world::LevelReader;

    struct GridLevel {
        default_state: BlockStateId,
        states: Vec<(BlockPos, BlockStateId)>,
    }

    impl GridLevel {
        fn new(default_state: BlockStateId) -> Self {
            Self {
                default_state,
                states: Vec::new(),
            }
        }

        fn with(mut self, pos: BlockPos, state: BlockStateId) -> Self {
            self.states.push((pos, state));
            self
        }
    }

    impl LevelReader for GridLevel {
        fn get_block_state(&self, pos: BlockPos) -> BlockStateId {
            self.states
                .iter()
                .find_map(|(state_pos, state)| (*state_pos == pos).then_some(*state))
                .unwrap_or(self.default_state)
        }

        fn raw_brightness(&self, _pos: BlockPos, _sky_darkening: u8) -> u8 {
            0
        }

        fn min_y(&self) -> i32 {
            -64
        }

        fn height(&self) -> i32 {
            384
        }
    }

    #[test]
    fn path_type_from_state_matches_core_vanilla_special_cases() {
        init_test_registry();
        init_behaviors();

        let air = REGISTRY.blocks.get_default_state_id(&vanilla_blocks::AIR);
        let water = REGISTRY.blocks.get_default_state_id(&vanilla_blocks::WATER);
        let lava = REGISTRY.blocks.get_default_state_id(&vanilla_blocks::LAVA);
        let cactus = REGISTRY
            .blocks
            .get_default_state_id(&vanilla_blocks::CACTUS);
        let honey = REGISTRY
            .blocks
            .get_default_state_id(&vanilla_blocks::HONEY_BLOCK);

        assert_eq!(classify(air), PathType::Open);
        assert_eq!(classify(water), PathType::Water);
        assert_eq!(classify(lava), PathType::Lava);
        assert_eq!(classify(cactus), PathType::Damaging);
        assert_eq!(classify(honey), PathType::StickyHoney);
    }

    #[test]
    fn doors_use_vanilla_mob_interactable_door_tag() {
        init_test_registry();
        init_behaviors();

        let oak_closed = vanilla_blocks::OAK_DOOR
            .default_state()
            .set_value(&BlockStateProperties::OPEN, false);
        let iron_closed = vanilla_blocks::IRON_DOOR
            .default_state()
            .set_value(&BlockStateProperties::OPEN, false);
        let copper_closed = vanilla_blocks::COPPER_DOOR
            .default_state()
            .set_value(&BlockStateProperties::OPEN, false);
        let oak_open = oak_closed.set_value(&BlockStateProperties::OPEN, true);

        assert_eq!(classify(oak_closed), PathType::DoorWoodClosed);
        assert_eq!(classify(copper_closed), PathType::DoorWoodClosed);
        assert_eq!(classify(iron_closed), PathType::DoorIronClosed);
        assert_eq!(classify(oak_open), PathType::DoorOpen);
    }

    #[test]
    fn block_state_pathfindable_uses_behavior_overrides() {
        init_test_registry();
        init_behaviors();

        let water = REGISTRY.blocks.get_default_state_id(&vanilla_blocks::WATER);
        let lava = REGISTRY.blocks.get_default_state_id(&vanilla_blocks::LAVA);
        let cactus = REGISTRY
            .blocks
            .get_default_state_id(&vanilla_blocks::CACTUS);
        let powder_snow = REGISTRY
            .blocks
            .get_default_state_id(&vanilla_blocks::POWDER_SNOW);
        let shallow_snow = vanilla_blocks::SNOW
            .default_state()
            .set_value(&BlockStateProperties::LAYERS, 4);
        let deep_snow = shallow_snow.set_value(&BlockStateProperties::LAYERS, 5);
        let oak_closed = vanilla_blocks::OAK_DOOR
            .default_state()
            .set_value(&BlockStateProperties::OPEN, false);
        let oak_open = oak_closed.set_value(&BlockStateProperties::OPEN, true);

        assert!(water.is_pathfindable(PathComputationType::Land));
        assert!(!lava.is_pathfindable(PathComputationType::Land));
        assert!(!cactus.is_pathfindable(PathComputationType::Land));
        assert!(powder_snow.is_pathfindable(PathComputationType::Land));
        assert!(shallow_snow.is_pathfindable(PathComputationType::Land));
        assert!(!deep_snow.is_pathfindable(PathComputationType::Land));
        assert!(!oak_closed.is_pathfindable(PathComputationType::Land));
        assert!(oak_open.is_pathfindable(PathComputationType::Air));
        assert!(!oak_open.is_pathfindable(PathComputationType::Water));
    }

    #[test]
    fn open_air_above_solid_ground_becomes_walkable() {
        init_test_registry();
        init_behaviors();

        let air = REGISTRY.blocks.get_default_state_id(&vanilla_blocks::AIR);
        let stone = REGISTRY.blocks.get_default_state_id(&vanilla_blocks::STONE);
        let level = GridLevel::new(air).with(BlockPos::new(0, 63, 0), stone);
        let mut context = PathfindingContext::new(&level, BlockPos::new(0, 64, 0));

        assert_eq!(
            WalkPathEvaluator::path_type_static(&mut context, BlockPos::new(0, 64, 0)),
            PathType::Walkable
        );
    }

    #[test]
    fn walkable_ground_adjacent_to_water_becomes_water_border() {
        init_test_registry();
        init_behaviors();

        let air = REGISTRY.blocks.get_default_state_id(&vanilla_blocks::AIR);
        let stone = REGISTRY.blocks.get_default_state_id(&vanilla_blocks::STONE);
        let water = REGISTRY.blocks.get_default_state_id(&vanilla_blocks::WATER);
        let level = GridLevel::new(air)
            .with(BlockPos::new(0, 63, 0), stone)
            .with(BlockPos::new(1, 64, 0), water);
        let mut context = PathfindingContext::new(&level, BlockPos::new(0, 64, 0));

        assert_eq!(
            WalkPathEvaluator::path_type_static(&mut context, BlockPos::new(0, 64, 0)),
            PathType::WaterBorder
        );
    }

    fn classify(state: BlockStateId) -> PathType {
        let level = GridLevel::new(state);
        WalkPathEvaluator::path_type_from_state(&level, BlockPos::ZERO)
    }
}
