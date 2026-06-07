//! Vanilla-shaped goal selector and movement goals.

use std::fmt;
use std::ops::BitOr;

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
const RANDOM_STROLL_DEFAULT_INTERVAL: i32 = 120;
const WATER_AVOIDING_RANDOM_STROLL_PROBABILITY: f32 = 0.001;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GoalControl {
    Move,
    Look,
    Jump,
    Target,
}

impl GoalControl {
    const ALL: [Self; 4] = [Self::Move, Self::Look, Self::Jump, Self::Target];
}

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct GoalControls(u8);

impl GoalControls {
    pub const EMPTY: Self = Self(0);
    pub const MOVE: Self = Self(1 << 0);
    pub const LOOK: Self = Self(1 << 1);
    pub const JUMP: Self = Self(1 << 2);
    pub const TARGET: Self = Self(1 << 3);

    #[must_use]
    pub const fn from_control(control: GoalControl) -> Self {
        match control {
            GoalControl::Move => Self::MOVE,
            GoalControl::Look => Self::LOOK,
            GoalControl::Jump => Self::JUMP,
            GoalControl::Target => Self::TARGET,
        }
    }

    #[must_use]
    pub const fn contains(self, control: GoalControl) -> bool {
        self.0 & Self::from_control(control).0 != 0
    }

    #[must_use]
    pub const fn intersects(self, other: Self) -> bool {
        self.0 & other.0 != 0
    }

    pub const fn insert(&mut self, control: GoalControl) {
        self.0 |= Self::from_control(control).0;
    }

    pub const fn remove(&mut self, control: GoalControl) {
        self.0 &= !Self::from_control(control).0;
    }

    pub fn iter(self) -> impl Iterator<Item = GoalControl> {
        GoalControl::ALL
            .into_iter()
            .filter(move |control| self.contains(*control))
    }
}

impl fmt::Debug for GoalControls {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_set().entries(self.iter()).finish()
    }
}

impl BitOr for GoalControls {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

pub trait Goal: Send {
    fn controls(&self) -> GoalControls;

    fn can_use(&mut self, mob: &dyn PathfinderMob) -> bool;

    fn can_continue_to_use(&mut self, mob: &dyn PathfinderMob) -> bool {
        self.can_use(mob)
    }

    fn is_interruptable(&self) -> bool {
        true
    }

    fn start(&mut self, _mob: &dyn PathfinderMob) {}

    fn stop(&mut self, _mob: &dyn PathfinderMob) {}

    fn requires_update_every_tick(&self) -> bool {
        false
    }

    fn tick(&mut self, _mob: &dyn PathfinderMob) {}
}

struct WrappedGoal {
    priority: i32,
    goal: Box<dyn Goal>,
    running: bool,
}

impl WrappedGoal {
    fn new(priority: i32, goal: Box<dyn Goal>) -> Self {
        Self {
            priority,
            goal,
            running: false,
        }
    }

    const fn is_running(&self) -> bool {
        self.running
    }

    fn controls(&self) -> GoalControls {
        self.goal.controls()
    }

    fn can_be_replaced_by(&self, candidate_priority: i32) -> bool {
        self.goal.is_interruptable() && candidate_priority < self.priority
    }

    fn can_use(&mut self, mob: &dyn PathfinderMob) -> bool {
        self.goal.can_use(mob)
    }

    fn can_continue_to_use(&mut self, mob: &dyn PathfinderMob) -> bool {
        self.goal.can_continue_to_use(mob)
    }

    fn start(&mut self, mob: &dyn PathfinderMob) {
        if self.running {
            return;
        }
        self.running = true;
        self.goal.start(mob);
    }

    fn stop(&mut self, mob: &dyn PathfinderMob) {
        if !self.running {
            return;
        }
        self.running = false;
        self.goal.stop(mob);
    }

    fn tick(&mut self, mob: &dyn PathfinderMob) {
        self.goal.tick(mob);
    }

