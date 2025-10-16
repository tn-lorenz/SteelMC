use std::fmt::Debug;

pub trait Property<T> {
    fn get_value(&self, value: &str) -> Option<T>;
    fn get_possible_values(&self) -> Vec<T>;
    fn get_internal_index(&self, value: &T) -> usize;
    fn value_from_index(&self, index: usize) -> T;
    fn as_dyn(&self) -> &dyn DynProperty;
}

pub trait DynProperty: Debug {
    fn get_possible_values(&self) -> Vec<String>;
    fn get_name(&self) -> &'static str;
}

#[derive(Debug, Clone)]
pub struct BooleanProperty {
    pub name: &'static str,
}
impl BooleanProperty {
    pub const fn new(name: &'static str) -> Self {
        Self { name }
    }
}

impl DynProperty for BooleanProperty {
    fn get_possible_values(&self) -> Vec<String> {
        vec!["true".to_string(), "false".to_string()]
    }

    fn get_name(&self) -> &'static str {
        self.name
    }
}

impl Property<bool> for BooleanProperty {
    fn get_value(&self, value: &str) -> Option<bool> {
        if value == "true" {
            Some(true)
        } else if value == "false" {
            Some(false)
        } else {
            None
        }
    }

    fn get_possible_values(&self) -> Vec<bool> {
        vec![true, false]
    }

    fn get_internal_index(&self, value: &bool) -> usize {
        if *value { 0 } else { 1 }
    }

    fn value_from_index(&self, index: usize) -> bool {
        index == 0
    }

    fn as_dyn(&self) -> &dyn DynProperty {
        self
    }
}

#[derive(Debug, Clone)]
pub struct IntProperty {
    pub min: u8,
    pub max: u8,
    pub name: &'static str,
}

impl IntProperty {
    pub const fn new(name: &'static str, min: u8, max: u8) -> Self {
        Self { name, min, max }
    }
}

impl DynProperty for IntProperty {
    fn get_possible_values(&self) -> Vec<String> {
        (self.min..=self.max).map(|v| v.to_string()).collect()
    }

    fn get_name(&self) -> &'static str {
        self.name
    }
}

impl Property<u8> for IntProperty {
    fn get_value(&self, value: &str) -> Option<u8> {
        value
            .parse()
            .ok()
            .filter(|v| v >= &self.min && v <= &self.max)
    }

    fn get_possible_values(&self) -> Vec<u8> {
        (self.min..=self.max).collect()
    }

    fn get_internal_index(&self, value: &u8) -> usize {
        return if *value <= self.max {
            (*value - self.min) as usize
        } else {
            0
        };
    }

    fn value_from_index(&self, index: usize) -> u8 {
        self.min + (index as u8)
    }

    fn as_dyn(&self) -> &dyn DynProperty {
        self
    }
}

#[derive(Debug, Clone)]
pub struct EnumProperty<T: ToString + PartialEq + Clone + Debug + 'static> {
    pub name: &'static str,
    pub possible_values: &'static [T],
}

impl<T: ToString + PartialEq + Clone + Debug + 'static> DynProperty for EnumProperty<T> {
    fn get_possible_values(&self) -> Vec<String> {
        self.possible_values.iter().map(|v| v.to_string()).collect()
    }

    fn get_name(&self) -> &'static str {
        self.name
    }
}

impl<T: ToString + PartialEq + Clone + Debug> EnumProperty<T> {
    pub const fn new(name: &'static str, possible_values: &'static [T]) -> Self {
        Self {
            name,
            possible_values,
        }
    }
}

impl<T: ToString + PartialEq + Clone + Debug> Property<T> for EnumProperty<T> {
    fn get_value(&self, value: &str) -> Option<T> {
        self.possible_values
            .iter()
            .find(|v| v.to_string() == value)
            .cloned()
    }

    fn get_possible_values(&self) -> Vec<T> {
        self.possible_values.to_vec()
    }

    fn get_internal_index(&self, value: &T) -> usize {
        self.possible_values
            .iter()
            .position(|v| v == value)
            .unwrap()
    }

    fn value_from_index(&self, index: usize) -> T {
        self.possible_values[index].clone()
    }

