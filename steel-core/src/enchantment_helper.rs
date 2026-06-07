use steel_registry::enchantment_effect::{
    DamageSourcePredicate, EnchantmentEffectComponent, EnchantmentEffectRequirements,
    EnchantmentEntityTarget, EntityPredicate, EntityTypePredicate,
};
use steel_registry::entity_type::EntityTypeRef;
use steel_registry::item_stack::ItemStack;
use steel_registry::{REGISTRY, RegistryExt, TaggedRegistryExt};

use crate::entity::damage::DamageSource;

#[derive(Debug, Clone, Copy)]
pub(crate) struct EnchantmentDamageContext<'a> {
    this_entity_type: EntityTypeRef,
    attacker_entity_type: Option<EntityTypeRef>,
    direct_attacker_entity_type: Option<EntityTypeRef>,
    damage_source: &'a DamageSource,
}

impl<'a> EnchantmentDamageContext<'a> {
    #[must_use]
    pub(crate) const fn new(
        this_entity_type: EntityTypeRef,
        attacker_entity_type: Option<EntityTypeRef>,
        direct_attacker_entity_type: Option<EntityTypeRef>,
        damage_source: &'a DamageSource,
    ) -> Self {
        Self {
            this_entity_type,
            attacker_entity_type,
            direct_attacker_entity_type,
            damage_source,
        }
    }

    const fn entity_type(self, target: EnchantmentEntityTarget) -> Option<EntityTypeRef> {
        match target {
            EnchantmentEntityTarget::This => Some(self.this_entity_type),
            EnchantmentEntityTarget::Attacker => self.attacker_entity_type,
            EnchantmentEntityTarget::DirectAttacker => self.direct_attacker_entity_type,
        }
    }
}

pub(crate) fn modify_damage(
    item: &ItemStack,
    context: &EnchantmentDamageContext<'_>,
    damage: f32,
) -> f32 {
    apply_value_effects(item, EnchantmentEffectComponent::Damage, context, damage)
}

pub(crate) fn modify_knockback(
    item: &ItemStack,
    context: &EnchantmentDamageContext<'_>,
    knockback: f32,
) -> f32 {
    apply_value_effects(
        item,
        EnchantmentEffectComponent::Knockback,
        context,
        knockback,
    )
}

fn apply_value_effects(
    item: &ItemStack,
    component: EnchantmentEffectComponent,
    context: &EnchantmentDamageContext<'_>,
    input: f32,
) -> f32 {
    let Some(enchantments) = item.get_enchantments() else {
        return input;
    };

    let mut value = input;
    for (key, level) in enchantments.iter() {
        if *level == 0 {
            continue;
        }
        let Some(enchantment) = REGISTRY.enchantments.by_key(key) else {
            continue;
        };
        let level = *level as i32;

        for effect in enchantment.effects.value_effects(component) {
            if !requirements_match(effect.requirements, context) {
                continue;
            }
            if let Some(updated) = effect.effect.process_without_random(level, value) {
                value = updated;
            }
        }

        let Some(effect) = enchantment.effects.single_value_effect(component) else {
            continue;
        };
        if let Some(updated) = effect.process_without_random(level, value) {
            value = updated;
        }
    }

    value
}

fn requirements_match(
    requirements: Option<&'static EnchantmentEffectRequirements>,
    context: &EnchantmentDamageContext<'_>,
) -> bool {
    let Some(requirements) = requirements else {
        return true;
    };

    matches!(requirements_state(requirements, context), Some(true))
}

fn requirements_state(
    requirements: &'static EnchantmentEffectRequirements,
    context: &EnchantmentDamageContext<'_>,
) -> Option<bool> {
    match requirements {
        EnchantmentEffectRequirements::AllOf(terms) => {
            let mut has_unknown = false;
            for term in *terms {
                match requirements_state(term, context) {
                    Some(true) => {}
                    Some(false) => return Some(false),
                    None => has_unknown = true,
                }
            }
            if has_unknown { None } else { Some(true) }
        }
        EnchantmentEffectRequirements::AnyOf(terms) => {
            let mut has_unknown = false;
            for term in *terms {
                match requirements_state(term, context) {
                    Some(true) => return Some(true),
                    Some(false) => {}
                    None => has_unknown = true,
                }
            }
            if has_unknown { None } else { Some(false) }
        }
        EnchantmentEffectRequirements::Inverted(term) => {
            requirements_state(term, context).map(|matched| !matched)
        }
        EnchantmentEffectRequirements::EntityProperties { entity, predicate } => context
            .entity_type(*entity)
            .map(|entity_type| entity_predicate_matches(predicate, entity_type)),
        EnchantmentEffectRequirements::DamageSourceProperties(predicate) => Some(
            damage_source_predicate_matches(predicate, context.damage_source),
        ),
        EnchantmentEffectRequirements::Unsupported { .. } => None,
    }
}

fn entity_predicate_matches(predicate: &EntityPredicate, entity_type: EntityTypeRef) -> bool {
    match &predicate.entity_type {
        EntityTypePredicate::Any => true,
        EntityTypePredicate::Type(expected) => entity_type.key == *expected,
        EntityTypePredicate::Tag(tag) => REGISTRY.entity_types.is_in_tag(entity_type, tag),
    }
}

