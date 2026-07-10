use steel_macros::entity_behavior;
use steel_registry::entity_type::EntityTypeRef;
use crate::entity::{Entity, EntityBase, Projectile, ProjectileBase};

#[entity_behavior]
pub struct FishingHook {
    // Logger LOGGER (???)
    // RandomSource synchronizedRandom (???)
    biting: bool,
    out_of_water_time: u32,
    // MAX_OUT_OF_WATER_TIME: f32,
    // DATA_HOOKED_ENTITY:
    life: u16,
    nibble: u16,
    time_until_lured: f32,
    time_until_hooked: f32,
    fish_angle: f32,
    open_water: bool,
    // private @Nullable Entity hookedIn;
    current_state: FishHookState,
    luck: u32,
    lure_speed: u32,
    // private final InterpolationHandler interpolationHandler = new InterpolationHandler(this);
}

impl FishingHook {
    pub const MAX_OUT_OF_WATER_TIME: f32 = 10.0;

    fn should_stop_fishing(){}
    fn check_collision(){}
    fn set_hooked_entity(){}
    fn catching_fish(){}
    fn calculate_open_water(){}
    fn get_open_water_type_for_area(){}
    fn get_open_water_type_for_block(){}
    // fn is_open_water_fishing(){}
    fn retrieve(){}
    fn pull_entity(){}
    fn update_owner_info(){}
    // fn get_player_owner(){}
    // fn get_hooked_in(){}
}

impl Entity for FishingHook {
    fn base(&self) -> &EntityBase {
        todo!()
    }

    fn entity_type(&self) -> EntityTypeRef {
        todo!()
    }
}

impl Projectile for FishingHook {
    fn projectile_base(&self) -> &ProjectileBase {
        todo!()
    }
}

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