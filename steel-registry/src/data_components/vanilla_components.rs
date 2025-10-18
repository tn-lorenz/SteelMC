use steel_utils::ResourceLocation;

use crate::data_components::{DataComponentRegistry, DataComponentType};

pub type TODO = ();

// Basic data components
pub const CUSTOM_DATA: &'static DataComponentType<TODO> =
    &DataComponentType::new(ResourceLocation::vanilla_static("custom_data"));

pub const MAX_STACK_SIZE: &'static DataComponentType<i32> =
    &DataComponentType::new(ResourceLocation::vanilla_static("max_stack_size"));

pub const MAX_DAMAGE: &'static DataComponentType<i32> =
    &DataComponentType::new(ResourceLocation::vanilla_static("max_damage"));

pub const DAMAGE: &'static DataComponentType<i32> =
    &DataComponentType::new(ResourceLocation::vanilla_static("damage"));

pub const UNBREAKABLE: &'static DataComponentType<()> =
    &DataComponentType::new(ResourceLocation::vanilla_static("unbreakable"));

pub const CUSTOM_NAME: &'static DataComponentType<TODO> =
    &DataComponentType::new(ResourceLocation::vanilla_static("custom_name"));

pub const ITEM_NAME: &'static DataComponentType<TODO> =
    &DataComponentType::new(ResourceLocation::vanilla_static("item_name"));

pub const ITEM_MODEL: &'static DataComponentType<ResourceLocation> =
    &DataComponentType::new(ResourceLocation::vanilla_static("item_model"));

pub const LORE: &'static DataComponentType<TODO> =
    &DataComponentType::new(ResourceLocation::vanilla_static("lore"));

pub const RARITY: &'static DataComponentType<TODO> =
    &DataComponentType::new(ResourceLocation::vanilla_static("rarity"));

pub const ENCHANTMENTS: &'static DataComponentType<TODO> =
    &DataComponentType::new(ResourceLocation::vanilla_static("enchantments"));

pub const CAN_PLACE_ON: &'static DataComponentType<TODO> =
    &DataComponentType::new(ResourceLocation::vanilla_static("can_place_on"));

pub const CAN_BREAK: &'static DataComponentType<TODO> =
    &DataComponentType::new(ResourceLocation::vanilla_static("can_break"));

pub const ATTRIBUTE_MODIFIERS: &'static DataComponentType<TODO> =
    &DataComponentType::new(ResourceLocation::vanilla_static("attribute_modifiers"));

pub const CUSTOM_MODEL_DATA: &'static DataComponentType<TODO> =
    &DataComponentType::new(ResourceLocation::vanilla_static("custom_model_data"));

pub const TOOLTIP_DISPLAY: &'static DataComponentType<TODO> =
    &DataComponentType::new(ResourceLocation::vanilla_static("tooltip_display"));

pub const REPAIR_COST: &'static DataComponentType<i32> =
    &DataComponentType::new(ResourceLocation::vanilla_static("repair_cost"));

pub const CREATIVE_SLOT_LOCK: &'static DataComponentType<()> =
    &DataComponentType::new(ResourceLocation::vanilla_static("creative_slot_lock"));

pub const ENCHANTMENT_GLINT_OVERRIDE: &'static DataComponentType<bool> = &DataComponentType::new(
    ResourceLocation::vanilla_static("enchantment_glint_override"),
);

pub const INTANGIBLE_PROJECTILE: &'static DataComponentType<()> =
    &DataComponentType::new(ResourceLocation::vanilla_static("intangible_projectile"));

pub const FOOD: &'static DataComponentType<TODO> =
    &DataComponentType::new(ResourceLocation::vanilla_static("food"));

pub const CONSUMABLE: &'static DataComponentType<TODO> =
    &DataComponentType::new(ResourceLocation::vanilla_static("consumable"));

pub const USE_REMAINDER: &'static DataComponentType<TODO> =
    &DataComponentType::new(ResourceLocation::vanilla_static("use_remainder"));

pub const USE_COOLDOWN: &'static DataComponentType<TODO> =
    &DataComponentType::new(ResourceLocation::vanilla_static("use_cooldown"));

