//! Pig entity implementation.
//!
//! This is the first concrete pathfinder mob foundation. Goal selectors,
//! breeding, saddle/riding, and loot/leash/home persistence are follow-up
//! systems; this entity owns the vanilla synchronized data, age state, mob
//! flags, living attributes, and shared mob control shell.

use std::str::FromStr;
use std::sync::{Arc, Weak};

use glam::DVec3;
use simdnbt::borrow::NbtCompound as BorrowedNbtCompoundView;
use simdnbt::owned::NbtCompound;
use steel_macros::entity_behavior;
use steel_protocol::packets::game::{AttributeSnapshot, EquipmentSlotItem, SoundSource};
use steel_registry::entity_type::EntityTypeRef;
use steel_registry::pig_sound_variant::{PigAge, PigSoundVariantRef};
use steel_registry::pig_variant::PigVariantRef;
use steel_registry::vanilla_entity_data::PigEntityData;
use steel_registry::vanilla_game_rules::MAX_ENTITY_CRAMMING;
use steel_registry::{
    REGISTRY, RegistryEntry, RegistryExt, vanilla_attributes, vanilla_damage_types,
    vanilla_particle_types, vanilla_pig_sound_variants, vanilla_pig_variants,
};
use steel_utils::locks::SyncMutex;
use steel_utils::random::Random as _;
use steel_utils::{BlockPos, BlockStateId, Identifier};

use crate::entity::ai::goal::WaterAvoidingRandomStrollGoal;
use crate::entity::damage::DamageSource;
use crate::entity::{
    Entity, EntityBase, EntityBaseLoad, EntitySyncedData, LivingEntity, LivingEntityBase, Mob,
    MobBase, MobEffectSyncChange, PathfinderMob, SharedEntity,
};
use crate::physics::MoveResult;
use crate::world::World;

#[derive(Debug)]
struct PigAgeState {
    age: i32,
    forced_age: i32,
}

impl PigAgeState {
    const fn new() -> Self {
        Self {
            age: 0,
            forced_age: 0,
        }
    }
}

/// Vanilla pig entity.
#[entity_behavior(class = "Pig")]
pub struct PigEntity {
    base: EntityBase,
    entity_type: EntityTypeRef,
    living_base: LivingEntityBase,
    mob_base: MobBase,
    entity_data: SyncMutex<PigEntityData>,
    age_state: SyncMutex<PigAgeState>,
}

impl PigEntity {
    /// Creates a new pig entity.
    #[must_use]
    pub fn new(entity_type: EntityTypeRef, id: i32, position: DVec3, world: Weak<World>) -> Self {
        Self::new_with_base(
            EntityBase::new(id, position, entity_type.dimensions, world),
            entity_type,
        )
    }

    /// Creates a pig entity from saved base data.
    #[must_use]
    pub fn from_saved(entity_type: EntityTypeRef, load: EntityBaseLoad) -> Self {
        Self::new_with_base(
            EntityBase::from_load(load, entity_type.dimensions),
            entity_type,
        )
    }

    fn new_with_base(base: EntityBase, entity_type: EntityTypeRef) -> Self {
        let living_base = LivingEntityBase::new(entity_type);
        let mob_base = MobBase::new();
        let mut entity_data = PigEntityData::new();
        living_base.initialize_synced_data(&mut entity_data);
        mob_base
            .goal_selector()
            .lock()
            .add_goal(6, WaterAvoidingRandomStrollGoal::new(1.0));

        Self {
            base,
            entity_type,
            living_base,
            mob_base,
            entity_data: SyncMutex::new(entity_data),
            age_state: SyncMutex::new(PigAgeState::new()),
        }
    }

    /// Returns the vanilla age counter. Negative values are babies.
    #[must_use]
    pub fn get_age(&self) -> i32 {
        self.age_state.lock().age
    }

    /// Sets the vanilla age counter and updates the synchronized baby flag.
    pub fn set_age(&self, age: i32) {
        let boundary_changed = {
            let mut state = self.age_state.lock();
            let old_age = state.age;
            state.age = age;
            old_age < 0 && age >= 0 || old_age >= 0 && age < 0
        };

        if boundary_changed {
            self.entity_data.lock().ageable_mob_mut().baby.set(age < 0);
            // TODO: Refresh dimensions when baby/adult size changes.
        }
    }

