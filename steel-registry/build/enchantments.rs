use std::fs;

use crate::generator_functions::generate_sound_event_ref;
use heck::ToShoutySnakeCase;
use proc_macro2::{Ident, Literal, Span, TokenStream};
use quote::quote;
use serde::Deserialize;
use steel_utils::Identifier;

#[derive(Deserialize, Debug)]
struct EnchantmentJson {
    max_level: u32,
    min_cost: CostJson,
    max_cost: CostJson,
    anvil_cost: i32,
    weight: u32,
    slots: Vec<String>,
    supported_items: String,
    primary_items: Option<String>,
    exclusive_set: Option<String>,
    #[serde(default)]
    effects: EnchantmentEffectsJson,
}

#[derive(Deserialize, Debug)]
struct CostJson {
    base: i32,
    per_level_above_first: i32,
}

#[derive(Deserialize, Debug, Default)]
struct EnchantmentEffectsJson {
    #[serde(rename = "minecraft:damage_protection", default)]
    damage_protection: Vec<ConditionalValueEffectJson>,
    #[serde(rename = "minecraft:damage_immunity", default)]
    damage_immunity: Vec<serde_json::Value>,
    #[serde(rename = "minecraft:damage", default)]
    damage: Vec<ConditionalValueEffectJson>,
    #[serde(rename = "minecraft:smash_damage_per_fallen_block", default)]
    smash_damage_per_fallen_block: Vec<ConditionalValueEffectJson>,
    #[serde(rename = "minecraft:knockback", default)]
    knockback: Vec<ConditionalValueEffectJson>,
    #[serde(rename = "minecraft:armor_effectiveness", default)]
    armor_effectiveness: Vec<ConditionalValueEffectJson>,
    #[serde(rename = "minecraft:post_attack", default)]
    post_attack: Vec<serde_json::Value>,
    #[serde(rename = "minecraft:post_piercing_attack", default)]
    post_piercing_attack: Vec<serde_json::Value>,
    #[serde(rename = "minecraft:hit_block", default)]
    hit_block: Vec<serde_json::Value>,
    #[serde(rename = "minecraft:item_damage", default)]
    item_damage: Vec<ConditionalValueEffectJson>,
    #[serde(rename = "minecraft:equipment_drops", default)]
    equipment_drops: Vec<ConditionalValueEffectJson>,
    #[serde(rename = "minecraft:location_changed", default)]
    location_changed: Vec<serde_json::Value>,
    #[serde(rename = "minecraft:tick", default)]
    tick: Vec<serde_json::Value>,
    #[serde(rename = "minecraft:ammo_use", default)]
    ammo_use: Vec<ConditionalValueEffectJson>,
    #[serde(rename = "minecraft:projectile_piercing", default)]
    projectile_piercing: Vec<ConditionalValueEffectJson>,
    #[serde(rename = "minecraft:projectile_spawned", default)]
    projectile_spawned: Vec<serde_json::Value>,
    #[serde(rename = "minecraft:projectile_spread", default)]
    projectile_spread: Vec<ConditionalValueEffectJson>,
    #[serde(rename = "minecraft:projectile_count", default)]
    projectile_count: Vec<ConditionalValueEffectJson>,
    #[serde(rename = "minecraft:trident_return_acceleration", default)]
    trident_return_acceleration: Vec<ConditionalValueEffectJson>,
    #[serde(rename = "minecraft:fishing_time_reduction", default)]
    fishing_time_reduction: Vec<ConditionalValueEffectJson>,
    #[serde(rename = "minecraft:fishing_luck_bonus", default)]
    fishing_luck_bonus: Vec<ConditionalValueEffectJson>,
    #[serde(rename = "minecraft:block_experience", default)]
    block_experience: Vec<ConditionalValueEffectJson>,
    #[serde(rename = "minecraft:mob_experience", default)]
    mob_experience: Vec<ConditionalValueEffectJson>,
    #[serde(rename = "minecraft:repair_with_xp", default)]
    repair_with_xp: Vec<ConditionalValueEffectJson>,
    #[serde(rename = "minecraft:attributes", default)]
    attributes: Vec<AttributeEffectJson>,
    #[serde(rename = "minecraft:crossbow_charge_time", default)]
    crossbow_charge_time: Option<ValueEffectJson>,
    #[serde(rename = "minecraft:crossbow_charging_sounds", default)]
    crossbow_charging_sounds: Vec<CrossbowChargingSoundsJson>,
    #[serde(rename = "minecraft:trident_sound", default)]
    trident_sound: Vec<Identifier>,
    #[serde(rename = "minecraft:prevent_equipment_drop", default)]
    prevent_equipment_drop: Option<serde_json::Value>,
    #[serde(rename = "minecraft:prevent_armor_change", default)]
    prevent_armor_change: Option<serde_json::Value>,
    #[serde(rename = "minecraft:trident_spin_attack_strength", default)]
    trident_spin_attack_strength: Option<ValueEffectJson>,
}

