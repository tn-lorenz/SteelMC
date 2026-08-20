use crate::behavior::{InteractionResult, ItemBehavior, UseItemContext};
use crate::entity::entities::objects::projectiles::FishingHookEntity;
use crate::entity::{Entity, Projectile, RemovalReason, SharedEntity, next_entity_id};
use glam::DVec3;
use rand::{RngExt, rng};
use std::sync::Arc;
use steel_macros::item_behavior;
use steel_protocol::packets::game::SoundSource;
use steel_registry::sound_events::{ENTITY_FISHING_BOBBER_RETRIEVE, ENTITY_FISHING_BOBBER_THROW};
use steel_registry::vanilla_entities;

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
            let damage = fishing.retrieve();
            context.inv.with_item(|item| {
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

            let hook = Arc::new(FishingHookEntity::new(
                &vanilla_entities::FISHING_BOBBER,
                next_entity_id(),
                spawn_pos,
                Arc::downgrade(world),
            ));

            if let Some(owner) = world.players.get_by_uuid(&player.gameprofile.id) {
                let owner: SharedEntity = owner;
                hook.set_owner(&owner);
            } else {
                hook.set_owner_uuid(Some(player.gameprofile.id));
                player.set_fishing_hook(&hook);
            }

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

#[cfg(test)]
mod tests {
    use steel_registry::{item_stack::ItemStack, vanilla_items};
    use steel_utils::types::InteractionHand;
    use uuid::Uuid;

    use super::*;
    use crate::behavior::UseItemContext;
    use crate::test_support::{TestPlayerBuilder, fresh_test_world};

    #[test]
    fn retrieving_grounded_hook_does_not_relock_inventory() {
        let world = fresh_test_world("fishing_rod_grounded_retrieve");
        let player =
            TestPlayerBuilder::new(Arc::clone(&world), Uuid::from_u128(1), "Fisher", 1).build();
        player
            .inventory
            .lock()
            .set_selected_item(ItemStack::new(&vanilla_items::FISHING_ROD));

        let player_owner = Arc::clone(&player);
        let owner: SharedEntity = player_owner;
        let hook = Arc::new(FishingHookEntity::new(
            &vanilla_entities::FISHING_BOBBER,
            2,
            DVec3::ZERO,
            Arc::downgrade(&world),
        ));
        hook.set_owner(&owner);
        hook.set_on_ground(true);

        let mut context = UseItemContext::new(
            &player,
            InteractionHand::MainHand,
            &world,
            Arc::clone(&player.inventory),
        );

        assert_eq!(
            FishingRodItem.use_item(&mut context),
            InteractionResult::Success
        );
        assert!(hook.is_removed());
        assert!(player.fishing_hook().is_none());
        assert_eq!(context.inv.with_item(|item| item.get_damage_value()), 2);
    }
}
