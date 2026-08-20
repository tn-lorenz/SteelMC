use crate::entity::damage::DamageSource;
use crate::entity::entities::ItemEntity;
use crate::entity::projectile::triangle_random;
use crate::entity::{
    Entity, EntityBase, EntityBaseLoad, Projectile, ProjectileBase, RemovalReason, SharedEntity,
    ThrowableProjectile,
};
use crate::physics::MoverType;
use crate::player::Player;
use crate::world::{LevelReader, World};
use glam::DVec3;
use rand::{RngExt, rng};
use std::cmp::PartialEq;
use std::f64::consts::PI;
use std::ops::Add;
use std::sync::{Arc, Weak};
use steel_macros::entity_behavior;
use steel_registry::blocks::block_state_ext::BlockStateExt;
use steel_registry::entity_type::EntityTypeRef;
use steel_registry::fluid::FluidStateExt;
use steel_registry::particle_type::ParticleData;
use steel_registry::vanilla_entity_data::FishingBobberEntityData;
use steel_registry::vanilla_particle_types::{BUBBLE, FISHING, SPLASH};
use steel_registry::{sound_events, vanilla_blocks, vanilla_damage_types, vanilla_items};
use steel_utils::locks::SyncMutex;
use steel_utils::types::InteractionHand;
use steel_utils::{BlockPos, Downcast, DowncastType, DowncastTypeKey};

/// A fishing hook.
#[entity_behavior(class = "FishingHook")]
pub struct FishingHookEntity {
    base: EntityBase,
    entity_type: EntityTypeRef,
    entity_data: SyncMutex<FishingBobberEntityData>,
    projectile_base: ProjectileBase,
    hook_state: SyncMutex<FishingHookState>,
}

/// This struct holds entity specific state information per fishing hook entity.
pub struct FishingHookState {
    out_of_water_time: i32,
    life: i32,
    nibble: i32,
    time_until_lured: i32,
    time_until_hooked: i32,
    fish_angle: f64,
    open_water: bool,
    current_state: FishHookState,
    hooked_in: Option<SharedEntity>,
    _luck: i32,
    lure_speed: i32,
}

impl FishingHookState {
    #[must_use]
    pub fn new(lure_speed: i32, luck: i32) -> Self {
        Self {
            out_of_water_time: 0,
            life: 0,
            nibble: 0,
            time_until_lured: 0,
            time_until_hooked: 0,
            fish_angle: 0.0,
            open_water: false,
            current_state: FishHookState::Flying,
            hooked_in: None,
            _luck: luck.max(0),
            lure_speed: lure_speed.max(0),
        }
    }
}

// SAFETY: This key is owned by Steel and uniquely identifies `FishingHookEntity`.
unsafe impl DowncastType for FishingHookEntity {
    const TYPE_KEY: DowncastTypeKey = DowncastTypeKey::new("steel:entity/fishing_hook");
}

pub const MAX_OUT_OF_WATER_TIME: i32 = 10;
const MAX_DISTANCE_SQR: f64 = 32.0 * 32.0;

const DMG_DEFAULT: i32 = 5;
const DMG_ITEM_ENTITY: i32 = 3;
const DMG_ON_GROUND: i32 = 2;

const DEGREE_180: f64 = 180.0;
const DEGREE_360: f64 = 360.0;

const ONE_SECOND: i32 = 20;
const TWO_SECONDS: i32 = 40;
const THREE_SECONDS: i32 = 60;
const FOUR_SECONDS: i32 = 80;
const FIVE_SECONDS: i32 = 100;
const THIRTY_SECONDS: i32 = 600;
const ONE_MINUTE: i32 = 1200;

impl FishingHookEntity {
    /// Creates a fishing hook entity.
    #[must_use]
    pub(crate) fn new(
        entity_type: EntityTypeRef,
        id: i32,
        position: DVec3,
        world: Weak<World>,
    ) -> Self {
        Self {
            base: EntityBase::new(id, position, entity_type.dimensions, world),
            entity_type,
            entity_data: SyncMutex::new(FishingBobberEntityData::new()),
            projectile_base: ProjectileBase::new(),
            hook_state: SyncMutex::new(FishingHookState::new(0, 0)),
        }
    }