    /// Returns whether this pig is a baby.
    #[must_use]
    pub fn is_baby(&self) -> bool {
        self.get_age() < 0
    }

    /// Sets the vanilla baby state using the `AgeableMob` start age.
    pub fn set_baby(&self, baby: bool) {
        self.set_age(if baby { -24_000 } else { 0 });
    }

    /// Returns vanilla `AgeableMob.forcedAge`.
    #[must_use]
    pub fn forced_age(&self) -> i32 {
        self.age_state.lock().forced_age
    }

    /// Sets vanilla `AgeableMob.forcedAge`.
    pub fn set_forced_age(&self, forced_age: i32) {
        self.age_state.lock().forced_age = forced_age;
    }

    /// Returns the synchronized vanilla age-lock flag.
    #[must_use]
    pub fn is_age_locked(&self) -> bool {
        *self.entity_data.lock().ageable_mob().age_locked.get()
    }

    /// Sets the synchronized vanilla age-lock flag.
    pub fn set_age_locked(&self, age_locked: bool) {
        self.entity_data
            .lock()
            .ageable_mob_mut()
            .age_locked
            .set(age_locked);
    }

    /// Returns the current pig variant registry ID stored in synced data.
    #[must_use]
    pub fn variant_id(&self) -> i32 {
        *self.entity_data.lock().variant.get()
    }

    /// Sets the current pig variant by registry entry.
    pub fn set_variant(&self, variant: PigVariantRef) {
        let Some(id) = REGISTRY.pig_variants.id_from_key(&variant.key) else {
            log::error!("pig variant {} is not registered", variant.key);
            return;
        };
        self.set_variant_id_from_usize(id);
    }

    /// Returns the current pig variant, falling back to vanilla's default holder.
    #[must_use]
    pub fn variant(&self) -> PigVariantRef {
        let id = self.variant_id();
        if let Ok(id) = usize::try_from(id)
            && let Some(variant) = REGISTRY.pig_variants.by_id(id)
        {
            return variant;
        }

        &vanilla_pig_variants::TEMPERATE
    }

    /// Returns the current pig sound variant registry ID stored in synced data.
    #[must_use]
    pub fn sound_variant_id(&self) -> i32 {
        *self.entity_data.lock().sound_variant.get()
    }

    /// Sets the current pig sound variant by registry entry.
    pub fn set_sound_variant(&self, sound_variant: PigSoundVariantRef) {
        let Some(id) = REGISTRY.pig_sound_variants.id_from_key(&sound_variant.key) else {
            log::error!("pig sound variant {} is not registered", sound_variant.key);
            return;
        };
        self.set_sound_variant_id_from_usize(id);
    }

    /// Returns the current pig sound variant, falling back to vanilla classic.
    #[must_use]
    pub fn sound_variant(&self) -> PigSoundVariantRef {
        let id = self.sound_variant_id();
        if let Ok(id) = usize::try_from(id)
            && let Some(sound_variant) = REGISTRY.pig_sound_variants.by_id(id)
        {
            return sound_variant;
        }

        &vanilla_pig_sound_variants::CLASSIC
    }

    fn set_variant_id_from_usize(&self, id: usize) {
        let Ok(id) = i32::try_from(id) else {
            log::error!("pig variant id {id} does not fit synced-data i32");
            return;
        };
        self.entity_data.lock().variant.set(id);
    }

    fn set_sound_variant_id_from_usize(&self, id: usize) {
        let Ok(id) = i32::try_from(id) else {
            log::error!("pig sound variant id {id} does not fit synced-data i32");
            return;
        };
        self.entity_data.lock().sound_variant.set(id);
    }

    fn set_variant_by_key(&self, key: &Identifier) {
        if let Some(id) = REGISTRY.pig_variants.id_from_key(key) {
            self.set_variant_id_from_usize(id);
        }
    }

    fn set_sound_variant_by_key(&self, key: &Identifier) {
        if let Some(id) = REGISTRY.pig_sound_variants.id_from_key(key) {
            self.set_sound_variant_id_from_usize(id);
        }
    }

