//! Equippable component for armor and equipment items.

use std::io::{Result, Write};
use std::str::FromStr;

use crate::{
    REGISTRY, RegistryExt, TaggedRegistryExt, entity_type::EntityTypeRef, equipment::EquipmentSlot,
    sound_event::SoundEventRef, sound_events,
};
use steel_utils::{
    Identifier,
    hash::{ComponentHasher, HashComponent},
    serial::{ReadFrom, WriteTo},
};

/// Entity types allowed to equip an item.
#[derive(Debug, Clone, PartialEq)]
pub enum EquippableAllowedEntities {
    /// A tag of entity types, such as `minecraft:can_equip_saddle`.
    Tag(Identifier),
    /// Direct entity type references.
    EntityTypes(Vec<EntityTypeRef>),
}

impl EquippableAllowedEntities {
    /// Returns whether this holder set contains the entity type.
    #[must_use]
    pub fn contains(&self, entity_type: EntityTypeRef) -> bool {
        match self {
            Self::Tag(tag) => REGISTRY.entity_types.is_in_tag(entity_type, tag),
            Self::EntityTypes(entity_types) => entity_types.contains(&entity_type),
        }
    }
}

/// The equippable component data.
#[derive(Debug, Clone, PartialEq)]
pub struct Equippable {
    pub slot: EquipmentSlot,
    pub equip_sound: SoundEventRef,
    pub allowed_entities: Option<EquippableAllowedEntities>,
    pub dispensable: bool,
    pub swappable: bool,
    pub damage_on_hurt: bool,
    pub equip_on_interact: bool,
    pub can_be_sheared: bool,
    pub shearing_sound: SoundEventRef,
}

impl Equippable {
    /// Returns whether this item can be equipped by the entity type.
    #[must_use]
    pub fn can_be_equipped_by(&self, entity_type: EntityTypeRef) -> bool {
        self.allowed_entities
            .as_ref()
            .is_none_or(|allowed| allowed.contains(entity_type))
    }
}

impl WriteTo for Equippable {
    fn write(&self, _writer: &mut impl Write) -> Result<()> {
        // TODO: Implement proper Equippable serialization
        // Format: slot (VarInt), equip_sound (SoundEvent), model (Optional), camera_overlay (Optional),
        //         allowed_entities (Optional HolderSet), dispensable (bool), swappable (bool),
        //         damage_on_hurt (bool), equip_on_interact (bool)
        Ok(())
    }
}

impl ReadFrom for Equippable {
    fn read(_data: &mut std::io::Cursor<&[u8]>) -> Result<Self> {
        // TODO: Implement proper Equippable deserialization
        Ok(Self {
            slot: EquipmentSlot::Chest,
            equip_sound: &sound_events::ITEM_ARMOR_EQUIP_GENERIC,
            allowed_entities: None,
            dispensable: true,
            swappable: true,
            damage_on_hurt: true,
            equip_on_interact: false,
            can_be_sheared: false,
            shearing_sound: &sound_events::ITEM_SHEARS_SNIP,
        })
    }
}

impl HashComponent for Equippable {
    fn hash_component(&self, hasher: &mut ComponentHasher) {
        // Equippable is hashed as a map
        // For now, hash as empty map since full implementation requires proper codec
        hasher.start_map();
        // TODO: Add proper field hashing when Equippable codec is implemented
        hasher.end_map();
    }
}

impl simdnbt::ToNbtTag for Equippable {
    fn to_nbt_tag(self) -> simdnbt::owned::NbtTag {
        use simdnbt::owned::{NbtCompound, NbtTag};

        let mut compound = NbtCompound::new();
        compound.insert("slot", self.slot.name());
        compound.insert("equip_sound", self.equip_sound.key.to_string());
        compound.insert("dispensable", i8::from(self.dispensable));
        compound.insert("swappable", i8::from(self.swappable));
        compound.insert("damage_on_hurt", i8::from(self.damage_on_hurt));
        compound.insert("equip_on_interact", i8::from(self.equip_on_interact));
        compound.insert("can_be_sheared", i8::from(self.can_be_sheared));
        compound.insert("shearing_sound", self.shearing_sound.key.to_string());
        if let Some(allowed_entities) = self.allowed_entities {
            match allowed_entities {
                EquippableAllowedEntities::Tag(tag) => {
                    compound.insert("allowed_entities", format!("#{tag}"));
                }
                EquippableAllowedEntities::EntityTypes(entity_types) => {
                    let values: Vec<NbtTag> = entity_types
                        .into_iter()
                        .map(|entity_type| NbtTag::String(entity_type.key.to_string().into()))
                        .collect();
                    compound.insert(
                        "allowed_entities",
                        simdnbt::owned::NbtList::String(
                            values
                                .into_iter()
                                .filter_map(|value| match value {
                                    NbtTag::String(value) => Some(value),
                                    _ => None,
                                })
                                .collect(),
                        ),
                    );
                }
            }
        }
        NbtTag::Compound(compound)
    }
}

