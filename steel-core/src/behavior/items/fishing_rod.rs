use crate::behavior::{InteractionResult, ItemBehavior, UseItemContext};
use crate::entity::entities::EnderPearlEntity;
use crate::entity::projectile::fishing_hook::{FishingHook, FishingHookState};
use crate::entity::{Entity, Projectile};
use crate::world::World;
use glam::DVec3;
use rand::{RngExt, rng};
use std::sync::Weak;
use steel_macros::item_behavior;
use steel_registry::sound_events::{ENTITY_FISHING_BOBBER_RETRIEVE, ENTITY_FISHING_BOBBER_THROW};
use steel_registry::vanilla_entities;
use steel_utils::locks::SyncMutex;

/// literally self-explanatory
#[item_behavior]
pub struct FishingRodItem;

impl ItemBehavior for FishingRodItem {
    fn use_item(&self, context: &mut UseItemContext) -> InteractionResult {
        let player = context.player;
        let infinite_materials = context.player.has_infinite_materials();
        if let Some(fishing) = &player.fishing {
            context.inv.with_item(|item| {
                let damage = fishing.retrieve(item);
                item.hurt_and_break(damage, infinite_materials);
            });

            player.play_sound(
                &ENTITY_FISHING_BOBBER_RETRIEVE,
                1.0,
                0.4 / (rng().random::<f32>() * 0.4 + 0.8),
            );
            // TODO: vibration
        } else {
            player.play_sound(
                &ENTITY_FISHING_BOBBER_THROW,
                0.5,
                0.4 / (rng().random::<f32>() * 0.4 + 0.8),
            );

            let hook = FishingHook::new(
                &vanilla_entities::ENDER_PEARL,
                1,
                DVec3::ZERO,
                Weak::<World>::new(),
                SyncMutex::new(FishingHookState::new(0, 0)),
            );

            hook.shoot(DVec3::new(0.0, 0.0, 1.0), 1.5, 1.0);
            // TODO: award stat
            // TODO: vibration
        }
        InteractionResult::Success
    }
}