    fn current_sound_set(&self) -> &'static PigAge {
        let sound_variant = self.sound_variant();
        if self.is_baby() {
            &sound_variant.baby_sounds
        } else {
            &sound_variant.adult_sounds
        }
    }

    fn can_age_up(&self) -> bool {
        self.is_baby() && !self.is_age_locked()
    }

    fn tick_ageable_mob(&self) {
        if !Entity::is_alive(self) {
            return;
        }

        let age = self.get_age();
        if self.can_age_up() {
            self.set_age(age + 1);
        } else if age > 0 {
            self.set_age(age - 1);
        }
    }

    fn update_dirty_mob_effect_entity_data(&self) {
        if !self.living_base.take_effects_dirty() {
            return;
        }

        let Some(particle_type_id) = vanilla_particle_types::ENTITY_EFFECT.try_id() else {
            log::error!("vanilla entity_effect particle type is not registered");
            return;
        };
        let Ok(particle_type_id) = i32::try_from(particle_type_id) else {
            log::error!("vanilla entity_effect particle type id does not fit protocol i32");
            return;
        };
        let display = self.living_base.mob_effect_display_state(particle_type_id);

        {
            let mut entity_data = self.entity_data.lock();
            let living = entity_data.living_entity_mut();
            living.effect_particles.set(display.particles);
            living.effect_ambience.set(display.ambient);
        }

        self.entity_data.set_base_invisible_flag(display.invisible);
        self.entity_data
            .set_base_glowing_flag(self.has_glowing_tag() || display.glowing);
    }

    fn push_entities(&self, world: &Arc<World>) {
        if !world.tick_runs_normally() {
            return;
        }

        let pusher = self as &dyn Entity;
        let pushable_entities = world.get_pushable_entities(pusher, &self.bounding_box());
        if pushable_entities.is_empty() {
            return;
        }

        self.apply_entity_cramming_damage(world, &pushable_entities);

        for entity in pushable_entities {
            entity.push_entity(pusher);
        }
    }

    fn apply_entity_cramming_damage(&self, world: &World, pushable_entities: &[SharedEntity]) {
        let max_cramming = world
            .get_game_rule(&MAX_ENTITY_CRAMMING)
            .as_int()
            .unwrap_or(24);

        if max_cramming <= 0 || pushable_entities.len() <= (max_cramming - 1) as usize {
            return;
        }

        let random_roll = self.base.random().lock().next_i32_bounded(4);
        let non_passenger_count = pushable_entities
            .iter()
            .filter(|entity| !entity.is_passenger())
            .count();

        if Self::should_apply_entity_cramming_damage(
            max_cramming,
            pushable_entities.len(),
            non_passenger_count,
            random_roll,
        ) {
            self.hurt(
                &DamageSource::environment(&vanilla_damage_types::CRAMMING),
                6.0,
            );
        }
    }

    const fn should_apply_entity_cramming_damage(
        max_cramming: i32,
        pushable_count: usize,
        non_passenger_count: usize,
        random_roll: i32,
    ) -> bool {
        if max_cramming <= 0 || random_roll != 0 {
            return false;
        }

        let threshold = (max_cramming - 1) as usize;
        pushable_count > threshold && non_passenger_count > threshold
    }
}

impl Entity for PigEntity {
    fn base(&self) -> &EntityBase {
        &self.base
    }

    fn entity_type(&self) -> EntityTypeRef {
        self.entity_type
    }

    fn tick(&self) {
        self.default_tick();
        self.living_base.decrement_invulnerable_time();
        self.tick_mob_effects();

        if self.is_dead_or_dying() {
            // TODO: Add LivingEntity.tickDeath removal once shared death ticking exists.
            self.tick_living_state();
            return;
        }

        if !self.is_removed() {
            self.ai_step();
        }

        self.tick_living_state();
    }

    fn check_despawn(&self) {
        Mob::check_mob_despawn(self);
    }

    fn is_living_entity(&self) -> bool {
        true
    }

    fn as_living_entity(&self) -> Option<&dyn LivingEntity> {
        Some(self)
    }

    fn is_alive(&self) -> bool {
        !self.is_removed() && self.get_health() > 0.0
    }

    fn is_pickable(&self) -> bool {
        !self.is_removed()
    }

    fn is_pushable(&self) -> bool {
        Entity::is_alive(self) && !self.is_spectator() && !self.on_climbable()
    }

    fn is_effective_ai(&self) -> bool {
        self.is_server_driven_movement() && !self.is_no_ai()
    }

    fn get_default_gravity(&self) -> f64 {
        LivingEntity::get_attribute_gravity(self)
    }

    fn can_freeze(&self) -> bool {
        self.default_living_can_freeze()
    }