    fn as_dyn(&self) -> &dyn DynProperty {
        self
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl ToString for Axis {
    fn to_string(&self) -> String {
        match self {
            Axis::X => "x".to_string(),
            Axis::Y => "y".to_string(),
            Axis::Z => "z".to_string(),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum Direction {
    Down,
    Up,
    North,
    South,
    West,
    East,
}

impl ToString for Direction {
    fn to_string(&self) -> String {
        match self {
            Direction::Down => "down".to_string(),
            Direction::Up => "up".to_string(),
            Direction::North => "north".to_string(),
            Direction::South => "south".to_string(),
            Direction::West => "west".to_string(),
            Direction::East => "east".to_string(),
        }
    }
}

// Additional enum types for properties
#[derive(Clone, PartialEq, Debug)]
pub enum FrontAndTop {
    NorthUp,
    EastUp,
    SouthUp,
    WestUp,
    UpNorth,
    UpEast,
    UpSouth,
    UpWest,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
}

impl ToString for FrontAndTop {
    fn to_string(&self) -> String {
        match self {
            FrontAndTop::NorthUp => "north_up".to_string(),
            FrontAndTop::EastUp => "east_up".to_string(),
            FrontAndTop::SouthUp => "south_up".to_string(),
            FrontAndTop::WestUp => "west_up".to_string(),
            FrontAndTop::UpNorth => "up_north".to_string(),
            FrontAndTop::UpEast => "up_east".to_string(),
            FrontAndTop::UpSouth => "up_south".to_string(),
            FrontAndTop::UpWest => "up_west".to_string(),
            FrontAndTop::NorthEast => "north_east".to_string(),
            FrontAndTop::NorthWest => "north_west".to_string(),
            FrontAndTop::SouthEast => "south_east".to_string(),
            FrontAndTop::SouthWest => "south_west".to_string(),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum AttachFace {
    Floor,
    Wall,
    Ceiling,
}

impl ToString for AttachFace {
    fn to_string(&self) -> String {
        match self {
            AttachFace::Floor => "floor".to_string(),
            AttachFace::Wall => "wall".to_string(),
            AttachFace::Ceiling => "ceiling".to_string(),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum BellAttachType {
    Floor,
    Ceiling,
    SingleWall,
    DoubleWall,
}

impl ToString for BellAttachType {
    fn to_string(&self) -> String {
        match self {
            BellAttachType::Floor => "floor".to_string(),
            BellAttachType::Ceiling => "ceiling".to_string(),
            BellAttachType::SingleWall => "single_wall".to_string(),
            BellAttachType::DoubleWall => "double_wall".to_string(),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum WallSide {
    None,
    Low,
    Tall,
}

impl ToString for WallSide {
    fn to_string(&self) -> String {
        match self {
            WallSide::None => "none".to_string(),
            WallSide::Low => "low".to_string(),
            WallSide::Tall => "tall".to_string(),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum RedstoneSide {
    None,
    Side,
    Up,
}

impl ToString for RedstoneSide {
    fn to_string(&self) -> String {
        match self {
            RedstoneSide::None => "none".to_string(),
            RedstoneSide::Side => "side".to_string(),
            RedstoneSide::Up => "up".to_string(),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum DoubleBlockHalf {
    Upper,
    Lower,
}

impl ToString for DoubleBlockHalf {
    fn to_string(&self) -> String {
        match self {
            DoubleBlockHalf::Upper => "upper".to_string(),
            DoubleBlockHalf::Lower => "lower".to_string(),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum Half {
    Top,
    Bottom,
}

impl ToString for Half {
    fn to_string(&self) -> String {
        match self {
            Half::Top => "top".to_string(),
            Half::Bottom => "bottom".to_string(),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum SideChainPart {
    None,
    Left,
    Right,
}

impl ToString for SideChainPart {
    fn to_string(&self) -> String {
        match self {
            SideChainPart::None => "none".to_string(),
            SideChainPart::Left => "left".to_string(),
            SideChainPart::Right => "right".to_string(),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum RailShape {
    NorthSouth,
    EastWest,
    AscendingEast,
    AscendingWest,
    AscendingNorth,
    AscendingSouth,
    SouthEast,
    SouthWest,
    NorthWest,
    NorthEast,
}

impl ToString for RailShape {
    fn to_string(&self) -> String {
        match self {
            RailShape::NorthSouth => "north_south".to_string(),
            RailShape::EastWest => "east_west".to_string(),
            RailShape::AscendingEast => "ascending_east".to_string(),
            RailShape::AscendingWest => "ascending_west".to_string(),
            RailShape::AscendingNorth => "ascending_north".to_string(),
            RailShape::AscendingSouth => "ascending_south".to_string(),
            RailShape::SouthEast => "south_east".to_string(),
            RailShape::SouthWest => "south_west".to_string(),
            RailShape::NorthWest => "north_west".to_string(),
            RailShape::NorthEast => "north_east".to_string(),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum BedPart {
    Head,
    Foot,
}

impl ToString for BedPart {
    fn to_string(&self) -> String {
        match self {
            BedPart::Head => "head".to_string(),
            BedPart::Foot => "foot".to_string(),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum ChestType {
    Single,
    Left,
    Right,
}

impl ToString for ChestType {
    fn to_string(&self) -> String {
        match self {
            ChestType::Single => "single".to_string(),
            ChestType::Left => "left".to_string(),
            ChestType::Right => "right".to_string(),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum ComparatorMode {
    Compare,
    Subtract,
}

impl ToString for ComparatorMode {
    fn to_string(&self) -> String {
        match self {
            ComparatorMode::Compare => "compare".to_string(),
            ComparatorMode::Subtract => "subtract".to_string(),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum DoorHingeSide {
    Left,
    Right,
}

impl ToString for DoorHingeSide {
    fn to_string(&self) -> String {
        match self {
            DoorHingeSide::Left => "left".to_string(),
            DoorHingeSide::Right => "right".to_string(),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum NoteBlockInstrument {
    Harp,
    Basedrum,
    Snare,
    Hat,
    Bass,
    Flute,
    Bell,
    Guitar,
    Chime,
    Xylophone,
    IronXylophone,
    CowBell,
    Didgeridoo,
    Bit,
    Banjo,
    Pling,
    Zombie,
    Skeleton,
    Creeper,
    Dragon,
    WitherSkeleton,
    Piglin,
    CustomHead,
}

impl ToString for NoteBlockInstrument {
    fn to_string(&self) -> String {
        match self {
            NoteBlockInstrument::Harp => "harp".to_string(),
            NoteBlockInstrument::Basedrum => "basedrum".to_string(),
            NoteBlockInstrument::Snare => "snare".to_string(),
            NoteBlockInstrument::Hat => "hat".to_string(),
            NoteBlockInstrument::Bass => "bass".to_string(),
            NoteBlockInstrument::Flute => "flute".to_string(),
            NoteBlockInstrument::Bell => "bell".to_string(),
            NoteBlockInstrument::Guitar => "guitar".to_string(),
            NoteBlockInstrument::Chime => "chime".to_string(),
            NoteBlockInstrument::Xylophone => "xylophone".to_string(),
            NoteBlockInstrument::IronXylophone => "iron_xylophone".to_string(),
            NoteBlockInstrument::CowBell => "cow_bell".to_string(),
            NoteBlockInstrument::Didgeridoo => "didgeridoo".to_string(),
            NoteBlockInstrument::Bit => "bit".to_string(),
            NoteBlockInstrument::Banjo => "banjo".to_string(),
            NoteBlockInstrument::Pling => "pling".to_string(),
            NoteBlockInstrument::Zombie => "zombie".to_string(),
            NoteBlockInstrument::Skeleton => "skeleton".to_string(),
            NoteBlockInstrument::Creeper => "creeper".to_string(),
            NoteBlockInstrument::Dragon => "dragon".to_string(),
            NoteBlockInstrument::WitherSkeleton => "wither_skeleton".to_string(),
            NoteBlockInstrument::Piglin => "piglin".to_string(),
            NoteBlockInstrument::CustomHead => "custom_head".to_string(),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum PistonType {
    Default,
    Sticky,
}

impl ToString for PistonType {
    fn to_string(&self) -> String {
        match self {
            PistonType::Default => "default".to_string(),
            PistonType::Sticky => "sticky".to_string(),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum SlabType {
    Bottom,
    Top,
    Double,
}

impl ToString for SlabType {
    fn to_string(&self) -> String {
        match self {
            SlabType::Bottom => "bottom".to_string(),
            SlabType::Top => "top".to_string(),
            SlabType::Double => "double".to_string(),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum StairsShape {
    Straight,
    InnerLeft,
    InnerRight,
    OuterLeft,
    OuterRight,
}

impl ToString for StairsShape {
    fn to_string(&self) -> String {
        match self {
            StairsShape::Straight => "straight".to_string(),
            StairsShape::InnerLeft => "inner_left".to_string(),
            StairsShape::InnerRight => "inner_right".to_string(),
            StairsShape::OuterLeft => "outer_left".to_string(),
            StairsShape::OuterRight => "outer_right".to_string(),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum StructureMode {
    Save,
    Load,
    Corner,
    Data,
}

impl ToString for StructureMode {
    fn to_string(&self) -> String {
        match self {
            StructureMode::Save => "save".to_string(),
            StructureMode::Load => "load".to_string(),
            StructureMode::Corner => "corner".to_string(),
            StructureMode::Data => "data".to_string(),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum BambooLeaves {
    None,
    Small,
    Large,
}

impl ToString for BambooLeaves {
    fn to_string(&self) -> String {
        match self {
            BambooLeaves::None => "none".to_string(),
            BambooLeaves::Small => "small".to_string(),
            BambooLeaves::Large => "large".to_string(),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum Tilt {
    None,
    Unstable,
    Partial,
    Full,
}

impl ToString for Tilt {
    fn to_string(&self) -> String {
        match self {
            Tilt::None => "none".to_string(),
            Tilt::Unstable => "unstable".to_string(),
            Tilt::Partial => "partial".to_string(),
            Tilt::Full => "full".to_string(),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum DripstoneThickness {
    TipMerge,
    Tip,
    Frustum,
    Middle,
    Base,
}

impl ToString for DripstoneThickness {
    fn to_string(&self) -> String {
        match self {
            DripstoneThickness::TipMerge => "tip_merge".to_string(),
            DripstoneThickness::Tip => "tip".to_string(),
            DripstoneThickness::Frustum => "frustum".to_string(),
            DripstoneThickness::Middle => "middle".to_string(),
            DripstoneThickness::Base => "base".to_string(),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum SculkSensorPhase {
    Inactive,
    Active,
    Cooldown,
}

impl ToString for SculkSensorPhase {
    fn to_string(&self) -> String {
        match self {
            SculkSensorPhase::Inactive => "inactive".to_string(),
            SculkSensorPhase::Active => "active".to_string(),
            SculkSensorPhase::Cooldown => "cooldown".to_string(),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum TrialSpawnerState {
    Inactive,
    WaitingForPlayers,
    Active,
    WaitingForRewardEjection,
    EjectingReward,
    Cooldown,
}

impl ToString for TrialSpawnerState {
    fn to_string(&self) -> String {
        match self {
            TrialSpawnerState::Inactive => "inactive".to_string(),
            TrialSpawnerState::WaitingForPlayers => "waiting_for_players".to_string(),
            TrialSpawnerState::Active => "active".to_string(),
            TrialSpawnerState::WaitingForRewardEjection => {
                "waiting_for_reward_ejection".to_string()
            }
            TrialSpawnerState::EjectingReward => "ejecting_reward".to_string(),
            TrialSpawnerState::Cooldown => "cooldown".to_string(),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum VaultState {
    Inactive,
    Active,
    Unlocking,
    Ejecting,
}

impl ToString for VaultState {
    fn to_string(&self) -> String {
        match self {
            VaultState::Inactive => "inactive".to_string(),
            VaultState::Active => "active".to_string(),
            VaultState::Unlocking => "unlocking".to_string(),
            VaultState::Ejecting => "ejecting".to_string(),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum CreakingHeartState {
    Inactive,
    Active,
}

impl ToString for CreakingHeartState {
    fn to_string(&self) -> String {
        match self {
            CreakingHeartState::Inactive => "inactive".to_string(),
            CreakingHeartState::Active => "active".to_string(),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum TestBlockMode {
    Off,
    On,
}

impl ToString for TestBlockMode {
    fn to_string(&self) -> String {
        match self {
            TestBlockMode::Off => "off".to_string(),
            TestBlockMode::On => "on".to_string(),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum CopperGolemPose {
    Standing,
    Sitting,
    Running,
    Star,
}

impl ToString for CopperGolemPose {
    fn to_string(&self) -> String {
        match self {
            CopperGolemPose::Standing => "standing".to_string(),
            CopperGolemPose::Sitting => "sitting".to_string(),
            CopperGolemPose::Running => "running".to_string(),
            CopperGolemPose::Star => "star".to_string(),
        }
    }
}

pub struct BlockStateProperties;

//TODO: These got quickly implemented so the ordering might be off. Fix in the future.
impl BlockStateProperties {
    pub const ATTACHED: BooleanProperty = BooleanProperty::new("attached");
    pub const BERRIES: BooleanProperty = BooleanProperty::new("berries");
    pub const BLOOM: BooleanProperty = BooleanProperty::new("bloom");
    pub const BOTTOM: BooleanProperty = BooleanProperty::new("bottom");
    pub const CAN_SUMMON: BooleanProperty = BooleanProperty::new("can_summon");
    pub const CONDITIONAL: BooleanProperty = BooleanProperty::new("conditional");
    pub const DISARMED: BooleanProperty = BooleanProperty::new("disarmed");
    pub const DRAG: BooleanProperty = BooleanProperty::new("drag");
    pub const ENABLED: BooleanProperty = BooleanProperty::new("enabled");
    pub const EXTENDED: BooleanProperty = BooleanProperty::new("extended");
    pub const EYE: BooleanProperty = BooleanProperty::new("eye");
    pub const FALLING: BooleanProperty = BooleanProperty::new("falling");
    pub const HANGING: BooleanProperty = BooleanProperty::new("hanging");
    pub const HAS_BOTTLE_0: BooleanProperty = BooleanProperty::new("has_bottle_0");
    pub const HAS_BOTTLE_1: BooleanProperty = BooleanProperty::new("has_bottle_1");
    pub const HAS_BOTTLE_2: BooleanProperty = BooleanProperty::new("has_bottle_2");
    pub const HAS_RECORD: BooleanProperty = BooleanProperty::new("has_record");
    pub const HAS_BOOK: BooleanProperty = BooleanProperty::new("has_book");
    pub const INVERTED: BooleanProperty = BooleanProperty::new("inverted");
    pub const IN_WALL: BooleanProperty = BooleanProperty::new("in_wall");
    pub const LIT: BooleanProperty = BooleanProperty::new("lit");
    pub const LOCKED: BooleanProperty = BooleanProperty::new("locked");
    pub const NATURAL: BooleanProperty = BooleanProperty::new("natural");
    pub const OCCUPIED: BooleanProperty = BooleanProperty::new("occupied");
    pub const OPEN: BooleanProperty = BooleanProperty::new("open");
    pub const PERSISTENT: BooleanProperty = BooleanProperty::new("persistent");
    pub const POWERED: BooleanProperty = BooleanProperty::new("powered");
    pub const SHORT: BooleanProperty = BooleanProperty::new("short");
    pub const SHRIEKING: BooleanProperty = BooleanProperty::new("shrieking");
    pub const SIGNAL_FIRE: BooleanProperty = BooleanProperty::new("signal_fire");
    pub const SNOWY: BooleanProperty = BooleanProperty::new("snowy");
    pub const TIP: BooleanProperty = BooleanProperty::new("tip");
    pub const TRIGGERED: BooleanProperty = BooleanProperty::new("triggered");
    pub const UNSTABLE: BooleanProperty = BooleanProperty::new("unstable");
    pub const WATERLOGGED: BooleanProperty = BooleanProperty::new("waterlogged");
    pub const HORIZONTAL_AXIS: EnumProperty<Axis> = EnumProperty::new("axis", &[Axis::X, Axis::Z]);
    pub const AXIS: EnumProperty<Axis> = EnumProperty::new("axis", &[Axis::X, Axis::Y, Axis::Z]);
    pub const UP: BooleanProperty = BooleanProperty::new("up");
    pub const DOWN: BooleanProperty = BooleanProperty::new("down");
    pub const NORTH: BooleanProperty = BooleanProperty::new("north");
    pub const EAST: BooleanProperty = BooleanProperty::new("east");
    pub const SOUTH: BooleanProperty = BooleanProperty::new("south");
    pub const WEST: BooleanProperty = BooleanProperty::new("west");
    pub const FACING: EnumProperty<Direction> = EnumProperty::new(
        "facing",
        &[
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
            Direction::Up,
            Direction::Down,
        ],
    );
    pub const FACING_HOPPER: EnumProperty<Direction> = EnumProperty::new(
        "facing",
        &[
            Direction::Down,
            Direction::North,
            Direction::South,
            Direction::West,
            Direction::East,
        ],
    );
    pub const HORIZONTAL_FACING: EnumProperty<Direction> = EnumProperty::new(
        "facing",
        &[
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ],
    );
    pub const FLOWER_AMOUNT: IntProperty = IntProperty::new("flower_amount", 1, 4);
    pub const SEGMENT_AMOUNT: IntProperty = IntProperty::new("segment_amount", 1, 4);

    // Additional enum types needed for properties
    pub const ORIENTATION: EnumProperty<FrontAndTop> = EnumProperty::new(
        "orientation",
        &[
            FrontAndTop::NorthUp,
            FrontAndTop::EastUp,
            FrontAndTop::SouthUp,
            FrontAndTop::WestUp,
            FrontAndTop::UpNorth,
            FrontAndTop::UpEast,
            FrontAndTop::UpSouth,
            FrontAndTop::UpWest,
            FrontAndTop::NorthEast,
            FrontAndTop::NorthWest,
            FrontAndTop::SouthEast,
            FrontAndTop::SouthWest,
        ],
    );
    pub const ATTACH_FACE: EnumProperty<AttachFace> = EnumProperty::new(
        "face",
        &[AttachFace::Floor, AttachFace::Wall, AttachFace::Ceiling],
    );
    pub const BELL_ATTACHMENT: EnumProperty<BellAttachType> = EnumProperty::new(
        "attachment",
        &[
            BellAttachType::Floor,
            BellAttachType::Ceiling,
            BellAttachType::SingleWall,
            BellAttachType::DoubleWall,
        ],
    );
    pub const EAST_WALL: EnumProperty<WallSide> =
        EnumProperty::new("east", &[WallSide::None, WallSide::Low, WallSide::Tall]);
    pub const NORTH_WALL: EnumProperty<WallSide> =
        EnumProperty::new("north", &[WallSide::None, WallSide::Low, WallSide::Tall]);
    pub const SOUTH_WALL: EnumProperty<WallSide> =
        EnumProperty::new("south", &[WallSide::None, WallSide::Low, WallSide::Tall]);
    pub const WEST_WALL: EnumProperty<WallSide> =
        EnumProperty::new("west", &[WallSide::None, WallSide::Low, WallSide::Tall]);
    pub const EAST_REDSTONE: EnumProperty<RedstoneSide> = EnumProperty::new(
        "east",
        &[RedstoneSide::None, RedstoneSide::Side, RedstoneSide::Up],
    );
    pub const NORTH_REDSTONE: EnumProperty<RedstoneSide> = EnumProperty::new(
        "north",
        &[RedstoneSide::None, RedstoneSide::Side, RedstoneSide::Up],
    );
    pub const SOUTH_REDSTONE: EnumProperty<RedstoneSide> = EnumProperty::new(
        "south",
        &[RedstoneSide::None, RedstoneSide::Side, RedstoneSide::Up],
    );
    pub const WEST_REDSTONE: EnumProperty<RedstoneSide> = EnumProperty::new(
        "west",
        &[RedstoneSide::None, RedstoneSide::Side, RedstoneSide::Up],
    );
    pub const DOUBLE_BLOCK_HALF: EnumProperty<DoubleBlockHalf> =
        EnumProperty::new("half", &[DoubleBlockHalf::Upper, DoubleBlockHalf::Lower]);
    pub const HALF: EnumProperty<Half> = EnumProperty::new("half", &[Half::Top, Half::Bottom]);
    pub const SIDE_CHAIN_PART: EnumProperty<SideChainPart> = EnumProperty::new(
        "side_chain",
        &[
            SideChainPart::None,
            SideChainPart::Left,
            SideChainPart::Right,
        ],
    );
    pub const RAIL_SHAPE: EnumProperty<RailShape> = EnumProperty::new(
        "shape",
        &[
            RailShape::NorthSouth,
            RailShape::EastWest,
            RailShape::AscendingEast,
            RailShape::AscendingWest,
            RailShape::AscendingNorth,
            RailShape::AscendingSouth,
            RailShape::SouthEast,
            RailShape::SouthWest,
            RailShape::NorthWest,
            RailShape::NorthEast,
        ],
    );
    pub const RAIL_SHAPE_STRAIGHT: EnumProperty<RailShape> = EnumProperty::new(
        "shape",
        &[
            RailShape::NorthSouth,
            RailShape::EastWest,
            RailShape::AscendingEast,
            RailShape::AscendingWest,
            RailShape::AscendingNorth,
            RailShape::AscendingSouth,
        ],
    );

    // Age properties
    pub const AGE_1: IntProperty = IntProperty::new("age", 0, 1);
    pub const AGE_2: IntProperty = IntProperty::new("age", 0, 2);
    pub const AGE_3: IntProperty = IntProperty::new("age", 0, 3);
    pub const AGE_4: IntProperty = IntProperty::new("age", 0, 4);
    pub const AGE_5: IntProperty = IntProperty::new("age", 0, 5);
    pub const AGE_7: IntProperty = IntProperty::new("age", 0, 7);
    pub const AGE_15: IntProperty = IntProperty::new("age", 0, 15);
    pub const AGE_25: IntProperty = IntProperty::new("age", 0, 25);

    // Other integer properties
    pub const BITES: IntProperty = IntProperty::new("bites", 0, 6);
    pub const CANDLES: IntProperty = IntProperty::new("candles", 1, 4);
    pub const DELAY: IntProperty = IntProperty::new("delay", 1, 4);
    pub const DISTANCE: IntProperty = IntProperty::new("distance", 1, 7);
    pub const EGGS: IntProperty = IntProperty::new("eggs", 1, 4);
    pub const HATCH: IntProperty = IntProperty::new("hatch", 0, 2);
    pub const LAYERS: IntProperty = IntProperty::new("layers", 1, 8);
    pub const LEVEL_CAULDRON: IntProperty = IntProperty::new("level", 1, 3);
    pub const LEVEL_COMPOSTER: IntProperty = IntProperty::new("level", 0, 8);
    pub const LEVEL_FLOWING: IntProperty = IntProperty::new("level", 1, 8);
    pub const LEVEL_HONEY: IntProperty = IntProperty::new("honey_level", 0, 5);
    pub const LEVEL: IntProperty = IntProperty::new("level", 0, 15);
    pub const MOISTURE: IntProperty = IntProperty::new("moisture", 0, 7);
    pub const NOTE: IntProperty = IntProperty::new("note", 0, 24);
    pub const PICKLES: IntProperty = IntProperty::new("pickles", 1, 4);
    pub const POWER: IntProperty = IntProperty::new("power", 0, 15);
    pub const STAGE: IntProperty = IntProperty::new("stage", 0, 1);
    pub const STABILITY_DISTANCE: IntProperty = IntProperty::new("distance", 0, 7);
    pub const RESPAWN_ANCHOR_CHARGES: IntProperty = IntProperty::new("charges", 0, 4);
    pub const DRIED_GHAST_HYDRATION_LEVELS: IntProperty = IntProperty::new("hydration", 0, 3);
    pub const ROTATION_16: IntProperty = IntProperty::new("rotation", 0, 15);
    pub const DUSTED: IntProperty = IntProperty::new("dusted", 0, 3);

    // Enum properties
    pub const BED_PART: EnumProperty<BedPart> =
        EnumProperty::new("part", &[BedPart::Head, BedPart::Foot]);
    pub const CHEST_TYPE: EnumProperty<ChestType> = EnumProperty::new(
        "type",
        &[ChestType::Single, ChestType::Left, ChestType::Right],
    );
    pub const MODE_COMPARATOR: EnumProperty<ComparatorMode> =
        EnumProperty::new("mode", &[ComparatorMode::Compare, ComparatorMode::Subtract]);
    pub const DOOR_HINGE: EnumProperty<DoorHingeSide> =
        EnumProperty::new("hinge", &[DoorHingeSide::Left, DoorHingeSide::Right]);
    pub const NOTEBLOCK_INSTRUMENT: EnumProperty<NoteBlockInstrument> = EnumProperty::new(
        "instrument",
        &[
            NoteBlockInstrument::Harp,
            NoteBlockInstrument::Basedrum,
            NoteBlockInstrument::Snare,
            NoteBlockInstrument::Hat,
            NoteBlockInstrument::Bass,
            NoteBlockInstrument::Flute,
            NoteBlockInstrument::Bell,
            NoteBlockInstrument::Guitar,
            NoteBlockInstrument::Chime,
            NoteBlockInstrument::Xylophone,
            NoteBlockInstrument::IronXylophone,
            NoteBlockInstrument::CowBell,
            NoteBlockInstrument::Didgeridoo,
            NoteBlockInstrument::Bit,
            NoteBlockInstrument::Banjo,
            NoteBlockInstrument::Pling,
            NoteBlockInstrument::Zombie,
            NoteBlockInstrument::Skeleton,
            NoteBlockInstrument::Creeper,
            NoteBlockInstrument::Dragon,
            NoteBlockInstrument::WitherSkeleton,
            NoteBlockInstrument::Piglin,
            NoteBlockInstrument::CustomHead,
        ],
    );
    pub const PISTON_TYPE: EnumProperty<PistonType> =
        EnumProperty::new("type", &[PistonType::Default, PistonType::Sticky]);
    pub const SLAB_TYPE: EnumProperty<SlabType> =
        EnumProperty::new("type", &[SlabType::Bottom, SlabType::Top, SlabType::Double]);
    pub const STAIRS_SHAPE: EnumProperty<StairsShape> = EnumProperty::new(
        "shape",
        &[
            StairsShape::Straight,
            StairsShape::InnerLeft,
            StairsShape::InnerRight,
            StairsShape::OuterLeft,
            StairsShape::OuterRight,
        ],
    );
    pub const STRUCTUREBLOCK_MODE: EnumProperty<StructureMode> = EnumProperty::new(
        "mode",
        &[
            StructureMode::Save,
            StructureMode::Load,
            StructureMode::Corner,
            StructureMode::Data,
        ],
    );
    pub const BAMBOO_LEAVES: EnumProperty<BambooLeaves> = EnumProperty::new(
        "leaves",
        &[BambooLeaves::None, BambooLeaves::Small, BambooLeaves::Large],
    );
    pub const TILT: EnumProperty<Tilt> = EnumProperty::new(
        "tilt",
        &[Tilt::None, Tilt::Unstable, Tilt::Partial, Tilt::Full],
    );
    pub const VERTICAL_DIRECTION: EnumProperty<Direction> =
        EnumProperty::new("vertical_direction", &[Direction::Up, Direction::Down]);
    pub const DRIPSTONE_THICKNESS: EnumProperty<DripstoneThickness> = EnumProperty::new(
        "thickness",
        &[
            DripstoneThickness::TipMerge,
            DripstoneThickness::Tip,
            DripstoneThickness::Frustum,
            DripstoneThickness::Middle,
            DripstoneThickness::Base,
        ],
    );
    pub const SCULK_SENSOR_PHASE: EnumProperty<SculkSensorPhase> = EnumProperty::new(
        "sculk_sensor_phase",
        &[
            SculkSensorPhase::Inactive,
            SculkSensorPhase::Active,
            SculkSensorPhase::Cooldown,
        ],
    );
    pub const TRIAL_SPAWNER_STATE: EnumProperty<TrialSpawnerState> = EnumProperty::new(
        "trial_spawner_state",
        &[
            TrialSpawnerState::Inactive,
            TrialSpawnerState::WaitingForPlayers,
            TrialSpawnerState::Active,
            TrialSpawnerState::WaitingForRewardEjection,
            TrialSpawnerState::EjectingReward,
            TrialSpawnerState::Cooldown,
        ],
    );
    pub const VAULT_STATE: EnumProperty<VaultState> = EnumProperty::new(
        "vault_state",
        &[
            VaultState::Inactive,
            VaultState::Active,
            VaultState::Unlocking,
            VaultState::Ejecting,
        ],
    );
    pub const CREAKING_HEART_STATE: EnumProperty<CreakingHeartState> = EnumProperty::new(
        "creaking_heart_state",
        &[CreakingHeartState::Inactive, CreakingHeartState::Active],
    );
    pub const TEST_BLOCK_MODE: EnumProperty<TestBlockMode> =
        EnumProperty::new("mode", &[TestBlockMode::Off, TestBlockMode::On]);
    pub const COPPER_GOLEM_POSE: EnumProperty<CopperGolemPose> = EnumProperty::new(
        "copper_golem_pose",
        &[
            CopperGolemPose::Standing,
            CopperGolemPose::Sitting,
            CopperGolemPose::Running,
            CopperGolemPose::Star,
        ],
    );

    // Additional boolean properties
    pub const SLOT_0_OCCUPIED: BooleanProperty = BooleanProperty::new("slot_0_occupied");
    pub const SLOT_1_OCCUPIED: BooleanProperty = BooleanProperty::new("slot_1_occupied");
    pub const SLOT_2_OCCUPIED: BooleanProperty = BooleanProperty::new("slot_2_occupied");
    pub const SLOT_3_OCCUPIED: BooleanProperty = BooleanProperty::new("slot_3_occupied");
    pub const SLOT_4_OCCUPIED: BooleanProperty = BooleanProperty::new("slot_4_occupied");
    pub const SLOT_5_OCCUPIED: BooleanProperty = BooleanProperty::new("slot_5_occupied");
    pub const CRACKED: BooleanProperty = BooleanProperty::new("cracked");
    pub const CRAFTING: BooleanProperty = BooleanProperty::new("crafting");
    pub const OMINOUS: BooleanProperty = BooleanProperty::new("ominous");
    pub const MAP: BooleanProperty = BooleanProperty::new("map");
}
