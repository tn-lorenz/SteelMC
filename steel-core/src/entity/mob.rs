//! Vanilla-shaped mob foundations.
#![expect(
    dead_code,
    reason = "mob control hooks are foundation code consumed by upcoming goals and pathfinding"
)]

use std::f32::consts::PI;

use glam::DVec3;
use steel_math::floor;
use steel_registry::blocks::block_state_ext::BlockStateExt as _;
use steel_registry::vanilla_attributes;
use steel_registry::vanilla_block_tags::BlockTag;
use steel_utils::locks::SyncMutex;
use steel_utils::{BlockPos, axis::Axis};

use crate::behavior::{BLOCK_BEHAVIORS, BlockCollisionContext};
use crate::entity::ai::control::{MobControls, MoveControlOperation};
use crate::entity::ai::navigation::PathNavigation;
use crate::entity::ai::path::{PathType, PathfindingContext, PathfindingMalus};
use crate::entity::ai::walk::WalkPathEvaluator;
use crate::entity::{LivingEntity, LivingTravelInput};

const MOB_FLAG_NO_AI: i8 = 1;
const MOB_FLAG_LEFT_HANDED: i8 = 2;
const MOB_FLAG_AGGRESSIVE: i8 = 4;
const MOVE_CONTROL_MIN_SPEED_SQR: f64 = 2.500_000_3e-7;
const MOVE_CONTROL_MAX_TURN: f32 = 90.0;

#[derive(Debug)]
pub struct MobBase {
    controls: SyncMutex<MobControls>,
    navigation: SyncMutex<PathNavigation>,
    pathfinding_malus: SyncMutex<PathfindingMalus>,
}

impl MobBase {
    #[must_use]
    pub const fn new() -> Self {
        let mut pathfinding_malus = PathfindingMalus::new();
        pathfinding_malus.set(PathType::FireInNeighbor, 16.0);
        pathfinding_malus.set(PathType::Fire, -1.0);

        Self {
            controls: SyncMutex::new(MobControls::new()),
            navigation: SyncMutex::new(PathNavigation::new()),
            pathfinding_malus: SyncMutex::new(pathfinding_malus),
        }
    }

    #[must_use]
    pub const fn controls(&self) -> &SyncMutex<MobControls> {
        &self.controls
    }

    #[must_use]
    pub const fn navigation(&self) -> &SyncMutex<PathNavigation> {
        &self.navigation
    }

    #[must_use]
    pub const fn pathfinding_malus(&self) -> &SyncMutex<PathfindingMalus> {
        &self.pathfinding_malus
    }
}

impl Default for MobBase {
    fn default() -> Self {
        Self::new()
    }
}

pub trait Mob: LivingEntity {
    fn mob_base(&self) -> &MobBase;

    fn mob_flags(&self) -> i8;

    fn set_mob_flags(&self, flags: i8);

    fn custom_server_ai_step(&self) {}

    fn get_pathfinding_malus(&self, path_type: PathType) -> f32 {
        self.mob_base().pathfinding_malus().lock().get(path_type)
    }

    /// Vanilla `Entity.getMaxFallDistance` baseline.
    fn max_fall_distance(&self) -> i32 {
        3
    }

    fn set_pathfinding_malus(&self, path_type: PathType, malus: f32) {
        self.mob_base()
            .pathfinding_malus()
            .lock()
            .set(path_type, malus);
    }

    fn is_no_ai(&self) -> bool {
        self.mob_flags() & MOB_FLAG_NO_AI != 0
    }

    fn set_no_ai(&self, no_ai: bool) {
        self.set_mob_flag(MOB_FLAG_NO_AI, no_ai);
    }

    fn is_left_handed(&self) -> bool {
        self.mob_flags() & MOB_FLAG_LEFT_HANDED != 0
    }

    fn set_left_handed(&self, left_handed: bool) {
        self.set_mob_flag(MOB_FLAG_LEFT_HANDED, left_handed);
    }

    fn is_aggressive(&self) -> bool {
        self.mob_flags() & MOB_FLAG_AGGRESSIVE != 0
    }

    fn set_aggressive(&self, aggressive: bool) {
        self.set_mob_flag(MOB_FLAG_AGGRESSIVE, aggressive);
    }

    fn set_mob_flag(&self, flag: i8, enabled: bool) {
        let flags = self.mob_flags();
        let next = if enabled { flags | flag } else { flags & !flag };
        self.set_mob_flags(next);
    }

    fn set_wanted_position(&self, position: DVec3, speed_modifier: f64) {
        self.mob_base()
            .controls()
            .lock()
            .move_control
            .set_wanted_position(position, speed_modifier);
    }