    /// Creates an fishing hook entity from saved base data.
    #[must_use]
    pub fn from_saved(entity_type: EntityTypeRef, load: EntityBaseLoad) -> Self {
        Self {
            base: EntityBase::from_load(load, entity_type.dimensions),
            entity_type,
            entity_data: SyncMutex::new(FishingBobberEntityData::new()),
            projectile_base: ProjectileBase::new(),
            hook_state: SyncMutex::new(FishingHookState::new(0, 0)),
        }
    }

    /// Sets the projectile owner and mirrors vanilla's `Player.fishing` update.
    pub(crate) fn set_owner(self: &Arc<Self>, owner: &SharedEntity) {
        self.set_owner_entity(Some(owner));
        if let Some(player) = owner.as_player() {
            player.set_fishing_hook(self);
        }
    }

    fn should_stop_fishing(&self, owner: &Player) -> bool {
        if !owner.can_interact_with_level() {
            self.set_removed(RemovalReason::Discarded);
            return true;
        }

        let inventory = owner.inventory.lock();

        let mainhand_item = inventory.get_item_in_hand(InteractionHand::MainHand);
        let offhand_item = inventory.get_offhand_item();

        let mainhand_fishing = mainhand_item.is(&vanilla_items::FISHING_ROD);
        let offhand_fishing = offhand_item.is(&vanilla_items::FISHING_ROD);

        if (mainhand_fishing || offhand_fishing)
            && self.distance_to_sqr(owner.position()) <= MAX_DISTANCE_SQR
        {
            return false;
        }

        self.set_removed(RemovalReason::Discarded);
        true
    }

    fn check_collision(&self) {
        if let Some(hit_result) = self.get_hit_result_on_move_vector() {
            self.hit_target_or_deflect_self(&hit_result);
        }
    }

    fn set_hooked_entity(&self, hooked: Option<SharedEntity>) {
        let hooked_entity_id = hooked.as_ref().map_or(0, |entity| {
            let id = entity.base().id();
            id + 1
        });

        {
            let mut hook_state = self.hook_state.lock();
            hook_state.hooked_in = hooked;
        }

        let mut entity_data = self.entity_data.lock();

        entity_data.fishing_hook.hooked_entity.set(hooked_entity_id);
    }

