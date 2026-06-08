//! Vanilla-shaped mob foundations.

use std::f32::consts::PI;
use std::sync::Arc;

use glam::DVec3;
use steel_math::floor;
use steel_registry::blocks::block_state_ext::BlockStateExt as _;
use steel_registry::enchantment_effect::EnchantmentEffectComponent;
use steel_registry::vanilla_attributes;
use steel_registry::vanilla_block_tags::BlockTag;
use steel_utils::locks::SyncMutex;
use steel_utils::random::Random as _;
use steel_utils::types::InteractionHand;
use steel_utils::{BlockPos, ChunkPos, axis::Axis};

use crate::behavior::{BLOCK_BEHAVIORS, BlockCollisionContext, InteractionResult};
use crate::entity::ai::control::{MobControls, MoveControlOperation};
use crate::entity::ai::goal::GoalSelector;
use crate::entity::ai::navigation::{
    NavigationPathRequest, NavigationRecomputeRequest, NavigationTickContext, PathNavigation,
};
use crate::entity::ai::path::{Path, PathType, PathfindingContext, PathfindingMalus};
use crate::entity::ai::walk::{MobPathSettings, WalkNodeEvaluator, WalkPathEvaluator};
use crate::entity::damage::DamageSource;
use crate::entity::{Entity, LivingEntity, LivingTravelInput, RemovalReason};
use crate::inventory::equipment::EquipmentSlot;
use crate::physics::WorldCollisionProvider;
use crate::player::Player;
use crate::world::{LevelReader, World};

const MOB_FLAG_NO_AI: i8 = 1;
const MOB_FLAG_LEFT_HANDED: i8 = 2;
const MOB_FLAG_AGGRESSIVE: i8 = 4;
const MOVE_CONTROL_MIN_SPEED_SQR: f64 = 2.500_000_3e-7;
const MOVE_CONTROL_MAX_TURN: f32 = 90.0;
const DEFAULT_EQUIPMENT_DROP_CHANCE: f32 = 0.085;
const PRESERVE_ITEM_DROP_CHANCE_THRESHOLD: f32 = 1.0;
const PRESERVE_ITEM_DROP_CHANCE: f32 = 2.0;

#[derive(Debug, Clone, Copy, PartialEq)]
struct DropChances {
    by_equipment: [f32; EquipmentSlot::ALL.len()],
}

impl DropChances {
    const DEFAULT: Self = Self {
        by_equipment: [DEFAULT_EQUIPMENT_DROP_CHANCE; EquipmentSlot::ALL.len()],
    };

    #[must_use]
    fn by_equipment(self, slot: EquipmentSlot) -> f32 {
        self.by_equipment[slot.index()]
    }

    fn set_guaranteed_drop(&mut self, slot: EquipmentSlot) {
        self.by_equipment[slot.index()] = PRESERVE_ITEM_DROP_CHANCE;
    }

    #[must_use]
    fn is_preserved(self, slot: EquipmentSlot) -> bool {
        self.by_equipment(slot) > PRESERVE_ITEM_DROP_CHANCE_THRESHOLD
    }
}

#[derive(Debug)]
pub struct MobBase {
    goal_selector: SyncMutex<GoalSelector>,
    target_selector: SyncMutex<GoalSelector>,
    controls: SyncMutex<MobControls>,
    navigation: SyncMutex<PathNavigation>,
    pathfinding_malus: SyncMutex<PathfindingMalus>,
    persistence_required: SyncMutex<bool>,
    drop_chances: SyncMutex<DropChances>,
    home_restriction: SyncMutex<MobHomeRestriction>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct MobHomeRestriction {
    position: BlockPos,
    radius: i32,
}

impl MobHomeRestriction {
    const fn none() -> Self {
        Self {
            position: BlockPos::ZERO,
            radius: -1,
        }
    }
}

impl MobBase {
    #[must_use]
    pub fn new() -> Self {
        let mut pathfinding_malus = PathfindingMalus::new();
        pathfinding_malus.set(PathType::FireInNeighbor, 16.0);
        pathfinding_malus.set(PathType::Fire, -1.0);

        Self {
            goal_selector: SyncMutex::new(GoalSelector::new()),
            target_selector: SyncMutex::new(GoalSelector::new()),
            controls: SyncMutex::new(MobControls::new()),
            navigation: SyncMutex::new(PathNavigation::new()),
            pathfinding_malus: SyncMutex::new(pathfinding_malus),
            persistence_required: SyncMutex::new(false),
            drop_chances: SyncMutex::new(DropChances::DEFAULT),
            home_restriction: SyncMutex::new(MobHomeRestriction::none()),
        }
    }