    fn requires_update_every_tick(&self) -> bool {
        self.goal.requires_update_every_tick()
    }
}

pub struct GoalSelector {
    available_goals: Vec<WrappedGoal>,
    disabled_controls: GoalControls,
}

impl GoalSelector {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            available_goals: Vec::new(),
            disabled_controls: GoalControls::EMPTY,
        }
    }

    pub fn add_goal<G>(&mut self, priority: i32, goal: G)
    where
        G: Goal + 'static,
    {
        self.available_goals
            .push(WrappedGoal::new(priority, Box::new(goal)));
    }

    pub fn tick(&mut self, mob: &dyn PathfinderMob) {
        for index in 0..self.available_goals.len() {
            let should_stop = {
                let disabled_controls = self.disabled_controls;
                let goal = &mut self.available_goals[index];
                goal.is_running()
                    && (goal.controls().intersects(disabled_controls)
                        || !goal.can_continue_to_use(mob))
            };
            if should_stop {
                self.available_goals[index].stop(mob);
            }
        }

        for index in 0..self.available_goals.len() {
            if !self.can_start_goal(index) {
                continue;
            }
            if !self.available_goals[index].can_use(mob) {
                continue;
            }

            let controls = self.available_goals[index].controls();
            for control in controls.iter() {
                if let Some(current_index) = self.running_goal_index_for(control) {
                    self.available_goals[current_index].stop(mob);
                }
            }
            self.available_goals[index].start(mob);
        }

        self.tick_running_goals(mob, true);
    }

    pub fn tick_running_goals(
        &mut self,
        mob: &dyn PathfinderMob,
        force_tick_all_running_goals: bool,
    ) {
        for goal in &mut self.available_goals {
            if goal.is_running()
                && (force_tick_all_running_goals || goal.requires_update_every_tick())
            {
                goal.tick(mob);
            }
        }
    }

    pub const fn disable_control(&mut self, control: GoalControl) {
        self.disabled_controls.insert(control);
    }

    pub const fn enable_control(&mut self, control: GoalControl) {
        self.disabled_controls.remove(control);
    }

    pub const fn set_control(&mut self, control: GoalControl, enabled: bool) {
        if enabled {
            self.enable_control(control);
        } else {
            self.disable_control(control);
        }
    }

    #[must_use]
    pub fn running_goal_count(&self) -> usize {
        self.available_goals
            .iter()
            .filter(|goal| goal.is_running())
            .count()
    }

    #[must_use]
    pub const fn available_goal_count(&self) -> usize {
        self.available_goals.len()
    }

    fn can_start_goal(&self, index: usize) -> bool {
        let goal = &self.available_goals[index];
        !goal.is_running()
            && !goal.controls().intersects(self.disabled_controls)
            && self.goal_can_be_replaced_for_all_controls(index)
    }

    fn goal_can_be_replaced_for_all_controls(&self, candidate_index: usize) -> bool {
        let candidate = &self.available_goals[candidate_index];
        for control in candidate.controls().iter() {
            if let Some(current_index) = self.running_goal_index_for(control)
                && !self.available_goals[current_index].can_be_replaced_by(candidate.priority)
            {
                return false;
            }
        }
        true
    }

    fn running_goal_index_for(&self, control: GoalControl) -> Option<usize> {
        self.available_goals
            .iter()
            .position(|goal| goal.is_running() && goal.controls().contains(control))
    }

    #[cfg(test)]
    fn is_priority_running(&self, priority: i32) -> bool {
        self.available_goals
            .iter()
            .any(|goal| goal.priority == priority && goal.is_running())
    }
}

impl Default for GoalSelector {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for GoalSelector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GoalSelector")
            .field("available_goals", &self.available_goals.len())
            .field("running_goals", &self.running_goal_count())
            .field("disabled_controls", &self.disabled_controls)
            .finish()
    }
}

pub struct RandomStrollGoal {
    wanted_position: Option<DVec3>,
    speed_modifier: f64,
    interval: i32,
    force_trigger: bool,
    check_no_action_time: bool,
}

impl RandomStrollGoal {
    #[must_use]
    pub const fn new(speed_modifier: f64) -> Self {
        Self::with_interval(speed_modifier, RANDOM_STROLL_DEFAULT_INTERVAL)
    }

    #[must_use]
    pub const fn with_interval(speed_modifier: f64, interval: i32) -> Self {
        Self::with_interval_and_no_action_time_check(speed_modifier, interval, true)
    }

    #[must_use]
    pub const fn with_interval_and_no_action_time_check(
        speed_modifier: f64,
        interval: i32,
        check_no_action_time: bool,
    ) -> Self {
        Self {
            wanted_position: None,
            speed_modifier,
            interval,
            force_trigger: false,
            check_no_action_time,
        }
    }

    pub const fn trigger(&mut self) {
        self.force_trigger = true;
    }

    pub const fn set_interval(&mut self, interval: i32) {
        self.interval = interval;
    }

    fn can_use_with_position(
        &mut self,
        mob: &dyn PathfinderMob,
        mut get_position: impl FnMut(&dyn PathfinderMob) -> Option<DVec3>,
    ) -> bool {
        if mob.has_controlling_passenger() {
            return false;
        }

        if !self.force_trigger {
            if self.check_no_action_time && mob.no_action_time() >= 100 {
                return false;
            }

            let should_skip = {
                let mut random = mob.base().random().lock();
                random.next_i32_bounded(reduced_tick_delay(self.interval)) != 0
            };
            if should_skip {
                return false;
            }
        }

        let Some(position) = get_position(mob) else {
            return false;
        };

        self.wanted_position = Some(position);
        self.force_trigger = false;
        true
    }
}

