use steel_macros::{ReadFrom, ServerPacket};
use steel_registry::item_stack::ItemStack;

#[derive(ServerPacket, ReadFrom, Clone, Debug)]
pub struct SSetCreativeModeSlot {
    pub slot_num: i16,
    pub item_stack: ItemStack,
}