#[derive(Deserialize, Debug)]
struct ConditionalValueEffectJson {
    effect: ValueEffectJson,
    #[serde(default)]
    requirements: Option<RequirementsJson>,
}

#[derive(Deserialize, Debug)]
struct RequirementsJson {
    condition: Identifier,
}

#[derive(Deserialize, Debug)]
struct AttributeEffectJson {
    amount: LevelBasedValueJson,
    attribute: Identifier,
    id: Identifier,
    operation: String,
}

#[derive(Deserialize, Debug)]
struct CrossbowChargingSoundsJson {
    start: Option<Identifier>,
    mid: Option<Identifier>,
    end: Option<Identifier>,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
enum ValueEffectJson {
    #[serde(rename = "minecraft:add")]
    Add { value: LevelBasedValueJson },
    #[serde(rename = "minecraft:set")]
    Set { value: LevelBasedValueJson },
    #[serde(rename = "minecraft:multiply")]
    Multiply { factor: LevelBasedValueJson },
    #[serde(rename = "minecraft:remove_binomial")]
    RemoveBinomial { chance: LevelBasedValueJson },
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum LevelBasedValueJson {
    Constant(f32),
    Typed(LevelBasedValueTypedJson),
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
enum LevelBasedValueTypedJson {
    #[serde(rename = "minecraft:clamped")]
    Clamped {
        value: Box<LevelBasedValueJson>,
        min: f32,
        max: f32,
    },
    #[serde(rename = "minecraft:exponent")]
    Exponent {
        base: Box<LevelBasedValueJson>,
        power: Box<LevelBasedValueJson>,
    },
    #[serde(rename = "minecraft:fraction")]
    Fraction {
        numerator: Box<LevelBasedValueJson>,
        denominator: Box<LevelBasedValueJson>,
    },
    #[serde(rename = "minecraft:levels_squared")]
    LevelsSquared { added: f32 },
    #[serde(rename = "minecraft:linear")]
    Linear {
        base: f32,
        per_level_above_first: f32,
    },
    #[serde(rename = "minecraft:lookup")]
    Lookup {
        values: Vec<f32>,
        fallback: Box<LevelBasedValueJson>,
    },
}

fn slot_to_tokens(slot: &str) -> TokenStream {
    match slot {
        "any" => quote! { EquipmentSlotGroup::Any },
        "hand" => quote! { EquipmentSlotGroup::Hand },
        "mainhand" => quote! { EquipmentSlotGroup::MainHand },
        "offhand" => quote! { EquipmentSlotGroup::OffHand },
        "armor" => quote! { EquipmentSlotGroup::Armor },
        "head" => quote! { EquipmentSlotGroup::Head },
        "chest" => quote! { EquipmentSlotGroup::Chest },
        "legs" => quote! { EquipmentSlotGroup::Legs },
        "feet" => quote! { EquipmentSlotGroup::Feet },
        "body" => quote! { EquipmentSlotGroup::Body },
        other => panic!("Unknown equipment slot group: {other}"),
    }
}

fn identifier_token(identifier: &Identifier) -> TokenStream {
    let namespace = identifier.namespace.as_ref();
    let path = identifier.path.as_ref();
    quote! { Identifier::new_static(#namespace, #path) }
}

fn attribute_ref_token(attribute: &Identifier) -> TokenStream {
    assert_eq!(
        attribute.namespace.as_ref(),
        "minecraft",
        "vanilla enchantment attribute references must use the minecraft namespace: {attribute}"
    );
    let ident = Ident::new(&attribute.path.to_shouty_snake_case(), Span::call_site());
    quote! { vanilla_attributes::#ident }
}

fn attribute_modifier_operation_token(operation: &str) -> TokenStream {
    match operation {
        "add_value" => quote! { AttributeModifierOperation::AddValue },
        "add_multiplied_base" => quote! { AttributeModifierOperation::AddMultipliedBase },
        "add_multiplied_total" => quote! { AttributeModifierOperation::AddMultipliedTotal },
        other => panic!("Unknown enchantment attribute modifier operation: {other}"),
    }
}

fn option_sound_event_ref_token(sound: Option<&Identifier>) -> TokenStream {
    match sound {
        Some(sound) => {
            let sound = generate_sound_event_ref(sound);
            quote! { Some(#sound) }
        }
        None => quote! { None },
    }
}

fn generate_level_based_value_ref(
    prefix: &str,
    value: &LevelBasedValueJson,
    statics: &mut TokenStream,
    counter: &mut usize,
) -> TokenStream {
    let ident = Ident::new(
        &format!("{prefix}_LEVEL_VALUE_{}", *counter),
        Span::call_site(),
    );
    *counter += 1;
    let value = generate_level_based_value(prefix, value, statics, counter);

    statics.extend(quote! {
        static #ident: LevelBasedValue = #value;
    });

    quote! { &#ident }
}

fn generate_level_based_value(
    prefix: &str,
    value: &LevelBasedValueJson,
    statics: &mut TokenStream,
    counter: &mut usize,
) -> TokenStream {
    match value {
        LevelBasedValueJson::Constant(value) => quote! { LevelBasedValue::Constant(#value) },
        LevelBasedValueJson::Typed(value) => match value {
            LevelBasedValueTypedJson::Clamped { value, min, max } => {
                let value = generate_level_based_value_ref(prefix, value, statics, counter);
                quote! { LevelBasedValue::Clamped { value: #value, min: #min, max: #max } }
            }
            LevelBasedValueTypedJson::Exponent { base, power } => {
                let base = generate_level_based_value_ref(prefix, base, statics, counter);
                let power = generate_level_based_value_ref(prefix, power, statics, counter);
                quote! { LevelBasedValue::Exponent { base: #base, power: #power } }
            }
            LevelBasedValueTypedJson::Fraction {
                numerator,
                denominator,
            } => {
                let numerator = generate_level_based_value_ref(prefix, numerator, statics, counter);
                let denominator =
                    generate_level_based_value_ref(prefix, denominator, statics, counter);
                quote! { LevelBasedValue::Fraction { numerator: #numerator, denominator: #denominator } }
            }
            LevelBasedValueTypedJson::LevelsSquared { added } => {
                quote! { LevelBasedValue::LevelsSquared { added: #added } }
            }
            LevelBasedValueTypedJson::Linear {
                base,
                per_level_above_first,
            } => {
                quote! { LevelBasedValue::Linear { base: #base, per_level_above_first: #per_level_above_first } }
            }
            LevelBasedValueTypedJson::Lookup { values, fallback } => {
                let fallback = generate_level_based_value_ref(prefix, fallback, statics, counter);
                quote! { LevelBasedValue::Lookup { values: &[#(#values),*], fallback: #fallback } }
            }
        },
    }
}

fn generate_value_effect(
    prefix: &str,
    effect: &ValueEffectJson,
    statics: &mut TokenStream,
    counter: &mut usize,
) -> TokenStream {
    match effect {
        ValueEffectJson::Add { value } => {
            let value = generate_level_based_value_ref(prefix, value, statics, counter);
            quote! { EnchantmentValueEffect::Add { value: #value } }
        }
        ValueEffectJson::Set { value } => {
            let value = generate_level_based_value_ref(prefix, value, statics, counter);
            quote! { EnchantmentValueEffect::Set { value: #value } }
        }
        ValueEffectJson::Multiply { factor } => {
            let factor = generate_level_based_value_ref(prefix, factor, statics, counter);
            quote! { EnchantmentValueEffect::Multiply { factor: #factor } }
        }
        ValueEffectJson::RemoveBinomial { chance } => {
            let chance = generate_level_based_value_ref(prefix, chance, statics, counter);
            quote! { EnchantmentValueEffect::RemoveBinomial { chance: #chance } }
        }
    }
}

fn generate_requirements(requirements: &Option<RequirementsJson>) -> TokenStream {
    match requirements {
        Some(requirements) => {
            let condition = identifier_token(&requirements.condition);
            quote! { Some(EnchantmentEffectRequirements { condition: #condition }) }
        }
        None => quote! { None },
    }
}

fn generate_conditional_value_effects(
    prefix: &str,
    effects: &[ConditionalValueEffectJson],
    statics: &mut TokenStream,
    counter: &mut usize,
) -> TokenStream {
    let entries = effects.iter().enumerate().map(|(index, effect)| {
        let entry_prefix = format!("{prefix}_{index}");
        let effect_token = generate_value_effect(&entry_prefix, &effect.effect, statics, counter);
        let requirements = generate_requirements(&effect.requirements);
        quote! {
            ConditionalEnchantmentEffect {
                effect: #effect_token,
                requirements: #requirements,
            }
        }
    });

    quote! { &[#(#entries),*] }
}

fn generate_attribute_effects(
    prefix: &str,
    attributes: &[AttributeEffectJson],
    statics: &mut TokenStream,
    counter: &mut usize,
) -> TokenStream {
    let entries = attributes.iter().enumerate().map(|(index, effect)| {
        let entry_prefix = format!("{prefix}_ATTRIBUTE_{index}");
        let amount =
            generate_level_based_value_ref(&entry_prefix, &effect.amount, statics, counter);
        let attribute = attribute_ref_token(&effect.attribute);
        let id = identifier_token(&effect.id);
        let operation = attribute_modifier_operation_token(&effect.operation);
        quote! {
            EnchantmentAttributeEffect {
                amount: #amount,
                attribute: #attribute,
                id: #id,
                operation: #operation,
            }
        }
    });

    quote! { &[#(#entries),*] }
}

fn generate_optional_value_effect(
    prefix: &str,
    effect: &Option<ValueEffectJson>,
    statics: &mut TokenStream,
    counter: &mut usize,
) -> TokenStream {
    match effect {
        Some(effect) => {
            let effect = generate_value_effect(prefix, effect, statics, counter);
            quote! { Some(#effect) }
        }
        None => quote! { None },
    }
}

fn generate_crossbow_charging_sounds(sounds: &[CrossbowChargingSoundsJson]) -> TokenStream {
    let entries = sounds.iter().map(|sounds| {
        let start = option_sound_event_ref_token(sounds.start.as_ref());
        let mid = option_sound_event_ref_token(sounds.mid.as_ref());
        let end = option_sound_event_ref_token(sounds.end.as_ref());
        quote! {
            CrossbowChargingSounds {
                start: #start,
                mid: #mid,
                end: #end,
            }
        }
    });

    quote! { &[#(#entries),*] }
}

fn generate_sound_event_refs(sounds: &[Identifier]) -> TokenStream {
    let sounds = sounds.iter().map(generate_sound_event_ref);
    quote! { &[#(#sounds),*] }
}

fn generate_enchantment_effects(
    name: &str,
    effects: &EnchantmentEffectsJson,
    statics: &mut TokenStream,
    counter: &mut usize,
) -> TokenStream {
    let prefix = name.to_shouty_snake_case();
    let damage_protection = generate_conditional_value_effects(
        &format!("{prefix}_DAMAGE_PROTECTION"),
        &effects.damage_protection,
        statics,
        counter,
    );
    let damage = generate_conditional_value_effects(
        &format!("{prefix}_DAMAGE"),
        &effects.damage,
        statics,
        counter,
    );
    let smash_damage_per_fallen_block = generate_conditional_value_effects(
        &format!("{prefix}_SMASH_DAMAGE_PER_FALLEN_BLOCK"),
        &effects.smash_damage_per_fallen_block,
        statics,
        counter,
    );
    let knockback = generate_conditional_value_effects(
        &format!("{prefix}_KNOCKBACK"),
        &effects.knockback,
        statics,
        counter,
    );
    let armor_effectiveness = generate_conditional_value_effects(
        &format!("{prefix}_ARMOR_EFFECTIVENESS"),
        &effects.armor_effectiveness,
        statics,
        counter,
    );
    let item_damage = generate_conditional_value_effects(
        &format!("{prefix}_ITEM_DAMAGE"),
        &effects.item_damage,
        statics,
        counter,
    );
    let equipment_drops = generate_conditional_value_effects(
        &format!("{prefix}_EQUIPMENT_DROPS"),
        &effects.equipment_drops,
        statics,
        counter,
    );
    let ammo_use = generate_conditional_value_effects(
        &format!("{prefix}_AMMO_USE"),
        &effects.ammo_use,
        statics,
        counter,
    );
    let projectile_piercing = generate_conditional_value_effects(
        &format!("{prefix}_PROJECTILE_PIERCING"),
        &effects.projectile_piercing,
        statics,
        counter,
    );
    let projectile_spread = generate_conditional_value_effects(
        &format!("{prefix}_PROJECTILE_SPREAD"),
        &effects.projectile_spread,
        statics,
        counter,
    );
    let projectile_count = generate_conditional_value_effects(
        &format!("{prefix}_PROJECTILE_COUNT"),
        &effects.projectile_count,
        statics,
        counter,
    );
    let trident_return_acceleration = generate_conditional_value_effects(
        &format!("{prefix}_TRIDENT_RETURN_ACCELERATION"),
        &effects.trident_return_acceleration,
        statics,
        counter,
    );
    let fishing_time_reduction = generate_conditional_value_effects(
        &format!("{prefix}_FISHING_TIME_REDUCTION"),
        &effects.fishing_time_reduction,
        statics,
        counter,
    );
    let fishing_luck_bonus = generate_conditional_value_effects(
        &format!("{prefix}_FISHING_LUCK_BONUS"),
        &effects.fishing_luck_bonus,
        statics,
        counter,
    );
    let block_experience = generate_conditional_value_effects(
        &format!("{prefix}_BLOCK_EXPERIENCE"),
        &effects.block_experience,
        statics,
        counter,
    );
    let mob_experience = generate_conditional_value_effects(
        &format!("{prefix}_MOB_EXPERIENCE"),
        &effects.mob_experience,
        statics,
        counter,
    );
    let repair_with_xp = generate_conditional_value_effects(
        &format!("{prefix}_REPAIR_WITH_XP"),
        &effects.repair_with_xp,
        statics,
        counter,
    );
    let attributes = generate_attribute_effects(
        &format!("{prefix}_ATTRIBUTES"),
        &effects.attributes,
        statics,
        counter,
    );
    let crossbow_charge_time = generate_optional_value_effect(
        &format!("{prefix}_CROSSBOW_CHARGE_TIME"),
        &effects.crossbow_charge_time,
        statics,
        counter,
    );
    let crossbow_charging_sounds =
        generate_crossbow_charging_sounds(&effects.crossbow_charging_sounds);
    let trident_sound = generate_sound_event_refs(&effects.trident_sound);
    let trident_spin_attack_strength = generate_optional_value_effect(
        &format!("{prefix}_TRIDENT_SPIN_ATTACK_STRENGTH"),
        &effects.trident_spin_attack_strength,
        statics,
        counter,
    );

    let damage_immunity = !effects.damage_immunity.is_empty();
    let post_attack = !effects.post_attack.is_empty();
    let post_piercing_attack = !effects.post_piercing_attack.is_empty();
    let hit_block = !effects.hit_block.is_empty();
    let location_changed = !effects.location_changed.is_empty();
    let tick = !effects.tick.is_empty();
    let projectile_spawned = !effects.projectile_spawned.is_empty();
    let prevent_equipment_drop = effects.prevent_equipment_drop.is_some();
    let prevent_armor_change = effects.prevent_armor_change.is_some();

    quote! {
        EnchantmentEffects {
            damage_protection: #damage_protection,
            damage_immunity: #damage_immunity,
            damage: #damage,
            smash_damage_per_fallen_block: #smash_damage_per_fallen_block,
            knockback: #knockback,
            armor_effectiveness: #armor_effectiveness,
            post_attack: #post_attack,
            post_piercing_attack: #post_piercing_attack,
            hit_block: #hit_block,
            item_damage: #item_damage,
            equipment_drops: #equipment_drops,
            location_changed: #location_changed,
            tick: #tick,
            ammo_use: #ammo_use,
            projectile_piercing: #projectile_piercing,
            projectile_spawned: #projectile_spawned,
            projectile_spread: #projectile_spread,
            projectile_count: #projectile_count,
            trident_return_acceleration: #trident_return_acceleration,
            fishing_time_reduction: #fishing_time_reduction,
            fishing_luck_bonus: #fishing_luck_bonus,
            block_experience: #block_experience,
            mob_experience: #mob_experience,
            repair_with_xp: #repair_with_xp,
            attributes: #attributes,
            crossbow_charge_time: #crossbow_charge_time,
            crossbow_charging_sounds: #crossbow_charging_sounds,
            trident_sound: #trident_sound,
            prevent_equipment_drop: #prevent_equipment_drop,
            prevent_armor_change: #prevent_armor_change,
            trident_spin_attack_strength: #trident_spin_attack_strength,
        }
    }
}

pub(crate) fn build() -> TokenStream {
    println!("cargo:rerun-if-changed=build_assets/builtin_datapacks/minecraft/enchantment/");

    let enchantment_dir = "build_assets/builtin_datapacks/minecraft/enchantment";
    let mut enchantments = Vec::new();

    for entry in fs::read_dir(enchantment_dir).expect("Failed to read enchantment directory") {
        let entry = entry.expect("Failed to read directory entry");
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) != Some("json") {
            continue;
        }

        let name = path
            .file_stem()
            .expect("No file stem")
            .to_str()
            .expect("Invalid UTF-8")
            .to_string();
        let content = fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("Failed to read {}: {e}", path.display()));
        let ench: EnchantmentJson = serde_json::from_str(&content)
            .unwrap_or_else(|e| panic!("Failed to parse {name}: {e}"));

        enchantments.push((name, ench));
    }

    enchantments.sort_by(|a, b| a.0.cmp(&b.0));

    let mut stream = TokenStream::new();

    stream.extend(quote! {
        use crate::attribute::AttributeModifierOperation;
        use crate::enchantment_effect::{
            ConditionalEnchantmentEffect, CrossbowChargingSounds, EnchantmentAttributeEffect,
            EnchantmentEffectRequirements, EnchantmentEffects, EnchantmentValueEffect,
            LevelBasedValue,
        };
        use crate::enchantment::{Enchantment, EnchantmentCost, EnchantmentRegistry};
        use crate::equipment::EquipmentSlotGroup;
        use crate::vanilla_attributes;
        use steel_utils::Identifier;
    });

    let mut register_stream = TokenStream::new();
    let mut value_statics = TokenStream::new();
    let mut value_static_counter = 0;

    for (name, ench) in &enchantments {
        let const_ident = Ident::new(&name.to_shouty_snake_case(), Span::call_site());

        let max_level = Literal::u32_unsuffixed(ench.max_level);
        let min_cost_base = Literal::i32_unsuffixed(ench.min_cost.base);
        let min_cost_per = Literal::i32_unsuffixed(ench.min_cost.per_level_above_first);
        let max_cost_base = Literal::i32_unsuffixed(ench.max_cost.base);
        let max_cost_per = Literal::i32_unsuffixed(ench.max_cost.per_level_above_first);
        let anvil_cost = Literal::i32_unsuffixed(ench.anvil_cost);
        let weight = Literal::u32_unsuffixed(ench.weight);

        let slots: Vec<TokenStream> = ench.slots.iter().map(|s| slot_to_tokens(s)).collect();

        let supported_items = ench.supported_items.as_str();
        let primary_items = match &ench.primary_items {
            Some(s) => {
                let s = s.as_str();
                quote! { Some(#s) }
            }
            None => quote! { None },
        };
        let exclusive_set = match &ench.exclusive_set {
            Some(s) => {
                let s = s.as_str();
                quote! { Some(#s) }
            }
            None => quote! { None },
        };
        let effects = generate_enchantment_effects(
            name,
            &ench.effects,
            &mut value_statics,
            &mut value_static_counter,
        );

        stream.extend(quote! {
            pub static #const_ident: Enchantment = Enchantment {
                key: Identifier::vanilla_static(#name),
                max_level: #max_level,
                min_cost: EnchantmentCost { base: #min_cost_base, per_level_above_first: #min_cost_per },
                max_cost: EnchantmentCost { base: #max_cost_base, per_level_above_first: #max_cost_per },
                anvil_cost: #anvil_cost,
                weight: #weight,
                slots: &[#(#slots),*],
                supported_items: #supported_items,
                primary_items: #primary_items,
                exclusive_set: #exclusive_set,
                effects: #effects,
            };
        });

        register_stream.extend(quote! {
            registry.register(&#const_ident);
        });
    }

    stream.extend(quote! {
        #value_statics

        pub fn register_enchantments(registry: &mut EnchantmentRegistry) {
            #register_stream
        }
    });

    stream
}
