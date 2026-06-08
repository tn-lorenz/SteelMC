//! Shared vanilla `Animal` state and hooks.

use std::sync::Arc;

use simdnbt::borrow::NbtCompound as BorrowedNbtCompoundView;
use simdnbt::owned::{NbtCompound, NbtTag};
use steel_utils::locks::SyncMutex;
use steel_utils::{Identifier, UuidExt};
use uuid::Uuid;

use crate::entity::{AgeableMob, ENTITIES, SharedEntity, next_entity_id};
use crate::world::World;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct AnimalState {
    in_love: i32,
    love_cause: Option<Uuid>,
}

impl AnimalState {
    const fn new() -> Self {
        Self {
            in_love: 0,
            love_cause: None,
        }
    }
}

/// Runtime fields shared by vanilla animals.
#[derive(Debug)]
pub struct AnimalBase {
    state: SyncMutex<AnimalState>,
}

impl AnimalBase {
    /// Creates default animal runtime state.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            state: SyncMutex::new(AnimalState::new()),
        }
    }

    /// Returns vanilla `Animal.inLove`.
    #[must_use]
    pub fn in_love_time(&self) -> i32 {
        self.state.lock().in_love
    }

    /// Sets vanilla `Animal.inLove`.
    pub fn set_in_love_time(&self, in_love: i32) {
        self.state.lock().in_love = in_love;
    }

    /// Decrements vanilla `Animal.inLove` when it is active.
    pub fn tick_in_love_time(&self) {
        let mut state = self.state.lock();
        if state.in_love > 0 {
            state.in_love -= 1;
        }
    }

    /// Returns vanilla `Animal.loveCause` as a persisted UUID.
    #[must_use]
    pub fn love_cause_uuid(&self) -> Option<Uuid> {
        self.state.lock().love_cause
    }

    /// Sets vanilla `Animal.loveCause` as a persisted UUID.
    pub fn set_love_cause_uuid(&self, love_cause: Option<Uuid>) {
        self.state.lock().love_cause = love_cause;
    }
}

impl Default for AnimalBase {
    fn default() -> Self {
        Self::new()
    }
}

/// Vanilla-shaped behavior shared by entities that extend `Animal`.
pub trait Animal: AgeableMob {
    /// Returns shared animal runtime state.
    fn animal_base(&self) -> &AnimalBase;

    /// Returns vanilla `Animal.inLove`.
    fn in_love_time(&self) -> i32 {
        self.animal_base().in_love_time()
    }

    /// Sets vanilla `Animal.inLove`.
    fn set_in_love_time(&self, in_love: i32) {
        self.animal_base().set_in_love_time(in_love);
    }

    /// Returns vanilla `Animal.loveCause` as a persisted UUID.
    fn love_cause_uuid(&self) -> Option<Uuid> {
        self.animal_base().love_cause_uuid()
    }

    /// Sets vanilla `Animal.loveCause` as a persisted UUID.
    fn set_love_cause_uuid(&self, love_cause: Option<Uuid>) {
        self.animal_base().set_love_cause_uuid(love_cause);
    }

    /// Returns vanilla `Animal.isInLove`.
    fn is_in_love(&self) -> bool {
        self.in_love_time() > 0
    }

    /// Resets vanilla love mode without clearing the stored love cause.
    fn reset_love(&self) {
        self.set_in_love_time(0);
    }

    /// Returns vanilla `Animal.canMate`.
    fn can_mate(&self, partner: &dyn Animal) -> bool {
        self.uuid() != partner.uuid()
            && self.entity_type() == partner.entity_type()
            && self.is_in_love()
            && partner.is_in_love()
    }

    /// Creates a same-type offspring using the registered entity factory.
    fn create_breed_offspring(&self, world: &Arc<World>) -> Option<SharedEntity> {
        ENTITIES.create(
            self.entity_type(),
            next_entity_id(),
            self.position(),
            Arc::downgrade(world),
        )
    }

    /// Returns this animal's breedable variant key when offspring inherit it.
    fn breed_variant_key(&self) -> Option<&Identifier> {
        None
    }

    /// Applies a breedable variant key to offspring that inherit one.
    fn set_breed_variant_key(&self, _key: &Identifier) -> bool {
        false
    }

    /// Applies entity-specific state to freshly created breeding offspring.
    fn initialize_breed_offspring(&self, _partner: &dyn Animal, _offspring: &dyn Animal) {}

    /// Creates this animal's vanilla breeding offspring.
    fn get_breed_offspring(
        &self,
        world: &Arc<World>,
        partner: &dyn Animal,
    ) -> Option<SharedEntity> {
        let offspring = self.create_breed_offspring(world)?;
        let Some(offspring_animal) = offspring.as_animal() else {
            log::error!(
                "breeding entity type {} created non-animal offspring",
                self.entity_type().key
            );
            return None;
        };

        self.initialize_breed_offspring(partner, offspring_animal);
        Some(offspring)
    }

    /// Ticks vanilla animal love state.
    fn tick_animal_love(&self) {
        if self.get_age() != 0 {
            self.reset_love();
            return;
        }

        self.animal_base().tick_in_love_time();
        // TODO: Spawn in-love heart particles every 10 ticks once particle spawning exists.
    }

    /// Runs vanilla `Animal.customServerAiStep`.
    fn custom_server_ai_step_animal(&self) {
        if self.get_age() != 0 {
            self.reset_love();
        }
    }

    /// Returns vanilla animal far-away despawn behavior.
    fn remove_when_far_away_animal(&self, _dist_sqr: f64) -> bool {
        false
    }

    /// Saves vanilla animal fields.
    fn save_animal(&self, nbt: &mut NbtCompound) {
        nbt.insert("InLove", self.in_love_time());
        if let Some(love_cause) = self.love_cause_uuid() {
            nbt.insert(
                "LoveCause",
                NbtTag::IntArray(love_cause.to_int_array().to_vec()),
            );
        }
    }

    /// Loads vanilla animal fields.
    fn load_animal(&self, nbt: BorrowedNbtCompoundView<'_, '_>) {
        self.set_in_love_time(nbt.int("InLove").unwrap_or(0));
        if let Some(love_cause) = nbt.int_array("LoveCause")
            && let Some(uuid) = Uuid::from_int_array(&love_cause)
        {
            self.set_love_cause_uuid(Some(uuid));
        }
    }
}
