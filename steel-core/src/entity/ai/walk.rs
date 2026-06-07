//! Vanilla walk path-type classification.

use steel_math::floor;
use steel_registry::blocks::block_state_ext::BlockStateExt as _;
use steel_registry::blocks::properties::BlockStateProperties;
use steel_registry::vanilla_block_tags::BlockTag;
use steel_registry::vanilla_blocks;
use steel_utils::BlockPos;

use crate::behavior::BlockStateBehaviorExt as _;
use crate::entity::Mob;
use crate::entity::ai::path::{
    PathComputationType, PathType, PathTypeSet, PathfindingContext, PathfindingMalus,
};
use crate::fluid::FluidStateExt as _;
use crate::world::LevelReader;

#[derive(Debug, Clone)]
pub struct MobPathSettings {
    entity_width: i32,
    entity_height: i32,
    entity_depth: i32,
    mob_position: BlockPos,
    malus: [f32; PathType::COUNT],
    can_pass_doors: bool,
    can_open_doors: bool,
    can_float: bool,
    can_walk_over_fences: bool,
}

impl MobPathSettings {
    #[must_use]
    pub fn from_mob<M: Mob + ?Sized>(mob: &M) -> Self {
        let bounding_box = mob.bounding_box();
        let mut malus = [0.0; PathType::COUNT];
        for path_type in PathType::ALL {
            malus[path_type.index()] = mob.get_pathfinding_malus(path_type);
        }

        Self {
            entity_width: floor(bounding_box.width() + 1.0),
            entity_height: floor(bounding_box.height() + 1.0),
            entity_depth: floor(bounding_box.width() + 1.0),
            mob_position: mob.block_position(),
            malus,
            can_pass_doors: true,
            can_open_doors: false,
            can_float: false,
            can_walk_over_fences: false,
        }
    }

    #[must_use]
    pub fn new(
        entity_width: i32,
        entity_height: i32,
        entity_depth: i32,
        mob_position: BlockPos,
        pathfinding_malus: &PathfindingMalus,
    ) -> Self {
        let mut malus = [0.0; PathType::COUNT];
        for path_type in PathType::ALL {
            malus[path_type.index()] = pathfinding_malus.get(path_type);
        }

        Self {
            entity_width: entity_width.max(1),
            entity_height: entity_height.max(1),
            entity_depth: entity_depth.max(1),
            mob_position,
            malus,
            can_pass_doors: true,
            can_open_doors: false,
            can_float: false,
            can_walk_over_fences: false,
        }
    }

    #[must_use]
    pub const fn with_can_pass_doors(mut self, can_pass_doors: bool) -> Self {
        self.can_pass_doors = can_pass_doors;
        self
    }

    #[must_use]
    pub const fn with_can_open_doors(mut self, can_open_doors: bool) -> Self {
        self.can_open_doors = can_open_doors;
        self
    }

    #[must_use]
    pub const fn with_can_float(mut self, can_float: bool) -> Self {
        self.can_float = can_float;
        self
    }

    #[must_use]
    pub const fn with_can_walk_over_fences(mut self, can_walk_over_fences: bool) -> Self {
        self.can_walk_over_fences = can_walk_over_fences;
        self
    }

    #[must_use]
    pub const fn entity_width(&self) -> i32 {
        self.entity_width
    }

    #[must_use]
    pub const fn entity_height(&self) -> i32 {
        self.entity_height
    }

    #[must_use]
    pub const fn entity_depth(&self) -> i32 {
        self.entity_depth
    }

    #[must_use]
    pub const fn mob_position(&self) -> BlockPos {
        self.mob_position
    }

    #[must_use]
    pub const fn pathfinding_malus(&self, path_type: PathType) -> f32 {
        self.malus[path_type.index()]
    }

    #[must_use]
    pub const fn can_pass_doors(&self) -> bool {
        self.can_pass_doors
    }

    #[must_use]
    pub const fn can_open_doors(&self) -> bool {
        self.can_open_doors
    }

