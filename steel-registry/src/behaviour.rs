use steel_utils::BlockStateId;

#[derive(Debug, Clone, Copy)]
pub enum MapColor {
    Stone,
    Dirt,
    Wood,
    None,
}

#[derive(Debug, Clone, Copy)]
pub enum SoundType {
    Stone,
    Wood,
    Gravel,
    Grass,
}

#[derive(Debug, Clone, Copy)]
pub enum PushReaction {
    Normal,
    Destroy,
    Block,
    Ignore,
}

#[derive(Debug, Clone, Copy)]
pub enum NoteBlockInstrument {
    Harp,
    Bass,
    Snare,
}

#[derive(Debug, Clone, Copy)]
pub enum OffsetType {
    None,
    Xz,
    Xyz,
}

pub struct BlockBehaviourProperties {
    pub map_color: MapColor,
    pub has_collision: bool,
    pub sound_type: SoundType,
    // A closure to determine light level, potentially based on block state.
    pub light_emission: Box<dyn Fn(BlockStateId) -> u8>,
    pub explosion_resistance: f32,
    pub destroy_time: f32,
    pub requires_correct_tool_for_drops: bool,
    pub is_randomly_ticking: bool,
    pub friction: f32,
    pub speed_factor: f32,
    pub jump_factor: f32,
    pub can_occlude: bool,
    pub is_air: bool,
    pub ignited_by_lava: bool,
    pub push_reaction: PushReaction,
    pub spawn_terrain_particles: bool,
    pub instrument: NoteBlockInstrument,
    pub replaceable: bool,
    pub dynamic_shape: bool,
    pub offset_type: OffsetType,
}

impl Default for BlockBehaviourProperties {
    /// Creates the default set of properties for a block.
    fn default() -> Self {
        Self {
            map_color: MapColor::None,
            has_collision: true,
            sound_type: SoundType::Stone,
            light_emission: Box::new(|_| 0),
            explosion_resistance: 0.0,
            destroy_time: 0.0,
            requires_correct_tool_for_drops: false,
            is_randomly_ticking: false,
            friction: 0.6,
            speed_factor: 1.0,
            jump_factor: 1.0,
            can_occlude: true,
            is_air: false,
            ignited_by_lava: false,
            push_reaction: PushReaction::Normal,
            spawn_terrain_particles: true,
            instrument: NoteBlockInstrument::Harp,
            replaceable: false,
            dynamic_shape: false,
            offset_type: OffsetType::None,
        }
    }
}

impl BlockBehaviourProperties {
    /// Starts building a new set of block properties.
    pub fn new() -> Self {
        Self::default()
    }

    pub fn map_color(mut self, color: MapColor) -> Self {
        self.map_color = color;
        self
    }

    pub fn no_collision(mut self) -> Self {
        self.has_collision = false;
        self.can_occlude = false;
        self
    }

    pub fn no_occlusion(mut self) -> Self {
        self.can_occlude = false;
        self
    }

    pub fn friction(mut self, friction: f32) -> Self {
        self.friction = friction;
        self
    }

    pub fn speed_factor(mut self, factor: f32) -> Self {
        self.speed_factor = factor;
        self
    }

    pub fn jump_factor(mut self, factor: f32) -> Self {
        self.jump_factor = factor;
        self
    }

    pub fn sound(mut self, sound: SoundType) -> Self {
        self.sound_type = sound;
        self
    }

    pub fn light_level(mut self, light_fn: impl Fn(BlockStateId) -> u8 + 'static) -> Self {
        self.light_emission = Box::new(light_fn);
        self
    }

    /// Sets destroy time and explosion resistance.
    pub fn strength(mut self, destroy_time: f32, explosion_resistance: f32) -> Self {
        self.destroy_time = destroy_time;
        self.explosion_resistance = explosion_resistance.max(0.0);
        self
    }

    /// A shortcut for blocks that break instantly.
    pub fn instabreak(self) -> Self {
        self.strength(0.0, 0.0)
    }

    pub fn random_ticks(mut self) -> Self {
        self.is_randomly_ticking = true;
        self
    }

    pub fn dynamic_shape(mut self) -> Self {
        self.dynamic_shape = true;
        self
    }

    pub fn ignited_by_lava(mut self) -> Self {
        self.ignited_by_lava = true;
        self
    }

    pub fn push_reaction(mut self, reaction: PushReaction) -> Self {
        self.push_reaction = reaction;
        self
    }

    pub fn air(mut self) -> Self {
        self.is_air = true;
        self
    }

    pub fn requires_correct_tool_for_drops(mut self) -> Self {
        self.requires_correct_tool_for_drops = true;
        self
    }

    pub fn offset(mut self, offset_type: OffsetType) -> Self {
        self.offset_type = offset_type;
        self
    }

    pub fn no_terrain_particles(mut self) -> Self {
        self.spawn_terrain_particles = false;
        self
    }

    pub fn instrument(mut self, instrument: NoteBlockInstrument) -> Self {
        self.instrument = instrument;
        self
    }

    pub fn replaceable(mut self) -> Self {
        self.replaceable = true;
        self
    }
}