pub const DAMAGE_RESISTANT: &'static DataComponentType<TODO> =
    &DataComponentType::new(ResourceLocation::vanilla_static("damage_resistant"));

pub const TOOL: &'static DataComponentType<TODO> =
    &DataComponentType::new(ResourceLocation::vanilla_static("tool"));

pub const WEAPON: &'static DataComponentType<TODO> =
    &DataComponentType::new(ResourceLocation::vanilla_static("weapon"));

pub const ENCHANTABLE: &'static DataComponentType<TODO> =
    &DataComponentType::new(ResourceLocation::vanilla_static("enchantable"));

pub const EQUIPPABLE: &'static DataComponentType<TODO> =
    &DataComponentType::new(ResourceLocation::vanilla_static("equippable"));

pub const REPAIRABLE: &'static DataComponentType<TODO> =
    &DataComponentType::new(ResourceLocation::vanilla_static("repairable"));

pub const GLIDER: &'static DataComponentType<()> =
    &DataComponentType::new(ResourceLocation::vanilla_static("glider"));

pub const TOOLTIP_STYLE: &'static DataComponentType<ResourceLocation> =
    &DataComponentType::new(ResourceLocation::vanilla_static("tooltip_style"));

pub const DEATH_PROTECTION: &'static DataComponentType<TODO> =
    &DataComponentType::new(ResourceLocation::vanilla_static("death_protection"));

pub const BLOCKS_ATTACKS: &'static DataComponentType<TODO> =
    &DataComponentType::new(ResourceLocation::vanilla_static("blocks_attacks"));

pub const STORED_ENCHANTMENTS: &'static DataComponentType<TODO> =
    &DataComponentType::new(ResourceLocation::vanilla_static("stored_enchantments"));

pub const DYED_COLOR: &'static DataComponentType<TODO> =
    &DataComponentType::new(ResourceLocation::vanilla_static("dyed_color"));

pub const MAP_COLOR: &'static DataComponentType<TODO> =
    &DataComponentType::new(ResourceLocation::vanilla_static("map_color"));

pub const MAP_ID: &'static DataComponentType<TODO> =
    &DataComponentType::new(ResourceLocation::vanilla_static("map_id"));

pub const MAP_DECORATIONS: &'static DataComponentType<TODO> =
    &DataComponentType::new(ResourceLocation::vanilla_static("map_decorations"));

pub const MAP_POST_PROCESSING: &'static DataComponentType<TODO> =
    &DataComponentType::new(ResourceLocation::vanilla_static("map_post_processing"));

pub const CHARGED_PROJECTILES: &'static DataComponentType<TODO> =
    &DataComponentType::new(ResourceLocation::vanilla_static("charged_projectiles"));

pub const BUNDLE_CONTENTS: &'static DataComponentType<TODO> =
    &DataComponentType::new(ResourceLocation::vanilla_static("bundle_contents"));

pub const POTION_CONTENTS: &'static DataComponentType<TODO> =
    &DataComponentType::new(ResourceLocation::vanilla_static("potion_contents"));

pub const POTION_DURATION_SCALE: &'static DataComponentType<f32> =
    &DataComponentType::new(ResourceLocation::vanilla_static("potion_duration_scale"));

pub const SUSPICIOUS_STEW_EFFECTS: &'static DataComponentType<TODO> =
    &DataComponentType::new(ResourceLocation::vanilla_static("suspicious_stew_effects"));

pub const WRITABLE_BOOK_CONTENT: &'static DataComponentType<TODO> =
    &DataComponentType::new(ResourceLocation::vanilla_static("writable_book_content"));

pub const WRITTEN_BOOK_CONTENT: &'static DataComponentType<TODO> =
    &DataComponentType::new(ResourceLocation::vanilla_static("written_book_content"));

pub const TRIM: &'static DataComponentType<TODO> =
    &DataComponentType::new(ResourceLocation::vanilla_static("trim"));

pub const DEBUG_STICK_STATE: &'static DataComponentType<TODO> =
    &DataComponentType::new(ResourceLocation::vanilla_static("debug_stick_state"));

pub const ENTITY_DATA: &'static DataComponentType<TODO> =
    &DataComponentType::new(ResourceLocation::vanilla_static("entity_data"));