impl simdnbt::FromNbtTag for Equippable {
    fn from_nbt_tag(tag: simdnbt::borrow::NbtTag) -> Option<Self> {
        let compound = tag.compound()?;
        let slot_str = compound.get("slot")?.string()?.to_str();
        let slot = EquipmentSlot::by_name(&slot_str)?;
        let equip_sound = compound
            .get("equip_sound")
            .and_then(parse_sound_event_nbt)
            .unwrap_or(&sound_events::ITEM_ARMOR_EQUIP_GENERIC);
        let allowed_entities = compound
            .get("allowed_entities")
            .and_then(parse_allowed_entities_nbt);
        let dispensable = compound
            .get("dispensable")
            .and_then(|tag| tag.byte())
            .map(|value| value != 0)
            .unwrap_or(true);
        let swappable = compound
            .get("swappable")
            .and_then(|tag| tag.byte())
            .map(|value| value != 0)
            .unwrap_or(true);
        let damage_on_hurt = compound
            .get("damage_on_hurt")
            .and_then(|tag| tag.byte())
            .map(|value| value != 0)
            .unwrap_or(true);
        let equip_on_interact = compound
            .get("equip_on_interact")
            .and_then(|tag| tag.byte())
            .map(|value| value != 0)
            .unwrap_or(false);
        let can_be_sheared = compound
            .get("can_be_sheared")
            .and_then(|tag| tag.byte())
            .map(|value| value != 0)
            .unwrap_or(false);
        let shearing_sound = compound
            .get("shearing_sound")
            .and_then(parse_sound_event_nbt)
            .unwrap_or(&sound_events::ITEM_SHEARS_SNIP);

        Some(Self {
            slot,
            equip_sound,
            allowed_entities,
            dispensable,
            swappable,
            damage_on_hurt,
            equip_on_interact,
            can_be_sheared,
            shearing_sound,
        })
    }
}

fn parse_sound_event_nbt(tag: simdnbt::borrow::NbtTag) -> Option<SoundEventRef> {
    let value = tag.string()?.to_str();
    let id = Identifier::from_str(&value).ok()?;
    REGISTRY.sound_events.by_key(&id)
}

fn parse_allowed_entities_nbt(tag: simdnbt::borrow::NbtTag) -> Option<EquippableAllowedEntities> {
    if let Some(value) = tag.string() {
        return parse_allowed_entities_string(&value.to_str());
    }

    let list = tag.list()?;
    let strings = list.strings()?;
    let mut entity_types = Vec::new();
    for value in strings {
        let id = Identifier::from_str(&value.to_str()).ok()?;
        entity_types.push(REGISTRY.entity_types.by_key(&id)?);
    }

    Some(EquippableAllowedEntities::EntityTypes(entity_types))
}

fn parse_allowed_entities_string(value: &str) -> Option<EquippableAllowedEntities> {
    if let Some(tag) = value.strip_prefix('#') {
        return Identifier::from_str(tag)
            .ok()
            .map(EquippableAllowedEntities::Tag);
    }

    let id = Identifier::from_str(value).ok()?;
    let entity_type = REGISTRY.entity_types.by_key(&id)?;
    Some(EquippableAllowedEntities::EntityTypes(vec![entity_type]))
}

#[cfg(test)]
mod tests {
    use super::EquippableAllowedEntities;

    use crate::item_stack::ItemStack;
    use crate::sound_events;
    use crate::test_support::init_test_registry;
    use crate::vanilla_entities::{LLAMA, PIG, PLAYER, WOLF};
    use crate::vanilla_entity_type_tags::EntityTypeTag;
    use crate::vanilla_items::ITEMS;

    #[test]
    fn extracted_equippable_fields_gate_swapping_and_entity_types() {
        init_test_registry();

        let pumpkin = ItemStack::new(&ITEMS.carved_pumpkin);
        let Some(pumpkin_equippable) = pumpkin.get_equippable() else {
            panic!("carved pumpkin should have equippable data");
        };
        assert!(!pumpkin_equippable.swappable);
        assert!(pumpkin_equippable.dispensable);

        let helmet = ItemStack::new(&ITEMS.diamond_helmet);
        let Some(helmet_equippable) = helmet.get_equippable() else {
            panic!("diamond helmet should have equippable data");
        };
        assert!(helmet_equippable.dispensable);
        assert!(helmet_equippable.swappable);
        assert!(helmet_equippable.damage_on_hurt);
        assert!(!helmet_equippable.can_be_sheared);
        assert_eq!(
            helmet_equippable.equip_sound,
            &sound_events::ITEM_ARMOR_EQUIP_DIAMOND
        );
        assert!(helmet_equippable.can_be_equipped_by(&PLAYER));

        let saddle = ItemStack::new(&ITEMS.saddle);
        let Some(saddle_equippable) = saddle.get_equippable() else {
            panic!("saddle should have equippable data");
        };
        assert!(saddle_equippable.dispensable);
        assert!(saddle_equippable.equip_on_interact);
        assert!(saddle_equippable.can_be_sheared);
        assert_eq!(
            saddle_equippable.shearing_sound,
            &sound_events::ITEM_SADDLE_UNEQUIP
        );
        assert_eq!(
            saddle_equippable.allowed_entities,
            Some(EquippableAllowedEntities::Tag(
                EntityTypeTag::CAN_EQUIP_SADDLE
            ))
        );

        let carpet = ItemStack::new(&ITEMS.white_carpet);
        let Some(carpet_equippable) = carpet.get_equippable() else {
            panic!("carpet should have equippable data");
        };
        assert!(carpet_equippable.can_be_sheared);
        assert_eq!(
            carpet_equippable.shearing_sound,
            &sound_events::ITEM_LLAMA_CARPET_UNEQUIP
        );
        assert!(carpet_equippable.can_be_equipped_by(&LLAMA));
        assert!(!carpet_equippable.can_be_equipped_by(&PIG));
        assert!(!carpet_equippable.can_be_equipped_by(&PLAYER));

        let wolf_armor = ItemStack::new(&ITEMS.wolf_armor);
        let Some(wolf_armor_equippable) = wolf_armor.get_equippable() else {
            panic!("wolf armor should have equippable data");
        };
        assert!(wolf_armor_equippable.can_be_equipped_by(&WOLF));
        assert!(!wolf_armor_equippable.can_be_equipped_by(&PLAYER));
    }
}
