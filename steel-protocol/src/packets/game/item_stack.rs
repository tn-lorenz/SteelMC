//! ItemStack protocol serialization types.
//!
//! There are two wire formats for item stacks:
//!
//! 1. `RawItemStack` - Full item data (used in clientbound packets)
//!    - VarInt count (0 = empty, >0 = has item)
//!    - If count > 0:
//!      - VarInt item_id
//!      - VarInt add_components_count
//!      - VarInt remove_components_count
//!      - For each added: VarInt type_id, component data (NBT)
//!      - For each removed: VarInt type_id
//!
//! 2. `HashedStack` - Hashed item data (used in serverbound packets)
//!    - Optional (bool present)
//!    - If present:
//!      - VarInt item_id
//!      - VarInt count
//!      - HashedPatchMap (map of component type -> hash, set of removed types)

use std::io::{Read, Result, Write};

use rustc_hash::{FxHashMap, FxHashSet};
use steel_registry::{Registry, item_stack::ItemStack};
use steel_utils::{
    codec::VarInt,
    serial::{ReadFrom, WriteTo},
};

// ============================================================================
// RawItemStack - for clientbound packets
// ============================================================================

/// Raw item stack data for network serialization (clientbound).
/// Can be converted to/from ItemStack with registry access.
#[derive(Clone, Debug, Default)]
pub struct RawItemStack {
    /// Item count. 0 means empty.
    pub count: i32,
    /// Item registry ID.
    pub item_id: i32,
    // TODO: Component patches when implemented
}

impl RawItemStack {
    pub fn empty() -> Self {
        Self::default()
    }

    pub fn new(item_id: i32, count: i32) -> Self {
        Self { count, item_id }
    }

    pub fn is_empty(&self) -> bool {
        self.count <= 0
    }

    /// Creates a RawItemStack from an ItemStack using the registry.
    pub fn from_item_stack(stack: &ItemStack, registry: &Registry) -> Self {
        if stack.is_empty() {
            Self::empty()
        } else {
            let item_id = *registry.items.get_id(stack.item()) as i32;
            Self {
                count: stack.count(),
                item_id,
            }
        }
    }

    /// Converts to an ItemStack using the registry.
    pub fn to_item_stack(&self, registry: &Registry) -> Option<ItemStack> {
        if self.is_empty() {
            return Some(ItemStack::empty());
        }

        let item = registry.items.by_id(self.item_id as usize)?;
        Some(ItemStack::with_count(item, self.count))
    }
}

impl WriteTo for RawItemStack {
    fn write(&self, writer: &mut impl Write) -> Result<()> {
        if self.is_empty() {
            VarInt(0).write(writer)?;
        } else {
            VarInt(self.count).write(writer)?;
            VarInt(self.item_id).write(writer)?;
            // TODO: Write component patches when implemented
            VarInt(0).write(writer)?; // add_components_count
            VarInt(0).write(writer)?; // remove_components_count
        }
        Ok(())
    }
}

impl ReadFrom for RawItemStack {
    fn read(reader: &mut impl Read) -> Result<Self> {
        let count = VarInt::read(reader)?.0;
        if count <= 0 {
            return Ok(Self::empty());
        }

        let item_id = VarInt::read(reader)?.0;
        let add_count = VarInt::read(reader)?.0;
        let remove_count = VarInt::read(reader)?.0;

        // TODO: Read component patches when implemented
        // For now, skip over component data
        if add_count > 0 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Unsupported,
                "Component patches not yet supported for reading",
            ));
        }

        for _ in 0..remove_count {
            let _component_type = VarInt::read(reader)?.0;
        }

        Ok(Self { count, item_id })
    }
}

// ============================================================================
// HashedStack - for serverbound packets
// ============================================================================

/// Hashed component patch map.
/// Maps component type IDs to their hashes, plus a set of removed component type IDs.
#[derive(Clone, Debug, Default)]
pub struct HashedPatchMap {
    /// Map of component type ID -> hash value.
    pub added_components: FxHashMap<i32, i32>,
    /// Set of removed component type IDs.
    pub removed_components: FxHashSet<i32>,
}

impl ReadFrom for HashedPatchMap {
    fn read(reader: &mut impl Read) -> Result<Self> {
        let add_count = VarInt::read(reader)?.0 as usize;
        let mut added_components = FxHashMap::default();
        for _ in 0..add_count {
            let type_id = VarInt::read(reader)?.0;
            let hash = i32::read(reader)?;
            added_components.insert(type_id, hash);
        }

        let remove_count = VarInt::read(reader)?.0 as usize;
        let mut removed_components = FxHashSet::default();
        for _ in 0..remove_count {
            let type_id = VarInt::read(reader)?.0;
            removed_components.insert(type_id);
        }

        Ok(Self {
            added_components,
            removed_components,
        })
    }
}

impl WriteTo for HashedPatchMap {
    fn write(&self, writer: &mut impl Write) -> Result<()> {
        VarInt(self.added_components.len() as i32).write(writer)?;
        for (&type_id, &hash) in &self.added_components {
            VarInt(type_id).write(writer)?;
            hash.write(writer)?;
        }

        VarInt(self.removed_components.len() as i32).write(writer)?;
        for &type_id in &self.removed_components {
            VarInt(type_id).write(writer)?;
        }

        Ok(())
    }
}

/// Hashed item stack for serverbound packets.
/// The client sends hashed component data instead of full component data.
#[derive(Clone, Debug, Default)]
pub struct HashedStack {
    /// Whether this stack is present (not empty).
    pub present: bool,
    /// Item registry ID.
    pub item_id: i32,
    /// Item count.
    pub count: i32,
    /// Hashed component patches.
    pub components: HashedPatchMap,
}

impl HashedStack {
    pub fn empty() -> Self {
        Self::default()
    }

    pub fn is_empty(&self) -> bool {
        !self.present
    }
}

impl ReadFrom for HashedStack {
    fn read(reader: &mut impl Read) -> Result<Self> {
        let present = bool::read(reader)?;
        if !present {
            return Ok(Self::empty());
        }

        let item_id = VarInt::read(reader)?.0;
        let count = VarInt::read(reader)?.0;
        let components = HashedPatchMap::read(reader)?;

        Ok(Self {
            present: true,
            item_id,
            count,
            components,
        })
    }
}

impl WriteTo for HashedStack {
    fn write(&self, writer: &mut impl Write) -> Result<()> {
        self.present.write(writer)?;
        if self.present {
            VarInt(self.item_id).write(writer)?;
            VarInt(self.count).write(writer)?;
            self.components.write(writer)?;
        }
        Ok(())
    }
}
