use crate::behavior::ItemBehavior;
use steel_macros::item_behavior;

/// literally self-explanatory
#[item_behavior]
pub struct FishingRodItem;

impl ItemBehavior for FishingRodItem {}
