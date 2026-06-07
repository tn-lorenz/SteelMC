//! Path type and malus state used by vanilla mob pathfinding.

/// Vanilla `PathType`.
///
/// Steel stores per-mob overrides in a fixed array keyed by this enum instead
/// of Java's enum map. The observable path cost result is the same, while the
/// hot path remains cache-local.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum PathType {
    Blocked,
    Open,
    Walkable,
    WalkableDoor,
    Trapdoor,
    PowderSnow,
    OnTopOfPowderSnow,
    Fence,
    Lava,
    Water,
    WaterBorder,
    Rail,
    UnpassableRail,
    FireInNeighbor,
    Fire,
    DamagingInNeighbor,
    Damaging,
    DoorOpen,
    DoorWoodClosed,
    DoorIronClosed,
    Breach,
    Leaves,
    StickyHoney,
    Cocoa,
    DamageCautious,
    OnTopOfTrapdoor,
    BigMobsCloseToDanger,
}

impl PathType {
    pub const COUNT: usize = 27;

    #[must_use]
    pub const fn index(self) -> usize {
        self as usize
    }

    #[must_use]
    #[expect(
        clippy::match_same_arms,
        reason = "one arm per vanilla PathType keeps the default table auditable"
    )]
    pub const fn default_malus(self) -> f32 {
        match self {
            Self::Blocked => -1.0,
            Self::Open => 0.0,
            Self::Walkable => 0.0,
            Self::WalkableDoor => 0.0,
            Self::Trapdoor => 0.0,
            Self::PowderSnow => -1.0,
            Self::OnTopOfPowderSnow => 0.0,
            Self::Fence => -1.0,
            Self::Lava => -1.0,
            Self::Water => 8.0,
            Self::WaterBorder => 8.0,
            Self::Rail => 0.0,
            Self::UnpassableRail => -1.0,
            Self::FireInNeighbor => 8.0,
            Self::Fire => 16.0,
            Self::DamagingInNeighbor => 8.0,
            Self::Damaging => -1.0,
            Self::DoorOpen => 0.0,
            Self::DoorWoodClosed => -1.0,
            Self::DoorIronClosed => -1.0,
            Self::Breach => 4.0,
            Self::Leaves => -1.0,
            Self::StickyHoney => 8.0,
            Self::Cocoa => 0.0,
            Self::DamageCautious => 0.0,
            Self::OnTopOfTrapdoor => 0.0,
            Self::BigMobsCloseToDanger => 4.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PathfindingMalus {
    overrides: [Option<f32>; PathType::COUNT],
}

impl PathfindingMalus {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            overrides: [None; PathType::COUNT],
        }
    }

    #[must_use]
    pub fn get(&self, path_type: PathType) -> f32 {
        self.overrides[path_type.index()].unwrap_or_else(|| path_type.default_malus())
    }

    pub const fn set(&mut self, path_type: PathType, malus: f32) {
        self.overrides[path_type.index()] = Some(malus);
    }
}

impl Default for PathfindingMalus {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::{PathType, PathfindingMalus};

    #[test]
    fn default_malus_matches_vanilla_path_types() {
        assert_eq!(
            PathType::Blocked.default_malus().to_bits(),
            (-1.0_f32).to_bits()
        );
        assert_eq!(
            PathType::Walkable.default_malus().to_bits(),
            0.0_f32.to_bits()
        );
        assert_eq!(PathType::Water.default_malus().to_bits(), 8.0_f32.to_bits());
        assert_eq!(PathType::Fire.default_malus().to_bits(), 16.0_f32.to_bits());
        assert_eq!(
            PathType::BigMobsCloseToDanger.default_malus().to_bits(),
            4.0_f32.to_bits()
        );
    }

    #[test]
    fn malus_overrides_are_indexed_by_path_type() {
        let mut malus = PathfindingMalus::new();
        assert_eq!(malus.get(PathType::Fire).to_bits(), 16.0_f32.to_bits());

        malus.set(PathType::Fire, -1.0);

        assert_eq!(malus.get(PathType::Fire).to_bits(), (-1.0_f32).to_bits());
        assert_eq!(malus.get(PathType::Water).to_bits(), 8.0_f32.to_bits());
    }
}