    #[expect(
        clippy::too_many_lines,
        clippy::similar_names,
        reason = "Logic that belongs together is being kept together + X and Z movement components intentionally have similar names"
    )]
    fn catching_fish(&self, pos: BlockPos, state: &mut FishingHookState) {
        let mut fishing_speed = 1;
        let above = pos.above();

        let Some(world) = self.level() else {
            return;
        };

        if rng().random::<f64>() < 0.25 && world.is_raining_at(above) {
            fishing_speed += 1;
        }

        if rng().random::<f64>() < 0.5 && world.can_see_sky(above) {
            fishing_speed -= 1;
        }

        if state.nibble > 0 {
            state.nibble -= 1;

            if state.nibble <= 0 {
                state.time_until_lured = 0;
                state.time_until_hooked = 0;
                self.entity_data.lock().fishing_hook_mut().biting.set(false);
            }
        } else if state.time_until_hooked > 0 {
            state.time_until_hooked -= fishing_speed;

            if state.time_until_hooked > 0 {
                state.fish_angle += triangle_random(0.0, 9.188);

                let angle = state.fish_angle * PI / DEGREE_180;
                let angle_sin = angle.sin();
                let angle_cos = angle.cos();

                let fish_x =
                    self.position().x + angle_sin * f64::from(state.time_until_hooked) * 0.1;
                let fish_y = self.position().y.floor() + 1.0;
                let fish_z =
                    self.position().z + angle_cos * f64::from(state.time_until_hooked) * 0.1;

                let Some(world) = self.level() else {
                    return;
                };

                let splash_block_state =
                    world.get_block_state(BlockPos::containing(fish_x, fish_y - 1.0, fish_z));

                if splash_block_state.get_block() == &vanilla_blocks::WATER {
                    if rng().random::<f64>() < 0.15 {
                        world.send_particles(
                            ParticleData::simple(&BUBBLE),
                            DVec3::new(fish_x, fish_y - 0.1, fish_z),
                            1,
                            DVec3::new(angle_sin, 0.1, angle_cos),
                            0.0,
                        );
                    }

                    let particle_x_mov = angle_sin * 0.04;
                    let particle_z_mov = angle_cos * 0.04;

                    // Yes, according to the src, x and z are swapped in the second `DVec3`
                    world.send_particles(
                        ParticleData::simple(&FISHING),
                        DVec3::new(fish_x, fish_y, fish_z),
                        0,
                        DVec3::new(particle_z_mov, 0.01, -particle_x_mov),
                        1.0,
                    );
                    world.send_particles(
                        ParticleData::simple(&FISHING),
                        DVec3::new(fish_x, fish_y, fish_z),
                        0,
                        DVec3::new(-particle_z_mov, 0.01, particle_x_mov),
                        1.0,
                    );
                }
            } else {
                // I check for the world here first, because we don't need to invoke `y` (the only call using `world` is the one using `y` through `particle_pos`) if it doesn't exist
                let Some(world) = self.level() else {
                    return;
                };

                self.play_sound(
                    &sound_events::ENTITY_FISHING_BOBBER_SPLASH,
                    0.25,
                    1.0 + (rng().random::<f32>() - rng().random::<f32>()) * 0.4,
                );

                let bb_width = self.bounding_box().width();
                let y = self.position().y + 0.5;
                let particle_pos = DVec3::new(self.position().x, y, self.position().z);
                let particle_count = (1.0 + bb_width * 20.0) as i32;
                let particle_spread = DVec3::new(bb_width, 0.0, bb_width);

                world.send_particles(
                    ParticleData::simple(&BUBBLE),
                    particle_pos,
                    particle_count,
                    particle_spread,
                    0.2,
                );
                world.send_particles(
                    ParticleData::simple(&FISHING),
                    particle_pos,
                    particle_count,
                    particle_spread,
                    0.2,
                );

                state.nibble = rng().random_range(ONE_SECOND..=TWO_SECONDS);
                self.entity_data.lock().fishing_hook_mut().biting.set(true);
            }
        } else if state.time_until_lured > 0 {
            state.time_until_lured -= fishing_speed;
            let mut tease_chance = 0.15;

            match state.time_until_lured {
                0..ONE_SECOND => {
                    tease_chance += f64::from(ONE_SECOND - state.time_until_lured) * 0.05;
                }
                ONE_SECOND..TWO_SECONDS => {
                    tease_chance += f64::from(TWO_SECONDS - state.time_until_lured) * 0.02;
                }
                TWO_SECONDS..THREE_SECONDS => {
                    tease_chance += f64::from(THREE_SECONDS - state.time_until_lured) * 0.01;
                }
                _ => {}
            }

            if rng().random::<f64>() < tease_chance {
                // same reason to call this early in here as well: no need to calculate the rest if there is no world to spawn the particle in.
                let Some(world) = self.level() else {
                    return;
                };

                let angle: f64 = rng().random_range(0.0..=DEGREE_360) * PI / DEGREE_180;
                let dist = rng().random_range(25.0..=60.0);

                let fish_x: f64 = self.position().x + angle.sin() * dist * 0.1;
                let fish_y = self.position().y.floor() + 1.0;
                let fish_z: f64 = self.position().z + angle.cos() * dist * 0.1;

                let splash_block_state =
                    world.get_block_state(BlockPos::containing(fish_x, fish_y - 1.0, fish_z));

                if splash_block_state.get_block() == &vanilla_blocks::WATER {
                    world.send_particles(
                        ParticleData::simple(&SPLASH),
                        DVec3::new(fish_x, fish_y, fish_z),
                        2 + rng().random_range(0..=2),
                        DVec3::new(0.1, 0.0, 0.1),
                        0.0,
                    );
                }
            }

            if state.time_until_lured <= 0 {
                state.fish_angle = rng().random_range(0.0..=DEGREE_360);
                state.time_until_hooked = rng().random_range(ONE_SECOND..=FOUR_SECONDS);
            }
        } else {
            state.time_until_lured = rng().random_range(FIVE_SECONDS..=THIRTY_SECONDS);
            state.time_until_lured -= state.lure_speed;
        }
    }

    fn calculate_open_water(&self, pos: BlockPos) -> bool {
        let mut prev_layer = OpenWaterType::Invalid;

        for y in -1..=2 {
            let offset_from = BlockPos::new(pos.x() - 2, y, pos.z() - 2);
            let offset_to = BlockPos::new(pos.x() + 2, y, pos.z() + 2);

            let layer = self.get_open_water_type_for_area(offset_from, offset_to);

            match layer {
                OpenWaterType::AboveWater => {
                    if prev_layer == OpenWaterType::Invalid {
                        return false;
                    }
                }
                OpenWaterType::InsideWater => {
                    if prev_layer == OpenWaterType::AboveWater {
                        return false;
                    }
                }

                OpenWaterType::Invalid => {
                    return false;
                }
            }
            prev_layer = layer;
        }

        true
    }

    fn get_open_water_type_for_area(&self, from: BlockPos, to: BlockPos) -> OpenWaterType {
        let mut iter =
            BlockPos::between_closed(from, to).map(|pos| self.get_open_water_type_for_block(pos));

        let Some(first) = iter.next() else {
            return OpenWaterType::Invalid;
        };

        if iter.all(|value| value == first) {
            first
        } else {
            OpenWaterType::Invalid
        }
    }

    fn get_open_water_type_for_block(&self, pos: BlockPos) -> OpenWaterType {
        let Some(world) = self.level() else {
            return OpenWaterType::Invalid;
        };

        let state = world.get_block_state(pos);

        if !state.is_air() && !(state.get_block() == &vanilla_blocks::LILY_PAD) {
            let fluid_state = state.get_fluid_state();
            // TODO: normally I'd need to check if the collision shape (`get_collision_shape()`) at the position `pos` is empty, idk how to do that from the `fluid_state` (like in vanilla) tho
            if fluid_state.is_water() && fluid_state.is_source() && fluid_state.is_empty() {
                OpenWaterType::InsideWater
            } else {
                OpenWaterType::Invalid
            }
        } else {
            OpenWaterType::AboveWater
        }
    }

    // TODO: The rod is needed for advancements and loot params.
    /// Retrieves the entity caught by this fishing hook and returns the resulting damage value.
    /// Mirrors vanilla's `FishingHook.retrieve()`.
    pub fn retrieve(&self) -> i32 {
        let mut damage = 0;

        if let Some(owner) = self.get_owner()
            && let Some(player) = owner.as_player()
            && !Self::should_stop_fishing(self, player)
        {
            let hooked_in = {
                let hook_state = self.hook_state.lock();
                hook_state.hooked_in.clone()
            };

            if let Some(hooked_in) = hooked_in {
                self.pull_entity(&hooked_in);
                // TODO: criteria triggers (advancements)
                damage = if hooked_in.as_ref().is::<ItemEntity>() {
                    DMG_ITEM_ENTITY
                } else {
                    DMG_DEFAULT
                };
            } else if self.hook_state.lock().nibble > 0 {
                // TODO: Looting
                // TODO: criteria triggers (advancements)
                // TODO: award stat when catching fish
            }

            if self.base.on_ground() {
                damage = DMG_ON_GROUND;
            }

            self.set_removed(RemovalReason::Discarded);
        } else {
            damage = 0;
        }
        damage
    }

    fn pull_entity(&self, entity: &Arc<dyn Entity>) {
        if let Some(owner) = self.get_owner() {
            let base = owner.base();
            let delta = DVec3::new(
                base.position().x - self.base.position().x,
                base.position().y - self.base.position().y,
                base.position().z - self.base.position().z,
            ) * 0.1;
            entity.set_velocity(entity.velocity().add(delta));
        }
    }

    fn clear_owner_info(&self) {
        let Some(owner) = self.get_owner() else {
            return;
        };
        let Some(player) = owner.as_player() else {
            return;
        };

        player.clear_fishing_hook(self);
    }
}

