//! Path navigation state shell.

use glam::DVec3;
use steel_math::floor;
use steel_utils::BlockPos;

use crate::entity::ai::path::{Path, PathTypeCache, PathfindingContext};
use crate::entity::ai::pathfinder::{PathFinder, PathRequest};
use crate::entity::ai::walk::{WalkNodeCollision, WalkNodeEvaluator};
use crate::world::LevelReader;

const DIRECT_TARGET_REACHED_DISTANCE_SQR: f64 = 2.500_000_3e-7;
const DEFAULT_REQUIRED_PATH_LENGTH: f32 = 16.0;
const MAX_VISITED_NODES_SCALE: f32 = 16.0;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NavigationPathRequest<'a> {
    pub mob_position: BlockPos,
    pub targets: &'a [BlockPos],
    pub max_path_length: f32,
    pub reach_range: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PathNavigation {
    path: Option<Path>,
    path_finder: PathFinder,
    path_type_cache: PathTypeCache,
    direct_target: Option<DVec3>,
    target_pos: Option<BlockPos>,
    speed_modifier: f64,
    max_visited_nodes_multiplier: f32,
    required_path_length: f32,
    reach_range: i32,
    tick: i32,
    done: bool,
}

impl PathNavigation {
    #[must_use]
    pub fn new() -> Self {
        Self {
            path: None,
            path_finder: PathFinder::new(0),
            path_type_cache: PathTypeCache::new(),
            direct_target: None,
            target_pos: None,
            speed_modifier: 0.0,
            max_visited_nodes_multiplier: 1.0,
            required_path_length: DEFAULT_REQUIRED_PATH_LENGTH,
            reach_range: 0,
            tick: 0,
            done: true,
        }
    }

    #[must_use]
    pub const fn path(&self) -> Option<&Path> {
        self.path.as_ref()
    }

    #[must_use]
    pub const fn target_pos(&self) -> Option<BlockPos> {
        self.target_pos
    }

    #[must_use]
    pub const fn speed_modifier(&self) -> f64 {
        self.speed_modifier
    }

    #[must_use]
    pub const fn reach_range(&self) -> i32 {
        self.reach_range
    }

    #[must_use]
    pub const fn required_path_length(&self) -> f32 {
        self.required_path_length
    }

    #[must_use]
    pub const fn max_visited_nodes_multiplier(&self) -> f32 {
        self.max_visited_nodes_multiplier
    }

    pub fn set_required_path_length(&mut self, required_path_length: f32, follow_range: f64) {
        self.required_path_length = required_path_length;
        self.update_pathfinder_max_visited_nodes(follow_range);
    }

    pub const fn reset_max_visited_nodes_multiplier(&mut self) {
        self.max_visited_nodes_multiplier = 1.0;
    }

    pub const fn set_max_visited_nodes_multiplier(&mut self, max_visited_nodes_multiplier: f32) {
        self.max_visited_nodes_multiplier = max_visited_nodes_multiplier;
    }

    #[must_use]
    pub const fn is_done(&self) -> bool {
        self.done
    }

    #[must_use]
    pub const fn tick_count(&self) -> i32 {
        self.tick
    }

    pub const fn tick(&mut self) {
        self.tick = self.tick.wrapping_add(1);
    }

    pub fn stop(&mut self) {
        self.path = None;
        self.direct_target = None;
        self.target_pos = None;
        self.speed_modifier = 0.0;
        self.done = true;
    }

    #[must_use]
    pub const fn max_path_length(&self, follow_range: f64) -> f32 {
        (follow_range as f32).max(self.required_path_length)
    }

    pub fn update_pathfinder_max_visited_nodes(&mut self, follow_range: f64) {
        let max_visited_nodes = floor(f64::from(
            self.max_path_length(follow_range) * MAX_VISITED_NODES_SCALE,
        ));
        self.path_finder.set_max_visited_nodes(max_visited_nodes);
    }

