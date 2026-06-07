use glam::DVec3;
use steel_registry::blocks::block_state_ext::BlockStateExt as _;
use steel_utils::BlockPos;
use steel_utils::random::Random as _;

use crate::behavior::BlockStateBehaviorExt as _;
use crate::entity::PathfinderMob;
use crate::entity::ai::path::PathfindingContext;
use crate::entity::ai::walk::WalkPathEvaluator;
use crate::fluid::FluidStateExt as _;

const RANDOM_POS_ATTEMPTS: usize = 10;

pub(super) fn default_random_pos(
    mob: &dyn PathfinderMob,
    horizontal_dist: i32,
    vertical_dist: i32,
) -> Option<DVec3> {
    let restrict = mob_restricted(mob, f64::from(horizontal_dist));
    generate_random_pos(mob, || {
        let direction = generate_random_direction(mob, horizontal_dist, vertical_dist);
        default_random_pos_toward_direction(mob, f64::from(horizontal_dist), restrict, direction)
    })
}

pub(super) fn land_random_pos(
    mob: &dyn PathfinderMob,
    horizontal_dist: i32,
    vertical_dist: i32,
) -> Option<DVec3> {
    let restrict = mob_restricted(mob, f64::from(horizontal_dist));
    generate_random_pos(mob, || {
        let direction = generate_random_direction(mob, horizontal_dist, vertical_dist);
        let pos =
            land_random_pos_toward_direction(mob, f64::from(horizontal_dist), restrict, direction)?;
        land_move_pos_up_out_of_solid(mob, pos)
    })
}

fn generate_random_pos(
    mob: &dyn PathfinderMob,
    mut pos_supplier: impl FnMut() -> Option<BlockPos>,
) -> Option<DVec3> {
    let mut best_weight = f32::NEG_INFINITY;
    let mut best_pos = None;

    for _ in 0..RANDOM_POS_ATTEMPTS {
        let Some(pos) = pos_supplier() else {
            continue;
        };
        let value = mob.get_walk_target_value(pos);
        if value > best_weight {
            best_weight = value;
            best_pos = Some(pos);
        }
    }

    best_pos.map(block_bottom_center)
}

fn generate_random_direction(
    mob: &dyn PathfinderMob,
    horizontal_dist: i32,
    vertical_dist: i32,
) -> BlockPos {
    let mut random = mob.base().random().lock();
    BlockPos::new(
        random.next_i32_bounded(2 * horizontal_dist + 1) - horizontal_dist,
        random.next_i32_bounded(2 * vertical_dist + 1) - vertical_dist,
        random.next_i32_bounded(2 * horizontal_dist + 1) - horizontal_dist,
    )
}

fn default_random_pos_toward_direction(
    mob: &dyn PathfinderMob,
    horizontal_dist: f64,
    restrict: bool,
    direction: BlockPos,
) -> Option<BlockPos> {
    let pos = generate_random_pos_toward_direction(mob, horizontal_dist, direction);
    if !is_outside_limits(mob, pos)
        && !is_restricted(restrict, mob, pos)
        && mob.is_stable_destination(pos)
        && !has_malus(mob, pos)
    {
        Some(pos)
    } else {
        None
    }
}

fn land_random_pos_toward_direction(
    mob: &dyn PathfinderMob,
    horizontal_dist: f64,
    restrict: bool,
    direction: BlockPos,
) -> Option<BlockPos> {
    let pos = generate_random_pos_toward_direction(mob, horizontal_dist, direction);
    if !is_outside_limits(mob, pos)
        && !is_restricted(restrict, mob, pos)
        && mob.is_stable_destination(pos)
    {
        Some(pos)
    } else {
        None
    }
}

fn generate_random_pos_toward_direction(
    mob: &dyn PathfinderMob,
    horizontal_dist: f64,
    direction: BlockPos,
) -> BlockPos {
    let mut xt = f64::from(direction.x());
    let mut zt = f64::from(direction.z());
    let position = mob.position();
    if mob.has_home() && horizontal_dist > 1.0 {
        let center = mob.home_position();
        let mut random = mob.base().random().lock();
        if position.x > f64::from(center.x()) {
            xt -= random.next_f64() * horizontal_dist / 2.0;
        } else {
            xt += random.next_f64() * horizontal_dist / 2.0;
        }

        if position.z > f64::from(center.z()) {
            zt -= random.next_f64() * horizontal_dist / 2.0;
        } else {
            zt += random.next_f64() * horizontal_dist / 2.0;
        }
    }

    BlockPos::containing(
        xt + position.x,
        f64::from(direction.y()) + position.y,
        zt + position.z,
    )
}

fn land_move_pos_up_out_of_solid(mob: &dyn PathfinderMob, pos: BlockPos) -> Option<BlockPos> {
    let pos = move_up_out_of_solid(mob, pos)?;
    if !is_water(mob, pos) && !has_malus(mob, pos) {
        Some(pos)
    } else {
        None
    }
}

fn move_up_out_of_solid(mob: &dyn PathfinderMob, pos: BlockPos) -> Option<BlockPos> {
    if !is_solid(mob, pos) {
        return Some(pos);
    }

    let world = mob.level()?;
    let mut pos = pos.above();
    while pos.y() <= world.get_max_y() && is_solid(mob, pos) {
        pos = pos.above();
    }
    Some(pos)
}

fn mob_restricted(mob: &dyn PathfinderMob, horizontal_dist: f64) -> bool {
    mob.has_home()
        && block_center_distance_sqr(mob.home_position(), mob.position())
            < (f64::from(mob.home_radius()) + horizontal_dist + 1.0).powi(2)
}

fn is_outside_limits(mob: &dyn PathfinderMob, pos: BlockPos) -> bool {
    mob.level()
        .is_none_or(|world| world.is_outside_build_height(pos.y()))
}

fn is_restricted(restrict: bool, mob: &dyn PathfinderMob, pos: BlockPos) -> bool {
    restrict && !mob.is_within_home_pos(pos)
}

fn is_water(mob: &dyn PathfinderMob, pos: BlockPos) -> bool {
    mob.level()
        .is_none_or(|world| world.get_block_state(pos).get_fluid_state().is_water())
}

fn has_malus(mob: &dyn PathfinderMob, pos: BlockPos) -> bool {
    let Some(world) = mob.level() else {
        return true;
    };
    let mut context = PathfindingContext::new(world.as_ref(), mob.block_position());
    let path_type = WalkPathEvaluator::path_type_static(&mut context, pos);
    mob.get_pathfinding_malus(path_type) != 0.0
}

fn is_solid(mob: &dyn PathfinderMob, pos: BlockPos) -> bool {
    mob.level()
        .is_some_and(|world| world.get_block_state(pos).is_solid())
}

fn block_bottom_center(pos: BlockPos) -> DVec3 {
    let (x, y, z) = pos.get_bottom_center();
    DVec3::new(x, y, z)
}

fn block_center_distance_sqr(pos: BlockPos, target: DVec3) -> f64 {
    let (x, y, z) = pos.get_center();
    DVec3::new(x, y, z).distance_squared(target)
}