pub const BUCKET_ENTITY_DATA: &'static DataComponentType<TODO> =
    &DataComponentType::new(ResourceLocation::vanilla_static("bucket_entity_data"));

pub const BLOCK_ENTITY_DATA: &'static DataComponentType<TODO> =
    &DataComponentType::new(ResourceLocation::vanilla_static("block_entity_data"));

pub const INSTRUMENT: &'static DataComponentType<TODO> =
    &DataComponentType::new(ResourceLocation::vanilla_static("instrument"));

pub const PROVIDES_TRIM_MATERIAL: &'static DataComponentType<TODO> =
    &DataComponentType::new(ResourceLocation::vanilla_static("provides_trim_material"));

pub const OMINOUS_BOTTLE_AMPLIFIER: &'static DataComponentType<TODO> =
    &DataComponentType::new(ResourceLocation::vanilla_static("ominous_bottle_amplifier"));

pub const JUKEBOX_PLAYABLE: &'static DataComponentType<TODO> =
    &DataComponentType::new(ResourceLocation::vanilla_static("jukebox_playable"));

pub const PROVIDES_BANNER_PATTERNS: &'static DataComponentType<TODO> =
    &DataComponentType::new(ResourceLocation::vanilla_static("provides_banner_patterns"));

pub const RECIPES: &'static DataComponentType<TODO> =
    &DataComponentType::new(ResourceLocation::vanilla_static("recipes"));

pub const LODESTONE_TRACKER: &'static DataComponentType<TODO> =
    &DataComponentType::new(ResourceLocation::vanilla_static("lodestone_tracker"));

pub const FIREWORK_EXPLOSION: &'static DataComponentType<TODO> =
    &DataComponentType::new(ResourceLocation::vanilla_static("firework_explosion"));

pub const FIREWORKS: &'static DataComponentType<TODO> =
    &DataComponentType::new(ResourceLocation::vanilla_static("fireworks"));

pub const PROFILE: &'static DataComponentType<TODO> =
    &DataComponentType::new(ResourceLocation::vanilla_static("profile"));

pub const NOTE_BLOCK_SOUND: &'static DataComponentType<ResourceLocation> =
    &DataComponentType::new(ResourceLocation::vanilla_static("note_block_sound"));

pub const BANNER_PATTERNS: &'static DataComponentType<TODO> =
    &DataComponentType::new(ResourceLocation::vanilla_static("banner_patterns"));

pub const BASE_COLOR: &'static DataComponentType<TODO> =
    &DataComponentType::new(ResourceLocation::vanilla_static("base_color"));

pub const POT_DECORATIONS: &'static DataComponentType<TODO> =
    &DataComponentType::new(ResourceLocation::vanilla_static("pot_decorations"));

pub const CONTAINER: &'static DataComponentType<TODO> =
    &DataComponentType::new(ResourceLocation::vanilla_static("container"));

pub const BLOCK_STATE: &'static DataComponentType<TODO> =
    &DataComponentType::new(ResourceLocation::vanilla_static("block_state"));

pub const BEES: &'static DataComponentType<TODO> =
    &DataComponentType::new(ResourceLocation::vanilla_static("bees"));

pub const LOCK: &'static DataComponentType<TODO> =
    &DataComponentType::new(ResourceLocation::vanilla_static("lock"));

pub const CONTAINER_LOOT: &'static DataComponentType<TODO> =
    &DataComponentType::new(ResourceLocation::vanilla_static("container_loot"));

pub const BREAK_SOUND: &'static DataComponentType<TODO> =
    &DataComponentType::new(ResourceLocation::vanilla_static("break_sound"));

// Entity variant components
pub const VILLAGER_VARIANT: &'static DataComponentType<TODO> =
    &DataComponentType::new(ResourceLocation::vanilla_static("villager/variant"));

pub const WOLF_VARIANT: &'static DataComponentType<TODO> =
    &DataComponentType::new(ResourceLocation::vanilla_static("wolf/variant"));

pub const WOLF_SOUND_VARIANT: &'static DataComponentType<TODO> =
    &DataComponentType::new(ResourceLocation::vanilla_static("wolf/sound_variant"));