impl Entity for FishingHookEntity {
    fn base(&self) -> &EntityBase {
        &self.base
    }

    fn entity_type(&self) -> EntityTypeRef {
        self.entity_type
    }

    fn spawn_data(&self) -> i32 {
        self.get_owner().map_or(self.id(), |owner| owner.id())
    }

    fn set_removed(&self, reason: RemovalReason) {
        self.clear_owner_info();
        self.base.set_removed(reason);
    }

    #[expect(
        clippy::too_many_lines,
        reason = "Logic that belongs together is being kept together."
    )]
    fn tick(&self) {
        if let Some(owner) = self.get_owner()
            && let Some(player) = owner.as_player()
            && !self.should_stop_fishing(player)
        {
            if self.on_ground() {
                let should_remove = {
                    let mut state = self.hook_state.lock();
                    state.life += 1;
                    state.life >= ONE_MINUTE
                };

                if should_remove {
                    self.set_removed(RemovalReason::Discarded);
                }
            } else {
                self.hook_state.lock().life = 0;
            }

            let mut liquid_height: f32 = 0.0;
            let pos = BlockPos::from(self.base.position());

            if let Some(world) = self.level() {
                let block_state = world.get_block_state(pos);
                let fluid_state = block_state.get_fluid_state();

                if fluid_state.is_water() {
                    liquid_height = fluid_state.own_height();
                }

                let is_in_water = liquid_height > 0.0;

                let current_state = {
                    let state = self.hook_state.lock();
                    state.current_state
                };

                match current_state {
                    FishHookState::Flying => {
                        let should_check_collision = {
                            let mut state = self.hook_state.lock();

                            if state.hooked_in.is_some() {
                                self.base.set_velocity(DVec3::ZERO);
                                state.current_state = FishHookState::HookedInEntity;
                                return;
                            }

                            if is_in_water {
                                self.base
                                    .set_velocity(self.base.velocity() * DVec3::new(0.3, 0.2, 0.3));
                                state.current_state = FishHookState::Bobbing;
                                return;
                            }

                            true
                        };

                        if should_check_collision {
                            self.check_collision();
                        }
                    }

                    FishHookState::HookedInEntity => {
                        let hooked = {
                            let state = self.hook_state.lock();
                            state.hooked_in.clone()
                        };

                        let Some(hooked) = hooked else {
                            let mut state = self.hook_state.lock();
                            state.current_state = FishHookState::Flying;
                            return;
                        };

                        let removed = hooked.is_removed();
                        let can_interact = hooked.can_interact_with_level();

                        if !removed && can_interact {
                            let pos = hooked.position();
                            let height = hooked.bounding_box().height();

                            self.try_set_position(DVec3::new(pos.x, pos.y + height * 0.8, pos.z))
                                .expect("...");
                        } else {
                            self.set_hooked_entity(None);

                            let mut state = self.hook_state.lock();
                            state.current_state = FishHookState::Flying;
                        }

                        return;
                    }

                    FishHookState::Bobbing => {
                        let mut state = self.hook_state.lock();

                        let velocity = self.base.velocity();

                        let mut force: f64 = self.position().y + velocity.y
                            - f64::from(pos.y())
                            - f64::from(liquid_height);

                        if force.abs() < 0.01 {
                            force += force.signum() * 0.1;
                        }

                        self.base.set_velocity(DVec3::new(
                            velocity.x * 0.9,
                            velocity.y - force * rng().random::<f64>() * 0.2,
                            velocity.z * 0.9,
                        ));

                        if state.nibble <= 0 && state.time_until_hooked <= 0 {
                            state.open_water = true;
                        } else {
                            state.open_water = state.open_water
                                && state.out_of_water_time < MAX_OUT_OF_WATER_TIME
                                && self.calculate_open_water(pos);
                        }

                        if is_in_water {
                            state.out_of_water_time = (state.out_of_water_time - 1).max(0);
                            if *self.entity_data.lock().fishing_hook().biting.get() {
                                // If you don't find this random thing in the src, remove the "h", I had to correct this spelling mistake due to lint
                                // TODO: -0.1 * this.synchronizedRandom.nextFloat() * this.synchronizedRandom.nextFloat()
                                self.base.set_velocity(
                                    self.base.velocity().add(DVec3::new(0.0, -0.1, 0.0)),
                                );
                            }

                            self.catching_fish(pos, &mut state);
                        } else {
                            state.out_of_water_time =
                                (state.out_of_water_time + 1).min(MAX_OUT_OF_WATER_TIME);
                        }
                    }
                }

                let hooked_in = {
                    let state = self.hook_state.lock();
                    state.hooked_in.is_some()
                };

                if !fluid_state.is_water() && !self.base.on_ground() && !hooked_in {
                    self.base
                        .set_velocity(self.base.velocity().add(DVec3::new(0.0, -0.03, 0.0)));
                }

                self.move_entity(MoverType::SelfMovement, self.base.velocity());
                self.apply_effects_from_blocks();
                self.update_rotation();

                let should_stop = {
                    let state = self.hook_state.lock();

                    state.current_state == FishHookState::Flying
                        && (self.base.on_ground() || self.base.horizontal_collision())
                };

                if should_stop {
                    self.base.set_velocity(DVec3::ZERO);
                }

                let inertia: f64 = 0.92;
                self.base.set_velocity(self.base.velocity() * inertia);
                self.base.set_old_position_to_current();
            }
        } else {
            self.set_removed(RemovalReason::Discarded);
        }
    }
}