    #[must_use]
    pub const fn can_float(&self) -> bool {
        self.can_float
    }

    #[must_use]
    pub const fn can_walk_over_fences(&self) -> bool {
        self.can_walk_over_fences
    }
}

#[derive(Debug, Clone)]
pub struct WalkNodeEvaluator {
    settings: MobPathSettings,
}

impl WalkNodeEvaluator {
    #[must_use]
    pub const fn new(settings: MobPathSettings) -> Self {
        Self { settings }
    }

    #[must_use]
    pub const fn settings(&self) -> &MobPathSettings {
        &self.settings
    }

    #[must_use]
    pub fn get_path_type_of_mob(
        &self,
        context: &mut PathfindingContext<'_>,
        x: i32,
        y: i32,
        z: i32,
    ) -> PathType {
        let block_types = self.get_path_type_within_mob_bb(context, x, y, z);
        if let Some(path_type) = block_types.single() {
            return path_type;
        }

        if block_types.contains(PathType::Fence) {
            return PathType::Fence;
        }

        if block_types.contains(PathType::UnpassableRail) {
            return PathType::UnpassableRail;
        }

        let mut highest_malus_path_type = PathType::Blocked;
        let mut highest_malus = self.settings.pathfinding_malus(highest_malus_path_type);
        for path_type in block_types.iter() {
            let malus = self.settings.pathfinding_malus(path_type);
            if malus < 0.0 {
                return path_type;
            }
            if malus >= highest_malus {
                highest_malus = malus;
                highest_malus_path_type = path_type;
            }
        }

        let current_node_path_type = WalkPathEvaluator::path_type(context, x, y, z);
        if self.settings.entity_width > 1 {
            let current_is_cheaper =
                self.settings.pathfinding_malus(current_node_path_type) < highest_malus;
            let cap_due_to_cheap_node = current_is_cheaper
                && self
                    .settings
                    .pathfinding_malus(PathType::BigMobsCloseToDanger)
                    < highest_malus;
            if cap_due_to_cheap_node {
                PathType::BigMobsCloseToDanger
            } else {
                highest_malus_path_type
            }
        } else if current_node_path_type == PathType::Open
            && highest_malus_path_type != PathType::Open
            && highest_malus == 0.0
        {
            PathType::Open
        } else {
            highest_malus_path_type
        }
    }

    #[must_use]
    pub fn get_path_type_within_mob_bb(
        &self,
        context: &mut PathfindingContext<'_>,
        x: i32,
        y: i32,
        z: i32,
    ) -> PathTypeSet {
        let mut block_types = PathTypeSet::new();
        let mut mob_on_rail = None;

        for dx in 0..self.settings.entity_width {
            for dy in 0..self.settings.entity_height {
                for dz in 0..self.settings.entity_depth {
                    let mut block_type =
                        WalkPathEvaluator::path_type(context, x + dx, y + dy, z + dz);
                    block_type =
                        self.adjust_path_type_for_mob(context, block_type, &mut mob_on_rail);
                    block_types.insert(block_type);
                }
            }
        }

        block_types
    }

