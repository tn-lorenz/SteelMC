use crate::entity::entities::ItemEntity;
use crate::entity::projectile::triangle_random;
use crate::entity::{
    Entity, EntityBase, Projectile, ProjectileBase, RemovalReason, SharedEntity,
    ThrowableProjectile,
};
use crate::physics::MoverType;
use crate::player::Player;
use crate::world::{LevelReader, World};
use glam::DVec3;
use rand::{RngExt, rng};
use std::cmp::PartialEq;
use std::ops::Add;
use std::sync::{Arc, Weak};
use steel_macros::entity_behavior;
use steel_registry::blocks::block_state_ext::BlockStateExt;
use steel_registry::entity_type::EntityTypeRef;
use steel_registry::fluid::FluidStateExt;
use steel_registry::particle_type::ParticleData;
use steel_registry::vanilla_entity_data::FishingBobberEntityData;
use steel_registry::vanilla_particle_types::{BUBBLE, FISHING, SPLASH};
use steel_registry::{sound_events, vanilla_blocks, vanilla_items};
use steel_utils::locks::SyncMutex;
use steel_utils::types::InteractionHand;
use steel_utils::{BlockPos, Downcast, DowncastType, DowncastTypeKey};

#[entity_behavior]
pub struct FishingHook {
    base: EntityBase,
    entity_type: EntityTypeRef,
    entity_data: SyncMutex<FishingBobberEntityData>,
    projectile_base: ProjectileBase,
    hook_state: SyncMutex<FishingHookState>,
}

pub(crate) struct FishingHookState {
    out_of_water_time: i32,
    life: i32,
    nibble: i32,
    time_until_lured: i32,
    time_until_hooked: i32,
    fish_angle: f64,
    open_water: bool,
    current_state: FishHookState,
    hooked_in: Option<SharedEntity>,
    luck: i32,
    lure_speed: i32,
}

impl FishingHookState {
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
            luck: luck.max(0),
            lure_speed: lure_speed.max(0),
        }
    }
}

// SAFETY: This key is owned by Steel and uniquely identifies `FishingHook`.
unsafe impl DowncastType for FishingHook {
    const TYPE_KEY: DowncastTypeKey = DowncastTypeKey::new("steel:entity/fishing_hook");
}