pub const WOLF_COLLAR: &'static DataComponentType<TODO> =
    &DataComponentType::new(ResourceLocation::vanilla_static("wolf/collar"));

pub const FOX_VARIANT: &'static DataComponentType<TODO> =
    &DataComponentType::new(ResourceLocation::vanilla_static("fox/variant"));

pub const SALMON_SIZE: &'static DataComponentType<TODO> =
    &DataComponentType::new(ResourceLocation::vanilla_static("salmon/size"));

pub const PARROT_VARIANT: &'static DataComponentType<TODO> =
    &DataComponentType::new(ResourceLocation::vanilla_static("parrot/variant"));

pub const TROPICAL_FISH_PATTERN: &'static DataComponentType<TODO> =
    &DataComponentType::new(ResourceLocation::vanilla_static("tropical_fish/pattern"));

pub const TROPICAL_FISH_BASE_COLOR: &'static DataComponentType<TODO> =
    &DataComponentType::new(ResourceLocation::vanilla_static("tropical_fish/base_color"));

pub const TROPICAL_FISH_PATTERN_COLOR: &'static DataComponentType<TODO> = &DataComponentType::new(
    ResourceLocation::vanilla_static("tropical_fish/pattern_color"),
);

pub const MOOSHROOM_VARIANT: &'static DataComponentType<TODO> =
    &DataComponentType::new(ResourceLocation::vanilla_static("mooshroom/variant"));

pub const RABBIT_VARIANT: &'static DataComponentType<TODO> =
    &DataComponentType::new(ResourceLocation::vanilla_static("rabbit/variant"));

pub const PIG_VARIANT: &'static DataComponentType<TODO> =
    &DataComponentType::new(ResourceLocation::vanilla_static("pig/variant"));

pub const COW_VARIANT: &'static DataComponentType<TODO> =
    &DataComponentType::new(ResourceLocation::vanilla_static("cow/variant"));

pub const CHICKEN_VARIANT: &'static DataComponentType<TODO> =
    &DataComponentType::new(ResourceLocation::vanilla_static("chicken/variant"));

pub const FROG_VARIANT: &'static DataComponentType<TODO> =
    &DataComponentType::new(ResourceLocation::vanilla_static("frog/variant"));

pub const HORSE_VARIANT: &'static DataComponentType<TODO> =
    &DataComponentType::new(ResourceLocation::vanilla_static("horse/variant"));

pub const PAINTING_VARIANT: &'static DataComponentType<TODO> =
    &DataComponentType::new(ResourceLocation::vanilla_static("painting/variant"));

pub const LLAMA_VARIANT: &'static DataComponentType<TODO> =
    &DataComponentType::new(ResourceLocation::vanilla_static("llama/variant"));

pub const AXOLOTL_VARIANT: &'static DataComponentType<TODO> =
    &DataComponentType::new(ResourceLocation::vanilla_static("axolotl/variant"));

pub const CAT_VARIANT: &'static DataComponentType<TODO> =
    &DataComponentType::new(ResourceLocation::vanilla_static("cat/variant"));

pub const CAT_COLLAR: &'static DataComponentType<TODO> =
    &DataComponentType::new(ResourceLocation::vanilla_static("cat/collar"));

pub const SHEEP_COLOR: &'static DataComponentType<TODO> =
    &DataComponentType::new(ResourceLocation::vanilla_static("sheep/color"));

pub const SHULKER_COLOR: &'static DataComponentType<TODO> =
    &DataComponentType::new(ResourceLocation::vanilla_static("shulker/color"));

