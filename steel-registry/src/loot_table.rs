use rustc_hash::FxHashMap;
use steel_utils::Identifier;

use crate::RegistryExt;

/// The type of loot table, determining when/how it's used.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LootType {
    Block,
    Entity,
    Chest,
    Fishing,
    Gift,
    Archaeology,
    Vault,
    Shearing,
    Equipment,
    Selector,
    EntityInteract,
    BlockInteract,
    Barter,
}

impl LootType {
    pub const fn from_str(s: &str) -> Self {
        if matches!(s, "minecraft:block") {
            Self::Block
        } else if matches!(s, "minecraft:entity") {
            Self::Entity
        } else if matches!(s, "minecraft:chest") {
            Self::Chest
        } else if matches!(s, "minecraft:fishing") {
            Self::Fishing
        } else if matches!(s, "minecraft:gift") {
            Self::Gift
        } else if matches!(s, "minecraft:archaeology") {
            Self::Archaeology
        } else if matches!(s, "minecraft:vault") {
            Self::Vault
        } else if matches!(s, "minecraft:shearing") {
            Self::Shearing
        } else if matches!(s, "minecraft:equipment") {
            Self::Equipment
        } else if matches!(s, "minecraft:selector") {
            Self::Selector
        } else if matches!(s, "minecraft:entity_interact") {
            Self::EntityInteract
        } else if matches!(s, "minecraft:block_interact") {
            Self::BlockInteract
        } else if matches!(s, "minecraft:barter") {
            Self::Barter
        } else {
            panic!("Unknown loot type")
        }
    }
}

/// A number provider that can be constant or random.
#[derive(Debug, Clone, Copy)]
pub enum NumberProvider {
    Constant(f32),
    Uniform { min: f32, max: f32 },
    Binomial { n: i32, p: f32 },
}

impl NumberProvider {
    /// Get a value from this provider using the given RNG.
    pub fn get(&self, rng: &mut impl rand::Rng) -> f32 {
        match self {
            Self::Constant(v) => *v,
            Self::Uniform { min, max } => rng.random_range(*min..=*max),
            Self::Binomial { n, p } => {
                let mut count = 0;
                for _ in 0..*n {
                    if rng.random::<f32>() < *p {
                        count += 1;
                    }
                }
                count as f32
            }
        }
    }

    /// Get the value as an integer.
    pub fn get_int(&self, rng: &mut impl rand::Rng) -> i32 {
        self.get(rng).floor() as i32
    }
}

/// A condition that must be met for a loot entry or pool to apply.
#[derive(Debug, Clone, Copy)]
pub enum LootCondition {
    /// The loot survives explosion damage.
    SurvivesExplosion,
    /// Condition type not yet implemented.
    Unknown,
}

/// A function that modifies loot items.
#[derive(Debug, Clone, Copy)]
pub enum LootFunction {
    /// Set the count of the item.
    SetCount { count: NumberProvider, add: bool },
    /// Function type not yet implemented.
    Unknown,
}

/// A loot table entry that can generate items.
#[derive(Debug, Clone)]
pub enum LootEntry {
    /// Drop a specific item.
    Item {
        name: Identifier,
        weight: i32,
        quality: i32,
        conditions: &'static [LootCondition],
        functions: &'static [LootFunction],
    },
    /// Reference another loot table.
    LootTableRef {
        name: Identifier,
        weight: i32,
        quality: i32,
        conditions: &'static [LootCondition],
        functions: &'static [LootFunction],
    },
    /// Drop items from a tag.
    Tag {
        name: Identifier,
        expand: bool,
        weight: i32,
        quality: i32,
        conditions: &'static [LootCondition],
        functions: &'static [LootFunction],
    },
    /// Try children in order, use first that matches.
    Alternatives {
        children: &'static [LootEntry],
        conditions: &'static [LootCondition],
    },
    /// Use all children.
    Group {
        children: &'static [LootEntry],
        conditions: &'static [LootCondition],
    },
    /// Use children in sequence until one fails.
    Sequence {
        children: &'static [LootEntry],
        conditions: &'static [LootCondition],
    },
    /// Empty entry (no drop).
    Empty {
        weight: i32,
        conditions: &'static [LootCondition],
    },
    /// Dynamic content (e.g., block entity contents).
    Dynamic {
        name: Identifier,
        conditions: &'static [LootCondition],
    },
}