    fn adjust_path_type_for_mob(
        &self,
        context: &mut PathfindingContext<'_>,
        block_type: PathType,
        mob_on_rail: &mut Option<bool>,
    ) -> PathType {
        if block_type == PathType::DoorWoodClosed
            && self.settings.can_open_doors
            && self.settings.can_pass_doors
        {
            return PathType::WalkableDoor;
        }

        if block_type == PathType::DoorOpen && !self.settings.can_pass_doors {
            return PathType::Blocked;
        }

        if block_type != PathType::Rail {
            return block_type;
        }

        if mob_on_rail.is_none() {
            let mob_position = self.settings.mob_position();
            *mob_on_rail = Some(
                WalkPathEvaluator::path_type(
                    context,
                    mob_position.x(),
                    mob_position.y(),
                    mob_position.z(),
                ) == PathType::Rail
                    || WalkPathEvaluator::path_type(
                        context,
                        mob_position.x(),
                        mob_position.y() - 1,
                        mob_position.z(),
                    ) == PathType::Rail,
            );
        }

        if matches!(mob_on_rail, Some(true)) {
            PathType::Rail
        } else {
            PathType::UnpassableRail
        }
    }
}

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

    use super::{MobPathSettings, WalkNodeEvaluator, WalkPathEvaluator};
    use crate::behavior::{BlockStateBehaviorExt as _, init_behaviors};
    use crate::entity::ai::path::{
        PathComputationType, PathType, PathfindingContext, PathfindingMalus,
    };
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
    fn walk_node_evaluator_applies_vanilla_door_adjustments_for_mobs() {
        init_test_registry();
        init_behaviors();

        let oak_closed = vanilla_blocks::OAK_DOOR
            .default_state()
            .set_value(&BlockStateProperties::OPEN, false);
        let oak_open = oak_closed.set_value(&BlockStateProperties::OPEN, true);
        let closed_level = GridLevel::new(oak_closed);
        let open_level = GridLevel::new(oak_open);
        let mut closed_context = PathfindingContext::new(&closed_level, BlockPos::ZERO);
        let mut open_context = PathfindingContext::new(&open_level, BlockPos::ZERO);

        let opener = WalkNodeEvaluator::new(
            test_settings(1, 1, 1)
                .with_can_open_doors(true)
                .with_can_pass_doors(true),
        );
        let blocker = WalkNodeEvaluator::new(test_settings(1, 1, 1).with_can_pass_doors(false));

        assert_eq!(
            opener.get_path_type_of_mob(&mut closed_context, 0, 64, 0),
            PathType::WalkableDoor
        );
        assert_eq!(
            blocker.get_path_type_of_mob(&mut open_context, 0, 64, 0),
            PathType::Blocked
        );
    }

    #[test]
    fn walk_node_evaluator_marks_rails_unpassable_when_mob_is_not_on_rails() {
        init_test_registry();
        init_behaviors();

        let air = REGISTRY.blocks.get_default_state_id(&vanilla_blocks::AIR);
        let rail = REGISTRY.blocks.get_default_state_id(&vanilla_blocks::RAIL);
        let level = GridLevel::new(air).with(BlockPos::new(1, 64, 0), rail);
        let mut context = PathfindingContext::new(&level, BlockPos::new(0, 64, 0));
        let evaluator = WalkNodeEvaluator::new(test_settings(1, 1, 1));

        assert_eq!(
            evaluator.get_path_type_of_mob(&mut context, 1, 64, 0),
            PathType::UnpassableRail
        );
    }

    #[test]
    fn large_walk_node_evaluator_caps_nearby_danger_cost_like_vanilla() {
        init_test_registry();
        init_behaviors();

        let air = REGISTRY.blocks.get_default_state_id(&vanilla_blocks::AIR);
        let stone = REGISTRY.blocks.get_default_state_id(&vanilla_blocks::STONE);
        let water = REGISTRY.blocks.get_default_state_id(&vanilla_blocks::WATER);
        let level = GridLevel::new(air)
            .with(BlockPos::new(0, 63, 0), stone)
            .with(BlockPos::new(3, 64, 0), water);
        let mut context = PathfindingContext::new(&level, BlockPos::new(0, 64, 0));
        let evaluator = WalkNodeEvaluator::new(test_settings(4, 1, 1));

        assert_eq!(
            evaluator.get_path_type_of_mob(&mut context, 0, 64, 0),
            PathType::BigMobsCloseToDanger
        );
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

    fn test_settings(entity_width: i32, entity_height: i32, entity_depth: i32) -> MobPathSettings {
        MobPathSettings::new(
            entity_width,
            entity_height,
            entity_depth,
            BlockPos::new(0, 64, 0),
            &PathfindingMalus::new(),
        )
    }
}