    fn jump_control_jump(&self) {
        self.mob_base().controls().lock().jump_control.jump();
    }

    fn mob_server_ai_step(&self) {
        self.mob_base().navigation().lock().tick();
        self.custom_server_ai_step();
        self.tick_move_control();
        self.tick_look_control();
        self.tick_jump_control();
    }

    fn tick_move_control(&self) {
        let move_control = {
            let mut controls = self.mob_base().controls().lock();
            let move_control = controls.move_control;
            if matches!(move_control.operation(), MoveControlOperation::MoveTo) {
                controls.move_control.set_wait();
            }
            move_control
        };

        match move_control.operation() {
            MoveControlOperation::Wait => {
                let input = self.travel_input();
                self.set_travel_input(LivingTravelInput::new(
                    input.sideways(),
                    input.vertical(),
                    0.0,
                ));
            }
            MoveControlOperation::MoveTo => self.tick_move_to_control(
                move_control.wanted_position(),
                move_control.speed_modifier(),
            ),
            MoveControlOperation::Strafe => {
                self.tick_strafe_control(
                    move_control.strafe_forward(),
                    move_control.strafe_right(),
                );
            }
            MoveControlOperation::Jumping => {
                self.tick_jumping_control(move_control.speed_modifier());
            }
        }
    }

    fn tick_move_to_control(&self, wanted_position: DVec3, speed_modifier: f64) {
        let position = self.position();
        let xd = wanted_position.x - position.x;
        let yd = wanted_position.y - position.y;
        let zd = wanted_position.z - position.z;
        let dd = xd * xd + yd * yd + zd * zd;
        if dd < MOVE_CONTROL_MIN_SPEED_SQR {
            let input = self.travel_input();
            self.set_travel_input(LivingTravelInput::new(
                input.sideways(),
                input.vertical(),
                0.0,
            ));
            return;
        }

        let y_rot = (zd.atan2(xd) as f32 * 180.0 / PI) - 90.0;
        let (_, pitch) = self.rotation();
        self.set_rotation((
            rotlerp(self.rotation().0, y_rot, MOVE_CONTROL_MAX_TURN),
            pitch,
        ));
        let movement_speed = self
            .attributes()
            .lock()
            .required_value(vanilla_attributes::MOVEMENT_SPEED);
        self.set_speed((speed_modifier * movement_speed) as f32);

        if should_jump_to_wanted_position(self, wanted_position, xd, yd, zd) {
            self.jump_control_jump();
            self.mob_base().controls().lock().move_control.set_jumping();
        }
    }

    fn tick_strafe_control(&self, forward: f32, right: f32) {
        let movement_speed = self
            .attributes()
            .lock()
            .required_value(vanilla_attributes::MOVEMENT_SPEED) as f32;
        let speed = movement_speed * 0.25;
        let mut strafe_forward = forward;
        let mut strafe_right = right;

        let mut distance = strafe_forward
            .mul_add(strafe_forward, strafe_right * strafe_right)
            .sqrt();
        if distance < 1.0 {
            distance = 1.0;
        }
        distance = speed / distance;
        let xa = strafe_forward * distance;
        let za = strafe_right * distance;
        let yaw_radians = self.rotation().0 * PI / 180.0;
        let sin = yaw_radians.sin();
        let cos = yaw_radians.cos();
        let dx = xa.mul_add(cos, -(za * sin));
        let dz = za.mul_add(cos, xa * sin);
        if !self.is_strafe_walkable(dx, dz) {
            strafe_forward = 1.0;
            strafe_right = 0.0;
        }

        self.set_speed(speed);
        self.set_travel_input(LivingTravelInput::new(strafe_right, 0.0, strafe_forward));
        self.mob_base().controls().lock().move_control.set_wait();
    }

    fn is_strafe_walkable(&self, dx: f32, dz: f32) -> bool {
        let Some(world) = self.level() else {
            return true;
        };
        let position = self.position();
        let pos = BlockPos::new(
            floor(position.x + f64::from(dx)),
            floor(position.y),
            floor(position.z + f64::from(dz)),
        );
        let mut context = PathfindingContext::new(world.as_ref(), self.block_position());
        WalkPathEvaluator::path_type_static(&mut context, pos) == PathType::Walkable
    }