impl FishingHook {
    pub const MAX_OUT_OF_WATER_TIME: i32 = 10;
    pub(crate) fn new(
        entity_type: EntityTypeRef,
        id: i32,
        position: DVec3,
        world: Weak<World>,
        hook_state: SyncMutex<FishingHookState>,
    ) -> Self {
        Self {
            base: EntityBase::new(id, position, entity_type.dimensions, world),
            entity_type,
            entity_data: SyncMutex::new(FishingBobberEntityData::new()),
            projectile_base: ProjectileBase::new(),
            hook_state: hook_state,
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

        if (mainhand_fishing || offhand_fishing) && self.distance_to_sqr(owner.position()) <= 1024.0
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
        let mut hook_state = self.hook_state.lock();
        hook_state.hooked_in = hooked;

        if let Some(hooked_entity) = hook_state.hooked_in.as_ref() {
            self.entity_data
                .lock()
                .fishing_hook
                .hooked_entity
                .set(hooked_entity.base().id() + 1);
        } else {
            self.entity_data.lock().fishing_hook.hooked_entity.set(0);
        }
    }

    fn catching_fish(&self, pos: BlockPos) {
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

        let mut state = self.hook_state.lock();

        if state.nibble > 0 {
            state.nibble -= 1;

            if state.nibble <= 0 {
                state.time_until_lured = 0;
                state.time_until_hooked = 0;
                // TODO: `fishing_hook_mut()` or `fishing_hook` ?
                self.entity_data.lock().fishing_hook_mut().biting.set(false);
            }
        } else if state.time_until_hooked > 0 {
            state.time_until_hooked -= fishing_speed;

            if state.time_until_hooked > 0 {
                state.fish_angle = state.fish_angle + triangle_random(0.0, 9.188);

                let angle = state.fish_angle * std::f64::consts::PI / 180.0;
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

                state.nibble = rng().random_range(20..=40);
                // TODO: `fishing_hook_mut()` or `fishing_hook` ?
                self.entity_data.lock().fishing_hook_mut().biting.set(true);
            }
        } else if state.time_until_lured > 0 {
            state.time_until_lured -= fishing_speed;
            let mut tease_chance = 0.15;

            match state.time_until_lured {
                0..20 => {
                    tease_chance += f64::from(20 - state.time_until_lured) * 0.05;
                }
                20..40 => {
                    tease_chance += f64::from(40 - state.time_until_lured) * 0.02;
                }
                40..60 => {
                    tease_chance += f64::from(60 - state.time_until_lured) * 0.01;
                }
                _ => {}
            }

            if rng().random::<f64>() < tease_chance {
                // same reason to call this early in here as well: no need to calculate the rest if there is no world to spawn the particle in.
                let Some(world) = self.level() else {
                    return;
                };

                let angle: f64 = rng().random_range(0.0..=360.0) * std::f64::consts::PI / 180.0;
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
                state.fish_angle = rng().random_range(0.0..=360.0);
                state.time_until_hooked = rng().random_range(20..=80);
            }
        } else {
            state.time_until_lured = rng().random_range(100..=600);
            state.time_until_lured = state.time_until_lured - state.lure_speed;
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
            return OpenWaterType::Invalid; // This is a bit weird ... am I doing this right?
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
    // fn is_open_water_fishing(){}

    // TODO: The rod is needed for advancements and loot params.
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
                    3
                } else {
                    5
                };
            } else if self.hook_state.lock().nibble > 0 {
                // TODO: Looting
                // TODO: criteria triggers (advancements)
                // TODO: award stat when catching fish
            }

            if self.base.on_ground() {
                damage = 2;
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
            );
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

impl Entity for FishingHook {
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

    fn tick(&self) {
        if let Some(owner) = self.get_owner()
            && let Some(player) = owner.as_player()
            && !self.should_stop_fishing(player)
        {
            let mut state = self.hook_state.lock();

            if self.on_ground() {
                state.life += 1;

                if state.life >= 1200 {
                    // TODO: discard
                }
            } else {
                state.life = 0;
            }

            let mut liquid_height: f32 = 0.0;
            let pos = BlockPos::from(self.base.position());

            if let Some(world) = self.level() {
                let block_state = world.get_block_state(pos);
                let fluid_state = block_state.get_fluid_state();

                if fluid_state.is_water() {
                    liquid_height = fluid_state.own_height(); // TODO: is this correct?
                }

                let is_in_water = liquid_height > 0.5;

                if state.current_state == FishHookState::Flying {
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

                    self.check_collision();
                } else {
                    if state.current_state == FishHookState::HookedInEntity {
                        if state.hooked_in.is_some() {
                            let hooked = state.hooked_in.as_ref().unwrap();
                            // TODO: && this.hookedIn.level().dimension() == this.level().dimension()
                            if !hooked.is_removed() && hooked.can_interact_with_level() {
                                self.try_set_position(hooked.position() * DVec3::new(1.0, 0.8, 1.0)).expect("error: due to dubious reasons, steel couldn't teleport the fishing hook to the hooked entity.");
                            } else {
                                self.set_hooked_entity(None);
                                state.current_state = FishHookState::Flying;
                            }
                        }
                        return;
                    }

                    if state.current_state == FishHookState::Bobbing {
                        let velocity = self.base.velocity();
                        let mut force: f64 = self.position().x + velocity.y
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
                                && state.out_of_water_time < 10
                                && self.calculate_open_water(pos);
                        }

                        if is_in_water {
                            state.out_of_water_time = (state.out_of_water_time - 1).max(0);
                            // TODO: `fishing_hook_mut()` or `fishing_hook` ?
                            if *self.entity_data.lock().fishing_hook_mut().biting.get() {
                                // TODO: -0.1 * this.syncronizedRandom.nextFloat() * this.syncronizedRandom.nextFloat()
                                self.base.set_velocity(
                                    self.base.velocity().add(DVec3::new(0.0, -0.1, 0.0)),
                                );
                            }

                            self.catching_fish(pos);
                        } else {
                            state.out_of_water_time = (state.out_of_water_time + 1).min(10);
                        }
                    }
                }

                if !fluid_state.is_water() && !self.base.on_ground() && state.hooked_in.is_some() {
                    self.base
                        .set_velocity(self.base.velocity().add(DVec3::new(0.0, -0.03, 0.0)));
                }

                self.move_entity(MoverType::SelfMovement, self.base.velocity());
                self.apply_effects_from_blocks();
                self.update_rotation();

                if state.current_state == FishHookState::Flying
                    && (self.base.on_ground() || self.base.horizontal_collision())
                {
                    self.base.set_velocity(DVec3::ZERO);
                }

                let inertia: f64 = 0.92;
                self.base.set_velocity(self.base.velocity() * inertia);
                // TODO: this.reapplyPosition();
                //self.base.set_old_position(DVec3::ZERO);
                self.base.set_old_position_to_current();
            }
        } else {
            self.set_removed(RemovalReason::Discarded);
        }
    }
}

impl Projectile for FishingHook {
    fn projectile_base(&self) -> &ProjectileBase {
        &self.projectile_base
    }
}

impl ThrowableProjectile for FishingHook {}

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

    fn test_hook(world: &Arc<World>, id: i32) -> Arc<FishingHook> {
        Arc::new(FishingHook::new(
            &vanilla_entities::FISHING_BOBBER,
            id,
            DVec3::ZERO,
            Arc::downgrade(world),
            SyncMutex::new(FishingHookState::new(0, 0)),
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
