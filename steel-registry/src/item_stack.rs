//! Item stack implementation.

use std::io::{Read, Result, Write};

use steel_utils::{
    Identifier,
    codec::VarInt,
    serial::{ReadFrom, WriteTo},
};

use crate::{
    REGISTRY,
    data_components::{
        ComponentPatchEntry, ComponentValue, DataComponentMap, DataComponentPatch,
        component_try_into,
        vanilla_components::{
            DAMAGE, EQUIPPABLE, Equippable, EquippableSlot, MAX_DAMAGE, MAX_STACK_SIZE, UNBREAKABLE,
        },
    },
    items::ItemRef,
    vanilla_items::ITEMS,
};

/// A stack of items with a count and component modifications.
#[derive(Debug, Clone)]
pub struct ItemStack {
    /// The item type. AIR represents an empty stack.
    pub item: ItemRef,
    /// The number of items in this stack.
    pub count: i32,
    /// Modifications to the prototype components.
    pub patch: DataComponentPatch,
}

impl Default for ItemStack {
    fn default() -> Self {
        Self::empty()
    }
}

impl ItemStack {
    /// Creates an empty item stack (using AIR).
    #[must_use]
    pub fn empty() -> Self {
        Self {
            item: &ITEMS.air,
            count: 0,
            patch: DataComponentPatch::new(),
        }
    }

    /// Creates a new item stack with count 1.
    #[must_use]
    pub fn new(item: ItemRef) -> Self {
        Self::with_count(item, 1)
    }

    /// Creates a new item stack with the specified count.
    #[must_use]
    pub fn with_count(item: ItemRef, count: i32) -> Self {
        Self {
            item,
            count,
            patch: DataComponentPatch::new(),
        }
    }