    #[must_use]
    pub const fn goal_selector(&self) -> &SyncMutex<GoalSelector> {
        &self.goal_selector
    }

    #[must_use]
    pub const fn target_selector(&self) -> &SyncMutex<GoalSelector> {
        &self.target_selector
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

    #[must_use]
    pub const fn persistence_required(&self) -> &SyncMutex<bool> {
        &self.persistence_required
    }

    const fn drop_chances(&self) -> &SyncMutex<DropChances> {
        &self.drop_chances
    }

    const fn home_restriction(&self) -> &SyncMutex<MobHomeRestriction> {
        &self.home_restriction
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

    fn tick_goal_selectors(&self) {}

    /// Handles vanilla `Mob.interact`.
    fn interact_mob(
        &self,
        player: &Player,
        hand: InteractionHand,
        _location: DVec3,
    ) -> InteractionResult {
        if !Entity::is_alive(self) {
            return InteractionResult::Pass;
        }

        // TODO: Handle name tags and spawn eggs once item-on-entity behavior exists.
        self.mob_interact(player, hand)
    }

    /// Handles vanilla `Mob.mobInteract`.
    fn mob_interact(&self, _player: &Player, _hand: InteractionHand) -> InteractionResult {
        InteractionResult::Pass
    }

    /// Applies vanilla `Mob.usePlayerItem`.
    fn use_player_item(&self, player: &Player, hand: InteractionHand) {
        player.inventory.lock().shrink_item_in_hand(hand, 1);
        // TODO: Apply USE_REMAINDER components once item use-remainder support exists.
    }

    fn remove_when_far_away(&self, _dist_sqr: f64) -> bool {
        true
    }

    fn requires_custom_persistence(&self) -> bool {
        // TODO: Include leash persistence once leash runtime exists.
        self.is_passenger()
    }

    fn is_persistence_required(&self) -> bool {
        *self.mob_base().persistence_required().lock()
    }

    fn set_persistence_required(&self) {
        *self.mob_base().persistence_required().lock() = true;
    }

    /// Returns vanilla `Mob.canPickUpLoot`.
    fn can_pick_up_loot(&self) -> bool {
        false
    }

    fn equipment_drop_chance(&self, slot: EquipmentSlot) -> f32 {
        self.mob_base().drop_chances().lock().by_equipment(slot)
    }

    fn is_equipment_drop_preserved(&self, slot: EquipmentSlot) -> bool {
        self.mob_base().drop_chances().lock().is_preserved(slot)
    }

    fn set_guaranteed_drop(&self, slot: EquipmentSlot) {
        self.mob_base()
            .drop_chances()
            .lock()
            .set_guaranteed_drop(slot);
    }

    fn drop_custom_death_loot_mob(&self, _source: &DamageSource, killed_by_player: bool) {
        if self.level().is_none() {
            return;
        }

        for slot in EquipmentSlot::ALL {
            let drop_chance = self.equipment_drop_chance(slot);
            let preserve = self.is_equipment_drop_preserved(slot);
            if !can_attempt_equipment_drop(drop_chance, preserve, killed_by_player) {
                continue;
            }

            let can_drop_item = {
                let equipment = self.living_base().equipment().lock();
                let item_stack = equipment.get_ref(slot);
                !item_stack.is_empty()
                    && !item_stack
                        .has_enchantment_effect(EnchantmentEffectComponent::PreventEquipmentDrop)
            };
            if !can_drop_item {
                continue;
            }

            // TODO: Apply EquipmentDrops enchantment value effects once damage
            // sources can resolve their living attacker context.
            let random_roll = self.base().random().lock().next_f32();
            if random_roll >= drop_chance {
                continue;
            }

            let mut item_stack = {
                let mut equipment = self.living_base().equipment().lock();
                let item_stack = equipment.get_ref(slot);
                if item_stack.is_empty()
                    || item_stack
                        .has_enchantment_effect(EnchantmentEffectComponent::PreventEquipmentDrop)
                {
                    continue;
                }

                equipment.take(slot)
            };
            if !preserve && item_stack.is_damageable_item() {
                let max_damage = item_stack.get_max_damage();
                let damage = {
                    let mut random = self.base().random().lock();
                    let inner = random.next_i32_bounded((max_damage - 3).max(1));
                    max_damage - random.next_i32_bounded(1 + inner)
                };
                item_stack.set_damage_value(damage);
            }

            self.spawn_at_location(item_stack, 0.0);
        }
    }

    fn is_within_home(&self) -> bool {
        self.is_within_home_pos(self.block_position())
    }

    fn is_within_home_pos(&self, pos: BlockPos) -> bool {
        let home = *self.mob_base().home_restriction().lock();
        home.radius == -1
            || block_pos_distance_sqr(home.position, pos) < home_radius_sqr(home.radius)
    }

    fn is_within_home_vec(&self, pos: DVec3) -> bool {
        let home = *self.mob_base().home_restriction().lock();
        home.radius == -1
            || block_center_distance_sqr(home.position, pos) < home_radius_sqr(home.radius)
    }

    fn set_home_to(&self, position: BlockPos, radius: i32) {
        *self.mob_base().home_restriction().lock() = MobHomeRestriction { position, radius };
    }

    fn home_position(&self) -> BlockPos {
        self.mob_base().home_restriction().lock().position
    }

    fn home_radius(&self) -> i32 {
        self.mob_base().home_restriction().lock().radius
    }

    fn clear_home(&self) {
        self.mob_base().home_restriction().lock().radius = -1;
    }

    fn has_home(&self) -> bool {
        self.home_radius() != -1
    }

    fn check_mob_despawn(&self) {
        // TODO: Apply peaceful hostile removal once EntityType.allowedInPeaceful is in registry data.
        if self.is_persistence_required() || self.requires_custom_persistence() {
            self.set_no_action_time(0);
            return;
        }

        let Some(nearest_player_dist_sqr) = self.nearest_player_distance_sqr() else {
            return;
        };

        let mob_category = self.entity_type().mob_category;
        let despawn_distance = mob_category.despawn_distance();
        let despawn_distance_sqr = despawn_distance * despawn_distance;
        if nearest_player_dist_sqr > f64::from(despawn_distance_sqr)
            && self.remove_when_far_away(nearest_player_dist_sqr)
        {
            self.set_removed(RemovalReason::Discarded);
            return;
        }

        let no_despawn_distance = mob_category.no_despawn_distance();
        let no_despawn_distance_sqr = no_despawn_distance * no_despawn_distance;
        if self.no_action_time() > 600
            && nearest_player_dist_sqr > f64::from(no_despawn_distance_sqr)
            && self.remove_when_far_away(nearest_player_dist_sqr)
        {
            let should_discard = {
                let mut random = self.base().random().lock();
                random.next_i32_bounded(800) == 0
            };
            if should_discard {
                self.set_removed(RemovalReason::Discarded);
            }
        } else if nearest_player_dist_sqr < f64::from(no_despawn_distance_sqr) {
            self.set_no_action_time(0);
        }
    }

    fn nearest_player_distance_sqr(&self) -> Option<f64> {
        let world = self.level()?;
        world.nearest_player_distance_sqr(self.position())
    }

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

    /// Returns vanilla `Mob.getMaxHeadXRot`.
    fn max_head_x_rot(&self) -> f32 {
        40.0
    }

    /// Returns vanilla `Mob.getMaxHeadYRot`.
    fn max_head_y_rot(&self) -> f32 {
        75.0
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

    /// Mirrors vanilla `Mob.setSpeed`: update cached speed and forward AI input.
    fn set_mob_speed(&self, speed: f32) {
        self.set_speed(speed);
        let input = self.travel_input();
        self.set_travel_input(LivingTravelInput::new(
            input.sideways(),
            input.vertical(),
            speed,
        ));
    }

    fn mob_server_ai_step(&self) {
        self.increment_no_action_time();
        self.tick_goal_selectors();
        self.tick_path_navigation();
        self.custom_server_ai_step();
        self.tick_move_control();
        self.tick_look_control();
        self.tick_jump_control();
    }

    fn tick_path_navigation(&self) {
        let Some(world) = self.level() else {
            return;
        };
        let game_time = world.game_time();
        self.mob_base().navigation().lock().tick();
        tick_path_navigation_target(self, &world, game_time);
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
        self.set_mob_speed((speed_modifier * movement_speed) as f32);

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
        self.set_mob_speed((speed_modifier * movement_speed) as f32);
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

fn can_attempt_equipment_drop(drop_chance: f32, preserve: bool, killed_by_player: bool) -> bool {
    drop_chance != 0.0 && (killed_by_player || preserve)
}

fn tick_path_navigation_target<M: Mob + ?Sized>(mob: &M, world: &Arc<World>, game_time: i64) {
    let (target, speed_modifier) = {
        let mut navigation = mob.mob_base().navigation().lock();
        let Some(target) = navigation.next_move_target(NavigationTickContext {
            mob_position: mob.position(),
            mob_bounding_box_width: mob.bounding_box().width(),
            mob_speed: mob.get_speed(),
            game_time,
        }) else {
            return;
        };
        target
    };

    let target_pos = BlockPos::containing(target.x, target.y, target.z);
    let ground_y = if world.get_block_state(target_pos.below()).is_air() {
        target.y
    } else {
        WalkNodeEvaluator::floor_level(world.as_ref(), target_pos)
    };
    mob.set_wanted_position(DVec3::new(target.x, ground_y, target.z), speed_modifier);
}

pub trait PathfinderMob: Mob {
    fn get_walk_target_value(&self, _pos: BlockPos) -> f32 {
        0.0
    }

    fn can_update_path(&self) -> bool {
        self.on_ground() || self.is_in_water() || self.is_in_lava() || self.is_passenger()
    }

    fn can_path_to_targets_below_surface(&self) -> bool {
        false
    }

    fn tick_pathfinder_path_navigation(&self) {
        let Some(world) = self.level() else {
            return;
        };
        let game_time = world.game_time();
        let recompute_request = {
            let mut navigation = self.mob_base().navigation().lock();
            navigation.tick();
            navigation.take_delayed_recompute_request(game_time, self.can_update_path())
        };
        if let Some(request) = recompute_request {
            self.recompute_path(request);
        }

        tick_path_navigation_target(self, &world, game_time);
    }

    fn tick_pathfinder_goal_selectors(&self)
    where
        Self: Sized,
    {
        let id_based_tick_count = self.tick_count().wrapping_add(self.id());
        if id_based_tick_count % 2 != 0 && self.tick_count() > 1 {
            self.mob_base()
                .target_selector()
                .lock()
                .tick_running_goals(self, false);
            self.mob_base()
                .goal_selector()
                .lock()
                .tick_running_goals(self, false);
        } else {
            self.mob_base().target_selector().lock().tick(self);
            self.mob_base().goal_selector().lock().tick(self);
        }
    }

    fn is_stable_destination(&self, pos: BlockPos) -> bool {
        self.level()
            .is_some_and(|world| world.get_block_state(pos.below()).is_solid_render())
    }

    fn create_path_to(&self, target: BlockPos, reach_range: i32) -> Option<Path> {
        let world = self.level()?;
        if !world.has_full_chunk(ChunkPos::from_block_pos(target)) {
            return None;
        }

        let target = if self.can_path_to_targets_below_surface() {
            target
        } else {
            find_ground_path_target_surface(world.as_ref(), target)
        };
        let targets = [target];
        self.create_path_to_targets(&world, &targets, reach_range)
    }

    fn recompute_path(&self, request: NavigationRecomputeRequest) {
        let path = self.create_path_to(request.target_pos, request.reach_range);
        self.mob_base()
            .navigation()
            .lock()
            .complete_recompute_path(path, request.game_time);
    }

    fn move_to_pos(&self, target: DVec3, speed_modifier: f64) -> bool {
        self.move_to_pos_with_reach(target, 1, speed_modifier)
    }

    fn move_to_pos_with_reach(&self, target: DVec3, reach_range: i32, speed_modifier: f64) -> bool {
        let path = self.create_path_to(
            BlockPos::containing(target.x, target.y, target.z),
            reach_range,
        );
        self.move_to_path(path, speed_modifier)
    }

    fn move_to_path(&self, path: Option<Path>, speed_modifier: f64) -> bool {
        let mut navigation = self.mob_base().navigation().lock();
        let Some(path) = path else {
            navigation.stop();
            return false;
        };

        navigation.move_to(path, speed_modifier, self.position())
    }

    fn is_path_finding(&self) -> bool {
        !self.mob_base().navigation().lock().is_done()
    }

    fn is_panicking(&self) -> bool {
        self.mob_base()
            .goal_selector()
            .lock()
            .has_running_panic_goal()
    }

    fn create_path_to_targets(
        &self,
        world: &Arc<World>,
        targets: &[BlockPos],
        reach_range: i32,
    ) -> Option<Path> {
        if targets.is_empty()
            || self.position().y < f64::from(world.min_y())
            || !self.can_update_path()
        {
            return None;
        }

        let follow_range = self
            .attributes()
            .lock()
            .required_value(vanilla_attributes::FOLLOW_RANGE);
        let max_path_length = {
            let mut navigation = self.mob_base().navigation().lock();
            navigation.update_pathfinder_max_visited_nodes(follow_range);
            navigation.max_path_length(follow_range)
        };

        let mob_position = self.block_position();
        let settings = MobPathSettings::from_mob(self);
        let mut evaluator = WalkNodeEvaluator::new(settings);
        let collision_world =
            WorldCollisionProvider::for_path_navigation(world, self.as_entity_event_source());
        let mut collision = |aabb| {
            collision_world.has_entity_context_collision(
                aabb,
                self.position().y,
                self.is_descending(),
            )
        };

        self.mob_base().navigation().lock().create_path(
            &mut evaluator,
            world.as_ref(),
            &mut collision,
            NavigationPathRequest {
                mob_position,
                targets,
                max_path_length,
                reach_range,
            },
        )
    }
}

fn find_ground_path_target_surface(level: &dyn LevelReader, mut pos: BlockPos) -> BlockPos {
    if level.get_block_state(pos).is_air() {
        let mut column_pos = pos.below();
        while column_pos.y() >= level.min_y() && level.get_block_state(column_pos).is_air() {
            column_pos = column_pos.below();
        }
        if column_pos.y() >= level.min_y() {
            return column_pos.above();
        }

        column_pos = pos.at_y(pos.y() + 1);
        while column_pos.y() < level.max_y_exclusive() && level.get_block_state(column_pos).is_air()
        {
            column_pos = column_pos.above();
        }
        pos = column_pos;
    }

    if !level.get_block_state(pos).is_solid() {
        return pos;
    }

    let mut column_pos = pos.above();
    while column_pos.y() < level.max_y_exclusive() && level.get_block_state(column_pos).is_solid() {
        column_pos = column_pos.above();
    }
    column_pos
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

fn block_pos_distance_sqr(a: BlockPos, b: BlockPos) -> f64 {
    let dx = f64::from(a.x() - b.x());
    let dy = f64::from(a.y() - b.y());
    let dz = f64::from(a.z() - b.z());
    dx.mul_add(dx, dy.mul_add(dy, dz * dz))
}

fn block_center_distance_sqr(pos: BlockPos, target: DVec3) -> f64 {
    let (x, y, z) = pos.get_center();
    DVec3::new(x, y, z).distance_squared(target)
}

fn home_radius_sqr(radius: i32) -> f64 {
    let radius = f64::from(radius);
    radius * radius
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
    use std::sync::Weak;

    use glam::DVec3;
    use steel_registry::entity_type::EntityTypeRef;
    use steel_registry::vanilla_entities;
    use steel_registry::{REGISTRY, test_support::init_test_registry, vanilla_blocks};
    use steel_utils::locks::SyncMutex;
    use steel_utils::{BlockPos, BlockStateId};

    use super::{can_attempt_equipment_drop, find_ground_path_target_surface};
    use crate::entity::ai::path::PathType;
    use crate::entity::mob::{Mob, MobBase};
    use crate::entity::{Entity, EntityBase, LivingEntity, LivingEntityBase};
    use crate::world::LevelReader;

    #[test]
    fn equipment_drop_attempt_gate_matches_vanilla_conditions() {
        assert!(!can_attempt_equipment_drop(0.0, true, true));
        assert!(!can_attempt_equipment_drop(0.085, false, false));
        assert!(can_attempt_equipment_drop(0.085, false, true));
        assert!(can_attempt_equipment_drop(2.0, true, false));
    }

    struct SurfaceLevel {
        default_state: BlockStateId,
        states: Vec<(BlockPos, BlockStateId)>,
    }

    impl SurfaceLevel {
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

    impl LevelReader for SurfaceLevel {
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

    struct DespawnTestMob {
        base: EntityBase,
        living_base: LivingEntityBase,
        mob_base: MobBase,
        flags: SyncMutex<i8>,
        health: SyncMutex<f32>,
        nearest_player_distance_sqr: Option<f64>,
        remove_when_far_away: bool,
    }

    impl DespawnTestMob {
        fn new(nearest_player_distance_sqr: Option<f64>, remove_when_far_away: bool) -> Self {
            init_test_registry();

            Self {
                base: EntityBase::new(
                    1,
                    DVec3::ZERO,
                    vanilla_entities::PIG.dimensions,
                    Weak::new(),
                ),
                living_base: LivingEntityBase::new(&vanilla_entities::PIG),
                mob_base: MobBase::new(),
                flags: SyncMutex::new(0),
                health: SyncMutex::new(10.0),
                nearest_player_distance_sqr,
                remove_when_far_away,
            }
        }
    }

    impl Entity for DespawnTestMob {
        fn base(&self) -> &EntityBase {
            &self.base
        }

        fn entity_type(&self) -> EntityTypeRef {
            &vanilla_entities::PIG
        }
    }

    impl LivingEntity for DespawnTestMob {
        fn living_base(&self) -> &LivingEntityBase {
            &self.living_base
        }

        fn get_health(&self) -> f32 {
            *self.health.lock()
        }

        fn set_health(&self, health: f32) {
            *self.health.lock() = health;
        }
    }

    impl Mob for DespawnTestMob {
        fn mob_base(&self) -> &MobBase {
            &self.mob_base
        }

        fn mob_flags(&self) -> i8 {
            *self.flags.lock()
        }

        fn set_mob_flags(&self, flags: i8) {
            *self.flags.lock() = flags;
        }

        fn remove_when_far_away(&self, _dist_sqr: f64) -> bool {
            self.remove_when_far_away
        }

        fn nearest_player_distance_sqr(&self) -> Option<f64> {
            self.nearest_player_distance_sqr
        }
    }

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

    #[test]
    fn mob_server_ai_step_increments_no_action_time() {
        let mob = DespawnTestMob::new(None, false);

        mob.set_no_action_time(12);
        mob.mob_server_ai_step();

        assert_eq!(mob.no_action_time(), 13);
    }

    #[test]
    fn mob_despawn_resets_no_action_time_near_player() {
        let mob = DespawnTestMob::new(Some(31.0 * 31.0), false);

        mob.set_no_action_time(42);
        mob.check_mob_despawn();

        assert_eq!(mob.no_action_time(), 0);
        assert!(!mob.is_removed());
    }

    #[test]
    fn mob_despawn_discards_far_removable_mob() {
        let mob = DespawnTestMob::new(Some(129.0 * 129.0), true);

        mob.check_mob_despawn();

        assert!(mob.is_removed());
    }

    #[test]
    fn mob_persistence_resets_no_action_time_and_blocks_removal() {
        let mob = DespawnTestMob::new(Some(129.0 * 129.0), true);

        mob.set_no_action_time(42);
        mob.set_persistence_required();
        mob.check_mob_despawn();

        assert_eq!(mob.no_action_time(), 0);
        assert!(!mob.is_removed());
    }

    #[test]
    fn mob_home_restriction_uses_vanilla_radius() {
        let mob = DespawnTestMob::new(None, false);

        assert!(mob.is_within_home_pos(BlockPos::new(1000, 64, 1000)));

        mob.set_home_to(BlockPos::ZERO, 4);
        assert!(mob.has_home());
        assert!(mob.is_within_home_pos(BlockPos::new(3, 0, 0)));
        assert!(!mob.is_within_home_pos(BlockPos::new(4, 0, 0)));

        mob.clear_home();
        assert!(!mob.has_home());
        assert!(mob.is_within_home_pos(BlockPos::new(1000, 64, 1000)));
    }

    #[test]
    fn ground_path_target_air_rewrites_to_surface_above_ground() {
        init_test_registry();

        let air = REGISTRY.blocks.get_default_state_id(&vanilla_blocks::AIR);
        let stone = REGISTRY.blocks.get_default_state_id(&vanilla_blocks::STONE);
        let level = SurfaceLevel::new(air).with(BlockPos::new(4, 63, 4), stone);

        assert_eq!(
            find_ground_path_target_surface(&level, BlockPos::new(4, 70, 4)),
            BlockPos::new(4, 64, 4)
        );
    }

    #[test]
    fn ground_path_target_solid_rewrites_to_first_open_block_above() {
        init_test_registry();

        let air = REGISTRY.blocks.get_default_state_id(&vanilla_blocks::AIR);
        let stone = REGISTRY.blocks.get_default_state_id(&vanilla_blocks::STONE);
        let level = SurfaceLevel::new(air)
            .with(BlockPos::new(4, 64, 4), stone)
            .with(BlockPos::new(4, 65, 4), stone);

        assert_eq!(
            find_ground_path_target_surface(&level, BlockPos::new(4, 64, 4)),
            BlockPos::new(4, 66, 4)
        );
    }
}
