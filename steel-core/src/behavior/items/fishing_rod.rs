use crate::behavior::{InteractionResult, ItemBehavior, UseItemContext};
use crate::entity::projectile::fishing_hook::{FishingHook, FishingHookState};
use crate::entity::{Entity, Projectile, RemovalReason, SharedEntity, next_entity_id};
use glam::DVec3;
use rand::{RngExt, rng};
use std::sync::Arc;
use steel_macros::item_behavior;
use steel_protocol::packets::game::SoundSource;
use steel_registry::sound_events::{ENTITY_FISHING_BOBBER_RETRIEVE, ENTITY_FISHING_BOBBER_THROW};
use steel_registry::vanilla_entities;
use steel_utils::locks::SyncMutex;

const SHOOT_POWER: f32 = 1.5;

/// literally self-explanatory
#[item_behavior]
pub struct FishingRodItem;

impl ItemBehavior for FishingRodItem {
    fn use_item(&self, context: &mut UseItemContext) -> InteractionResult {
        let player = context.player;
        let world = context.world;
        let infinite_materials = context.player.has_infinite_materials();

        let pitch = 0.4 / (rng().random::<f32>() * 0.4 + 0.8);

        if let Some(fishing) = player.fishing_hook() {
            context.inv.with_item(|item| {
                let damage = fishing.retrieve(item);
                item.hurt_and_break(damage, infinite_materials);
            });

            world.play_sound_at(
                &ENTITY_FISHING_BOBBER_RETRIEVE,
                SoundSource::Neutral,
                player.position(),
                1.0,
                pitch,
                None,
            );
            // TODO: vibration
        } else {
            world.play_sound_at(
                &ENTITY_FISHING_BOBBER_THROW,
                SoundSource::Neutral,
                player.position(),
                0.5,
                pitch,
                None,
            );

            let player_pos = player.position();
            let spawn_pos = DVec3::new(player_pos.x, player.get_eye_y() - 0.1, player_pos.z);

            let hook = Arc::new(FishingHook::new(
                &vanilla_entities::FISHING_BOBBER,
                next_entity_id(),
                spawn_pos,
                Arc::downgrade(world),
                SyncMutex::new(FishingHookState::new(0, 0)),
            ));

            if let Some(owner) = world.players.get_by_uuid(&player.gameprofile.id) {
                let owner: SharedEntity = owner;
                hook.set_owner(&owner);
            } else {
                hook.set_owner_uuid(Some(player.gameprofile.id));
                player.set_fishing_hook(&hook);
            }
            //hook.set_item_clamped(thrown_item);

            let (yaw, player_pitch) = player.rotation();
            hook.shoot_from_rotation(player, player_pitch, yaw, 0.0, SHOOT_POWER, 1.0);

            let entity: SharedEntity = hook;
            if let Err(error) = world.try_add_entity(Arc::clone(&entity)) {
                entity.set_removed(RemovalReason::Discarded);
                log::debug!("failed to spawn fishing hook: {error}");
                return InteractionResult::Fail;
            }
            // TODO: award stat
            // TODO: vibration
        }
        InteractionResult::Success
    }
}
