use crate::entity::entities::ItemEntity;
use crate::entity::projectile::triangle_random;
use crate::entity::{Entity, EntityBase, Projectile, ProjectileBase, RemovalReason, SharedEntity};
use crate::player::Player;
use crate::world::{LevelReader, World};
use glam::{DVec3, IVec3};
use rand::{RngExt, rng};
use std::any::Any;
use std::cmp::PartialEq;
use std::ops::Add;
use std::sync::{Arc, Weak};
use steel_macros::entity_behavior;
use steel_registry::blocks::block_state_ext::BlockStateExt;
use steel_registry::entity_type::EntityTypeRef;
use steel_registry::fluid::FluidStateExt;
use steel_registry::item_stack::ItemStack;
use steel_registry::particle_type::ParticleData;
use steel_registry::vanilla_entity_data::FishingBobberEntityData;
use steel_registry::vanilla_particle_types::{BUBBLE, FISHING};
use steel_registry::{sound_events, vanilla_blocks, vanilla_items, vanilla_particle_types};
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

    fn should_stop_fishing(&self, owner: &Player) -> bool {
        if !owner.can_interact_with_level() {
            // TODO: does this actually discard the entity?
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

        if let Some(hooked_entity) = self.hook_state.lock().hooked_in {
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

        if rng.random() < 0.25 && world.is_raining_at(above) {
            fishing_speed += 1;
        }

        if rng.random() < 0.5 && world.can_see_sky(above) {
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
                    if rng.random() < 0.15 {
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
                    1.0 + (rng.random() - rng.random()) * 0.4,
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
                    if (prev_layer == OpenWaterType::Invalid) {
                        return false;
                    }
                }
                OpenWaterType::InsideWater => {
                    if (prev_layer == OpenWaterType::AboveWater) {
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

        if (!state.is_air() && !state.get_block() == &vanilla_blocks::LILY_PAD) {
            let fluid_state = state.get_fluid_state();
            // TODO: normally I'd need to check if the collision shape (`get_collision_shape()`) at the position `pos` is empty, idk how to do that from the `fluid_state` (like in vanilla) tho
            if (fluid_state.is_water() && fluid_state.is_source() && fluid_state.is_empty()) {
                OpenWaterType::InsideWater
            } else {
                OpenWaterType::Invalid
            }
        } else {
            OpenWaterType::AboveWater
        }
    }
    // fn is_open_water_fishing(){}

    // TODO: `rod` is needed for advancements and loot params
    pub fn retrieve(&self, _rod: &ItemStack) -> i32 {
        let mut damage = 0;

        if let Some(owner) = self.projectile_owner()
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
                damage = 2
            }
        } else {
            damage = 0
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

    fn update_owner_info() {}
    // fn get_player_owner(){} we have get_owner() from the projectile trait (SharedEntity)
    // fn get_hooked_in(){}
}

impl Entity for FishingHook {
    fn base(&self) -> &EntityBase {
        &self.base
    }

    fn entity_type(&self) -> EntityTypeRef {
        self.entity_type
    }
}

impl Projectile for FishingHook {
    fn projectile_base(&self) -> &ProjectileBase {
        &self.projectile_base
    }
}

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
