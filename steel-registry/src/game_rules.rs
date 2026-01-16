use std::any::Any;

use rustc_hash::FxHashMap;
use steel_utils::Identifier;

use crate::RegistryExt;

/// Categories for game rules, used for organization in the UI.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameRuleCategory {
    Chat,
    Drops,
    Misc,
    Mobs,
    Player,
    Spawning,
    Updates,
}

/// Trait for game rule value types.
pub trait GameRuleValue: Any + Send + Sync + 'static {
    fn as_any(&self) -> &dyn Any;
}

impl GameRuleValue for bool {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl GameRuleValue for i32 {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// A strongly typed game rule definition.
#[derive(Debug)]
pub struct GameRule<T: GameRuleValue> {
    /// The key/name of the game rule (e.g., "keep_inventory").
    pub key: Identifier,
    /// The category this game rule belongs to.
    pub category: GameRuleCategory,
    /// The default value of this game rule.
    pub default_value: T,
}

/// Type-erased game rule for storage in the registry.
pub trait GameRuleDyn: Send + Sync {
    fn key(&self) -> &Identifier;
    fn category(&self) -> GameRuleCategory;
    fn default_as_any(&self) -> &dyn Any;
}

impl<T: GameRuleValue> GameRuleDyn for GameRule<T> {
    fn key(&self) -> &Identifier {
        &self.key
    }

    fn category(&self) -> GameRuleCategory {
        self.category
    }

    fn default_as_any(&self) -> &dyn Any {
        self.default_value.as_any()
    }
}

pub type GameRuleRef<T> = &'static GameRule<T>;
pub type GameRuleDynRef = &'static dyn GameRuleDyn;

pub struct GameRuleRegistry {
    game_rules_by_id: Vec<GameRuleDynRef>,
    game_rules_by_key: FxHashMap<Identifier, usize>,
    allows_registering: bool,
}

impl GameRuleRegistry {
    #[must_use]
    pub fn new() -> Self {
        Self {
            game_rules_by_id: Vec::new(),
            game_rules_by_key: FxHashMap::default(),
            allows_registering: true,
        }
    }

    pub fn register<T: GameRuleValue>(&mut self, game_rule: GameRuleRef<T>) -> usize {
        assert!(
            self.allows_registering,
            "Cannot register game rules after the registry has been frozen"
        );

        let id = self.game_rules_by_id.len();
        self.game_rules_by_key.insert(game_rule.key.clone(), id);
        self.game_rules_by_id.push(game_rule);
        id
    }

    #[must_use]
    pub fn by_id(&self, id: usize) -> Option<GameRuleDynRef> {
        self.game_rules_by_id.get(id).copied()
    }

    #[must_use]
    pub fn get_id<T: GameRuleValue>(&self, game_rule: GameRuleRef<T>) -> &usize {
        self.game_rules_by_key
            .get(&game_rule.key)
            .expect("Game rule not found")
    }

    #[must_use]
    pub fn by_key(&self, key: &Identifier) -> Option<GameRuleDynRef> {
        self.game_rules_by_key
            .get(key)
            .and_then(|id| self.by_id(*id))
    }

    pub fn iter(&self) -> impl Iterator<Item = (usize, GameRuleDynRef)> + '_ {
        self.game_rules_by_id
            .iter()
            .enumerate()
            .map(|(id, &gr)| (id, gr))
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.game_rules_by_id.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.game_rules_by_id.is_empty()
    }

    /// Gets the ID of a game rule by its key.
    #[must_use]
    pub fn get_id_by_key(&self, key: &Identifier) -> Option<usize> {
        self.game_rules_by_key.get(key).copied()
    }
}

impl RegistryExt for GameRuleRegistry {
    fn freeze(&mut self) {
        self.allows_registering = false;
    }
}

impl Default for GameRuleRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Stores per-world game rule values.
///
/// This is separate from the registry - the registry holds static definitions,
/// while `GameRuleValues` holds the actual mutable values for a specific world.
#[derive(Default)]
pub struct GameRuleValues {
    /// Values stored as type-erased Any, indexed by game rule registry ID.
    values: Vec<Box<dyn Any + Send + Sync>>,
}

impl std::fmt::Debug for GameRuleValues {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GameRuleValues")
            .field("values_count", &self.values.len())
            .finish()
    }
}

impl Clone for GameRuleValues {
    fn clone(&self) -> Self {
        let values = self
            .values
            .iter()
            .map(|v| {
                if let Some(&b) = v.downcast_ref::<bool>() {
                    Box::new(b) as Box<dyn Any + Send + Sync>
                } else if let Some(&i) = v.downcast_ref::<i32>() {
                    Box::new(i) as Box<dyn Any + Send + Sync>
                } else {
                    panic!("Unknown game rule type in clone")
                }
            })
            .collect();
        Self { values }
    }
}

impl GameRuleValues {
    /// Creates a new `GameRuleValues` with all default values from the registry.
    #[must_use]
    pub fn new(registry: &GameRuleRegistry) -> Self {
        let mut values = Vec::with_capacity(registry.len());
        for (_, rule) in registry.iter() {
            // Clone the default value into a boxed Any
            let boxed: Box<dyn Any + Send + Sync> =
                if let Some(&b) = rule.default_as_any().downcast_ref::<bool>() {
                    Box::new(b)
                } else if let Some(&i) = rule.default_as_any().downcast_ref::<i32>() {
                    Box::new(i)
                } else {
                    panic!("Unknown game rule type");
                };
            values.push(boxed);
        }
        Self { values }
    }

    /// Gets the value of a game rule.
    #[must_use]
    pub fn get<T: GameRuleValue + Copy>(
        &self,
        rule: GameRuleRef<T>,
        registry: &GameRuleRegistry,
    ) -> T {
        let id = *registry.get_id(rule);
        *self.values[id]
            .downcast_ref::<T>()
            .expect("Game rule type mismatch")
    }

    /// Sets the value of a game rule.
    pub fn set<T: GameRuleValue>(
        &mut self,
        rule: GameRuleRef<T>,
        value: T,
        registry: &GameRuleRegistry,
    ) {
        let id = *registry.get_id(rule);
        self.values[id] = Box::new(value);
    }

    /// Gets a boolean game rule value by dynamic reference.
    #[must_use]
    pub fn get_bool_dyn(&self, rule: GameRuleDynRef, registry: &GameRuleRegistry) -> Option<bool> {
        let id = registry.get_id_by_key(rule.key())?;
        self.values.get(id)?.downcast_ref::<bool>().copied()
    }

    /// Gets an integer game rule value by dynamic reference.
    #[must_use]
    pub fn get_int_dyn(&self, rule: GameRuleDynRef, registry: &GameRuleRegistry) -> Option<i32> {
        let id = registry.get_id_by_key(rule.key())?;
        self.values.get(id)?.downcast_ref::<i32>().copied()
    }

    /// Sets a boolean game rule value by name.
    pub fn set_bool_by_name(
        &mut self,
        name: &str,
        value: bool,
        registry: &GameRuleRegistry,
    ) -> bool {
        let key = Identifier::vanilla(name.to_string());
        if let Some(id) = registry.get_id_by_key(&key) {
            self.values[id] = Box::new(value);
            true
        } else {
            false
        }
    }

    /// Sets an integer game rule value by name.
    pub fn set_int_by_name(&mut self, name: &str, value: i32, registry: &GameRuleRegistry) -> bool {
        let key = Identifier::vanilla(name.to_string());
        if let Some(id) = registry.get_id_by_key(&key) {
            self.values[id] = Box::new(value);
            true
        } else {
            false
        }
    }
}