impl Projectile for FishingHookEntity {
    fn projectile_base(&self) -> &ProjectileBase {
        &self.projectile_base
    }

    fn can_hit_entity(&self, entity: &dyn Entity) -> bool {
        self.base_can_hit_entity(entity) || (entity.is_alive() && entity.is::<ItemEntity>())
    }

    fn on_hit_entity(&self, entity: &SharedEntity, _location: DVec3) {
        let mut damage =
            DamageSource::environment(&vanilla_damage_types::THROWN).with_direct_entity(self.id());

        if let Some(owner) = self.get_owner() {
            damage = damage.with_causing_entity(owner.id());
        }

        if let Some(world) = entity.level() {
            entity.hurt(&world, &damage, 0.0);
        }

        self.set_hooked_entity(Some(Arc::clone(entity)));
    }
}

impl ThrowableProjectile for FishingHookEntity {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FishHookState {
    Flying,
    HookedInEntity,
    Bobbing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OpenWaterType {
    AboveWater,
    InsideWater,
    Invalid,
}

#[cfg(test)]
mod tests {
    use steel_registry::item_stack::ItemStack;
    use steel_registry::vanilla_entities;
    use uuid::Uuid;

    use super::*;
    use crate::test_support::{TestPlayerBuilder, fresh_test_world};