    pub fn create_path(
        &mut self,
        evaluator: &mut WalkNodeEvaluator,
        level: &dyn LevelReader,
        collision: &mut impl WalkNodeCollision,
        request: NavigationPathRequest<'_>,
    ) -> Option<Path> {
        let mut context =
            PathfindingContext::with_cache(level, request.mob_position, &mut self.path_type_cache);
        let path = self.path_finder.find_path(
            evaluator,
            &mut context,
            collision,
            PathRequest {
                targets: request.targets,
                max_path_length: request.max_path_length,
                reach_range: request.reach_range,
                max_visited_nodes_multiplier: self.max_visited_nodes_multiplier,
            },
        )?;
        self.target_pos = Some(path.target());
        self.reach_range = request.reach_range;
        Some(path)
    }

    pub fn move_to(&mut self, path: Path, speed_modifier: f64) -> bool {
        self.direct_target = None;
        if path.node_count() == 0 {
            self.path = None;
            self.done = true;
            return false;
        }

        let same_as_current = self
            .path
            .as_ref()
            .is_some_and(|current| path.same_as(current));
        if !same_as_current {
            self.path = Some(path);
        }
        if self.path.as_ref().is_none_or(Path::is_done) {
            self.done = true;
            return false;
        }

        self.target_pos = self.path.as_ref().map(Path::target);
        self.speed_modifier = speed_modifier;
        self.done = false;
        true
    }

    pub fn set_direct_target(&mut self, target: DVec3, speed_modifier: f64) {
        self.path = None;
        self.direct_target = Some(target);
        self.target_pos = Some(BlockPos::new(
            target.x.floor() as i32,
            target.y.floor() as i32,
            target.z.floor() as i32,
        ));
        self.speed_modifier = speed_modifier;
        self.done = false;
    }

    pub fn next_move_target(
        &mut self,
        mob_position: DVec3,
        mob_bounding_box_width: f64,
    ) -> Option<(DVec3, f64)> {
        if self.done {
            return None;
        }

        if self.path.is_some() {
            return self.next_path_move_target(mob_position, mob_bounding_box_width);
        }

        let target = self.direct_target?;
        if target.distance_squared(mob_position) < DIRECT_TARGET_REACHED_DISTANCE_SQR {
            self.stop();
            return None;
        }

        Some((target, self.speed_modifier))
    }

    fn next_path_move_target(
        &mut self,
        mob_position: DVec3,
        mob_bounding_box_width: f64,
    ) -> Option<(DVec3, f64)> {
        let Some(path) = self.path.as_mut() else {
            self.done = true;
            return None;
        };

        let Some(current_node_pos) = path.next_node_pos() else {
            self.stop();
            return None;
        };

        let max_distance_to_waypoint = if mob_bounding_box_width > 0.75 {
            mob_bounding_box_width / 2.0
        } else {
            0.75 - mob_bounding_box_width / 2.0
        };
        let x_distance = (mob_position.x - (f64::from(current_node_pos.x()) + 0.5)).abs();
        let y_distance = (mob_position.y - f64::from(current_node_pos.y())).abs();
        let z_distance = (mob_position.z - (f64::from(current_node_pos.z()) + 0.5)).abs();
        if x_distance < max_distance_to_waypoint
            && z_distance < max_distance_to_waypoint
            && y_distance < 1.0
        {
            path.advance();
        }

        if path.is_done() {
            self.stop();
            return None;
        }

        let target = path.next_node().map(|node| {
            let offset = f64::from(floor(mob_bounding_box_width + 1.0)) * 0.5;
            DVec3::new(
                f64::from(node.x) + offset,
                f64::from(node.y),
                f64::from(node.z) + offset,
            )
        })?;
        Some((target, self.speed_modifier))
    }
}

impl Default for PathNavigation {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use glam::DVec3;
    use steel_registry::{REGISTRY, test_support::init_test_registry, vanilla_blocks};
    use steel_utils::{BlockPos, BlockStateId, WorldAabb};