impl Goal for RandomStrollGoal {
    fn controls(&self) -> GoalControls {
        GoalControls::MOVE
    }

    fn can_use(&mut self, mob: &dyn PathfinderMob) -> bool {
        self.can_use_with_position(mob, |mob| default_random_pos(mob, 10, 7))
    }

    fn can_continue_to_use(&mut self, mob: &dyn PathfinderMob) -> bool {
        !mob.mob_base().navigation().lock().is_done() && !mob.has_controlling_passenger()
    }

    fn start(&mut self, mob: &dyn PathfinderMob) {
        if let Some(wanted_position) = self.wanted_position {
            mob.move_to_pos(wanted_position, self.speed_modifier);
        }
    }

    fn stop(&mut self, mob: &dyn PathfinderMob) {
        mob.mob_base().navigation().lock().stop();
    }
}

pub struct WaterAvoidingRandomStrollGoal {
    stroll: RandomStrollGoal,
    probability: f32,
}

impl WaterAvoidingRandomStrollGoal {
    #[must_use]
    pub const fn new(speed_modifier: f64) -> Self {
        Self::with_probability(speed_modifier, WATER_AVOIDING_RANDOM_STROLL_PROBABILITY)
    }

    #[must_use]
    pub const fn with_probability(speed_modifier: f64, probability: f32) -> Self {
        Self {
            stroll: RandomStrollGoal::new(speed_modifier),
            probability,
        }
    }
}

impl Goal for WaterAvoidingRandomStrollGoal {
    fn controls(&self) -> GoalControls {
        self.stroll.controls()
    }

    fn can_use(&mut self, mob: &dyn PathfinderMob) -> bool {
        let probability = self.probability;
        self.stroll.can_use_with_position(mob, |mob| {
            water_avoiding_random_stroll_pos(mob, probability)
        })
    }

    fn can_continue_to_use(&mut self, mob: &dyn PathfinderMob) -> bool {
        self.stroll.can_continue_to_use(mob)
    }

    fn start(&mut self, mob: &dyn PathfinderMob) {
        self.stroll.start(mob);
    }

    fn stop(&mut self, mob: &dyn PathfinderMob) {
        self.stroll.stop(mob);
    }
}

const fn reduced_tick_delay(ticks: i32) -> i32 {
    (ticks + 1) / 2
}

fn water_avoiding_random_stroll_pos(mob: &dyn PathfinderMob, probability: f32) -> Option<DVec3> {
    if mob.is_in_water() {
        return land_random_pos(mob, 15, 7).or_else(|| default_random_pos(mob, 10, 7));
    }

    let use_land_random_pos = {
        let mut random = mob.base().random().lock();
        random.next_f32() >= probability
    };
    if use_land_random_pos {
        land_random_pos(mob, 10, 7)
    } else {
        default_random_pos(mob, 10, 7)
    }
}