    fn test_hook(world: &Arc<World>, id: i32) -> Arc<FishingHookEntity> {
        Arc::new(FishingHookEntity::new(
            &vanilla_entities::FISHING_BOBBER,
            id,
            DVec3::ZERO,
            Arc::downgrade(world),
        ))
    }

    #[test]
    fn spawn_data_identifies_the_owning_player() {
        let world = fresh_test_world("fishing_hook_spawn_data");
        let player =
            TestPlayerBuilder::new(Arc::clone(&world), Uuid::from_u128(1), "Fisher", 37).build();
        let owner: SharedEntity = player;
        let hook = test_hook(&world, 38);
        hook.set_owner_entity(Some(&owner));

        assert_eq!(hook.spawn_data(), owner.id());
    }

    #[test]
    fn removal_only_clears_the_matching_active_hook() {
        let world = fresh_test_world("fishing_hook_owner_lifecycle");
        let player =
            TestPlayerBuilder::new(Arc::clone(&world), Uuid::from_u128(2), "Fisher", 40).build();
        let player_owner = Arc::clone(&player);
        let owner: SharedEntity = player_owner;
        let first = test_hook(&world, 41);
        let second = test_hook(&world, 42);

        first.set_owner(&owner);
        assert!(
            player
                .fishing_hook()
                .is_some_and(|active| Arc::ptr_eq(&active, &first))
        );

        second.set_owner(&owner);
        first.set_removed(RemovalReason::Discarded);
        assert!(
            player
                .fishing_hook()
                .is_some_and(|active| Arc::ptr_eq(&active, &second))
        );

        second.set_removed(RemovalReason::Discarded);
        assert!(player.fishing_hook().is_none());
    }

    #[test]
    fn retrieving_discards_the_active_hook() {
        let world = fresh_test_world("fishing_hook_retrieve_lifecycle");
        let player =
            TestPlayerBuilder::new(Arc::clone(&world), Uuid::from_u128(3), "Fisher", 50).build();
        player
            .inventory
            .lock()
            .set_selected_item(ItemStack::new(&vanilla_items::FISHING_ROD));
        let player_owner = Arc::clone(&player);
        let owner: SharedEntity = player_owner;
        let hook = test_hook(&world, 51);
        hook.set_owner(&owner);

        assert_eq!(hook.retrieve(), 0);
        assert!(hook.is_removed());
        assert!(player.fishing_hook().is_none());
    }
}
