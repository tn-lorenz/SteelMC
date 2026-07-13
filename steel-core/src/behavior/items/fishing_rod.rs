use rand::{rng, RngExt};
use crate::behavior::{InteractionResult, ItemBehavior, UseItemContext};
use steel_macros::item_behavior;
use steel_registry::item_stack::ItemStack;
use steel_registry::sound_events::{ENTITY_FISHING_BOBBER_RETRIEVE, ENTITY_FISHING_BOBBER_THROW};
use crate::entity::Entity;

/// literally self-explanatory
#[item_behavior]
pub struct FishingRodItem;

impl ItemBehavior for FishingRodItem {
    fn use_item(&self, context: &mut UseItemContext) -> InteractionResult {
        let player = context.player;
        let infinite_materials = context.player.has_infinite_materials();
        if let Some(fishing) = &player.fishing {
            context.inv.with_item(|item| {
                let damage = fishing.retrieve(*item);
                item.hurt_and_break(damage, infinite_materials);
            });

            player.play_sound(
                &ENTITY_FISHING_BOBBER_RETRIEVE,
                1.0,
                0.4 / (rng().random::<f32>() * 0.4 + 0.8),
            );
        } else {
            player.play_sound(
                &ENTITY_FISHING_BOBBER_THROW,
                0.5,
                0.4 / (rng().random::<f32>() * 0.4 + 0.8),
            );
        }
        InteractionResult::Success
    }
}