pub fn register_vanilla_data_components(registry: &mut DataComponentRegistry) {
    // Basic components
    registry.register(CUSTOM_DATA);
    registry.register(MAX_STACK_SIZE);
    registry.register(MAX_DAMAGE);
    registry.register(DAMAGE);
    registry.register(UNBREAKABLE);
    registry.register(CUSTOM_NAME);
    registry.register(ITEM_NAME);
    registry.register(ITEM_MODEL);
    registry.register(LORE);
    registry.register(RARITY);
    registry.register(ENCHANTMENTS);
    registry.register(CAN_PLACE_ON);
    registry.register(CAN_BREAK);
    registry.register(ATTRIBUTE_MODIFIERS);
    registry.register(CUSTOM_MODEL_DATA);
    registry.register(TOOLTIP_DISPLAY);
    registry.register(REPAIR_COST);
    registry.register(CREATIVE_SLOT_LOCK);
    registry.register(ENCHANTMENT_GLINT_OVERRIDE);
    registry.register(INTANGIBLE_PROJECTILE);
    registry.register(FOOD);
    registry.register(CONSUMABLE);
    registry.register(USE_REMAINDER);
    registry.register(USE_COOLDOWN);
    registry.register(DAMAGE_RESISTANT);
    registry.register(TOOL);
    registry.register(WEAPON);
    registry.register(ENCHANTABLE);
    registry.register(EQUIPPABLE);
    registry.register(REPAIRABLE);
    registry.register(GLIDER);
    registry.register(TOOLTIP_STYLE);
    registry.register(DEATH_PROTECTION);
    registry.register(BLOCKS_ATTACKS);
    registry.register(STORED_ENCHANTMENTS);
    registry.register(DYED_COLOR);
    registry.register(MAP_COLOR);
    registry.register(MAP_ID);
    registry.register(MAP_DECORATIONS);
    registry.register(MAP_POST_PROCESSING);
    registry.register(CHARGED_PROJECTILES);
    registry.register(BUNDLE_CONTENTS);
    registry.register(POTION_CONTENTS);
    registry.register(POTION_DURATION_SCALE);
    registry.register(SUSPICIOUS_STEW_EFFECTS);
    registry.register(WRITABLE_BOOK_CONTENT);
    registry.register(WRITTEN_BOOK_CONTENT);
    registry.register(TRIM);
    registry.register(DEBUG_STICK_STATE);
    registry.register(ENTITY_DATA);
    registry.register(BUCKET_ENTITY_DATA);
    registry.register(BLOCK_ENTITY_DATA);
    registry.register(INSTRUMENT);
    registry.register(PROVIDES_TRIM_MATERIAL);
    registry.register(OMINOUS_BOTTLE_AMPLIFIER);
    registry.register(JUKEBOX_PLAYABLE);
    registry.register(PROVIDES_BANNER_PATTERNS);
    registry.register(RECIPES);
    registry.register(LODESTONE_TRACKER);
    registry.register(FIREWORK_EXPLOSION);
    registry.register(FIREWORKS);
    registry.register(PROFILE);
    registry.register(NOTE_BLOCK_SOUND);
    registry.register(BANNER_PATTERNS);
    registry.register(BASE_COLOR);
    registry.register(POT_DECORATIONS);
    registry.register(CONTAINER);
    registry.register(BLOCK_STATE);
    registry.register(BEES);
    registry.register(LOCK);
    registry.register(CONTAINER_LOOT);
    registry.register(BREAK_SOUND);

    // Entity variant components
    registry.register(VILLAGER_VARIANT);
    registry.register(WOLF_VARIANT);
    registry.register(WOLF_SOUND_VARIANT);
    registry.register(WOLF_COLLAR);
    registry.register(FOX_VARIANT);
    registry.register(SALMON_SIZE);
    registry.register(PARROT_VARIANT);
    registry.register(TROPICAL_FISH_PATTERN);
    registry.register(TROPICAL_FISH_BASE_COLOR);
    registry.register(TROPICAL_FISH_PATTERN_COLOR);
    registry.register(MOOSHROOM_VARIANT);
    registry.register(RABBIT_VARIANT);
    registry.register(PIG_VARIANT);
    registry.register(COW_VARIANT);
    registry.register(CHICKEN_VARIANT);
    registry.register(FROG_VARIANT);
    registry.register(HORSE_VARIANT);
    registry.register(PAINTING_VARIANT);
    registry.register(LLAMA_VARIANT);
    registry.register(AXOLOTL_VARIANT);
    registry.register(CAT_VARIANT);
    registry.register(CAT_COLLAR);
    registry.register(SHEEP_COLOR);
    registry.register(SHULKER_COLOR);
}