    fn can_walk_on_powder_snow(&self) -> bool {
        self.default_living_can_walk_on_powder_snow()
    }

    fn synced_data(&self) -> Option<&dyn EntitySyncedData> {
        Some(&self.entity_data)
    }

    fn update_data_before_sync(&self) {
        self.update_dirty_mob_effect_entity_data();
    }

    fn pack_syncable_attributes(&self) -> Vec<AttributeSnapshot> {
        self.attributes().lock().syncable_snapshots()
    }

    fn drain_dirty_syncable_attributes(&self) -> Vec<AttributeSnapshot> {
        self.attributes().lock().drain_dirty_sync()
    }

    fn drain_dirty_mob_effects(&self) -> Vec<MobEffectSyncChange> {
        self.living_base.drain_dirty_mob_effects()
    }

    fn pack_all_equipment(&self) -> Vec<EquipmentSlotItem> {
        self.pack_living_equipment()
    }

    fn drain_dirty_equipment(&self) -> Vec<EquipmentSlotItem> {
        self.drain_dirty_living_equipment()
    }

    fn max_up_step(&self) -> f32 {
        self.attributes()
            .lock()
            .get_value(vanilla_attributes::STEP_HEIGHT)
            .unwrap_or(0.6) as f32
    }

    fn sound_source(&self) -> SoundSource {
        SoundSource::Neutral
    }

    fn play_step_sound(&self, _pos: BlockPos, _block_state: BlockStateId) {
        self.play_sound(self.current_sound_set().step_sound, 0.15, 1.0);
    }

    fn hurt(&self, source: &DamageSource, amount: f32) -> bool {
        LivingEntity::hurt_server(self, source, amount)
    }

    fn save_additional(&self, nbt: &mut NbtCompound) {
        // TODO: Persist mob loot pickup, leash, home, and death-loot data once those foundations exist.
        nbt.insert("LeftHanded", i8::from(self.is_left_handed()));
        if self.is_no_ai() {
            nbt.insert("NoAI", i8::from(true));
        }

        nbt.insert("Age", self.get_age());
        nbt.insert("ForcedAge", self.forced_age());
        nbt.insert("AgeLocked", i8::from(self.is_age_locked()));
        nbt.insert("variant", self.variant().key.to_string());
        nbt.insert("sound_variant", self.sound_variant().key.to_string());
    }

    fn load_additional(&self, nbt: BorrowedNbtCompoundView<'_, '_>) {
        if let Some(left_handed) = nbt.byte("LeftHanded") {
            self.set_left_handed(left_handed != 0);
        }
        if let Some(no_ai) = nbt.byte("NoAI") {
            self.set_no_ai(no_ai != 0);
        }

        self.set_age(nbt.int("Age").unwrap_or(0));
        self.set_forced_age(nbt.int("ForcedAge").unwrap_or(0));
        self.set_age_locked(nbt.byte("AgeLocked").is_some_and(|value| value != 0));

        if let Some(variant) = nbt.string("variant")
            && let Ok(key) = Identifier::from_str(variant.to_str().as_ref())
        {
            self.set_variant_by_key(&key);
        }
        if let Some(sound_variant) = nbt.string("sound_variant")
            && let Ok(key) = Identifier::from_str(sound_variant.to_str().as_ref())
        {
            self.set_sound_variant_by_key(&key);
        }
    }
}

impl LivingEntity for PigEntity {
    fn living_base(&self) -> &LivingEntityBase {
        &self.living_base
    }

    fn get_health(&self) -> f32 {
        *self.entity_data.lock().living_entity().health.get()
    }

    fn set_health(&self, health: f32) {
        let max_health = self.get_max_health();
        let clamped = health.clamp(0.0, max_health);
        self.entity_data
            .lock()
            .living_entity_mut()
            .health
            .set(clamped);
    }

    fn server_ai_step(&self) {
        Mob::mob_server_ai_step(self);
    }

    fn ai_step(&self) -> Option<MoveResult> {
        let result = self.default_ai_step();

        if !self.is_removed() {
            self.apply_effects_from_blocks();
        }
        if !self.is_removed()
            && let Some(world) = self.level()
        {
            self.push_entities(&world);
        }

        self.tick_ageable_mob();
        result
    }
}

impl Mob for PigEntity {
    fn mob_base(&self) -> &MobBase {
        &self.mob_base
    }

