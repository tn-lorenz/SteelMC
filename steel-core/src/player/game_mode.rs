//! Game mode specific logic for player interactions.
//!
//! This module implements the logic from Java's `ServerPlayerGameMode`, particularly
//! the `useItemOn` method that handles block placement and block interactions.

use std::mem::swap;
use std::sync::Arc;

use glam::DVec3;
use steel_protocol::packets::game::{
    CBlockChangedAck, CBlockUpdate, CChangeDifficulty, CGameEvent, COpenSignEditor,
    CPlayerInfoUpdate, CSetCamera, CSetEntityMotion, CSetHeldSlot, GameEventType, PlayerAction,
    SAttack, SInteract, SPickItemFromBlock, SPlayerAction, SSignUpdate, SSpectatorAction, SUseItem,
    SUseItemOn,
};
use steel_registry::blocks::block_state_ext::BlockStateExt;
use steel_registry::blocks::properties::Direction;
use steel_registry::damage_type::DamageType;
use steel_registry::data_components::components::PiercingWeapon;
use steel_registry::entity_type::EntityTypeRef;
use steel_registry::item_stack::ItemStack;
use steel_registry::sound_event::{SoundEventHolder, SoundEventRef};
use steel_registry::{REGISTRY, vanilla_attributes, vanilla_damage_types, vanilla_entities};
use steel_utils::entity_events::EntityStatus;
use steel_utils::types::{Difficulty, GameType, InteractionHand};
use steel_utils::{BlockPos, Downcast as _, Identifier, WorldAabb};
use text_components::TextComponent;
use text_components::translation::TranslatedMessage;

use crate::behavior::{
    BLOCK_BEHAVIORS, BlockCollisionContext, BlockHitResult, ITEM_BEHAVIORS, InteractionResult,
    InventoryAccess, UseOnContext,
};
use crate::block_entity::BlockEntity;
use crate::block_entity::entities::SignBlockEntity;
use crate::command::player_can_change_difficulty;
use crate::enchantment_helper::{self, EnchantmentDamageContext, EnchantmentPostAttackContext};
use crate::entity::attribute::{AttributeModifier, AttributeModifierOperation};
use crate::entity::damage::DamageSource;
use crate::entity::{Entity, LivingEntity, SharedEntity};
use crate::inventory::equipment::EquipmentSlot;
use crate::inventory::menu::Menu;
use crate::physics::collision::{CollisionWorld, WorldCollisionProvider};
use crate::physics::shapes;
use crate::player::Player;
use crate::player::block_breaking::BlockBreakAction;
use crate::player::movement::wrap_degrees;
use crate::player::player_inventory::PlayerInventory;
use crate::world::{ClipBlockShape, ClipFluid, World};
use steel_utils::axis::Axis;

mod block_interaction;
mod entity_interaction;
mod item_interaction;
mod player_methods;
mod raycast;

pub use item_interaction::{use_item, use_item_on};

use raycast::piercing_ray_hit_t;

const CREATIVE_BLOCK_RANGE_MODIFIER_AMOUNT: f64 = 0.5;
const CREATIVE_ENTITY_RANGE_MODIFIER_AMOUNT: f64 = 2.0;
const ATTACK_RANGE_BUFFER: f64 = 3.0;
const ENTITY_INTERACTION_RANGE_BUFFER: f64 = 3.0;
const FLIGHT_DISABLE_RANGE: f64 = 1.0;

#[cfg(test)]
mod tests;