fn damage_source_predicate_matches(
    predicate: &DamageSourcePredicate,
    damage_source: &DamageSource,
) -> bool {
    predicate
        .tags
        .iter()
        .all(|tag| damage_source.is(&tag.tag) == tag.expected)
}

#[cfg(test)]
mod tests {
    use steel_registry::data_components::vanilla_components::{ENCHANTMENTS, ItemEnchantments};
    use steel_registry::items::ItemRef;
    use steel_registry::{
        test_support::init_test_registry, vanilla_damage_types, vanilla_entities, vanilla_items,
    };
    use steel_utils::Identifier;

    use super::*;

    fn enchanted_item(item: ItemRef, enchantment: Identifier, level: u32) -> ItemStack {
        let mut enchantments = ItemEnchantments::empty();
        enchantments.set(enchantment, level);

        let mut stack = ItemStack::new(item);
        stack.set(ENCHANTMENTS, enchantments);
        stack
    }

    fn assert_f32_eq(actual: f32, expected: f32) {
        assert_eq!(
            actual.to_bits(),
            expected.to_bits(),
            "actual: {actual}, expected: {expected}"
        );
    }

    #[test]
    fn unsupported_requirement_does_not_match_through_inversion() {
        static UNSUPPORTED: EnchantmentEffectRequirements =
            EnchantmentEffectRequirements::Unsupported {
                condition: Identifier::vanilla_static("match_tool"),
            };
        static INVERTED: EnchantmentEffectRequirements =
            EnchantmentEffectRequirements::Inverted(&UNSUPPORTED);

        let damage_source = DamageSource::environment(&vanilla_damage_types::PLAYER_ATTACK);
        let context = EnchantmentDamageContext::new(
            &vanilla_entities::PLAYER,
            Some(&vanilla_entities::PLAYER),
            Some(&vanilla_entities::PLAYER),
            &damage_source,
        );

        assert!(!requirements_match(Some(&UNSUPPORTED), &context));
        assert!(!requirements_match(Some(&INVERTED), &context));
    }

    #[test]
    fn damage_enchantments_match_target_entity_tags() {
        init_test_registry();

        let stack = enchanted_item(
            &vanilla_items::ITEMS.diamond_sword,
            Identifier::vanilla_static("smite"),
            5,
        );
        let damage_source = DamageSource::environment(&vanilla_damage_types::PLAYER_ATTACK);
        let zombie_context = EnchantmentDamageContext::new(
            &vanilla_entities::ZOMBIE,
            Some(&vanilla_entities::PLAYER),
            Some(&vanilla_entities::PLAYER),
            &damage_source,
        );
        let spider_context = EnchantmentDamageContext::new(
            &vanilla_entities::SPIDER,
            Some(&vanilla_entities::PLAYER),
            Some(&vanilla_entities::PLAYER),
            &damage_source,
        );

        assert_f32_eq(modify_damage(&stack, &zombie_context, 7.0), 19.5);
        assert_f32_eq(modify_damage(&stack, &spider_context, 7.0), 7.0);
    }

    #[test]
    fn projectile_knockback_checks_direct_attacker_entity_tag() {
        init_test_registry();

        let stack = enchanted_item(
            &vanilla_items::ITEMS.bow,
            Identifier::vanilla_static("punch"),
            2,
        );
        let damage_source = DamageSource::environment(&vanilla_damage_types::ARROW);
        let melee_context = EnchantmentDamageContext::new(
            &vanilla_entities::ZOMBIE,
            Some(&vanilla_entities::PLAYER),
            Some(&vanilla_entities::PLAYER),
            &damage_source,
        );
        let arrow_context = EnchantmentDamageContext::new(
            &vanilla_entities::ZOMBIE,
            Some(&vanilla_entities::PLAYER),
            Some(&vanilla_entities::ARROW),
            &damage_source,
        );

        assert_f32_eq(modify_knockback(&stack, &melee_context, 0.0), 0.0);
        assert_f32_eq(modify_knockback(&stack, &arrow_context, 0.0), 2.0);
    }

    #[test]
    fn damage_source_properties_match_damage_type_tags() {
        init_test_registry();

        let stack = enchanted_item(
            &vanilla_items::ITEMS.diamond_sword,
            Identifier::vanilla_static("fire_protection"),
            4,
        );
        let fire_source = DamageSource::environment(&vanilla_damage_types::IN_FIRE);
        let fall_source = DamageSource::environment(&vanilla_damage_types::FALL);
        let fire_context =
            EnchantmentDamageContext::new(&vanilla_entities::PLAYER, None, None, &fire_source);
        let fall_context =
            EnchantmentDamageContext::new(&vanilla_entities::PLAYER, None, None, &fall_source);

        assert_f32_eq(
            apply_value_effects(
                &stack,
                EnchantmentEffectComponent::DamageProtection,
                &fire_context,
                0.0,
            ),
            8.0,
        );
        assert_f32_eq(
            apply_value_effects(
                &stack,
                EnchantmentEffectComponent::DamageProtection,
                &fall_context,
                0.0,
            ),
            0.0,
        );
    }
}
