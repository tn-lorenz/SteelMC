//! Clientbound set equipment packet.

use std::io::{Result, Write};

use steel_macros::ClientPacket;
use steel_registry::{item_stack::ItemStack, packets::play::C_SET_EQUIPMENT};
use steel_utils::{codec::VarInt, serial::WriteTo};

/// Vanilla equipment slot id used by `ClientboundSetEquipmentPacket`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EquipmentSlotId {
    /// Main hand.
    MainHand,
    /// Off hand.
    OffHand,
    /// Feet armor.
    Feet,
    /// Legs armor.
    Legs,
    /// Chest armor.
    Chest,
    /// Head armor.
    Head,
    /// Body armor.
    Body,
    /// Saddle.
    Saddle,
}

impl EquipmentSlotId {
    const CONTINUE_MASK: u8 = 0x80;

    const fn vanilla_id(self) -> u8 {
        match self {
            Self::MainHand => 0,
            Self::OffHand => 1,
            Self::Feet => 2,
            Self::Legs => 3,
            Self::Chest => 4,
            Self::Head => 5,
            Self::Body => 6,
            Self::Saddle => 7,
        }
    }

    const fn packet_id(self, has_next: bool) -> u8 {
        if has_next {
            self.vanilla_id() | Self::CONTINUE_MASK
        } else {
            self.vanilla_id()
        }
    }
}

/// One equipment slot update.
#[derive(Clone, Debug, PartialEq)]
pub struct EquipmentSlotItem {
    /// Slot being updated.
    pub slot: EquipmentSlotId,
    /// New item stack for the slot. Empty stacks clear the slot on the client.
    pub item_stack: ItemStack,
}

/// Updates one or more equipment slots for an entity.
#[derive(ClientPacket, Clone, Debug)]
#[packet_id(Play = C_SET_EQUIPMENT)]
pub struct CSetEquipment {
    /// Entity id whose equipment changed.
    pub entity_id: i32,
    /// Slot updates. Vanilla requires at least one entry when this packet is sent.
    pub slots: Vec<EquipmentSlotItem>,
}

impl CSetEquipment {
    /// Creates a new equipment update packet.
    #[must_use]
    pub fn new(entity_id: i32, slots: Vec<EquipmentSlotItem>) -> Self {
        Self { entity_id, slots }
    }
}

impl WriteTo for CSetEquipment {
    fn write(&self, writer: &mut impl Write) -> Result<()> {
        VarInt(self.entity_id).write(writer)?;
        let last_index = self.slots.len().saturating_sub(1);
        for (index, slot_item) in self.slots.iter().enumerate() {
            writer.write_all(&[slot_item.slot.packet_id(index != last_index)])?;
            slot_item.item_stack.write(writer)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use steel_utils::serial::WriteTo as _;

    use super::*;

    #[test]
    fn equipment_packet_uses_vanilla_continue_bit() {
        let packet = CSetEquipment::new(
            42,
            vec![
                EquipmentSlotItem {
                    slot: EquipmentSlotId::MainHand,
                    item_stack: ItemStack::empty(),
                },
                EquipmentSlotItem {
                    slot: EquipmentSlotId::Head,
                    item_stack: ItemStack::empty(),
                },
            ],
        );
        let mut bytes = Vec::new();

        packet.write(&mut bytes).expect("packet should encode");

        assert_eq!(bytes, vec![42, 0x80, 0, 5, 0]);
    }
}