fn default_random_pos(
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

fn land_random_pos(
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

#[cfg(test)]
mod tests {
    use std::sync::Weak;
    use std::sync::atomic::{AtomicUsize, Ordering};

    use steel_registry::entity_type::EntityTypeRef;
    use steel_registry::{test_support::init_test_registry, vanilla_entities};
    use steel_utils::locks::SyncMutex;

    use super::*;
    use crate::entity::{Entity, EntityBase, LivingEntity, LivingEntityBase, Mob, MobBase};

    struct TestPathfinderMob {
        base: EntityBase,
        living_base: LivingEntityBase,
        mob_base: MobBase,
        mob_flags: SyncMutex<i8>,
        health: SyncMutex<f32>,
    }

    impl TestPathfinderMob {
        fn new() -> Self {
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
                mob_flags: SyncMutex::new(0),
                health: SyncMutex::new(10.0),
            }
        }
    }

    impl Entity for TestPathfinderMob {
        fn base(&self) -> &EntityBase {
            &self.base
        }

        fn entity_type(&self) -> EntityTypeRef {
            &vanilla_entities::PIG
        }
    }

    impl LivingEntity for TestPathfinderMob {
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

    impl Mob for TestPathfinderMob {
        fn mob_base(&self) -> &MobBase {
            &self.mob_base
        }

        fn mob_flags(&self) -> i8 {
            *self.mob_flags.lock()
        }

        fn set_mob_flags(&self, flags: i8) {
            *self.mob_flags.lock() = flags;
        }
    }

    impl PathfinderMob for TestPathfinderMob {}

    struct StaticGoal {
        controls: GoalControls,
        can_use: bool,
        can_continue: bool,
        interruptable: bool,
        requires_update_every_tick: bool,
        tick_count: Option<&'static AtomicUsize>,
        can_use_once: bool,
    }

    impl StaticGoal {
        const fn new(controls: GoalControls) -> Self {
            Self {
                controls,
                can_use: true,
                can_continue: true,
                interruptable: true,
                requires_update_every_tick: false,
                tick_count: None,
                can_use_once: false,
            }
        }

        const fn non_interruptable(mut self) -> Self {
            self.interruptable = false;
            self
        }

        const fn with_can_continue(mut self, can_continue: bool) -> Self {
            self.can_continue = can_continue;
            self
        }

        const fn with_can_use_once(mut self) -> Self {
            self.can_use_once = true;
            self
        }

        const fn with_update_every_tick(mut self) -> Self {
            self.requires_update_every_tick = true;
            self
        }

        const fn with_tick_counter(mut self, tick_count: &'static AtomicUsize) -> Self {
            self.tick_count = Some(tick_count);
            self
        }
    }

    impl Goal for StaticGoal {
        fn controls(&self) -> GoalControls {
            self.controls
        }

        fn can_use(&mut self, _mob: &dyn PathfinderMob) -> bool {
            if self.can_use_once {
                if !self.can_use {
                    return false;
                }
                self.can_use = false;
                return true;
            }
            self.can_use
        }

        fn can_continue_to_use(&mut self, _mob: &dyn PathfinderMob) -> bool {
            self.can_continue
        }

        fn is_interruptable(&self) -> bool {
            self.interruptable
        }

        fn requires_update_every_tick(&self) -> bool {
            self.requires_update_every_tick
        }

        fn tick(&mut self, _mob: &dyn PathfinderMob) {
            if let Some(tick_count) = self.tick_count {
                tick_count.fetch_add(1, Ordering::Relaxed);
            }
        }
    }

    static RUNNING_TICK_COUNT: AtomicUsize = AtomicUsize::new(0);

    #[test]
    fn lower_priority_goal_replaces_running_goal_for_same_control() {
        let mob = TestPathfinderMob::new();
        let mut selector = GoalSelector::new();
        selector.add_goal(5, StaticGoal::new(GoalControls::MOVE));
        selector.tick(&mob);

        selector.add_goal(3, StaticGoal::new(GoalControls::MOVE));
        selector.tick(&mob);

        assert_eq!(selector.running_goal_count(), 1);
        assert!(selector.is_priority_running(3));
    }

    #[test]
    fn non_interruptable_goal_blocks_replacement() {
        let mob = TestPathfinderMob::new();
        let mut selector = GoalSelector::new();
        selector.add_goal(5, StaticGoal::new(GoalControls::MOVE).non_interruptable());
        selector.tick(&mob);

        selector.add_goal(3, StaticGoal::new(GoalControls::MOVE));
        selector.tick(&mob);

        assert_eq!(selector.running_goal_count(), 1);
        assert!(selector.is_priority_running(5));
    }

    #[test]
    fn disabled_control_stops_running_goal() {
        let mob = TestPathfinderMob::new();
        let mut selector = GoalSelector::new();
        selector.add_goal(5, StaticGoal::new(GoalControls::MOVE));
        selector.tick(&mob);

        selector.disable_control(GoalControl::Move);
        selector.tick(&mob);

        assert_eq!(selector.running_goal_count(), 0);
    }

    #[test]
    fn tick_running_goals_respects_requires_update_every_tick() {
        RUNNING_TICK_COUNT.store(0, Ordering::Relaxed);
        let mob = TestPathfinderMob::new();
        let mut selector = GoalSelector::new();
        selector.add_goal(
            5,
            StaticGoal::new(GoalControls::MOVE)
                .with_update_every_tick()
                .with_tick_counter(&RUNNING_TICK_COUNT),
        );
        selector.tick(&mob);

        selector.tick_running_goals(&mob, false);

        assert_eq!(RUNNING_TICK_COUNT.load(Ordering::Relaxed), 2);
    }

    #[test]
    fn cleanup_stops_goal_that_can_no_longer_continue() {
        let mob = TestPathfinderMob::new();
        let mut selector = GoalSelector::new();
        selector.add_goal(
            5,
            StaticGoal::new(GoalControls::MOVE)
                .with_can_continue(false)
                .with_can_use_once(),
        );

        selector.tick(&mob);
        selector.tick(&mob);

        assert_eq!(selector.running_goal_count(), 0);
    }
}
