use crate::entity::entities::ItemEntity;
use crate::entity::{Entity, EntityBase, Projectile, ProjectileBase, RemovalReason, SharedEntity};
use crate::world::World;
use glam::DVec3;
use std::ops::Add;
use std::sync::{Arc, Weak};
use steel_macros::entity_behavior;
use steel_registry::entity_type::EntityTypeRef;
use steel_registry::item_stack::ItemStack;
use steel_registry::vanilla_entity_data::FishingBobberEntityData;
use steel_registry::vanilla_items;
use steel_utils::locks::SyncMutex;
use steel_utils::{Downcast, DowncastType, DowncastTypeKey};
use steel_utils::types::InteractionHand;
use crate::player::Player;

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
    fish_angle: f32,
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
            self.set_removed(RemovalReason::Discarded);
            return true
        }

        let inventory = owner.inventory.lock();

        let mainhand_item = inventory.get_item_in_hand(InteractionHand::MainHand);
        let offhand_item = inventory.get_offhand_item();

        let mainhand_fishing = mainhand_item.is(&vanilla_items::ITEMS.fishing_rod);
        let offhand_fishing = offhand_item.is(&vanilla_items::ITEMS.fishing_rod);

        if (mainhand_fishing || offhand_fishing) && self.distance_to_sqr(owner.position()) <= 1024.0 {
            false
        }

        self.set_removed(RemovalReason::Discarded);
        true
    }

    fn check_collision() {}
    fn set_hooked_entity() {}
    fn catching_fish() {}
    fn calculate_open_water() {}
    fn get_open_water_type_for_area() {}
    fn get_open_water_type_for_block() {}
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
    // fn get_player_owner(){}
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

enum OpenWaterType {
    AboveWater,
    InsideWater,
    Invalid,
}
