use steel_macros::item_behavior;
use steel_registry::item_stack::ItemStack;
use steel_registry::vanilla_damage_types;

use crate::behavior::ItemBehavior;
use crate::enchantment_helper::{self, EnchantmentDamageContext};
use crate::entity::damage::DamageSource;
use crate::entity::{Entity, LivingEntity};
use crate::inventory::equipment::EquipmentSlot;

/// Vanilla mace item combat behavior.
#[item_behavior]
pub struct MaceItem;

impl MaceItem {
    const SMASH_ATTACK_FALL_THRESHOLD: f64 = 1.5;

    fn can_smash_attack(attacker: &dyn LivingEntity) -> bool {
        attacker.fall_distance() > Self::SMASH_ATTACK_FALL_THRESHOLD && !attacker.is_fall_flying()
    }
}

impl ItemBehavior for MaceItem {
    fn get_item_damage_source(&self, attacker: &dyn LivingEntity) -> Option<DamageSource> {
        Self::can_smash_attack(attacker).then(|| {
            DamageSource::environment(&vanilla_damage_types::MACE_SMASH)
                .with_causing_entity(attacker.id())
                .with_direct_entity(attacker.id())
                .with_source_position(attacker.position())
        })
    }

    fn get_attack_damage_bonus(
        &self,
        attacker: &dyn LivingEntity,
        victim: &dyn Entity,
        _base_damage: f32,
        damage_source: &DamageSource,
    ) -> f32 {
        if !Self::can_smash_attack(attacker) {
            return 0.0;
        }

        let fall_distance = attacker.fall_distance();
        let damage = if fall_distance <= 3.0 {
            4.0 * fall_distance
        } else if fall_distance <= 8.0 {
            12.0 + 2.0 * (fall_distance - 3.0)
        } else {
            22.0 + fall_distance - 8.0
        };
        let context = EnchantmentDamageContext::new(
            victim.entity_type(),
            Some(attacker.entity_type()),
            Some(attacker.entity_type()),
            damage_source,
        );
        let mut damage_per_fallen_block = 0.0;
        attacker.with_equipment_slot(EquipmentSlot::MainHand, &mut |item| {
            damage_per_fallen_block =
                enchantment_helper::modify_smash_damage_per_fallen_block(item, &context, 0.0);
        });
        (damage + f64::from(damage_per_fallen_block) * fall_distance) as f32
    }

    fn hurt_enemy(
        &self,
        _stack: &mut ItemStack,
        _target: &dyn LivingEntity,
        _attacker: &dyn LivingEntity,
    ) {
        // TODO: Apply vanilla mace smash knockback, impulse fall-damage immunity, and sounds.
    }

    fn post_hurt_enemy(
        &self,
        _stack: &mut ItemStack,
        _target: &dyn LivingEntity,
        attacker: &dyn LivingEntity,
    ) {
        if Self::can_smash_attack(attacker) {
            attacker.reset_fall_distance();
        }
    }
}