    fn tick_goal_selectors(&self) {
        PathfinderMob::tick_pathfinder_goal_selectors(self);
    }

    fn tick_path_navigation(&self) {
        PathfinderMob::tick_pathfinder_path_navigation(self);
    }

    fn remove_when_far_away(&self, _dist_sqr: f64) -> bool {
        false
    }

    fn mob_flags(&self) -> i8 {
        *self.entity_data.lock().mob().mob_flags.get()
    }

    fn set_mob_flags(&self, flags: i8) {
        self.entity_data.lock().mob_mut().mob_flags.set(flags);
    }
}

impl PathfinderMob for PigEntity {}

#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use std::string::ToString;

    use simdnbt::borrow::read_compound as read_borrowed_compound;
    use steel_registry::test_support::init_test_registry;
    use steel_registry::vanilla_entities;

    use crate::entity::ai::navigation::NavigationTickContext;
    use crate::entity::ai::node::Node;
    use crate::entity::ai::path::{Path, PathType};

    use super::*;

    #[test]
    fn pig_initializes_vanilla_living_attributes_and_health() {
        init_test_registry();

        let pig = PigEntity::new(&vanilla_entities::PIG, 1, DVec3::ZERO, Weak::new());

        assert_eq!(pig.get_health().to_bits(), 10.0_f32.to_bits());
        let attributes = pig.attributes().lock();
        assert_eq!(
            attributes
                .required_value(vanilla_attributes::MAX_HEALTH)
                .to_bits(),
            10.0_f64.to_bits()
        );
        assert_eq!(
            attributes
                .required_value(vanilla_attributes::MOVEMENT_SPEED)
                .to_bits(),
            0.25_f64.to_bits()
        );
        assert_eq!(
            attributes
                .required_value(vanilla_attributes::FOLLOW_RANGE)
                .to_bits(),
            16.0_f64.to_bits()
        );
        assert_eq!(
            attributes
                .required_value(vanilla_attributes::TEMPT_RANGE)
                .to_bits(),
            10.0_f64.to_bits()
        );
    }

    #[test]
    fn pig_exposes_living_entity_behavior_without_downcasting() {
        init_test_registry();

        let pig = PigEntity::new(&vanilla_entities::PIG, 1, DVec3::ZERO, Weak::new());
        let entity = &pig as &dyn Entity;

        assert!(entity.is_living_entity());
        let Some(living) = entity.as_living_entity() else {
            panic!("pig should expose living behavior");
        };
        assert_eq!(living.get_health().to_bits(), 10.0_f32.to_bits());
    }

    #[test]
    fn pig_mob_ai_increments_no_action_time() {
        init_test_registry();

        let pig = PigEntity::new(&vanilla_entities::PIG, 1, DVec3::ZERO, Weak::new());

        pig.set_no_action_time(12);
        Mob::mob_server_ai_step(&pig);

        assert_eq!(pig.no_action_time(), 13);
    }

    #[test]
    fn pig_damage_resets_no_action_time() {
        init_test_registry();

        let pig = PigEntity::new(&vanilla_entities::PIG, 1, DVec3::ZERO, Weak::new());
        let source = DamageSource::environment(&vanilla_damage_types::GENERIC);

        pig.set_no_action_time(42);
        assert!(pig.hurt_server(&source, 1.0));

        assert_eq!(pig.no_action_time(), 0);
    }

    #[test]
    fn pig_keeps_vanilla_animal_far_away_persistence() {
        init_test_registry();

        let pig = PigEntity::new(&vanilla_entities::PIG, 1, DVec3::ZERO, Weak::new());

        assert!(!pig.remove_when_far_away(f64::MAX));
    }

    #[test]
    fn pig_registers_vanilla_random_stroll_goal_foundation() {
        init_test_registry();

        let pig = PigEntity::new(&vanilla_entities::PIG, 1, DVec3::ZERO, Weak::new());

        assert_eq!(
            pig.mob_base().goal_selector().lock().available_goal_count(),
            1
        );
    }

    #[test]
    fn pig_path_target_feeds_move_control_forward_input() {
        init_test_registry();

        let pig = PigEntity::new(&vanilla_entities::PIG, 1, DVec3::ZERO, Weak::new());
        let path = Path::new(vec![Node::new(1, 0, 0)], BlockPos::new(1, 0, 0), true);

        assert!(pig.move_to_path(Some(path), 1.0));
        let target = {
            let mut navigation = pig.mob_base().navigation().lock();
            navigation.next_move_target(NavigationTickContext {
                mob_position: pig.position(),
                mob_bounding_box_width: pig.bounding_box().width(),
                mob_speed: pig.get_speed(),
                game_time: 0,
            })
        };
        let Some((target, speed_modifier)) = target else {
            panic!("navigation should provide a move target");
        };

        pig.set_wanted_position(target, speed_modifier);
        Mob::tick_move_control(&pig);

        assert_eq!(pig.get_speed().to_bits(), 0.25_f32.to_bits());
        assert_eq!(pig.travel_input().forward().to_bits(), 0.25_f32.to_bits());
    }

    #[test]
    fn pig_age_updates_synchronized_baby_flag_on_boundary() {
        init_test_registry();

        let pig = PigEntity::new(&vanilla_entities::PIG, 1, DVec3::ZERO, Weak::new());

        pig.set_age(-1);
        assert!(pig.is_baby());
        assert!(*pig.entity_data.lock().ageable_mob().baby.get());

        pig.set_age(0);
        assert!(!pig.is_baby());
        assert!(!*pig.entity_data.lock().ageable_mob().baby.get());
    }

    #[test]
    fn pig_saves_vanilla_mob_age_and_variant_data() {
        init_test_registry();

        let pig = PigEntity::new(&vanilla_entities::PIG, 1, DVec3::ZERO, Weak::new());
        pig.set_no_ai(true);
        pig.set_left_handed(true);
        pig.set_age(-24_000);
        pig.set_forced_age(12);
        pig.set_age_locked(true);
        pig.set_variant(&vanilla_pig_variants::WARM);
        pig.set_sound_variant(&vanilla_pig_sound_variants::BIG);

        let mut nbt = NbtCompound::new();
        pig.save_additional(&mut nbt);

        assert_eq!(nbt.byte("NoAI"), Some(1));
        assert_eq!(nbt.byte("LeftHanded"), Some(1));
        assert_eq!(nbt.int("Age"), Some(-24_000));
        assert_eq!(nbt.int("ForcedAge"), Some(12));
        assert_eq!(nbt.byte("AgeLocked"), Some(1));
        assert_eq!(
            nbt.string("variant").map(ToString::to_string),
            Some("minecraft:warm".to_owned())
        );
        assert_eq!(
            nbt.string("sound_variant").map(ToString::to_string),
            Some("minecraft:big".to_owned())
        );
    }

    #[test]
    fn pig_loads_vanilla_mob_age_and_variant_data() {
        init_test_registry();

        let mut nbt = NbtCompound::new();
        nbt.insert("NoAI", 1_i8);
        nbt.insert("LeftHanded", 1_i8);
        nbt.insert("Age", -24_000_i32);
        nbt.insert("ForcedAge", 12_i32);
        nbt.insert("AgeLocked", 1_i8);
        nbt.insert("variant", "minecraft:cold");
        nbt.insert("sound_variant", "minecraft:mini");

        let mut bytes = Vec::new();
        nbt.write(&mut bytes);
        let borrowed = read_borrowed_compound(&mut Cursor::new(&bytes))
            .unwrap_or_else(|error| panic!("test nbt should reborrow: {error}"));

        let pig = PigEntity::new(&vanilla_entities::PIG, 1, DVec3::ZERO, Weak::new());
        pig.load_additional((&borrowed).into());

        assert!(pig.is_no_ai());
        assert!(pig.is_left_handed());
        assert_eq!(pig.get_age(), -24_000);
        assert_eq!(pig.forced_age(), 12);
        assert!(pig.is_age_locked());
        assert_eq!(pig.variant().key, vanilla_pig_variants::COLD.key);
        assert_eq!(
            pig.sound_variant().key,
            vanilla_pig_sound_variants::MINI.key
        );
    }

    #[test]
    fn pig_uses_vanilla_fire_path_malus_from_mob_base() {
        init_test_registry();

        let pig = PigEntity::new(&vanilla_entities::PIG, 1, DVec3::ZERO, Weak::new());

        assert_eq!(
            pig.get_pathfinding_malus(PathType::FireInNeighbor)
                .to_bits(),
            16.0_f32.to_bits()
        );
        assert_eq!(
            pig.get_pathfinding_malus(PathType::Fire).to_bits(),
            (-1.0_f32).to_bits()
        );
    }
}