    fn tick_jumping_control(&self, speed_modifier: f64) {
        let movement_speed = self
            .attributes()
            .lock()
            .required_value(vanilla_attributes::MOVEMENT_SPEED);
        self.set_speed((speed_modifier * movement_speed) as f32);
        if self.on_ground()
            || (self.is_in_water() || self.is_in_lava()) && self.is_affected_by_fluids()
        {
            self.mob_base().controls().lock().move_control.set_wait();
        }
    }

    fn tick_look_control(&self) {
        let look_control = {
            let mut controls = self.mob_base().controls().lock();
            let look_control = controls.look_control;
            controls.look_control.tick_cooldown();
            look_control
        };

        let mut rotation = self.rotation();
        if look_control.is_looking_at_target() {
            let position = self.position();
            let wanted_position = look_control.wanted_position();
            let xd = wanted_position.x - position.x;
            let yd = wanted_position.y - self.get_eye_y();
            let zd = wanted_position.z - position.z;
            let horizontal = xd.hypot(zd);
            if horizontal.abs() > 1.0e-5 || yd.abs() > 1.0e-5 {
                let target_pitch = (-(yd.atan2(horizontal)) as f32 * 180.0 / PI).clamp(
                    -look_control.x_max_rot_angle(),
                    look_control.x_max_rot_angle(),
                );
                rotation.1 = rotlerp(rotation.1, target_pitch, look_control.x_max_rot_angle());
            }
            if zd.abs() > 1.0e-5 || xd.abs() > 1.0e-5 {
                let target_yaw = (zd.atan2(xd) as f32 * 180.0 / PI) - 90.0;
                rotation.0 = rotlerp(rotation.0, target_yaw, look_control.y_max_rot_speed());
            }
        } else {
            rotation.1 = 0.0;
        }

        self.set_rotation(rotation);
    }

    fn tick_jump_control(&self) {
        let jumping = self.mob_base().controls().lock().jump_control.tick();
        self.set_jumping(jumping);
    }
}

pub trait PathfinderMob: Mob {
    fn get_walk_target_value(&self, _pos: BlockPos) -> f32 {
        0.0
    }

    fn is_path_finding(&self) -> bool {
        !self.mob_base().navigation().lock().is_done()
    }
}

fn should_jump_to_wanted_position<M: Mob + ?Sized>(
    mob: &M,
    wanted_position: DVec3,
    xd: f64,
    yd: f64,
    zd: f64,
) -> bool {
    let max_up_step = f64::from(mob.max_up_step());
    if yd > max_up_step && xd * xd + zd * zd < mob.bounding_box().width().max(1.0) {
        return true;
    }

    let Some(world) = mob.level() else {
        return false;
    };
    let pos = mob.block_position();
    let block_state = world.get_block_state(pos);
    let behavior = BLOCK_BEHAVIORS.get_behavior(block_state.get_block());
    let shape = behavior.get_collision_shape(
        block_state,
        world.as_ref(),
        pos,
        BlockCollisionContext::empty(),
    );
    let shape_top = position_shape_top(pos, shape.max(Axis::Y));
    let block = block_state.get_block();
    !shape.is_empty()
        && wanted_position.y > shape_top
        && mob.position().y < shape_top
        && !block.has_tag(&BlockTag::DOORS)
        && !block.has_tag(&BlockTag::FENCES)
}

fn position_shape_top(pos: BlockPos, local_y: f64) -> f64 {
    f64::from(pos.y()) + local_y
}

fn rotlerp(a: f32, b: f32, max: f32) -> f32 {
    let mut diff = wrap_degrees(b - a);
    if diff > max {
        diff = max;
    }
    if diff < -max {
        diff = -max;
    }

    let mut result = a + diff;
    if result < 0.0 {
        result += 360.0;
    } else if result > 360.0 {
        result -= 360.0;
    }
    result
}

fn wrap_degrees(mut degrees: f32) -> f32 {
    degrees %= 360.0;
    if degrees >= 180.0 {
        degrees -= 360.0;
    }
    if degrees < -180.0 {
        degrees += 360.0;
    }
    degrees
}

#[cfg(test)]
mod tests {
    use crate::entity::ai::path::PathType;
    use crate::entity::mob::MobBase;

    #[test]
    fn mob_base_uses_vanilla_fire_path_malus_overrides() {
        let base = MobBase::new();
        let malus = base.pathfinding_malus().lock();

        assert_eq!(
            malus.get(PathType::FireInNeighbor).to_bits(),
            16.0_f32.to_bits()
        );
        assert_eq!(malus.get(PathType::Fire).to_bits(), (-1.0_f32).to_bits());
        assert_eq!(malus.get(PathType::Water).to_bits(), 8.0_f32.to_bits());
    }
}
