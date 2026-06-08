//! Shared vanilla `Animal` state and hooks.

use std::sync::Arc;

use simdnbt::borrow::NbtCompound as BorrowedNbtCompoundView;
use simdnbt::owned::{NbtCompound, NbtTag};
use steel_registry::vanilla_game_rules::MOB_DROPS;
use steel_utils::entity_events::EntityStatus;
use steel_utils::locks::SyncMutex;
use steel_utils::random::Random as _;
use steel_utils::{Identifier, UuidExt};
use uuid::Uuid;

use crate::entity::entities::ExperienceOrbEntity;
use crate::entity::{AgeableMob, ENTITIES, SharedEntity, next_entity_id};
use crate::player::Player;
use crate::world::World;

const PARENT_AGE_AFTER_BREEDING: i32 = 6000;
const IN_LOVE_TIME: i32 = 600;

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

    /// Returns vanilla `Animal.canFallInLove`.
    fn can_fall_in_love(&self) -> bool {
        self.in_love_time() <= 0
    }

    /// Sets vanilla love mode and records the player that caused it.
    fn set_in_love(&self, player: Option<&Player>) {
        self.set_in_love_time(IN_LOVE_TIME);
        if let Some(player) = player {
            self.set_love_cause_uuid(Some(player.gameprofile.id));
        }

        self.broadcast_entity_event(EntityStatus::InLoveHearts);
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

    /// Creates, initializes, and inserts vanilla breeding offspring.
    fn spawn_child_from_breeding(&self, world: &Arc<World>, partner: &dyn Animal) {
        let Some(offspring) = self.get_breed_offspring(world, partner) else {
            return;
        };

        {
            let Some(offspring_animal) = offspring.as_animal() else {
                log::error!(
                    "breeding entity type {} created non-animal offspring",
                    self.entity_type().key
                );
                return;
            };
            offspring_animal.set_baby(true);
            if let Err(error) = offspring_animal.try_set_position(self.position()) {
                log::error!(
                    "failed to position breeding offspring {} at parent {}: {error}",
                    offspring.id(),
                    self.id()
                );
                return;
            }
            offspring_animal.set_rotation((0.0, 0.0));
            offspring_animal.set_old_position_to_current();

            self.finalize_spawn_child_from_breeding(world, partner, Some(offspring_animal));
        }

        if let Err(error) = world.try_add_entity(offspring) {
            log::error!(
                "failed to add breeding offspring for entity {} to world: {error}",
                self.id()
            );
        }
    }

    /// Applies vanilla breeding side effects after offspring creation.
    fn finalize_spawn_child_from_breeding(
        &self,
        world: &Arc<World>,
        partner: &dyn Animal,
        _offspring: Option<&dyn Animal>,
    ) {
        if self
            .love_cause_uuid()
            .or_else(|| partner.love_cause_uuid())
            .is_some()
        {
            // TODO: Award the animals-bred stat and advancement once those foundations exist.
        }

        self.set_age(PARENT_AGE_AFTER_BREEDING);
        partner.set_age(PARENT_AGE_AFTER_BREEDING);
        self.reset_love();
        partner.reset_love();
        self.broadcast_entity_event(EntityStatus::InLoveHearts);

        if world.get_game_rule(&MOB_DROPS).as_bool() == Some(true) {
            let xp = self.base().random().lock().next_i32_bounded(7) + 1;
            ExperienceOrbEntity::award(world, self.position(), xp);
        }
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