    #[must_use]
    pub fn prototype(&self) -> &'static DataComponentMap {
        &self.item.components
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        std::ptr::eq(self.item, &ITEMS.air) || self.count <= 0
    }

    #[must_use]
    pub fn item(&self) -> ItemRef {
        if self.is_empty() {
            &ITEMS.air
        } else {
            self.item
        }
    }

    #[must_use]
    pub fn count(&self) -> i32 {
        if self.is_empty() { 0 } else { self.count }
    }

    pub fn set_count(&mut self, count: i32) {
        self.count = count;
    }

    /// Increases the count by the given amount.
    pub fn grow(&mut self, amount: i32) {
        self.count += amount;
    }

    /// Decreases the count by the given amount.
    pub fn shrink(&mut self, amount: i32) {
        self.count -= amount;
    }

    /// Returns true if this item can stack (max stack size > 1 and not damaged).
    /// Damaged items cannot stack.
    #[must_use]
    pub fn is_stackable(&self) -> bool {
        self.max_stack_size() > 1 && (!self.is_damageable_item() || !self.is_damaged())
    }

    /// Returns true if this item can take damage.
    #[must_use]
    pub fn is_damageable_item(&self) -> bool {
        self.has_component(&MAX_DAMAGE.key)
            && !self.has_component(&UNBREAKABLE.key)
            && self.has_component(&DAMAGE.key)
    }

    /// Returns true if this item has taken damage.
    #[must_use]
    pub fn is_damaged(&self) -> bool {
        self.is_damageable_item() && self.get_damage_value() > 0
    }

    /// Gets the current damage value of this item.
    #[must_use]
    pub fn get_damage_value(&self) -> i32 {
        self.get_effective_value_raw(&DAMAGE.key)
            .map_or(0, |v| component_try_into(v, DAMAGE).copied().unwrap_or(0))
            .clamp(0, self.get_max_damage())
    }

    /// Gets the maximum damage this item can take before breaking.
    #[must_use]
    pub fn get_max_damage(&self) -> i32 {
        self.get_effective_value_raw(&MAX_DAMAGE.key)
            .map_or(0, |v| {
                component_try_into(v, MAX_DAMAGE).copied().unwrap_or(0)
            })
    }

    /// Returns true if this item has the specified component.
    #[must_use]
    pub fn has_component(&self, key: &Identifier) -> bool {
        match self.patch.get_entry(key) {
            Some(ComponentPatchEntry::Set(_)) => true,
            Some(ComponentPatchEntry::Removed) => false,
            None => self.prototype().get_raw(key).is_some(),
        }
    }

    #[must_use]
    pub fn is_same_item(a: &Self, b: &Self) -> bool {
        a.item().key == b.item().key
    }

    /// Checks if two stacks have the same item and components.
    #[must_use]
    pub fn is_same_item_same_components(a: &Self, b: &Self) -> bool {
        if !Self::is_same_item(a, b) {
            return false;
        }
        if a.is_empty() && b.is_empty() {
            return true;
        }
        a.components_equal(b)
    }

    #[must_use]
    pub fn matches(a: &Self, b: &Self) -> bool {
        a.count() == b.count() && Self::is_same_item_same_components(a, b)
    }

    #[must_use]
    pub fn is(&self, item: ItemRef) -> bool {
        self.item().key == item.key
    }

    pub fn max_stack_size(&self) -> i32 {
        self.get_effective_value_raw(&MAX_STACK_SIZE.key)
            .map_or(64, |v| {
                component_try_into(v, MAX_STACK_SIZE).copied().unwrap_or(64)
            })
    }

    /// Returns the equippable component if this item has one.
    #[must_use]
    pub fn get_equippable(&self) -> Option<&Equippable> {
        self.get_effective_value_raw(&EQUIPPABLE.key)
            .and_then(|v| component_try_into(v, EQUIPPABLE))
    }

    /// Returns the equipment slot this item can be equipped to, if any.
    #[must_use]
    pub fn get_equippable_slot(&self) -> Option<EquippableSlot> {
        self.get_equippable().map(|e| e.slot)
    }

    /// Returns true if this item can be equipped in the given slot.
    #[must_use]
    pub fn is_equippable_in_slot(&self, slot: EquippableSlot) -> bool {
        self.get_equippable_slot() == Some(slot)
    }

    pub fn get_effective_value_raw(&self, key: &Identifier) -> Option<&dyn ComponentValue> {
        match self.patch.get_entry(key) {
            Some(ComponentPatchEntry::Set(v)) => Some(v.as_ref()),
            Some(ComponentPatchEntry::Removed) => None,
            None => self.prototype().get_raw(key),
        }
    }

    pub fn components_equal(&self, other: &Self) -> bool {
        let mut all_keys = rustc_hash::FxHashSet::default();

        for key in self.prototype().keys() {
            if !self.patch.is_removed(key) {
                all_keys.insert(key);
            }
        }
        for (key, entry) in self.patch.iter() {
            if matches!(entry, ComponentPatchEntry::Set(_)) {
                all_keys.insert(key);
            }
        }
        for key in other.prototype().keys() {
            if !other.patch.is_removed(key) {
                all_keys.insert(key);
            }
        }
        for (key, entry) in other.patch.iter() {
            if matches!(entry, ComponentPatchEntry::Set(_)) {
                all_keys.insert(key);
            }
        }
        for key in all_keys {
            let val_a = self.get_effective_value_raw(key);
            let val_b = other.get_effective_value_raw(key);

            match (val_a, val_b) {
                (Some(a), Some(b)) => {
                    if !a.eq_value(b) {
                        return false;
                    }
                }
                (None, None) => {}
                _ => return false,
            }
        }

        true
    }
}

impl std::fmt::Display for ItemStack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_empty() {
            write!(f, "Empty")
        } else {
            write!(f, "{} {}", self.count, self.item.key)
        }
    }
}

impl WriteTo for ItemStack {
    fn write(&self, writer: &mut impl Write) -> Result<()> {
        if self.is_empty() {
            VarInt(0).write(writer)?;
        } else {
            VarInt(self.count).write(writer)?;
            // Write item ID as VarInt
            let item_id = *REGISTRY.items.get_id(self.item);
            VarInt(item_id as i32).write(writer)?;
            // Write DataComponentPatch
            self.patch.write(writer)?;
        }
        Ok(())
    }
}

impl ReadFrom for ItemStack {
    fn read(data: &mut impl Read) -> Result<Self> {
        let count = VarInt::read(data)?.0;
        if count <= 0 {
            return Ok(Self::empty());
        }

        let item_id = VarInt::read(data)?.0 as usize;
        let item = REGISTRY.items.by_id(item_id).unwrap_or(&ITEMS.air);

        // Read DataComponentPatch
        let patch = DataComponentPatch::read(data)?;

        Ok(Self { item, count, patch })
    }
}