    use super::{NavigationPathRequest, PathNavigation};
    use crate::behavior::init_behaviors;
    use crate::entity::ai::node::Node;
    use crate::entity::ai::path::{Path, PathfindingMalus};
    use crate::entity::ai::walk::{MobPathSettings, WalkNodeEvaluator};
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
    fn move_to_path_targets_next_node_after_current_node_is_reached() {
        let path = Path::new(
            vec![Node::new(0, 64, 0), Node::new(1, 64, 0)],
            BlockPos::new(1, 64, 0),
            true,
        );
        let mut navigation = PathNavigation::new();

        assert!(navigation.move_to(path, 1.25));

        let target = navigation.next_move_target(DVec3::new(0.5, 64.0, 0.5), 0.9);

        let Some((target, speed)) = target else {
            panic!("navigation should target the next path node");
        };
        assert_eq!(target, DVec3::new(1.5, 64.0, 0.5));
        assert_eq!(speed.to_bits(), 1.25_f64.to_bits());
        assert!(!navigation.is_done());
    }

    #[test]
    fn path_navigation_stops_after_final_node_is_reached() {
        let path = Path::new(vec![Node::new(0, 64, 0)], BlockPos::new(0, 64, 0), true);
        let mut navigation = PathNavigation::new();

        assert!(navigation.move_to(path, 1.0));

        assert!(
            navigation
                .next_move_target(DVec3::new(0.5, 64.0, 0.5), 0.9)
                .is_none()
        );
        assert!(navigation.is_done());
        assert!(navigation.path().is_none());
        assert_eq!(navigation.target_pos(), None);
    }

    #[test]
    fn move_to_same_path_keeps_current_progress() {
        let path = Path::new(
            vec![
                Node::new(0, 64, 0),
                Node::new(1, 64, 0),
                Node::new(2, 64, 0),
            ],
            BlockPos::new(2, 64, 0),
            true,
        );
        let same_path = Path::new(
            vec![
                Node::new(0, 64, 0),
                Node::new(1, 64, 0),
                Node::new(2, 64, 0),
            ],
            BlockPos::new(2, 64, 0),
            true,
        );
        let mut navigation = PathNavigation::new();

        assert!(navigation.move_to(path, 1.0));
        assert!(
            navigation
                .next_move_target(DVec3::new(0.5, 64.0, 0.5), 0.9)
                .is_some()
        );
        assert_eq!(navigation.path().map(Path::next_node_index), Some(1));

        assert!(navigation.move_to(same_path, 1.5));

        assert_eq!(navigation.path().map(Path::next_node_index), Some(1));
        assert_eq!(navigation.speed_modifier().to_bits(), 1.5_f64.to_bits());
    }

    #[test]
    fn direct_target_stops_when_reached() {
        let mut navigation = PathNavigation::new();
        navigation.set_direct_target(DVec3::new(1.0, 64.0, 1.0), 0.5);

        assert!(
            navigation
                .next_move_target(DVec3::new(1.0, 64.0, 1.0), 0.9)
                .is_none()
        );
        assert!(navigation.is_done());
    }

    #[test]
    fn create_path_finds_walkable_target_with_cached_navigation_state() {
        init_test_registry();
        init_behaviors();

        let air = REGISTRY.blocks.get_default_state_id(&vanilla_blocks::AIR);
        let stone = REGISTRY.blocks.get_default_state_id(&vanilla_blocks::STONE);
        let level = GridLevel::new(air)
            .with(BlockPos::new(0, 63, 0), stone)
            .with(BlockPos::new(1, 63, 0), stone)
            .with(BlockPos::new(2, 63, 0), stone);
        let malus = PathfindingMalus::new();
        let mut evaluator = WalkNodeEvaluator::new(MobPathSettings::new(
            1,
            1,
            1,
            BlockPos::new(0, 64, 0),
            &malus,
        ));
        let mut navigation = PathNavigation::new();
        navigation.update_pathfinder_max_visited_nodes(16.0);
        let mut no_collision = |_aabb: WorldAabb| false;

        let path = navigation.create_path(
            &mut evaluator,
            &level,
            &mut no_collision,
            NavigationPathRequest {
                mob_position: BlockPos::new(0, 64, 0),
                targets: &[BlockPos::new(2, 64, 0)],
                max_path_length: 16.0,
                reach_range: 0,
            },
        );

        let Some(path) = path else {
            panic!("path should be found");
        };
        assert!(path.can_reach());
        assert_eq!(navigation.target_pos(), Some(BlockPos::new(2, 64, 0)));
        assert_eq!(navigation.reach_range(), 0);
    }
}
