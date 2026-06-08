//! Equippable component for armor and equipment items.

use std::io::{Result, Write};
use std::str::FromStr;

use crate::{
    REGISTRY, RegistryExt, TaggedRegistryExt, entity_type::EntityTypeRef, equipment::EquipmentSlot,
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
    pub allowed_entities: Option<EquippableAllowedEntities>,
    pub dispensable: bool,
    pub swappable: bool,
    pub equip_on_interact: bool,
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
            allowed_entities: None,
            dispensable: true,
            swappable: true,
            equip_on_interact: false,
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
        compound.insert("dispensable", i8::from(self.dispensable));
        compound.insert("swappable", i8::from(self.swappable));
        compound.insert("equip_on_interact", i8::from(self.equip_on_interact));
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
        let equip_on_interact = compound
            .get("equip_on_interact")
            .and_then(|tag| tag.byte())
            .map(|value| value != 0)
            .unwrap_or(false);

        Some(Self {
            slot,
            allowed_entities,
            dispensable,
            swappable,
            equip_on_interact,
        })
    }
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
        assert!(helmet_equippable.can_be_equipped_by(&PLAYER));

        let saddle = ItemStack::new(&ITEMS.saddle);
        let Some(saddle_equippable) = saddle.get_equippable() else {
            panic!("saddle should have equippable data");
        };
        assert!(saddle_equippable.dispensable);
        assert!(saddle_equippable.equip_on_interact);
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