impl LootEntry {
    /// Get the weight of this entry for random selection.
    pub fn weight(&self) -> i32 {
        match self {
            Self::Item { weight, .. } => *weight,
            Self::LootTableRef { weight, .. } => *weight,
            Self::Tag { weight, .. } => *weight,
            Self::Empty { weight, .. } => *weight,
            // Composite entries don't have weight
            Self::Alternatives { .. }
            | Self::Group { .. }
            | Self::Sequence { .. }
            | Self::Dynamic { .. } => 1,
        }
    }

    /// Get the quality modifier for luck-based weight adjustment.
    pub fn quality(&self) -> i32 {
        match self {
            Self::Item { quality, .. } => *quality,
            Self::LootTableRef { quality, .. } => *quality,
            Self::Tag { quality, .. } => *quality,
            _ => 0,
        }
    }

    /// Get the conditions for this entry.
    pub fn conditions(&self) -> &'static [LootCondition] {
        match self {
            Self::Item { conditions, .. } => conditions,
            Self::LootTableRef { conditions, .. } => conditions,
            Self::Tag { conditions, .. } => conditions,
            Self::Alternatives { conditions, .. } => conditions,
            Self::Group { conditions, .. } => conditions,
            Self::Sequence { conditions, .. } => conditions,
            Self::Empty { conditions, .. } => conditions,
            Self::Dynamic { conditions, .. } => conditions,
        }
    }
}

/// A pool of loot entries with roll counts.
#[derive(Debug, Clone)]
pub struct LootPool {
    pub rolls: NumberProvider,
    pub bonus_rolls: f32,
    pub entries: &'static [LootEntry],
    pub conditions: &'static [LootCondition],
}

/// A complete loot table definition.
#[derive(Debug)]
pub struct LootTable {
    pub key: Identifier,
    pub loot_type: LootType,
    pub pools: &'static [LootPool],
    pub random_sequence: Option<Identifier>,
}

pub type LootTableRef = &'static LootTable;

/// Registry for loot tables.
pub struct LootTableRegistry {
    tables_by_id: Vec<LootTableRef>,
    tables_by_key: FxHashMap<Identifier, usize>,
    allows_registering: bool,
}

impl LootTableRegistry {
    #[must_use]
    pub fn new() -> Self {
        Self {
            tables_by_id: Vec::new(),
            tables_by_key: FxHashMap::default(),
            allows_registering: true,
        }
    }

    pub fn register(&mut self, table: LootTableRef) -> usize {
        assert!(
            self.allows_registering,
            "Cannot register loot tables after the registry has been frozen"
        );

        let id = self.tables_by_id.len();
        self.tables_by_key.insert(table.key.clone(), id);
        self.tables_by_id.push(table);
        id
    }

    #[must_use]
    pub fn by_id(&self, id: usize) -> Option<LootTableRef> {
        self.tables_by_id.get(id).copied()
    }

    #[must_use]
    pub fn by_key(&self, key: &Identifier) -> Option<LootTableRef> {
        self.tables_by_key.get(key).and_then(|id| self.by_id(*id))
    }

    pub fn iter(&self) -> impl Iterator<Item = (usize, LootTableRef)> + '_ {
        self.tables_by_id
            .iter()
            .enumerate()
            .map(|(id, &table)| (id, table))
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.tables_by_id.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.tables_by_id.is_empty()
    }
}

impl RegistryExt for LootTableRegistry {
    fn freeze(&mut self) {
        self.allows_registering = false;
    }
}

impl Default for LootTableRegistry {
    fn default() -> Self {
        Self::new()
    }
}
