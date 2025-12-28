//! Equipment slot definitions for entities.

/// Equipment slot types for categorization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EquipmentSlotType {
    /// Hand slots (main hand, off hand).
    Hand,
    /// Humanoid armor slots (head, chest, legs, feet).
    HumanoidArmor,
    /// Animal armor slot (body).
    AnimalArmor,
    /// Saddle slot.
    Saddle,
}

/// Equipment slots for entities.
///
/// Based on Minecraft's `EquipmentSlot` enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EquipmentSlot {
    /// The main hand slot.
    MainHand,
    /// The off hand slot.
    OffHand,
    /// The feet armor slot (boots).
    Feet,
    /// The legs armor slot (leggings).
    Legs,
    /// The chest armor slot (chestplate).
    Chest,
    /// The head armor slot (helmet).
    Head,
    /// The body armor slot (for animals like horses).
    Body,
    /// The saddle slot (for rideable animals).
    Saddle,
}

impl EquipmentSlot {
    /// All equipment slots in order.
    pub const ALL: [EquipmentSlot; 8] = [
        EquipmentSlot::MainHand,
        EquipmentSlot::OffHand,
        EquipmentSlot::Feet,
        EquipmentSlot::Legs,
        EquipmentSlot::Chest,
        EquipmentSlot::Head,
        EquipmentSlot::Body,
        EquipmentSlot::Saddle,
    ];

    /// Humanoid armor slots (head, chest, legs, feet).
    pub const ARMOR_SLOTS: [EquipmentSlot; 4] = [
        EquipmentSlot::Head,
        EquipmentSlot::Chest,
        EquipmentSlot::Legs,
        EquipmentSlot::Feet,
    ];

    /// Returns the slot type for this equipment slot.
    #[must_use]
    pub const fn slot_type(self) -> EquipmentSlotType {
        match self {
            EquipmentSlot::MainHand | EquipmentSlot::OffHand => EquipmentSlotType::Hand,
            EquipmentSlot::Feet
            | EquipmentSlot::Legs
            | EquipmentSlot::Chest
            | EquipmentSlot::Head => EquipmentSlotType::HumanoidArmor,
            EquipmentSlot::Body => EquipmentSlotType::AnimalArmor,
            EquipmentSlot::Saddle => EquipmentSlotType::Saddle,
        }
    }

    /// Returns the index of this slot for array storage (0-7).
    #[must_use]
    pub const fn index(self) -> usize {
        match self {
            EquipmentSlot::MainHand => 0,
            EquipmentSlot::OffHand => 1,
            EquipmentSlot::Feet => 2,
            EquipmentSlot::Legs => 3,
            EquipmentSlot::Chest => 4,
            EquipmentSlot::Head => 5,
            EquipmentSlot::Body => 6,
            EquipmentSlot::Saddle => 7,
        }
    }

    /// Returns true if this is an armor slot (humanoid or animal).
    #[must_use]
    pub const fn is_armor(self) -> bool {
        matches!(
            self.slot_type(),
            EquipmentSlotType::HumanoidArmor | EquipmentSlotType::AnimalArmor
        )
    }

    /// Returns the equipment slot with the given name, or None if not found.
    #[must_use]
    pub fn by_name(name: &str) -> Option<Self> {
        match name {
            "mainhand" => Some(EquipmentSlot::MainHand),
            "offhand" => Some(EquipmentSlot::OffHand),
            "feet" => Some(EquipmentSlot::Feet),
            "legs" => Some(EquipmentSlot::Legs),
            "chest" => Some(EquipmentSlot::Chest),
            "head" => Some(EquipmentSlot::Head),
            "body" => Some(EquipmentSlot::Body),
            "saddle" => Some(EquipmentSlot::Saddle),
            _ => None,
        }
    }

    /// Returns the name of this equipment slot.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            EquipmentSlot::MainHand => "mainhand",
            EquipmentSlot::OffHand => "offhand",
            EquipmentSlot::Feet => "feet",
            EquipmentSlot::Legs => "legs",
            EquipmentSlot::Chest => "chest",
            EquipmentSlot::Head => "head",
            EquipmentSlot::Body => "body",
            EquipmentSlot::Saddle => "saddle",
        }
    }
}
