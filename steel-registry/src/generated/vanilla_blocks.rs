use crate::{
    behaviour::BlockBehaviourProperties,
    blocks::{Block, BlockRegistry},
    properties::BlockStateProperties,
};
pub const AIR: Block = Block::new("air", BlockBehaviourProperties::new(), &[]);
pub const STONE: Block = Block::new("stone", BlockBehaviourProperties::new(), &[]);
pub const GRANITE: Block = Block::new("granite", BlockBehaviourProperties::new(), &[]);
pub const POLISHED_GRANITE: Block =
    Block::new("polished_granite", BlockBehaviourProperties::new(), &[]);
pub const DIORITE: Block = Block::new("diorite", BlockBehaviourProperties::new(), &[]);
pub const POLISHED_DIORITE: Block =
    Block::new("polished_diorite", BlockBehaviourProperties::new(), &[]);
pub const ANDESITE: Block = Block::new("andesite", BlockBehaviourProperties::new(), &[]);
pub const POLISHED_ANDESITE: Block =
    Block::new("polished_andesite", BlockBehaviourProperties::new(), &[]);
pub const GRASS_BLOCK : Block = Block :: new ("grass_block" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: SNOWY] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: SNOWY => BlockStateProperties :: SNOWY . index_of (false))) ;
pub const DIRT: Block = Block::new("dirt", BlockBehaviourProperties::new(), &[]);
pub const COARSE_DIRT: Block = Block::new("coarse_dirt", BlockBehaviourProperties::new(), &[]);
pub const PODZOL : Block = Block :: new ("podzol" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: SNOWY] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: SNOWY => BlockStateProperties :: SNOWY . index_of (false))) ;
pub const COBBLESTONE: Block = Block::new("cobblestone", BlockBehaviourProperties::new(), &[]);
pub const OAK_PLANKS: Block = Block::new("oak_planks", BlockBehaviourProperties::new(), &[]);
pub const SPRUCE_PLANKS: Block = Block::new("spruce_planks", BlockBehaviourProperties::new(), &[]);
pub const BIRCH_PLANKS: Block = Block::new("birch_planks", BlockBehaviourProperties::new(), &[]);
pub const JUNGLE_PLANKS: Block = Block::new("jungle_planks", BlockBehaviourProperties::new(), &[]);
pub const ACACIA_PLANKS: Block = Block::new("acacia_planks", BlockBehaviourProperties::new(), &[]);
pub const CHERRY_PLANKS: Block = Block::new("cherry_planks", BlockBehaviourProperties::new(), &[]);
pub const DARK_OAK_PLANKS: Block =
    Block::new("dark_oak_planks", BlockBehaviourProperties::new(), &[]);
pub const PALE_OAK_WOOD : Block = Block :: new ("pale_oak_wood" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: AXIS] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: AXIS => crate :: properties :: Axis :: Y as usize)) ;
pub const PALE_OAK_PLANKS: Block =
    Block::new("pale_oak_planks", BlockBehaviourProperties::new(), &[]);
pub const MANGROVE_PLANKS: Block =
    Block::new("mangrove_planks", BlockBehaviourProperties::new(), &[]);
pub const BAMBOO_PLANKS: Block = Block::new("bamboo_planks", BlockBehaviourProperties::new(), &[]);
pub const BAMBOO_MOSAIC: Block = Block::new("bamboo_mosaic", BlockBehaviourProperties::new(), &[]);
pub const OAK_SAPLING: Block = Block::new(
    "oak_sapling",
    BlockBehaviourProperties::new(),
    &[&BlockStateProperties::STAGE],
)
.with_default_state(crate :: blocks :: offset ! (BlockStateProperties :: STAGE => 0usize));
pub const SPRUCE_SAPLING: Block = Block::new(
    "spruce_sapling",
    BlockBehaviourProperties::new(),
    &[&BlockStateProperties::STAGE],
)
.with_default_state(crate :: blocks :: offset ! (BlockStateProperties :: STAGE => 0usize));
pub const BIRCH_SAPLING: Block = Block::new(
    "birch_sapling",
    BlockBehaviourProperties::new(),
    &[&BlockStateProperties::STAGE],
)
.with_default_state(crate :: blocks :: offset ! (BlockStateProperties :: STAGE => 0usize));
pub const JUNGLE_SAPLING: Block = Block::new(
    "jungle_sapling",
    BlockBehaviourProperties::new(),
    &[&BlockStateProperties::STAGE],
)
.with_default_state(crate :: blocks :: offset ! (BlockStateProperties :: STAGE => 0usize));
pub const ACACIA_SAPLING: Block = Block::new(
    "acacia_sapling",
    BlockBehaviourProperties::new(),
    &[&BlockStateProperties::STAGE],
)
.with_default_state(crate :: blocks :: offset ! (BlockStateProperties :: STAGE => 0usize));
pub const CHERRY_SAPLING: Block = Block::new(
    "cherry_sapling",
    BlockBehaviourProperties::new(),
    &[&BlockStateProperties::STAGE],
)
.with_default_state(crate :: blocks :: offset ! (BlockStateProperties :: STAGE => 0usize));
pub const DARK_OAK_SAPLING: Block = Block::new(
    "dark_oak_sapling",
    BlockBehaviourProperties::new(),
    &[&BlockStateProperties::STAGE],
)
.with_default_state(crate :: blocks :: offset ! (BlockStateProperties :: STAGE => 0usize));
pub const PALE_OAK_SAPLING: Block = Block::new(
    "pale_oak_sapling",
    BlockBehaviourProperties::new(),
    &[&BlockStateProperties::STAGE],
)
.with_default_state(crate :: blocks :: offset ! (BlockStateProperties :: STAGE => 0usize));
pub const MANGROVE_PROPAGULE : Block = Block :: new ("mangrove_propagule" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: AGE_4 , & BlockStateProperties :: HANGING , & BlockStateProperties :: STAGE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: AGE_4 => 0usize , BlockStateProperties :: HANGING => BlockStateProperties :: HANGING . index_of (false) , BlockStateProperties :: STAGE => 0usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const BEDROCK: Block = Block::new("bedrock", BlockBehaviourProperties::new(), &[]);
pub const WATER: Block = Block::new(
    "water",
    BlockBehaviourProperties::new(),
    &[&BlockStateProperties::LEVEL],
)
.with_default_state(crate :: blocks :: offset ! (BlockStateProperties :: LEVEL => 0usize));
pub const LAVA: Block = Block::new(
    "lava",
    BlockBehaviourProperties::new(),
    &[&BlockStateProperties::LEVEL],
)
.with_default_state(crate :: blocks :: offset ! (BlockStateProperties :: LEVEL => 0usize));
pub const SAND: Block = Block::new("sand", BlockBehaviourProperties::new(), &[]);
pub const SUSPICIOUS_SAND: Block = Block::new(
    "suspicious_sand",
    BlockBehaviourProperties::new(),
    &[&BlockStateProperties::DUSTED],
)
.with_default_state(crate :: blocks :: offset ! (BlockStateProperties :: DUSTED => 0usize));
pub const RED_SAND: Block = Block::new("red_sand", BlockBehaviourProperties::new(), &[]);
pub const GRAVEL: Block = Block::new("gravel", BlockBehaviourProperties::new(), &[]);
pub const SUSPICIOUS_GRAVEL: Block = Block::new(
    "suspicious_gravel",
    BlockBehaviourProperties::new(),
    &[&BlockStateProperties::DUSTED],
)
.with_default_state(crate :: blocks :: offset ! (BlockStateProperties :: DUSTED => 0usize));
pub const GOLD_ORE: Block = Block::new("gold_ore", BlockBehaviourProperties::new(), &[]);
pub const DEEPSLATE_GOLD_ORE: Block =
    Block::new("deepslate_gold_ore", BlockBehaviourProperties::new(), &[]);
pub const IRON_ORE: Block = Block::new("iron_ore", BlockBehaviourProperties::new(), &[]);
pub const DEEPSLATE_IRON_ORE: Block =
    Block::new("deepslate_iron_ore", BlockBehaviourProperties::new(), &[]);
pub const COAL_ORE: Block = Block::new("coal_ore", BlockBehaviourProperties::new(), &[]);
pub const DEEPSLATE_COAL_ORE: Block =
    Block::new("deepslate_coal_ore", BlockBehaviourProperties::new(), &[]);
pub const NETHER_GOLD_ORE: Block =
    Block::new("nether_gold_ore", BlockBehaviourProperties::new(), &[]);
pub const OAK_LOG : Block = Block :: new ("oak_log" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: AXIS] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: AXIS => crate :: properties :: Axis :: Y as usize)) ;
pub const SPRUCE_LOG : Block = Block :: new ("spruce_log" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: AXIS] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: AXIS => crate :: properties :: Axis :: Y as usize)) ;
pub const BIRCH_LOG : Block = Block :: new ("birch_log" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: AXIS] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: AXIS => crate :: properties :: Axis :: Y as usize)) ;
pub const JUNGLE_LOG : Block = Block :: new ("jungle_log" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: AXIS] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: AXIS => crate :: properties :: Axis :: Y as usize)) ;
pub const ACACIA_LOG : Block = Block :: new ("acacia_log" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: AXIS] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: AXIS => crate :: properties :: Axis :: Y as usize)) ;
pub const CHERRY_LOG : Block = Block :: new ("cherry_log" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: AXIS] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: AXIS => crate :: properties :: Axis :: Y as usize)) ;
pub const DARK_OAK_LOG : Block = Block :: new ("dark_oak_log" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: AXIS] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: AXIS => crate :: properties :: Axis :: Y as usize)) ;
pub const PALE_OAK_LOG : Block = Block :: new ("pale_oak_log" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: AXIS] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: AXIS => crate :: properties :: Axis :: Y as usize)) ;
pub const MANGROVE_LOG : Block = Block :: new ("mangrove_log" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: AXIS] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: AXIS => crate :: properties :: Axis :: Y as usize)) ;
pub const MANGROVE_ROOTS : Block = Block :: new ("mangrove_roots" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const MUDDY_MANGROVE_ROOTS : Block = Block :: new ("muddy_mangrove_roots" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: AXIS] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: AXIS => crate :: properties :: Axis :: Y as usize)) ;
pub const BAMBOO_BLOCK : Block = Block :: new ("bamboo_block" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: AXIS] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: AXIS => crate :: properties :: Axis :: Y as usize)) ;
pub const STRIPPED_SPRUCE_LOG : Block = Block :: new ("stripped_spruce_log" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: AXIS] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: AXIS => crate :: properties :: Axis :: Y as usize)) ;
pub const STRIPPED_BIRCH_LOG : Block = Block :: new ("stripped_birch_log" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: AXIS] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: AXIS => crate :: properties :: Axis :: Y as usize)) ;
pub const STRIPPED_JUNGLE_LOG : Block = Block :: new ("stripped_jungle_log" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: AXIS] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: AXIS => crate :: properties :: Axis :: Y as usize)) ;
pub const STRIPPED_ACACIA_LOG : Block = Block :: new ("stripped_acacia_log" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: AXIS] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: AXIS => crate :: properties :: Axis :: Y as usize)) ;
pub const STRIPPED_CHERRY_LOG : Block = Block :: new ("stripped_cherry_log" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: AXIS] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: AXIS => crate :: properties :: Axis :: Y as usize)) ;
pub const STRIPPED_DARK_OAK_LOG : Block = Block :: new ("stripped_dark_oak_log" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: AXIS] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: AXIS => crate :: properties :: Axis :: Y as usize)) ;
pub const STRIPPED_PALE_OAK_LOG : Block = Block :: new ("stripped_pale_oak_log" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: AXIS] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: AXIS => crate :: properties :: Axis :: Y as usize)) ;
pub const STRIPPED_OAK_LOG : Block = Block :: new ("stripped_oak_log" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: AXIS] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: AXIS => crate :: properties :: Axis :: Y as usize)) ;
pub const STRIPPED_MANGROVE_LOG : Block = Block :: new ("stripped_mangrove_log" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: AXIS] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: AXIS => crate :: properties :: Axis :: Y as usize)) ;
pub const STRIPPED_BAMBOO_BLOCK : Block = Block :: new ("stripped_bamboo_block" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: AXIS] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: AXIS => crate :: properties :: Axis :: Y as usize)) ;
pub const OAK_WOOD : Block = Block :: new ("oak_wood" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: AXIS] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: AXIS => crate :: properties :: Axis :: Y as usize)) ;
pub const SPRUCE_WOOD : Block = Block :: new ("spruce_wood" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: AXIS] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: AXIS => crate :: properties :: Axis :: Y as usize)) ;
pub const BIRCH_WOOD : Block = Block :: new ("birch_wood" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: AXIS] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: AXIS => crate :: properties :: Axis :: Y as usize)) ;
pub const JUNGLE_WOOD : Block = Block :: new ("jungle_wood" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: AXIS] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: AXIS => crate :: properties :: Axis :: Y as usize)) ;
pub const ACACIA_WOOD : Block = Block :: new ("acacia_wood" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: AXIS] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: AXIS => crate :: properties :: Axis :: Y as usize)) ;
pub const CHERRY_WOOD : Block = Block :: new ("cherry_wood" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: AXIS] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: AXIS => crate :: properties :: Axis :: Y as usize)) ;
pub const DARK_OAK_WOOD : Block = Block :: new ("dark_oak_wood" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: AXIS] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: AXIS => crate :: properties :: Axis :: Y as usize)) ;
pub const MANGROVE_WOOD : Block = Block :: new ("mangrove_wood" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: AXIS] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: AXIS => crate :: properties :: Axis :: Y as usize)) ;
pub const STRIPPED_OAK_WOOD : Block = Block :: new ("stripped_oak_wood" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: AXIS] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: AXIS => crate :: properties :: Axis :: Y as usize)) ;
pub const STRIPPED_SPRUCE_WOOD : Block = Block :: new ("stripped_spruce_wood" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: AXIS] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: AXIS => crate :: properties :: Axis :: Y as usize)) ;
pub const STRIPPED_BIRCH_WOOD : Block = Block :: new ("stripped_birch_wood" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: AXIS] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: AXIS => crate :: properties :: Axis :: Y as usize)) ;
pub const STRIPPED_JUNGLE_WOOD : Block = Block :: new ("stripped_jungle_wood" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: AXIS] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: AXIS => crate :: properties :: Axis :: Y as usize)) ;
pub const STRIPPED_ACACIA_WOOD : Block = Block :: new ("stripped_acacia_wood" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: AXIS] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: AXIS => crate :: properties :: Axis :: Y as usize)) ;
pub const STRIPPED_CHERRY_WOOD : Block = Block :: new ("stripped_cherry_wood" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: AXIS] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: AXIS => crate :: properties :: Axis :: Y as usize)) ;
pub const STRIPPED_DARK_OAK_WOOD : Block = Block :: new ("stripped_dark_oak_wood" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: AXIS] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: AXIS => crate :: properties :: Axis :: Y as usize)) ;
pub const STRIPPED_PALE_OAK_WOOD : Block = Block :: new ("stripped_pale_oak_wood" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: AXIS] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: AXIS => crate :: properties :: Axis :: Y as usize)) ;
pub const STRIPPED_MANGROVE_WOOD : Block = Block :: new ("stripped_mangrove_wood" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: AXIS] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: AXIS => crate :: properties :: Axis :: Y as usize)) ;
pub const OAK_LEAVES : Block = Block :: new ("oak_leaves" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: DISTANCE , & BlockStateProperties :: PERSISTENT , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: DISTANCE => 7usize , BlockStateProperties :: PERSISTENT => BlockStateProperties :: PERSISTENT . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const SPRUCE_LEAVES : Block = Block :: new ("spruce_leaves" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: DISTANCE , & BlockStateProperties :: PERSISTENT , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: DISTANCE => 7usize , BlockStateProperties :: PERSISTENT => BlockStateProperties :: PERSISTENT . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const BIRCH_LEAVES : Block = Block :: new ("birch_leaves" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: DISTANCE , & BlockStateProperties :: PERSISTENT , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: DISTANCE => 7usize , BlockStateProperties :: PERSISTENT => BlockStateProperties :: PERSISTENT . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const JUNGLE_LEAVES : Block = Block :: new ("jungle_leaves" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: DISTANCE , & BlockStateProperties :: PERSISTENT , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: DISTANCE => 7usize , BlockStateProperties :: PERSISTENT => BlockStateProperties :: PERSISTENT . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const ACACIA_LEAVES : Block = Block :: new ("acacia_leaves" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: DISTANCE , & BlockStateProperties :: PERSISTENT , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: DISTANCE => 7usize , BlockStateProperties :: PERSISTENT => BlockStateProperties :: PERSISTENT . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const CHERRY_LEAVES : Block = Block :: new ("cherry_leaves" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: DISTANCE , & BlockStateProperties :: PERSISTENT , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: DISTANCE => 7usize , BlockStateProperties :: PERSISTENT => BlockStateProperties :: PERSISTENT . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const DARK_OAK_LEAVES : Block = Block :: new ("dark_oak_leaves" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: DISTANCE , & BlockStateProperties :: PERSISTENT , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: DISTANCE => 7usize , BlockStateProperties :: PERSISTENT => BlockStateProperties :: PERSISTENT . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const PALE_OAK_LEAVES : Block = Block :: new ("pale_oak_leaves" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: DISTANCE , & BlockStateProperties :: PERSISTENT , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: DISTANCE => 7usize , BlockStateProperties :: PERSISTENT => BlockStateProperties :: PERSISTENT . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const MANGROVE_LEAVES : Block = Block :: new ("mangrove_leaves" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: DISTANCE , & BlockStateProperties :: PERSISTENT , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: DISTANCE => 7usize , BlockStateProperties :: PERSISTENT => BlockStateProperties :: PERSISTENT . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const AZALEA_LEAVES : Block = Block :: new ("azalea_leaves" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: DISTANCE , & BlockStateProperties :: PERSISTENT , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: DISTANCE => 7usize , BlockStateProperties :: PERSISTENT => BlockStateProperties :: PERSISTENT . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const FLOWERING_AZALEA_LEAVES : Block = Block :: new ("flowering_azalea_leaves" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: DISTANCE , & BlockStateProperties :: PERSISTENT , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: DISTANCE => 7usize , BlockStateProperties :: PERSISTENT => BlockStateProperties :: PERSISTENT . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const SPONGE: Block = Block::new("sponge", BlockBehaviourProperties::new(), &[]);
pub const WET_SPONGE: Block = Block::new("wet_sponge", BlockBehaviourProperties::new(), &[]);
pub const GLASS: Block = Block::new("glass", BlockBehaviourProperties::new(), &[]);
pub const LAPIS_ORE: Block = Block::new("lapis_ore", BlockBehaviourProperties::new(), &[]);
pub const DEEPSLATE_LAPIS_ORE: Block =
    Block::new("deepslate_lapis_ore", BlockBehaviourProperties::new(), &[]);
pub const LAPIS_BLOCK: Block = Block::new("lapis_block", BlockBehaviourProperties::new(), &[]);
pub const DISPENSER : Block = Block :: new ("dispenser" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: FACING , & BlockStateProperties :: TRIGGERED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: TRIGGERED => BlockStateProperties :: TRIGGERED . index_of (false))) ;
pub const SANDSTONE: Block = Block::new("sandstone", BlockBehaviourProperties::new(), &[]);
pub const CHISELED_SANDSTONE: Block =
    Block::new("chiseled_sandstone", BlockBehaviourProperties::new(), &[]);
pub const CUT_SANDSTONE: Block = Block::new("cut_sandstone", BlockBehaviourProperties::new(), &[]);
pub const NOTE_BLOCK : Block = Block :: new ("note_block" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: NOTEBLOCK_INSTRUMENT , & BlockStateProperties :: NOTE , & BlockStateProperties :: POWERED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: NOTEBLOCK_INSTRUMENT => crate :: properties :: NoteBlockInstrument :: Harp as usize , BlockStateProperties :: NOTE => 0usize , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false))) ;
pub const WHITE_BED : Block = Block :: new ("white_bed" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: OCCUPIED , & BlockStateProperties :: BED_PART] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: OCCUPIED => BlockStateProperties :: OCCUPIED . index_of (false) , BlockStateProperties :: BED_PART => crate :: properties :: BedPart :: Foot as usize)) ;
pub const ORANGE_BED : Block = Block :: new ("orange_bed" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: OCCUPIED , & BlockStateProperties :: BED_PART] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: OCCUPIED => BlockStateProperties :: OCCUPIED . index_of (false) , BlockStateProperties :: BED_PART => crate :: properties :: BedPart :: Foot as usize)) ;
pub const MAGENTA_BED : Block = Block :: new ("magenta_bed" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: OCCUPIED , & BlockStateProperties :: BED_PART] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: OCCUPIED => BlockStateProperties :: OCCUPIED . index_of (false) , BlockStateProperties :: BED_PART => crate :: properties :: BedPart :: Foot as usize)) ;
pub const LIGHT_BLUE_BED : Block = Block :: new ("light_blue_bed" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: OCCUPIED , & BlockStateProperties :: BED_PART] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: OCCUPIED => BlockStateProperties :: OCCUPIED . index_of (false) , BlockStateProperties :: BED_PART => crate :: properties :: BedPart :: Foot as usize)) ;
pub const YELLOW_BED : Block = Block :: new ("yellow_bed" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: OCCUPIED , & BlockStateProperties :: BED_PART] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: OCCUPIED => BlockStateProperties :: OCCUPIED . index_of (false) , BlockStateProperties :: BED_PART => crate :: properties :: BedPart :: Foot as usize)) ;
pub const LIME_BED : Block = Block :: new ("lime_bed" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: OCCUPIED , & BlockStateProperties :: BED_PART] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: OCCUPIED => BlockStateProperties :: OCCUPIED . index_of (false) , BlockStateProperties :: BED_PART => crate :: properties :: BedPart :: Foot as usize)) ;
pub const PINK_BED : Block = Block :: new ("pink_bed" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: OCCUPIED , & BlockStateProperties :: BED_PART] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: OCCUPIED => BlockStateProperties :: OCCUPIED . index_of (false) , BlockStateProperties :: BED_PART => crate :: properties :: BedPart :: Foot as usize)) ;
pub const GRAY_BED : Block = Block :: new ("gray_bed" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: OCCUPIED , & BlockStateProperties :: BED_PART] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: OCCUPIED => BlockStateProperties :: OCCUPIED . index_of (false) , BlockStateProperties :: BED_PART => crate :: properties :: BedPart :: Foot as usize)) ;
pub const LIGHT_GRAY_BED : Block = Block :: new ("light_gray_bed" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: OCCUPIED , & BlockStateProperties :: BED_PART] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: OCCUPIED => BlockStateProperties :: OCCUPIED . index_of (false) , BlockStateProperties :: BED_PART => crate :: properties :: BedPart :: Foot as usize)) ;
pub const CYAN_BED : Block = Block :: new ("cyan_bed" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: OCCUPIED , & BlockStateProperties :: BED_PART] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: OCCUPIED => BlockStateProperties :: OCCUPIED . index_of (false) , BlockStateProperties :: BED_PART => crate :: properties :: BedPart :: Foot as usize)) ;
pub const PURPLE_BED : Block = Block :: new ("purple_bed" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: OCCUPIED , & BlockStateProperties :: BED_PART] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: OCCUPIED => BlockStateProperties :: OCCUPIED . index_of (false) , BlockStateProperties :: BED_PART => crate :: properties :: BedPart :: Foot as usize)) ;
pub const BLUE_BED : Block = Block :: new ("blue_bed" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: OCCUPIED , & BlockStateProperties :: BED_PART] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: OCCUPIED => BlockStateProperties :: OCCUPIED . index_of (false) , BlockStateProperties :: BED_PART => crate :: properties :: BedPart :: Foot as usize)) ;
pub const BROWN_BED : Block = Block :: new ("brown_bed" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: OCCUPIED , & BlockStateProperties :: BED_PART] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: OCCUPIED => BlockStateProperties :: OCCUPIED . index_of (false) , BlockStateProperties :: BED_PART => crate :: properties :: BedPart :: Foot as usize)) ;
pub const GREEN_BED : Block = Block :: new ("green_bed" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: OCCUPIED , & BlockStateProperties :: BED_PART] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: OCCUPIED => BlockStateProperties :: OCCUPIED . index_of (false) , BlockStateProperties :: BED_PART => crate :: properties :: BedPart :: Foot as usize)) ;
pub const RED_BED : Block = Block :: new ("red_bed" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: OCCUPIED , & BlockStateProperties :: BED_PART] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: OCCUPIED => BlockStateProperties :: OCCUPIED . index_of (false) , BlockStateProperties :: BED_PART => crate :: properties :: BedPart :: Foot as usize)) ;
pub const BLACK_BED : Block = Block :: new ("black_bed" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: OCCUPIED , & BlockStateProperties :: BED_PART] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: OCCUPIED => BlockStateProperties :: OCCUPIED . index_of (false) , BlockStateProperties :: BED_PART => crate :: properties :: BedPart :: Foot as usize)) ;
pub const POWERED_RAIL : Block = Block :: new ("powered_rail" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: POWERED , & BlockStateProperties :: RAIL_SHAPE_STRAIGHT , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false) , BlockStateProperties :: RAIL_SHAPE_STRAIGHT => crate :: properties :: RailShape :: NorthSouth as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const DETECTOR_RAIL : Block = Block :: new ("detector_rail" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: POWERED , & BlockStateProperties :: RAIL_SHAPE_STRAIGHT , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false) , BlockStateProperties :: RAIL_SHAPE_STRAIGHT => crate :: properties :: RailShape :: NorthSouth as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const STICKY_PISTON : Block = Block :: new ("sticky_piston" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: EXTENDED , & BlockStateProperties :: FACING] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: EXTENDED => BlockStateProperties :: EXTENDED . index_of (false) , BlockStateProperties :: FACING => crate :: properties :: Direction :: North as usize)) ;
pub const COBWEB: Block = Block::new("cobweb", BlockBehaviourProperties::new(), &[]);
pub const SHORT_GRASS: Block = Block::new("short_grass", BlockBehaviourProperties::new(), &[]);
pub const FERN: Block = Block::new("fern", BlockBehaviourProperties::new(), &[]);
pub const DEAD_BUSH: Block = Block::new("dead_bush", BlockBehaviourProperties::new(), &[]);
pub const BUSH: Block = Block::new("bush", BlockBehaviourProperties::new(), &[]);
pub const SHORT_DRY_GRASS: Block =
    Block::new("short_dry_grass", BlockBehaviourProperties::new(), &[]);
pub const TALL_DRY_GRASS: Block =
    Block::new("tall_dry_grass", BlockBehaviourProperties::new(), &[]);
pub const SEAGRASS: Block = Block::new("seagrass", BlockBehaviourProperties::new(), &[]);
pub const TALL_SEAGRASS : Block = Block :: new ("tall_seagrass" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: DOUBLE_BLOCK_HALF] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: DOUBLE_BLOCK_HALF => crate :: properties :: DoubleBlockHalf :: Lower as usize)) ;
pub const PISTON : Block = Block :: new ("piston" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: EXTENDED , & BlockStateProperties :: FACING] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: EXTENDED => BlockStateProperties :: EXTENDED . index_of (false) , BlockStateProperties :: FACING => crate :: properties :: Direction :: North as usize)) ;
pub const PISTON_HEAD : Block = Block :: new ("piston_head" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: FACING , & BlockStateProperties :: SHORT , & BlockStateProperties :: PISTON_TYPE] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: SHORT => BlockStateProperties :: SHORT . index_of (false) , BlockStateProperties :: PISTON_TYPE => crate :: properties :: PistonType :: Normal as usize)) ;
pub const WHITE_WOOL: Block = Block::new("white_wool", BlockBehaviourProperties::new(), &[]);
pub const ORANGE_WOOL: Block = Block::new("orange_wool", BlockBehaviourProperties::new(), &[]);
pub const MAGENTA_WOOL: Block = Block::new("magenta_wool", BlockBehaviourProperties::new(), &[]);
pub const LIGHT_BLUE_WOOL: Block =
    Block::new("light_blue_wool", BlockBehaviourProperties::new(), &[]);
pub const YELLOW_WOOL: Block = Block::new("yellow_wool", BlockBehaviourProperties::new(), &[]);
pub const LIME_WOOL: Block = Block::new("lime_wool", BlockBehaviourProperties::new(), &[]);
pub const PINK_WOOL: Block = Block::new("pink_wool", BlockBehaviourProperties::new(), &[]);
pub const GRAY_WOOL: Block = Block::new("gray_wool", BlockBehaviourProperties::new(), &[]);
pub const LIGHT_GRAY_WOOL: Block =
    Block::new("light_gray_wool", BlockBehaviourProperties::new(), &[]);
pub const CYAN_WOOL: Block = Block::new("cyan_wool", BlockBehaviourProperties::new(), &[]);
pub const PURPLE_WOOL: Block = Block::new("purple_wool", BlockBehaviourProperties::new(), &[]);
pub const BLUE_WOOL: Block = Block::new("blue_wool", BlockBehaviourProperties::new(), &[]);
pub const BROWN_WOOL: Block = Block::new("brown_wool", BlockBehaviourProperties::new(), &[]);
pub const GREEN_WOOL: Block = Block::new("green_wool", BlockBehaviourProperties::new(), &[]);
pub const RED_WOOL: Block = Block::new("red_wool", BlockBehaviourProperties::new(), &[]);
pub const BLACK_WOOL: Block = Block::new("black_wool", BlockBehaviourProperties::new(), &[]);
pub const MOVING_PISTON : Block = Block :: new ("moving_piston" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: FACING , & BlockStateProperties :: PISTON_TYPE] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: PISTON_TYPE => crate :: properties :: PistonType :: Normal as usize)) ;
pub const DANDELION: Block = Block::new("dandelion", BlockBehaviourProperties::new(), &[]);
pub const TORCHFLOWER: Block = Block::new("torchflower", BlockBehaviourProperties::new(), &[]);
pub const POPPY: Block = Block::new("poppy", BlockBehaviourProperties::new(), &[]);
pub const BLUE_ORCHID: Block = Block::new("blue_orchid", BlockBehaviourProperties::new(), &[]);
pub const ALLIUM: Block = Block::new("allium", BlockBehaviourProperties::new(), &[]);
pub const AZURE_BLUET: Block = Block::new("azure_bluet", BlockBehaviourProperties::new(), &[]);
pub const RED_TULIP: Block = Block::new("red_tulip", BlockBehaviourProperties::new(), &[]);
pub const ORANGE_TULIP: Block = Block::new("orange_tulip", BlockBehaviourProperties::new(), &[]);
pub const WHITE_TULIP: Block = Block::new("white_tulip", BlockBehaviourProperties::new(), &[]);
pub const PINK_TULIP: Block = Block::new("pink_tulip", BlockBehaviourProperties::new(), &[]);
pub const OXEYE_DAISY: Block = Block::new("oxeye_daisy", BlockBehaviourProperties::new(), &[]);
pub const CORNFLOWER: Block = Block::new("cornflower", BlockBehaviourProperties::new(), &[]);
pub const WITHER_ROSE: Block = Block::new("wither_rose", BlockBehaviourProperties::new(), &[]);
pub const LILY_OF_THE_VALLEY: Block =
    Block::new("lily_of_the_valley", BlockBehaviourProperties::new(), &[]);
pub const BROWN_MUSHROOM: Block =
    Block::new("brown_mushroom", BlockBehaviourProperties::new(), &[]);
pub const RED_MUSHROOM: Block = Block::new("red_mushroom", BlockBehaviourProperties::new(), &[]);
pub const GOLD_BLOCK: Block = Block::new("gold_block", BlockBehaviourProperties::new(), &[]);
pub const IRON_BLOCK: Block = Block::new("iron_block", BlockBehaviourProperties::new(), &[]);
pub const BRICKS: Block = Block::new("bricks", BlockBehaviourProperties::new(), &[]);
pub const TNT : Block = Block :: new ("tnt" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: UNSTABLE] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: UNSTABLE => BlockStateProperties :: UNSTABLE . index_of (false))) ;
pub const BOOKSHELF: Block = Block::new("bookshelf", BlockBehaviourProperties::new(), &[]);
pub const CHISELED_BOOKSHELF : Block = Block :: new ("chiseled_bookshelf" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: SLOT_0_OCCUPIED , & BlockStateProperties :: SLOT_1_OCCUPIED , & BlockStateProperties :: SLOT_2_OCCUPIED , & BlockStateProperties :: SLOT_3_OCCUPIED , & BlockStateProperties :: SLOT_4_OCCUPIED , & BlockStateProperties :: SLOT_5_OCCUPIED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: SLOT_0_OCCUPIED => BlockStateProperties :: SLOT_0_OCCUPIED . index_of (false) , BlockStateProperties :: SLOT_1_OCCUPIED => BlockStateProperties :: SLOT_1_OCCUPIED . index_of (false) , BlockStateProperties :: SLOT_2_OCCUPIED => BlockStateProperties :: SLOT_2_OCCUPIED . index_of (false) , BlockStateProperties :: SLOT_3_OCCUPIED => BlockStateProperties :: SLOT_3_OCCUPIED . index_of (false) , BlockStateProperties :: SLOT_4_OCCUPIED => BlockStateProperties :: SLOT_4_OCCUPIED . index_of (false) , BlockStateProperties :: SLOT_5_OCCUPIED => BlockStateProperties :: SLOT_5_OCCUPIED . index_of (false))) ;
pub const ACACIA_SHELF : Block = Block :: new ("acacia_shelf" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: POWERED , & BlockStateProperties :: SIDE_CHAIN_PART , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false) , BlockStateProperties :: SIDE_CHAIN_PART => crate :: properties :: SideChainPart :: Unconnected as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const BAMBOO_SHELF : Block = Block :: new ("bamboo_shelf" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: POWERED , & BlockStateProperties :: SIDE_CHAIN_PART , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false) , BlockStateProperties :: SIDE_CHAIN_PART => crate :: properties :: SideChainPart :: Unconnected as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const BIRCH_SHELF : Block = Block :: new ("birch_shelf" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: POWERED , & BlockStateProperties :: SIDE_CHAIN_PART , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false) , BlockStateProperties :: SIDE_CHAIN_PART => crate :: properties :: SideChainPart :: Unconnected as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const CHERRY_SHELF : Block = Block :: new ("cherry_shelf" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: POWERED , & BlockStateProperties :: SIDE_CHAIN_PART , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false) , BlockStateProperties :: SIDE_CHAIN_PART => crate :: properties :: SideChainPart :: Unconnected as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const CRIMSON_SHELF : Block = Block :: new ("crimson_shelf" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: POWERED , & BlockStateProperties :: SIDE_CHAIN_PART , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false) , BlockStateProperties :: SIDE_CHAIN_PART => crate :: properties :: SideChainPart :: Unconnected as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const DARK_OAK_SHELF : Block = Block :: new ("dark_oak_shelf" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: POWERED , & BlockStateProperties :: SIDE_CHAIN_PART , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false) , BlockStateProperties :: SIDE_CHAIN_PART => crate :: properties :: SideChainPart :: Unconnected as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const JUNGLE_SHELF : Block = Block :: new ("jungle_shelf" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: POWERED , & BlockStateProperties :: SIDE_CHAIN_PART , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false) , BlockStateProperties :: SIDE_CHAIN_PART => crate :: properties :: SideChainPart :: Unconnected as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const MANGROVE_SHELF : Block = Block :: new ("mangrove_shelf" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: POWERED , & BlockStateProperties :: SIDE_CHAIN_PART , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false) , BlockStateProperties :: SIDE_CHAIN_PART => crate :: properties :: SideChainPart :: Unconnected as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const OAK_SHELF : Block = Block :: new ("oak_shelf" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: POWERED , & BlockStateProperties :: SIDE_CHAIN_PART , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false) , BlockStateProperties :: SIDE_CHAIN_PART => crate :: properties :: SideChainPart :: Unconnected as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const PALE_OAK_SHELF : Block = Block :: new ("pale_oak_shelf" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: POWERED , & BlockStateProperties :: SIDE_CHAIN_PART , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false) , BlockStateProperties :: SIDE_CHAIN_PART => crate :: properties :: SideChainPart :: Unconnected as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const SPRUCE_SHELF : Block = Block :: new ("spruce_shelf" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: POWERED , & BlockStateProperties :: SIDE_CHAIN_PART , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false) , BlockStateProperties :: SIDE_CHAIN_PART => crate :: properties :: SideChainPart :: Unconnected as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const WARPED_SHELF : Block = Block :: new ("warped_shelf" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: POWERED , & BlockStateProperties :: SIDE_CHAIN_PART , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false) , BlockStateProperties :: SIDE_CHAIN_PART => crate :: properties :: SideChainPart :: Unconnected as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const MOSSY_COBBLESTONE: Block =
    Block::new("mossy_cobblestone", BlockBehaviourProperties::new(), &[]);
pub const OBSIDIAN: Block = Block::new("obsidian", BlockBehaviourProperties::new(), &[]);
pub const TORCH: Block = Block::new("torch", BlockBehaviourProperties::new(), &[]);
pub const WALL_TORCH : Block = Block :: new ("wall_torch" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize)) ;
pub const FIRE : Block = Block :: new ("fire" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: AGE_15 , & BlockStateProperties :: EAST , & BlockStateProperties :: NORTH , & BlockStateProperties :: SOUTH , & BlockStateProperties :: UP , & BlockStateProperties :: WEST] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: AGE_15 => 0usize , BlockStateProperties :: EAST => BlockStateProperties :: EAST . index_of (false) , BlockStateProperties :: NORTH => BlockStateProperties :: NORTH . index_of (false) , BlockStateProperties :: SOUTH => BlockStateProperties :: SOUTH . index_of (false) , BlockStateProperties :: UP => BlockStateProperties :: UP . index_of (false) , BlockStateProperties :: WEST => BlockStateProperties :: WEST . index_of (false))) ;
pub const SOUL_FIRE: Block = Block::new("soul_fire", BlockBehaviourProperties::new(), &[]);
pub const SPAWNER: Block = Block::new("spawner", BlockBehaviourProperties::new(), &[]);
pub const CREAKING_HEART : Block = Block :: new ("creaking_heart" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: AXIS , & BlockStateProperties :: CREAKING_HEART_STATE , & BlockStateProperties :: NATURAL] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: AXIS => crate :: properties :: Axis :: Y as usize , BlockStateProperties :: CREAKING_HEART_STATE => crate :: properties :: CreakingHeartState :: Uprooted as usize , BlockStateProperties :: NATURAL => BlockStateProperties :: NATURAL . index_of (false))) ;
pub const OAK_STAIRS : Block = Block :: new ("oak_stairs" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: STAIRS_SHAPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: HALF => crate :: properties :: Half :: Bottom as usize , BlockStateProperties :: STAIRS_SHAPE => crate :: properties :: StairsShape :: Straight as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const CHEST : Block = Block :: new ("chest" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: CHEST_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: CHEST_TYPE => crate :: properties :: ChestType :: Single as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const REDSTONE_WIRE : Block = Block :: new ("redstone_wire" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: EAST_REDSTONE , & BlockStateProperties :: NORTH_REDSTONE , & BlockStateProperties :: POWER , & BlockStateProperties :: SOUTH_REDSTONE , & BlockStateProperties :: WEST_REDSTONE] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: EAST_REDSTONE => crate :: properties :: RedstoneSide :: None as usize , BlockStateProperties :: NORTH_REDSTONE => crate :: properties :: RedstoneSide :: None as usize , BlockStateProperties :: POWER => 0usize , BlockStateProperties :: SOUTH_REDSTONE => crate :: properties :: RedstoneSide :: None as usize , BlockStateProperties :: WEST_REDSTONE => crate :: properties :: RedstoneSide :: None as usize)) ;
pub const DIAMOND_ORE: Block = Block::new("diamond_ore", BlockBehaviourProperties::new(), &[]);
pub const DEEPSLATE_DIAMOND_ORE: Block = Block::new(
    "deepslate_diamond_ore",
    BlockBehaviourProperties::new(),
    &[],
);
pub const DIAMOND_BLOCK: Block = Block::new("diamond_block", BlockBehaviourProperties::new(), &[]);
pub const CRAFTING_TABLE: Block =
    Block::new("crafting_table", BlockBehaviourProperties::new(), &[]);
pub const WHEAT: Block = Block::new(
    "wheat",
    BlockBehaviourProperties::new(),
    &[&BlockStateProperties::AGE_7],
)
.with_default_state(crate :: blocks :: offset ! (BlockStateProperties :: AGE_7 => 0usize));
pub const FARMLAND: Block = Block::new(
    "farmland",
    BlockBehaviourProperties::new(),
    &[&BlockStateProperties::MOISTURE],
)
.with_default_state(crate :: blocks :: offset ! (BlockStateProperties :: MOISTURE => 0usize));
pub const FURNACE : Block = Block :: new ("furnace" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: LIT] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: LIT => BlockStateProperties :: LIT . index_of (false))) ;
pub const OAK_SIGN : Block = Block :: new ("oak_sign" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: ROTATION_16 , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: ROTATION_16 => 0usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const SPRUCE_SIGN : Block = Block :: new ("spruce_sign" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: ROTATION_16 , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: ROTATION_16 => 0usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const BIRCH_SIGN : Block = Block :: new ("birch_sign" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: ROTATION_16 , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: ROTATION_16 => 0usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const ACACIA_SIGN : Block = Block :: new ("acacia_sign" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: ROTATION_16 , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: ROTATION_16 => 0usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const CHERRY_SIGN : Block = Block :: new ("cherry_sign" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: ROTATION_16 , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: ROTATION_16 => 0usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const JUNGLE_SIGN : Block = Block :: new ("jungle_sign" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: ROTATION_16 , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: ROTATION_16 => 0usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const DARK_OAK_SIGN : Block = Block :: new ("dark_oak_sign" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: ROTATION_16 , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: ROTATION_16 => 0usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const PALE_OAK_SIGN : Block = Block :: new ("pale_oak_sign" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: ROTATION_16 , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: ROTATION_16 => 0usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const MANGROVE_SIGN : Block = Block :: new ("mangrove_sign" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: ROTATION_16 , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: ROTATION_16 => 0usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const BAMBOO_SIGN : Block = Block :: new ("bamboo_sign" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: ROTATION_16 , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: ROTATION_16 => 0usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const OAK_DOOR : Block = Block :: new ("oak_door" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: DOUBLE_BLOCK_HALF , & BlockStateProperties :: DOOR_HINGE , & BlockStateProperties :: OPEN , & BlockStateProperties :: POWERED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: DOUBLE_BLOCK_HALF => crate :: properties :: DoubleBlockHalf :: Lower as usize , BlockStateProperties :: DOOR_HINGE => crate :: properties :: DoorHingeSide :: Left as usize , BlockStateProperties :: OPEN => BlockStateProperties :: OPEN . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false))) ;
pub const LADDER : Block = Block :: new ("ladder" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const RAIL : Block = Block :: new ("rail" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: RAIL_SHAPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: RAIL_SHAPE => crate :: properties :: RailShape :: NorthSouth as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const COBBLESTONE_STAIRS : Block = Block :: new ("cobblestone_stairs" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: STAIRS_SHAPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: HALF => crate :: properties :: Half :: Bottom as usize , BlockStateProperties :: STAIRS_SHAPE => crate :: properties :: StairsShape :: Straight as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const OAK_WALL_SIGN : Block = Block :: new ("oak_wall_sign" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const SPRUCE_WALL_SIGN : Block = Block :: new ("spruce_wall_sign" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const BIRCH_WALL_SIGN : Block = Block :: new ("birch_wall_sign" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const ACACIA_WALL_SIGN : Block = Block :: new ("acacia_wall_sign" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const CHERRY_WALL_SIGN : Block = Block :: new ("cherry_wall_sign" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const JUNGLE_WALL_SIGN : Block = Block :: new ("jungle_wall_sign" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const DARK_OAK_WALL_SIGN : Block = Block :: new ("dark_oak_wall_sign" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const PALE_OAK_WALL_SIGN : Block = Block :: new ("pale_oak_wall_sign" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const MANGROVE_WALL_SIGN : Block = Block :: new ("mangrove_wall_sign" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const BAMBOO_WALL_SIGN : Block = Block :: new ("bamboo_wall_sign" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const OAK_HANGING_SIGN : Block = Block :: new ("oak_hanging_sign" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: ATTACHED , & BlockStateProperties :: ROTATION_16 , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: ATTACHED => BlockStateProperties :: ATTACHED . index_of (false) , BlockStateProperties :: ROTATION_16 => 0usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const SPRUCE_HANGING_SIGN : Block = Block :: new ("spruce_hanging_sign" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: ATTACHED , & BlockStateProperties :: ROTATION_16 , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: ATTACHED => BlockStateProperties :: ATTACHED . index_of (false) , BlockStateProperties :: ROTATION_16 => 0usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const BIRCH_HANGING_SIGN : Block = Block :: new ("birch_hanging_sign" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: ATTACHED , & BlockStateProperties :: ROTATION_16 , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: ATTACHED => BlockStateProperties :: ATTACHED . index_of (false) , BlockStateProperties :: ROTATION_16 => 0usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const ACACIA_HANGING_SIGN : Block = Block :: new ("acacia_hanging_sign" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: ATTACHED , & BlockStateProperties :: ROTATION_16 , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: ATTACHED => BlockStateProperties :: ATTACHED . index_of (false) , BlockStateProperties :: ROTATION_16 => 0usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const CHERRY_HANGING_SIGN : Block = Block :: new ("cherry_hanging_sign" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: ATTACHED , & BlockStateProperties :: ROTATION_16 , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: ATTACHED => BlockStateProperties :: ATTACHED . index_of (false) , BlockStateProperties :: ROTATION_16 => 0usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const JUNGLE_HANGING_SIGN : Block = Block :: new ("jungle_hanging_sign" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: ATTACHED , & BlockStateProperties :: ROTATION_16 , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: ATTACHED => BlockStateProperties :: ATTACHED . index_of (false) , BlockStateProperties :: ROTATION_16 => 0usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const DARK_OAK_HANGING_SIGN : Block = Block :: new ("dark_oak_hanging_sign" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: ATTACHED , & BlockStateProperties :: ROTATION_16 , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: ATTACHED => BlockStateProperties :: ATTACHED . index_of (false) , BlockStateProperties :: ROTATION_16 => 0usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const PALE_OAK_HANGING_SIGN : Block = Block :: new ("pale_oak_hanging_sign" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: ATTACHED , & BlockStateProperties :: ROTATION_16 , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: ATTACHED => BlockStateProperties :: ATTACHED . index_of (false) , BlockStateProperties :: ROTATION_16 => 0usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const CRIMSON_HANGING_SIGN : Block = Block :: new ("crimson_hanging_sign" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: ATTACHED , & BlockStateProperties :: ROTATION_16 , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: ATTACHED => BlockStateProperties :: ATTACHED . index_of (false) , BlockStateProperties :: ROTATION_16 => 0usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const WARPED_HANGING_SIGN : Block = Block :: new ("warped_hanging_sign" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: ATTACHED , & BlockStateProperties :: ROTATION_16 , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: ATTACHED => BlockStateProperties :: ATTACHED . index_of (false) , BlockStateProperties :: ROTATION_16 => 0usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const MANGROVE_HANGING_SIGN : Block = Block :: new ("mangrove_hanging_sign" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: ATTACHED , & BlockStateProperties :: ROTATION_16 , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: ATTACHED => BlockStateProperties :: ATTACHED . index_of (false) , BlockStateProperties :: ROTATION_16 => 0usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const BAMBOO_HANGING_SIGN : Block = Block :: new ("bamboo_hanging_sign" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: ATTACHED , & BlockStateProperties :: ROTATION_16 , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: ATTACHED => BlockStateProperties :: ATTACHED . index_of (false) , BlockStateProperties :: ROTATION_16 => 0usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const OAK_WALL_HANGING_SIGN : Block = Block :: new ("oak_wall_hanging_sign" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const SPRUCE_WALL_HANGING_SIGN : Block = Block :: new ("spruce_wall_hanging_sign" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const BIRCH_WALL_HANGING_SIGN : Block = Block :: new ("birch_wall_hanging_sign" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const ACACIA_WALL_HANGING_SIGN : Block = Block :: new ("acacia_wall_hanging_sign" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const CHERRY_WALL_HANGING_SIGN : Block = Block :: new ("cherry_wall_hanging_sign" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const JUNGLE_WALL_HANGING_SIGN : Block = Block :: new ("jungle_wall_hanging_sign" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const DARK_OAK_WALL_HANGING_SIGN : Block = Block :: new ("dark_oak_wall_hanging_sign" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const PALE_OAK_WALL_HANGING_SIGN : Block = Block :: new ("pale_oak_wall_hanging_sign" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const MANGROVE_WALL_HANGING_SIGN : Block = Block :: new ("mangrove_wall_hanging_sign" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const CRIMSON_WALL_HANGING_SIGN : Block = Block :: new ("crimson_wall_hanging_sign" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const WARPED_WALL_HANGING_SIGN : Block = Block :: new ("warped_wall_hanging_sign" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const BAMBOO_WALL_HANGING_SIGN : Block = Block :: new ("bamboo_wall_hanging_sign" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const LEVER : Block = Block :: new ("lever" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: ATTACH_FACE , & BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: POWERED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: ATTACH_FACE => crate :: properties :: AttachFace :: Wall as usize , BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false))) ;
pub const STONE_PRESSURE_PLATE : Block = Block :: new ("stone_pressure_plate" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: POWERED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false))) ;
pub const IRON_DOOR : Block = Block :: new ("iron_door" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: DOUBLE_BLOCK_HALF , & BlockStateProperties :: DOOR_HINGE , & BlockStateProperties :: OPEN , & BlockStateProperties :: POWERED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: DOUBLE_BLOCK_HALF => crate :: properties :: DoubleBlockHalf :: Lower as usize , BlockStateProperties :: DOOR_HINGE => crate :: properties :: DoorHingeSide :: Left as usize , BlockStateProperties :: OPEN => BlockStateProperties :: OPEN . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false))) ;
pub const OAK_PRESSURE_PLATE : Block = Block :: new ("oak_pressure_plate" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: POWERED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false))) ;
pub const SPRUCE_PRESSURE_PLATE : Block = Block :: new ("spruce_pressure_plate" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: POWERED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false))) ;
pub const BIRCH_PRESSURE_PLATE : Block = Block :: new ("birch_pressure_plate" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: POWERED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false))) ;
pub const JUNGLE_PRESSURE_PLATE : Block = Block :: new ("jungle_pressure_plate" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: POWERED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false))) ;
pub const ACACIA_PRESSURE_PLATE : Block = Block :: new ("acacia_pressure_plate" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: POWERED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false))) ;
pub const CHERRY_PRESSURE_PLATE : Block = Block :: new ("cherry_pressure_plate" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: POWERED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false))) ;
pub const DARK_OAK_PRESSURE_PLATE : Block = Block :: new ("dark_oak_pressure_plate" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: POWERED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false))) ;
pub const PALE_OAK_PRESSURE_PLATE : Block = Block :: new ("pale_oak_pressure_plate" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: POWERED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false))) ;
pub const MANGROVE_PRESSURE_PLATE : Block = Block :: new ("mangrove_pressure_plate" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: POWERED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false))) ;
pub const BAMBOO_PRESSURE_PLATE : Block = Block :: new ("bamboo_pressure_plate" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: POWERED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false))) ;
pub const REDSTONE_ORE : Block = Block :: new ("redstone_ore" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: LIT] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: LIT => BlockStateProperties :: LIT . index_of (false))) ;
pub const DEEPSLATE_REDSTONE_ORE : Block = Block :: new ("deepslate_redstone_ore" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: LIT] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: LIT => BlockStateProperties :: LIT . index_of (false))) ;
pub const REDSTONE_TORCH : Block = Block :: new ("redstone_torch" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: LIT] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: LIT => BlockStateProperties :: LIT . index_of (true))) ;
pub const REDSTONE_WALL_TORCH : Block = Block :: new ("redstone_wall_torch" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: LIT] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: LIT => BlockStateProperties :: LIT . index_of (true))) ;
pub const STONE_BUTTON : Block = Block :: new ("stone_button" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: ATTACH_FACE , & BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: POWERED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: ATTACH_FACE => crate :: properties :: AttachFace :: Wall as usize , BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false))) ;
pub const SNOW: Block = Block::new(
    "snow",
    BlockBehaviourProperties::new(),
    &[&BlockStateProperties::LAYERS],
)
.with_default_state(crate :: blocks :: offset ! (BlockStateProperties :: LAYERS => 1usize));
pub const ICE: Block = Block::new("ice", BlockBehaviourProperties::new(), &[]);
pub const SNOW_BLOCK: Block = Block::new("snow_block", BlockBehaviourProperties::new(), &[]);
pub const CACTUS: Block = Block::new(
    "cactus",
    BlockBehaviourProperties::new(),
    &[&BlockStateProperties::AGE_15],
)
.with_default_state(crate :: blocks :: offset ! (BlockStateProperties :: AGE_15 => 0usize));
pub const CACTUS_FLOWER: Block = Block::new("cactus_flower", BlockBehaviourProperties::new(), &[]);
pub const CLAY: Block = Block::new("clay", BlockBehaviourProperties::new(), &[]);
pub const SUGAR_CANE: Block = Block::new(
    "sugar_cane",
    BlockBehaviourProperties::new(),
    &[&BlockStateProperties::AGE_15],
)
.with_default_state(crate :: blocks :: offset ! (BlockStateProperties :: AGE_15 => 0usize));
pub const JUKEBOX : Block = Block :: new ("jukebox" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HAS_RECORD] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HAS_RECORD => BlockStateProperties :: HAS_RECORD . index_of (false))) ;
pub const OAK_FENCE : Block = Block :: new ("oak_fence" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: EAST , & BlockStateProperties :: NORTH , & BlockStateProperties :: SOUTH , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: EAST => BlockStateProperties :: EAST . index_of (false) , BlockStateProperties :: NORTH => BlockStateProperties :: NORTH . index_of (false) , BlockStateProperties :: SOUTH => BlockStateProperties :: SOUTH . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST => BlockStateProperties :: WEST . index_of (false))) ;
pub const NETHERRACK: Block = Block::new("netherrack", BlockBehaviourProperties::new(), &[]);
pub const SOUL_SAND: Block = Block::new("soul_sand", BlockBehaviourProperties::new(), &[]);
pub const SOUL_SOIL: Block = Block::new("soul_soil", BlockBehaviourProperties::new(), &[]);
pub const BASALT : Block = Block :: new ("basalt" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: AXIS] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: AXIS => crate :: properties :: Axis :: Y as usize)) ;
pub const POLISHED_BASALT : Block = Block :: new ("polished_basalt" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: AXIS] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: AXIS => crate :: properties :: Axis :: Y as usize)) ;
pub const SOUL_TORCH: Block = Block::new("soul_torch", BlockBehaviourProperties::new(), &[]);
pub const SOUL_WALL_TORCH : Block = Block :: new ("soul_wall_torch" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize)) ;
pub const COPPER_TORCH: Block = Block::new("copper_torch", BlockBehaviourProperties::new(), &[]);
pub const COPPER_WALL_TORCH : Block = Block :: new ("copper_wall_torch" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize)) ;
pub const GLOWSTONE: Block = Block::new("glowstone", BlockBehaviourProperties::new(), &[]);
pub const NETHER_PORTAL : Block = Block :: new ("nether_portal" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_AXIS] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_AXIS => crate :: properties :: Axis :: X as usize)) ;
pub const CARVED_PUMPKIN : Block = Block :: new ("carved_pumpkin" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize)) ;
pub const JACK_O_LANTERN : Block = Block :: new ("jack_o_lantern" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize)) ;
pub const CAKE: Block = Block::new(
    "cake",
    BlockBehaviourProperties::new(),
    &[&BlockStateProperties::BITES],
)
.with_default_state(crate :: blocks :: offset ! (BlockStateProperties :: BITES => 0usize));
pub const REPEATER : Block = Block :: new ("repeater" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: DELAY , & BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: LOCKED , & BlockStateProperties :: POWERED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: DELAY => 1usize , BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: LOCKED => BlockStateProperties :: LOCKED . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false))) ;
pub const WHITE_STAINED_GLASS: Block =
    Block::new("white_stained_glass", BlockBehaviourProperties::new(), &[]);
pub const ORANGE_STAINED_GLASS: Block =
    Block::new("orange_stained_glass", BlockBehaviourProperties::new(), &[]);
pub const MAGENTA_STAINED_GLASS: Block = Block::new(
    "magenta_stained_glass",
    BlockBehaviourProperties::new(),
    &[],
);
pub const LIGHT_BLUE_STAINED_GLASS: Block = Block::new(
    "light_blue_stained_glass",
    BlockBehaviourProperties::new(),
    &[],
);
pub const YELLOW_STAINED_GLASS: Block =
    Block::new("yellow_stained_glass", BlockBehaviourProperties::new(), &[]);
pub const LIME_STAINED_GLASS: Block =
    Block::new("lime_stained_glass", BlockBehaviourProperties::new(), &[]);
pub const PINK_STAINED_GLASS: Block =
    Block::new("pink_stained_glass", BlockBehaviourProperties::new(), &[]);
pub const GRAY_STAINED_GLASS: Block =
    Block::new("gray_stained_glass", BlockBehaviourProperties::new(), &[]);
pub const LIGHT_GRAY_STAINED_GLASS: Block = Block::new(
    "light_gray_stained_glass",
    BlockBehaviourProperties::new(),
    &[],
);
pub const CYAN_STAINED_GLASS: Block =
    Block::new("cyan_stained_glass", BlockBehaviourProperties::new(), &[]);
pub const PURPLE_STAINED_GLASS: Block =
    Block::new("purple_stained_glass", BlockBehaviourProperties::new(), &[]);
pub const BLUE_STAINED_GLASS: Block =
    Block::new("blue_stained_glass", BlockBehaviourProperties::new(), &[]);
pub const BROWN_STAINED_GLASS: Block =
    Block::new("brown_stained_glass", BlockBehaviourProperties::new(), &[]);
pub const GREEN_STAINED_GLASS: Block =
    Block::new("green_stained_glass", BlockBehaviourProperties::new(), &[]);
pub const RED_STAINED_GLASS: Block =
    Block::new("red_stained_glass", BlockBehaviourProperties::new(), &[]);
pub const BLACK_STAINED_GLASS: Block =
    Block::new("black_stained_glass", BlockBehaviourProperties::new(), &[]);
pub const OAK_TRAPDOOR : Block = Block :: new ("oak_trapdoor" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: OPEN , & BlockStateProperties :: POWERED , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: HALF => crate :: properties :: Half :: Bottom as usize , BlockStateProperties :: OPEN => BlockStateProperties :: OPEN . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const SPRUCE_TRAPDOOR : Block = Block :: new ("spruce_trapdoor" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: OPEN , & BlockStateProperties :: POWERED , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: HALF => crate :: properties :: Half :: Bottom as usize , BlockStateProperties :: OPEN => BlockStateProperties :: OPEN . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const BIRCH_TRAPDOOR : Block = Block :: new ("birch_trapdoor" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: OPEN , & BlockStateProperties :: POWERED , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: HALF => crate :: properties :: Half :: Bottom as usize , BlockStateProperties :: OPEN => BlockStateProperties :: OPEN . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const JUNGLE_TRAPDOOR : Block = Block :: new ("jungle_trapdoor" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: OPEN , & BlockStateProperties :: POWERED , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: HALF => crate :: properties :: Half :: Bottom as usize , BlockStateProperties :: OPEN => BlockStateProperties :: OPEN . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const ACACIA_TRAPDOOR : Block = Block :: new ("acacia_trapdoor" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: OPEN , & BlockStateProperties :: POWERED , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: HALF => crate :: properties :: Half :: Bottom as usize , BlockStateProperties :: OPEN => BlockStateProperties :: OPEN . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const CHERRY_TRAPDOOR : Block = Block :: new ("cherry_trapdoor" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: OPEN , & BlockStateProperties :: POWERED , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: HALF => crate :: properties :: Half :: Bottom as usize , BlockStateProperties :: OPEN => BlockStateProperties :: OPEN . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const DARK_OAK_TRAPDOOR : Block = Block :: new ("dark_oak_trapdoor" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: OPEN , & BlockStateProperties :: POWERED , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: HALF => crate :: properties :: Half :: Bottom as usize , BlockStateProperties :: OPEN => BlockStateProperties :: OPEN . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const PALE_OAK_TRAPDOOR : Block = Block :: new ("pale_oak_trapdoor" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: OPEN , & BlockStateProperties :: POWERED , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: HALF => crate :: properties :: Half :: Bottom as usize , BlockStateProperties :: OPEN => BlockStateProperties :: OPEN . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const MANGROVE_TRAPDOOR : Block = Block :: new ("mangrove_trapdoor" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: OPEN , & BlockStateProperties :: POWERED , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: HALF => crate :: properties :: Half :: Bottom as usize , BlockStateProperties :: OPEN => BlockStateProperties :: OPEN . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const BAMBOO_TRAPDOOR : Block = Block :: new ("bamboo_trapdoor" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: OPEN , & BlockStateProperties :: POWERED , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: HALF => crate :: properties :: Half :: Bottom as usize , BlockStateProperties :: OPEN => BlockStateProperties :: OPEN . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const STONE_BRICKS: Block = Block::new("stone_bricks", BlockBehaviourProperties::new(), &[]);
pub const MOSSY_STONE_BRICKS: Block =
    Block::new("mossy_stone_bricks", BlockBehaviourProperties::new(), &[]);
pub const CRACKED_STONE_BRICKS: Block =
    Block::new("cracked_stone_bricks", BlockBehaviourProperties::new(), &[]);
pub const CHISELED_STONE_BRICKS: Block = Block::new(
    "chiseled_stone_bricks",
    BlockBehaviourProperties::new(),
    &[],
);
pub const PACKED_MUD: Block = Block::new("packed_mud", BlockBehaviourProperties::new(), &[]);
pub const MUD_BRICKS: Block = Block::new("mud_bricks", BlockBehaviourProperties::new(), &[]);
pub const INFESTED_STONE: Block =
    Block::new("infested_stone", BlockBehaviourProperties::new(), &[]);
pub const INFESTED_COBBLESTONE: Block =
    Block::new("infested_cobblestone", BlockBehaviourProperties::new(), &[]);
pub const INFESTED_STONE_BRICKS: Block = Block::new(
    "infested_stone_bricks",
    BlockBehaviourProperties::new(),
    &[],
);
pub const INFESTED_MOSSY_STONE_BRICKS: Block = Block::new(
    "infested_mossy_stone_bricks",
    BlockBehaviourProperties::new(),
    &[],
);
pub const INFESTED_CRACKED_STONE_BRICKS: Block = Block::new(
    "infested_cracked_stone_bricks",
    BlockBehaviourProperties::new(),
    &[],
);
pub const INFESTED_CHISELED_STONE_BRICKS: Block = Block::new(
    "infested_chiseled_stone_bricks",
    BlockBehaviourProperties::new(),
    &[],
);
pub const BROWN_MUSHROOM_BLOCK : Block = Block :: new ("brown_mushroom_block" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: DOWN , & BlockStateProperties :: EAST , & BlockStateProperties :: NORTH , & BlockStateProperties :: SOUTH , & BlockStateProperties :: UP , & BlockStateProperties :: WEST] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: DOWN => BlockStateProperties :: DOWN . index_of (true) , BlockStateProperties :: EAST => BlockStateProperties :: EAST . index_of (true) , BlockStateProperties :: NORTH => BlockStateProperties :: NORTH . index_of (true) , BlockStateProperties :: SOUTH => BlockStateProperties :: SOUTH . index_of (true) , BlockStateProperties :: UP => BlockStateProperties :: UP . index_of (true) , BlockStateProperties :: WEST => BlockStateProperties :: WEST . index_of (true))) ;
pub const RED_MUSHROOM_BLOCK : Block = Block :: new ("red_mushroom_block" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: DOWN , & BlockStateProperties :: EAST , & BlockStateProperties :: NORTH , & BlockStateProperties :: SOUTH , & BlockStateProperties :: UP , & BlockStateProperties :: WEST] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: DOWN => BlockStateProperties :: DOWN . index_of (true) , BlockStateProperties :: EAST => BlockStateProperties :: EAST . index_of (true) , BlockStateProperties :: NORTH => BlockStateProperties :: NORTH . index_of (true) , BlockStateProperties :: SOUTH => BlockStateProperties :: SOUTH . index_of (true) , BlockStateProperties :: UP => BlockStateProperties :: UP . index_of (true) , BlockStateProperties :: WEST => BlockStateProperties :: WEST . index_of (true))) ;
pub const MUSHROOM_STEM : Block = Block :: new ("mushroom_stem" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: DOWN , & BlockStateProperties :: EAST , & BlockStateProperties :: NORTH , & BlockStateProperties :: SOUTH , & BlockStateProperties :: UP , & BlockStateProperties :: WEST] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: DOWN => BlockStateProperties :: DOWN . index_of (true) , BlockStateProperties :: EAST => BlockStateProperties :: EAST . index_of (true) , BlockStateProperties :: NORTH => BlockStateProperties :: NORTH . index_of (true) , BlockStateProperties :: SOUTH => BlockStateProperties :: SOUTH . index_of (true) , BlockStateProperties :: UP => BlockStateProperties :: UP . index_of (true) , BlockStateProperties :: WEST => BlockStateProperties :: WEST . index_of (true))) ;
pub const IRON_BARS : Block = Block :: new ("iron_bars" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: EAST , & BlockStateProperties :: NORTH , & BlockStateProperties :: SOUTH , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: EAST => BlockStateProperties :: EAST . index_of (false) , BlockStateProperties :: NORTH => BlockStateProperties :: NORTH . index_of (false) , BlockStateProperties :: SOUTH => BlockStateProperties :: SOUTH . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST => BlockStateProperties :: WEST . index_of (false))) ;
pub const COPPER_BARS : Block = Block :: new ("copper_bars" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: EAST , & BlockStateProperties :: NORTH , & BlockStateProperties :: SOUTH , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: EAST => BlockStateProperties :: EAST . index_of (false) , BlockStateProperties :: NORTH => BlockStateProperties :: NORTH . index_of (false) , BlockStateProperties :: SOUTH => BlockStateProperties :: SOUTH . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST => BlockStateProperties :: WEST . index_of (false))) ;
pub const EXPOSED_COPPER_BARS : Block = Block :: new ("exposed_copper_bars" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: EAST , & BlockStateProperties :: NORTH , & BlockStateProperties :: SOUTH , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: EAST => BlockStateProperties :: EAST . index_of (false) , BlockStateProperties :: NORTH => BlockStateProperties :: NORTH . index_of (false) , BlockStateProperties :: SOUTH => BlockStateProperties :: SOUTH . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST => BlockStateProperties :: WEST . index_of (false))) ;
pub const WEATHERED_COPPER_BARS : Block = Block :: new ("weathered_copper_bars" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: EAST , & BlockStateProperties :: NORTH , & BlockStateProperties :: SOUTH , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: EAST => BlockStateProperties :: EAST . index_of (false) , BlockStateProperties :: NORTH => BlockStateProperties :: NORTH . index_of (false) , BlockStateProperties :: SOUTH => BlockStateProperties :: SOUTH . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST => BlockStateProperties :: WEST . index_of (false))) ;
pub const OXIDIZED_COPPER_BARS : Block = Block :: new ("oxidized_copper_bars" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: EAST , & BlockStateProperties :: NORTH , & BlockStateProperties :: SOUTH , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: EAST => BlockStateProperties :: EAST . index_of (false) , BlockStateProperties :: NORTH => BlockStateProperties :: NORTH . index_of (false) , BlockStateProperties :: SOUTH => BlockStateProperties :: SOUTH . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST => BlockStateProperties :: WEST . index_of (false))) ;
pub const WAXED_COPPER_BARS : Block = Block :: new ("waxed_copper_bars" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: EAST , & BlockStateProperties :: NORTH , & BlockStateProperties :: SOUTH , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: EAST => BlockStateProperties :: EAST . index_of (false) , BlockStateProperties :: NORTH => BlockStateProperties :: NORTH . index_of (false) , BlockStateProperties :: SOUTH => BlockStateProperties :: SOUTH . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST => BlockStateProperties :: WEST . index_of (false))) ;
pub const WAXED_EXPOSED_COPPER_BARS : Block = Block :: new ("waxed_exposed_copper_bars" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: EAST , & BlockStateProperties :: NORTH , & BlockStateProperties :: SOUTH , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: EAST => BlockStateProperties :: EAST . index_of (false) , BlockStateProperties :: NORTH => BlockStateProperties :: NORTH . index_of (false) , BlockStateProperties :: SOUTH => BlockStateProperties :: SOUTH . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST => BlockStateProperties :: WEST . index_of (false))) ;
pub const WAXED_WEATHERED_COPPER_BARS : Block = Block :: new ("waxed_weathered_copper_bars" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: EAST , & BlockStateProperties :: NORTH , & BlockStateProperties :: SOUTH , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: EAST => BlockStateProperties :: EAST . index_of (false) , BlockStateProperties :: NORTH => BlockStateProperties :: NORTH . index_of (false) , BlockStateProperties :: SOUTH => BlockStateProperties :: SOUTH . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST => BlockStateProperties :: WEST . index_of (false))) ;
pub const WAXED_OXIDIZED_COPPER_BARS : Block = Block :: new ("waxed_oxidized_copper_bars" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: EAST , & BlockStateProperties :: NORTH , & BlockStateProperties :: SOUTH , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: EAST => BlockStateProperties :: EAST . index_of (false) , BlockStateProperties :: NORTH => BlockStateProperties :: NORTH . index_of (false) , BlockStateProperties :: SOUTH => BlockStateProperties :: SOUTH . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST => BlockStateProperties :: WEST . index_of (false))) ;
pub const IRON_CHAIN : Block = Block :: new ("iron_chain" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: AXIS , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: AXIS => crate :: properties :: Axis :: Y as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const COPPER_CHAIN : Block = Block :: new ("copper_chain" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: AXIS , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: AXIS => crate :: properties :: Axis :: Y as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const EXPOSED_COPPER_CHAIN : Block = Block :: new ("exposed_copper_chain" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: AXIS , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: AXIS => crate :: properties :: Axis :: Y as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const WEATHERED_COPPER_CHAIN : Block = Block :: new ("weathered_copper_chain" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: AXIS , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: AXIS => crate :: properties :: Axis :: Y as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const OXIDIZED_COPPER_CHAIN : Block = Block :: new ("oxidized_copper_chain" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: AXIS , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: AXIS => crate :: properties :: Axis :: Y as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const WAXED_COPPER_CHAIN : Block = Block :: new ("waxed_copper_chain" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: AXIS , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: AXIS => crate :: properties :: Axis :: Y as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const WAXED_EXPOSED_COPPER_CHAIN : Block = Block :: new ("waxed_exposed_copper_chain" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: AXIS , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: AXIS => crate :: properties :: Axis :: Y as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const WAXED_WEATHERED_COPPER_CHAIN : Block = Block :: new ("waxed_weathered_copper_chain" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: AXIS , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: AXIS => crate :: properties :: Axis :: Y as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const WAXED_OXIDIZED_COPPER_CHAIN : Block = Block :: new ("waxed_oxidized_copper_chain" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: AXIS , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: AXIS => crate :: properties :: Axis :: Y as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const GLASS_PANE : Block = Block :: new ("glass_pane" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: EAST , & BlockStateProperties :: NORTH , & BlockStateProperties :: SOUTH , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: EAST => BlockStateProperties :: EAST . index_of (false) , BlockStateProperties :: NORTH => BlockStateProperties :: NORTH . index_of (false) , BlockStateProperties :: SOUTH => BlockStateProperties :: SOUTH . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST => BlockStateProperties :: WEST . index_of (false))) ;
pub const PUMPKIN: Block = Block::new("pumpkin", BlockBehaviourProperties::new(), &[]);
pub const MELON: Block = Block::new("melon", BlockBehaviourProperties::new(), &[]);
pub const ATTACHED_PUMPKIN_STEM : Block = Block :: new ("attached_pumpkin_stem" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize)) ;
pub const ATTACHED_MELON_STEM : Block = Block :: new ("attached_melon_stem" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize)) ;
pub const PUMPKIN_STEM: Block = Block::new(
    "pumpkin_stem",
    BlockBehaviourProperties::new(),
    &[&BlockStateProperties::AGE_7],
)
.with_default_state(crate :: blocks :: offset ! (BlockStateProperties :: AGE_7 => 0usize));
pub const MELON_STEM: Block = Block::new(
    "melon_stem",
    BlockBehaviourProperties::new(),
    &[&BlockStateProperties::AGE_7],
)
.with_default_state(crate :: blocks :: offset ! (BlockStateProperties :: AGE_7 => 0usize));
pub const VINE : Block = Block :: new ("vine" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: EAST , & BlockStateProperties :: NORTH , & BlockStateProperties :: SOUTH , & BlockStateProperties :: UP , & BlockStateProperties :: WEST] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: EAST => BlockStateProperties :: EAST . index_of (false) , BlockStateProperties :: NORTH => BlockStateProperties :: NORTH . index_of (false) , BlockStateProperties :: SOUTH => BlockStateProperties :: SOUTH . index_of (false) , BlockStateProperties :: UP => BlockStateProperties :: UP . index_of (false) , BlockStateProperties :: WEST => BlockStateProperties :: WEST . index_of (false))) ;
pub const GLOW_LICHEN : Block = Block :: new ("glow_lichen" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: DOWN , & BlockStateProperties :: EAST , & BlockStateProperties :: NORTH , & BlockStateProperties :: SOUTH , & BlockStateProperties :: UP , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: DOWN => BlockStateProperties :: DOWN . index_of (false) , BlockStateProperties :: EAST => BlockStateProperties :: EAST . index_of (false) , BlockStateProperties :: NORTH => BlockStateProperties :: NORTH . index_of (false) , BlockStateProperties :: SOUTH => BlockStateProperties :: SOUTH . index_of (false) , BlockStateProperties :: UP => BlockStateProperties :: UP . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST => BlockStateProperties :: WEST . index_of (false))) ;
pub const RESIN_CLUMP : Block = Block :: new ("resin_clump" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: DOWN , & BlockStateProperties :: EAST , & BlockStateProperties :: NORTH , & BlockStateProperties :: SOUTH , & BlockStateProperties :: UP , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: DOWN => BlockStateProperties :: DOWN . index_of (false) , BlockStateProperties :: EAST => BlockStateProperties :: EAST . index_of (false) , BlockStateProperties :: NORTH => BlockStateProperties :: NORTH . index_of (false) , BlockStateProperties :: SOUTH => BlockStateProperties :: SOUTH . index_of (false) , BlockStateProperties :: UP => BlockStateProperties :: UP . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST => BlockStateProperties :: WEST . index_of (false))) ;
pub const OAK_FENCE_GATE : Block = Block :: new ("oak_fence_gate" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: IN_WALL , & BlockStateProperties :: OPEN , & BlockStateProperties :: POWERED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: IN_WALL => BlockStateProperties :: IN_WALL . index_of (false) , BlockStateProperties :: OPEN => BlockStateProperties :: OPEN . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false))) ;
pub const BRICK_STAIRS : Block = Block :: new ("brick_stairs" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: STAIRS_SHAPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: HALF => crate :: properties :: Half :: Bottom as usize , BlockStateProperties :: STAIRS_SHAPE => crate :: properties :: StairsShape :: Straight as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const STONE_BRICK_STAIRS : Block = Block :: new ("stone_brick_stairs" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: STAIRS_SHAPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: HALF => crate :: properties :: Half :: Bottom as usize , BlockStateProperties :: STAIRS_SHAPE => crate :: properties :: StairsShape :: Straight as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const MUD_BRICK_STAIRS : Block = Block :: new ("mud_brick_stairs" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: STAIRS_SHAPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: HALF => crate :: properties :: Half :: Bottom as usize , BlockStateProperties :: STAIRS_SHAPE => crate :: properties :: StairsShape :: Straight as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const MYCELIUM : Block = Block :: new ("mycelium" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: SNOWY] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: SNOWY => BlockStateProperties :: SNOWY . index_of (false))) ;
pub const LILY_PAD: Block = Block::new("lily_pad", BlockBehaviourProperties::new(), &[]);
pub const RESIN_BLOCK: Block = Block::new("resin_block", BlockBehaviourProperties::new(), &[]);
pub const RESIN_BRICKS: Block = Block::new("resin_bricks", BlockBehaviourProperties::new(), &[]);
pub const RESIN_BRICK_STAIRS : Block = Block :: new ("resin_brick_stairs" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: STAIRS_SHAPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: HALF => crate :: properties :: Half :: Bottom as usize , BlockStateProperties :: STAIRS_SHAPE => crate :: properties :: StairsShape :: Straight as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const RESIN_BRICK_SLAB : Block = Block :: new ("resin_brick_slab" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: SLAB_TYPE => crate :: properties :: SlabType :: Bottom as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const RESIN_BRICK_WALL : Block = Block :: new ("resin_brick_wall" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: EAST_WALL , & BlockStateProperties :: NORTH_WALL , & BlockStateProperties :: SOUTH_WALL , & BlockStateProperties :: UP , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST_WALL] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: EAST_WALL => crate :: properties :: WallSide :: None as usize , BlockStateProperties :: NORTH_WALL => crate :: properties :: WallSide :: None as usize , BlockStateProperties :: SOUTH_WALL => crate :: properties :: WallSide :: None as usize , BlockStateProperties :: UP => BlockStateProperties :: UP . index_of (true) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST_WALL => crate :: properties :: WallSide :: None as usize)) ;
pub const CHISELED_RESIN_BRICKS: Block = Block::new(
    "chiseled_resin_bricks",
    BlockBehaviourProperties::new(),
    &[],
);
pub const NETHER_BRICKS: Block = Block::new("nether_bricks", BlockBehaviourProperties::new(), &[]);
pub const NETHER_BRICK_FENCE : Block = Block :: new ("nether_brick_fence" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: EAST , & BlockStateProperties :: NORTH , & BlockStateProperties :: SOUTH , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: EAST => BlockStateProperties :: EAST . index_of (false) , BlockStateProperties :: NORTH => BlockStateProperties :: NORTH . index_of (false) , BlockStateProperties :: SOUTH => BlockStateProperties :: SOUTH . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST => BlockStateProperties :: WEST . index_of (false))) ;
pub const NETHER_BRICK_STAIRS : Block = Block :: new ("nether_brick_stairs" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: STAIRS_SHAPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: HALF => crate :: properties :: Half :: Bottom as usize , BlockStateProperties :: STAIRS_SHAPE => crate :: properties :: StairsShape :: Straight as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const NETHER_WART: Block = Block::new(
    "nether_wart",
    BlockBehaviourProperties::new(),
    &[&BlockStateProperties::AGE_3],
)
.with_default_state(crate :: blocks :: offset ! (BlockStateProperties :: AGE_3 => 0usize));
pub const ENCHANTING_TABLE: Block =
    Block::new("enchanting_table", BlockBehaviourProperties::new(), &[]);
pub const BREWING_STAND : Block = Block :: new ("brewing_stand" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HAS_BOTTLE_0 , & BlockStateProperties :: HAS_BOTTLE_1 , & BlockStateProperties :: HAS_BOTTLE_2] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HAS_BOTTLE_0 => BlockStateProperties :: HAS_BOTTLE_0 . index_of (false) , BlockStateProperties :: HAS_BOTTLE_1 => BlockStateProperties :: HAS_BOTTLE_1 . index_of (false) , BlockStateProperties :: HAS_BOTTLE_2 => BlockStateProperties :: HAS_BOTTLE_2 . index_of (false))) ;
pub const CAULDRON: Block = Block::new("cauldron", BlockBehaviourProperties::new(), &[]);
pub const WATER_CAULDRON: Block = Block::new(
    "water_cauldron",
    BlockBehaviourProperties::new(),
    &[&BlockStateProperties::LEVEL_CAULDRON],
)
.with_default_state(crate :: blocks :: offset ! (BlockStateProperties :: LEVEL_CAULDRON => 1usize));
pub const LAVA_CAULDRON: Block = Block::new("lava_cauldron", BlockBehaviourProperties::new(), &[]);
pub const POWDER_SNOW_CAULDRON: Block = Block::new(
    "powder_snow_cauldron",
    BlockBehaviourProperties::new(),
    &[&BlockStateProperties::LEVEL_CAULDRON],
)
.with_default_state(crate :: blocks :: offset ! (BlockStateProperties :: LEVEL_CAULDRON => 1usize));
pub const END_PORTAL: Block = Block::new("end_portal", BlockBehaviourProperties::new(), &[]);
pub const END_PORTAL_FRAME : Block = Block :: new ("end_portal_frame" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: EYE , & BlockStateProperties :: HORIZONTAL_FACING] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: EYE => BlockStateProperties :: EYE . index_of (false) , BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize)) ;
pub const END_STONE: Block = Block::new("end_stone", BlockBehaviourProperties::new(), &[]);
pub const DRAGON_EGG: Block = Block::new("dragon_egg", BlockBehaviourProperties::new(), &[]);
pub const REDSTONE_LAMP : Block = Block :: new ("redstone_lamp" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: LIT] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: LIT => BlockStateProperties :: LIT . index_of (false))) ;
pub const COCOA : Block = Block :: new ("cocoa" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: AGE_2 , & BlockStateProperties :: HORIZONTAL_FACING] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: AGE_2 => 0usize , BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize)) ;
pub const SANDSTONE_STAIRS : Block = Block :: new ("sandstone_stairs" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: STAIRS_SHAPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: HALF => crate :: properties :: Half :: Bottom as usize , BlockStateProperties :: STAIRS_SHAPE => crate :: properties :: StairsShape :: Straight as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const EMERALD_ORE: Block = Block::new("emerald_ore", BlockBehaviourProperties::new(), &[]);
pub const DEEPSLATE_EMERALD_ORE: Block = Block::new(
    "deepslate_emerald_ore",
    BlockBehaviourProperties::new(),
    &[],
);
pub const ENDER_CHEST : Block = Block :: new ("ender_chest" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const TRIPWIRE_HOOK : Block = Block :: new ("tripwire_hook" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: ATTACHED , & BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: POWERED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: ATTACHED => BlockStateProperties :: ATTACHED . index_of (false) , BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false))) ;
pub const TRIPWIRE : Block = Block :: new ("tripwire" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: ATTACHED , & BlockStateProperties :: DISARMED , & BlockStateProperties :: EAST , & BlockStateProperties :: NORTH , & BlockStateProperties :: POWERED , & BlockStateProperties :: SOUTH , & BlockStateProperties :: WEST] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: ATTACHED => BlockStateProperties :: ATTACHED . index_of (false) , BlockStateProperties :: DISARMED => BlockStateProperties :: DISARMED . index_of (false) , BlockStateProperties :: EAST => BlockStateProperties :: EAST . index_of (false) , BlockStateProperties :: NORTH => BlockStateProperties :: NORTH . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false) , BlockStateProperties :: SOUTH => BlockStateProperties :: SOUTH . index_of (false) , BlockStateProperties :: WEST => BlockStateProperties :: WEST . index_of (false))) ;
pub const EMERALD_BLOCK: Block = Block::new("emerald_block", BlockBehaviourProperties::new(), &[]);
pub const SPRUCE_STAIRS : Block = Block :: new ("spruce_stairs" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: STAIRS_SHAPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: HALF => crate :: properties :: Half :: Bottom as usize , BlockStateProperties :: STAIRS_SHAPE => crate :: properties :: StairsShape :: Straight as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const BIRCH_STAIRS : Block = Block :: new ("birch_stairs" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: STAIRS_SHAPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: HALF => crate :: properties :: Half :: Bottom as usize , BlockStateProperties :: STAIRS_SHAPE => crate :: properties :: StairsShape :: Straight as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const JUNGLE_STAIRS : Block = Block :: new ("jungle_stairs" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: STAIRS_SHAPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: HALF => crate :: properties :: Half :: Bottom as usize , BlockStateProperties :: STAIRS_SHAPE => crate :: properties :: StairsShape :: Straight as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const COMMAND_BLOCK : Block = Block :: new ("command_block" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: CONDITIONAL , & BlockStateProperties :: FACING] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: CONDITIONAL => BlockStateProperties :: CONDITIONAL . index_of (false) , BlockStateProperties :: FACING => crate :: properties :: Direction :: North as usize)) ;
pub const BEACON: Block = Block::new("beacon", BlockBehaviourProperties::new(), &[]);
pub const COBBLESTONE_WALL : Block = Block :: new ("cobblestone_wall" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: EAST_WALL , & BlockStateProperties :: NORTH_WALL , & BlockStateProperties :: SOUTH_WALL , & BlockStateProperties :: UP , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST_WALL] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: EAST_WALL => crate :: properties :: WallSide :: None as usize , BlockStateProperties :: NORTH_WALL => crate :: properties :: WallSide :: None as usize , BlockStateProperties :: SOUTH_WALL => crate :: properties :: WallSide :: None as usize , BlockStateProperties :: UP => BlockStateProperties :: UP . index_of (true) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST_WALL => crate :: properties :: WallSide :: None as usize)) ;
pub const MOSSY_COBBLESTONE_WALL : Block = Block :: new ("mossy_cobblestone_wall" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: EAST_WALL , & BlockStateProperties :: NORTH_WALL , & BlockStateProperties :: SOUTH_WALL , & BlockStateProperties :: UP , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST_WALL] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: EAST_WALL => crate :: properties :: WallSide :: None as usize , BlockStateProperties :: NORTH_WALL => crate :: properties :: WallSide :: None as usize , BlockStateProperties :: SOUTH_WALL => crate :: properties :: WallSide :: None as usize , BlockStateProperties :: UP => BlockStateProperties :: UP . index_of (true) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST_WALL => crate :: properties :: WallSide :: None as usize)) ;
pub const FLOWER_POT: Block = Block::new("flower_pot", BlockBehaviourProperties::new(), &[]);
pub const POTTED_TORCHFLOWER: Block =
    Block::new("potted_torchflower", BlockBehaviourProperties::new(), &[]);
pub const POTTED_OAK_SAPLING: Block =
    Block::new("potted_oak_sapling", BlockBehaviourProperties::new(), &[]);
pub const POTTED_SPRUCE_SAPLING: Block = Block::new(
    "potted_spruce_sapling",
    BlockBehaviourProperties::new(),
    &[],
);
pub const POTTED_BIRCH_SAPLING: Block =
    Block::new("potted_birch_sapling", BlockBehaviourProperties::new(), &[]);
pub const POTTED_JUNGLE_SAPLING: Block = Block::new(
    "potted_jungle_sapling",
    BlockBehaviourProperties::new(),
    &[],
);
pub const POTTED_ACACIA_SAPLING: Block = Block::new(
    "potted_acacia_sapling",
    BlockBehaviourProperties::new(),
    &[],
);
pub const POTTED_CHERRY_SAPLING: Block = Block::new(
    "potted_cherry_sapling",
    BlockBehaviourProperties::new(),
    &[],
);
pub const POTTED_DARK_OAK_SAPLING: Block = Block::new(
    "potted_dark_oak_sapling",
    BlockBehaviourProperties::new(),
    &[],
);
pub const POTTED_PALE_OAK_SAPLING: Block = Block::new(
    "potted_pale_oak_sapling",
    BlockBehaviourProperties::new(),
    &[],
);
pub const POTTED_MANGROVE_PROPAGULE: Block = Block::new(
    "potted_mangrove_propagule",
    BlockBehaviourProperties::new(),
    &[],
);
pub const POTTED_FERN: Block = Block::new("potted_fern", BlockBehaviourProperties::new(), &[]);
pub const POTTED_DANDELION: Block =
    Block::new("potted_dandelion", BlockBehaviourProperties::new(), &[]);
pub const POTTED_POPPY: Block = Block::new("potted_poppy", BlockBehaviourProperties::new(), &[]);
pub const POTTED_BLUE_ORCHID: Block =
    Block::new("potted_blue_orchid", BlockBehaviourProperties::new(), &[]);
pub const POTTED_ALLIUM: Block = Block::new("potted_allium", BlockBehaviourProperties::new(), &[]);
pub const POTTED_AZURE_BLUET: Block =
    Block::new("potted_azure_bluet", BlockBehaviourProperties::new(), &[]);
pub const POTTED_RED_TULIP: Block =
    Block::new("potted_red_tulip", BlockBehaviourProperties::new(), &[]);
pub const POTTED_ORANGE_TULIP: Block =
    Block::new("potted_orange_tulip", BlockBehaviourProperties::new(), &[]);
pub const POTTED_WHITE_TULIP: Block =
    Block::new("potted_white_tulip", BlockBehaviourProperties::new(), &[]);
pub const POTTED_PINK_TULIP: Block =
    Block::new("potted_pink_tulip", BlockBehaviourProperties::new(), &[]);
pub const POTTED_OXEYE_DAISY: Block =
    Block::new("potted_oxeye_daisy", BlockBehaviourProperties::new(), &[]);
pub const POTTED_CORNFLOWER: Block =
    Block::new("potted_cornflower", BlockBehaviourProperties::new(), &[]);
pub const POTTED_LILY_OF_THE_VALLEY: Block = Block::new(
    "potted_lily_of_the_valley",
    BlockBehaviourProperties::new(),
    &[],
);
pub const POTTED_WITHER_ROSE: Block =
    Block::new("potted_wither_rose", BlockBehaviourProperties::new(), &[]);
pub const POTTED_RED_MUSHROOM: Block =
    Block::new("potted_red_mushroom", BlockBehaviourProperties::new(), &[]);
pub const POTTED_BROWN_MUSHROOM: Block = Block::new(
    "potted_brown_mushroom",
    BlockBehaviourProperties::new(),
    &[],
);
pub const POTTED_DEAD_BUSH: Block =
    Block::new("potted_dead_bush", BlockBehaviourProperties::new(), &[]);
pub const POTTED_CACTUS: Block = Block::new("potted_cactus", BlockBehaviourProperties::new(), &[]);
pub const CARROTS: Block = Block::new(
    "carrots",
    BlockBehaviourProperties::new(),
    &[&BlockStateProperties::AGE_7],
)
.with_default_state(crate :: blocks :: offset ! (BlockStateProperties :: AGE_7 => 0usize));
pub const POTATOES: Block = Block::new(
    "potatoes",
    BlockBehaviourProperties::new(),
    &[&BlockStateProperties::AGE_7],
)
.with_default_state(crate :: blocks :: offset ! (BlockStateProperties :: AGE_7 => 0usize));
pub const OAK_BUTTON : Block = Block :: new ("oak_button" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: ATTACH_FACE , & BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: POWERED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: ATTACH_FACE => crate :: properties :: AttachFace :: Wall as usize , BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false))) ;
pub const SPRUCE_BUTTON : Block = Block :: new ("spruce_button" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: ATTACH_FACE , & BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: POWERED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: ATTACH_FACE => crate :: properties :: AttachFace :: Wall as usize , BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false))) ;
pub const BIRCH_BUTTON : Block = Block :: new ("birch_button" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: ATTACH_FACE , & BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: POWERED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: ATTACH_FACE => crate :: properties :: AttachFace :: Wall as usize , BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false))) ;
pub const JUNGLE_BUTTON : Block = Block :: new ("jungle_button" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: ATTACH_FACE , & BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: POWERED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: ATTACH_FACE => crate :: properties :: AttachFace :: Wall as usize , BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false))) ;
pub const ACACIA_BUTTON : Block = Block :: new ("acacia_button" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: ATTACH_FACE , & BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: POWERED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: ATTACH_FACE => crate :: properties :: AttachFace :: Wall as usize , BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false))) ;
pub const CHERRY_BUTTON : Block = Block :: new ("cherry_button" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: ATTACH_FACE , & BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: POWERED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: ATTACH_FACE => crate :: properties :: AttachFace :: Wall as usize , BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false))) ;
pub const DARK_OAK_BUTTON : Block = Block :: new ("dark_oak_button" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: ATTACH_FACE , & BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: POWERED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: ATTACH_FACE => crate :: properties :: AttachFace :: Wall as usize , BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false))) ;
pub const PALE_OAK_BUTTON : Block = Block :: new ("pale_oak_button" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: ATTACH_FACE , & BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: POWERED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: ATTACH_FACE => crate :: properties :: AttachFace :: Wall as usize , BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false))) ;
pub const MANGROVE_BUTTON : Block = Block :: new ("mangrove_button" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: ATTACH_FACE , & BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: POWERED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: ATTACH_FACE => crate :: properties :: AttachFace :: Wall as usize , BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false))) ;
pub const BAMBOO_BUTTON : Block = Block :: new ("bamboo_button" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: ATTACH_FACE , & BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: POWERED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: ATTACH_FACE => crate :: properties :: AttachFace :: Wall as usize , BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false))) ;
pub const SKELETON_SKULL : Block = Block :: new ("skeleton_skull" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: POWERED , & BlockStateProperties :: ROTATION_16] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false) , BlockStateProperties :: ROTATION_16 => 0usize)) ;
pub const SKELETON_WALL_SKULL : Block = Block :: new ("skeleton_wall_skull" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: POWERED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false))) ;
pub const WITHER_SKELETON_SKULL : Block = Block :: new ("wither_skeleton_skull" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: POWERED , & BlockStateProperties :: ROTATION_16] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false) , BlockStateProperties :: ROTATION_16 => 0usize)) ;
pub const WITHER_SKELETON_WALL_SKULL : Block = Block :: new ("wither_skeleton_wall_skull" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: POWERED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false))) ;
pub const ZOMBIE_HEAD : Block = Block :: new ("zombie_head" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: POWERED , & BlockStateProperties :: ROTATION_16] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false) , BlockStateProperties :: ROTATION_16 => 0usize)) ;
pub const ZOMBIE_WALL_HEAD : Block = Block :: new ("zombie_wall_head" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: POWERED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false))) ;
pub const PLAYER_HEAD : Block = Block :: new ("player_head" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: POWERED , & BlockStateProperties :: ROTATION_16] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false) , BlockStateProperties :: ROTATION_16 => 0usize)) ;
pub const PLAYER_WALL_HEAD : Block = Block :: new ("player_wall_head" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: POWERED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false))) ;
pub const CREEPER_HEAD : Block = Block :: new ("creeper_head" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: POWERED , & BlockStateProperties :: ROTATION_16] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false) , BlockStateProperties :: ROTATION_16 => 0usize)) ;
pub const CREEPER_WALL_HEAD : Block = Block :: new ("creeper_wall_head" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: POWERED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false))) ;
pub const DRAGON_HEAD : Block = Block :: new ("dragon_head" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: POWERED , & BlockStateProperties :: ROTATION_16] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false) , BlockStateProperties :: ROTATION_16 => 0usize)) ;
pub const DRAGON_WALL_HEAD : Block = Block :: new ("dragon_wall_head" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: POWERED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false))) ;
pub const PIGLIN_HEAD : Block = Block :: new ("piglin_head" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: POWERED , & BlockStateProperties :: ROTATION_16] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false) , BlockStateProperties :: ROTATION_16 => 0usize)) ;
pub const PIGLIN_WALL_HEAD : Block = Block :: new ("piglin_wall_head" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: POWERED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false))) ;
pub const ANVIL : Block = Block :: new ("anvil" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize)) ;
pub const CHIPPED_ANVIL : Block = Block :: new ("chipped_anvil" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize)) ;
pub const DAMAGED_ANVIL : Block = Block :: new ("damaged_anvil" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize)) ;
pub const TRAPPED_CHEST : Block = Block :: new ("trapped_chest" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: CHEST_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: CHEST_TYPE => crate :: properties :: ChestType :: Single as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const LIGHT_WEIGHTED_PRESSURE_PLATE: Block = Block::new(
    "light_weighted_pressure_plate",
    BlockBehaviourProperties::new(),
    &[&BlockStateProperties::POWER],
)
.with_default_state(crate :: blocks :: offset ! (BlockStateProperties :: POWER => 0usize));
pub const HEAVY_WEIGHTED_PRESSURE_PLATE: Block = Block::new(
    "heavy_weighted_pressure_plate",
    BlockBehaviourProperties::new(),
    &[&BlockStateProperties::POWER],
)
.with_default_state(crate :: blocks :: offset ! (BlockStateProperties :: POWER => 0usize));
pub const COMPARATOR : Block = Block :: new ("comparator" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: MODE_COMPARATOR , & BlockStateProperties :: POWERED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: MODE_COMPARATOR => crate :: properties :: ComparatorMode :: Compare as usize , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false))) ;
pub const DAYLIGHT_DETECTOR : Block = Block :: new ("daylight_detector" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: INVERTED , & BlockStateProperties :: POWER] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: INVERTED => BlockStateProperties :: INVERTED . index_of (false) , BlockStateProperties :: POWER => 0usize)) ;
pub const REDSTONE_BLOCK: Block =
    Block::new("redstone_block", BlockBehaviourProperties::new(), &[]);
pub const NETHER_QUARTZ_ORE: Block =
    Block::new("nether_quartz_ore", BlockBehaviourProperties::new(), &[]);
pub const HOPPER : Block = Block :: new ("hopper" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: ENABLED , & BlockStateProperties :: FACING_HOPPER] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: ENABLED => BlockStateProperties :: ENABLED . index_of (true) , BlockStateProperties :: FACING_HOPPER => crate :: properties :: Direction :: Down as usize)) ;
pub const QUARTZ_BLOCK: Block = Block::new("quartz_block", BlockBehaviourProperties::new(), &[]);
pub const CHISELED_QUARTZ_BLOCK: Block = Block::new(
    "chiseled_quartz_block",
    BlockBehaviourProperties::new(),
    &[],
);
pub const QUARTZ_PILLAR : Block = Block :: new ("quartz_pillar" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: AXIS] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: AXIS => crate :: properties :: Axis :: Y as usize)) ;
pub const QUARTZ_STAIRS : Block = Block :: new ("quartz_stairs" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: STAIRS_SHAPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: HALF => crate :: properties :: Half :: Bottom as usize , BlockStateProperties :: STAIRS_SHAPE => crate :: properties :: StairsShape :: Straight as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const ACTIVATOR_RAIL : Block = Block :: new ("activator_rail" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: POWERED , & BlockStateProperties :: RAIL_SHAPE_STRAIGHT , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false) , BlockStateProperties :: RAIL_SHAPE_STRAIGHT => crate :: properties :: RailShape :: NorthSouth as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const DROPPER : Block = Block :: new ("dropper" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: FACING , & BlockStateProperties :: TRIGGERED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: TRIGGERED => BlockStateProperties :: TRIGGERED . index_of (false))) ;
pub const WHITE_TERRACOTTA: Block =
    Block::new("white_terracotta", BlockBehaviourProperties::new(), &[]);
pub const ORANGE_TERRACOTTA: Block =
    Block::new("orange_terracotta", BlockBehaviourProperties::new(), &[]);
pub const MAGENTA_TERRACOTTA: Block =
    Block::new("magenta_terracotta", BlockBehaviourProperties::new(), &[]);
pub const LIGHT_BLUE_TERRACOTTA: Block = Block::new(
    "light_blue_terracotta",
    BlockBehaviourProperties::new(),
    &[],
);
pub const YELLOW_TERRACOTTA: Block =
    Block::new("yellow_terracotta", BlockBehaviourProperties::new(), &[]);
pub const LIME_TERRACOTTA: Block =
    Block::new("lime_terracotta", BlockBehaviourProperties::new(), &[]);
pub const PINK_TERRACOTTA: Block =
    Block::new("pink_terracotta", BlockBehaviourProperties::new(), &[]);
pub const GRAY_TERRACOTTA: Block =
    Block::new("gray_terracotta", BlockBehaviourProperties::new(), &[]);
pub const LIGHT_GRAY_TERRACOTTA: Block = Block::new(
    "light_gray_terracotta",
    BlockBehaviourProperties::new(),
    &[],
);
pub const CYAN_TERRACOTTA: Block =
    Block::new("cyan_terracotta", BlockBehaviourProperties::new(), &[]);
pub const PURPLE_TERRACOTTA: Block =
    Block::new("purple_terracotta", BlockBehaviourProperties::new(), &[]);
pub const BLUE_TERRACOTTA: Block =
    Block::new("blue_terracotta", BlockBehaviourProperties::new(), &[]);
pub const BROWN_TERRACOTTA: Block =
    Block::new("brown_terracotta", BlockBehaviourProperties::new(), &[]);
pub const GREEN_TERRACOTTA: Block =
    Block::new("green_terracotta", BlockBehaviourProperties::new(), &[]);
pub const RED_TERRACOTTA: Block =
    Block::new("red_terracotta", BlockBehaviourProperties::new(), &[]);
pub const BLACK_TERRACOTTA: Block =
    Block::new("black_terracotta", BlockBehaviourProperties::new(), &[]);
pub const WHITE_STAINED_GLASS_PANE : Block = Block :: new ("white_stained_glass_pane" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: EAST , & BlockStateProperties :: NORTH , & BlockStateProperties :: SOUTH , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: EAST => BlockStateProperties :: EAST . index_of (false) , BlockStateProperties :: NORTH => BlockStateProperties :: NORTH . index_of (false) , BlockStateProperties :: SOUTH => BlockStateProperties :: SOUTH . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST => BlockStateProperties :: WEST . index_of (false))) ;
pub const ORANGE_STAINED_GLASS_PANE : Block = Block :: new ("orange_stained_glass_pane" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: EAST , & BlockStateProperties :: NORTH , & BlockStateProperties :: SOUTH , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: EAST => BlockStateProperties :: EAST . index_of (false) , BlockStateProperties :: NORTH => BlockStateProperties :: NORTH . index_of (false) , BlockStateProperties :: SOUTH => BlockStateProperties :: SOUTH . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST => BlockStateProperties :: WEST . index_of (false))) ;
pub const MAGENTA_STAINED_GLASS_PANE : Block = Block :: new ("magenta_stained_glass_pane" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: EAST , & BlockStateProperties :: NORTH , & BlockStateProperties :: SOUTH , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: EAST => BlockStateProperties :: EAST . index_of (false) , BlockStateProperties :: NORTH => BlockStateProperties :: NORTH . index_of (false) , BlockStateProperties :: SOUTH => BlockStateProperties :: SOUTH . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST => BlockStateProperties :: WEST . index_of (false))) ;
pub const LIGHT_BLUE_STAINED_GLASS_PANE : Block = Block :: new ("light_blue_stained_glass_pane" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: EAST , & BlockStateProperties :: NORTH , & BlockStateProperties :: SOUTH , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: EAST => BlockStateProperties :: EAST . index_of (false) , BlockStateProperties :: NORTH => BlockStateProperties :: NORTH . index_of (false) , BlockStateProperties :: SOUTH => BlockStateProperties :: SOUTH . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST => BlockStateProperties :: WEST . index_of (false))) ;
pub const YELLOW_STAINED_GLASS_PANE : Block = Block :: new ("yellow_stained_glass_pane" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: EAST , & BlockStateProperties :: NORTH , & BlockStateProperties :: SOUTH , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: EAST => BlockStateProperties :: EAST . index_of (false) , BlockStateProperties :: NORTH => BlockStateProperties :: NORTH . index_of (false) , BlockStateProperties :: SOUTH => BlockStateProperties :: SOUTH . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST => BlockStateProperties :: WEST . index_of (false))) ;
pub const LIME_STAINED_GLASS_PANE : Block = Block :: new ("lime_stained_glass_pane" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: EAST , & BlockStateProperties :: NORTH , & BlockStateProperties :: SOUTH , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: EAST => BlockStateProperties :: EAST . index_of (false) , BlockStateProperties :: NORTH => BlockStateProperties :: NORTH . index_of (false) , BlockStateProperties :: SOUTH => BlockStateProperties :: SOUTH . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST => BlockStateProperties :: WEST . index_of (false))) ;
pub const PINK_STAINED_GLASS_PANE : Block = Block :: new ("pink_stained_glass_pane" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: EAST , & BlockStateProperties :: NORTH , & BlockStateProperties :: SOUTH , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: EAST => BlockStateProperties :: EAST . index_of (false) , BlockStateProperties :: NORTH => BlockStateProperties :: NORTH . index_of (false) , BlockStateProperties :: SOUTH => BlockStateProperties :: SOUTH . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST => BlockStateProperties :: WEST . index_of (false))) ;
pub const GRAY_STAINED_GLASS_PANE : Block = Block :: new ("gray_stained_glass_pane" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: EAST , & BlockStateProperties :: NORTH , & BlockStateProperties :: SOUTH , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: EAST => BlockStateProperties :: EAST . index_of (false) , BlockStateProperties :: NORTH => BlockStateProperties :: NORTH . index_of (false) , BlockStateProperties :: SOUTH => BlockStateProperties :: SOUTH . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST => BlockStateProperties :: WEST . index_of (false))) ;
pub const LIGHT_GRAY_STAINED_GLASS_PANE : Block = Block :: new ("light_gray_stained_glass_pane" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: EAST , & BlockStateProperties :: NORTH , & BlockStateProperties :: SOUTH , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: EAST => BlockStateProperties :: EAST . index_of (false) , BlockStateProperties :: NORTH => BlockStateProperties :: NORTH . index_of (false) , BlockStateProperties :: SOUTH => BlockStateProperties :: SOUTH . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST => BlockStateProperties :: WEST . index_of (false))) ;
pub const CYAN_STAINED_GLASS_PANE : Block = Block :: new ("cyan_stained_glass_pane" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: EAST , & BlockStateProperties :: NORTH , & BlockStateProperties :: SOUTH , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: EAST => BlockStateProperties :: EAST . index_of (false) , BlockStateProperties :: NORTH => BlockStateProperties :: NORTH . index_of (false) , BlockStateProperties :: SOUTH => BlockStateProperties :: SOUTH . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST => BlockStateProperties :: WEST . index_of (false))) ;
pub const PURPLE_STAINED_GLASS_PANE : Block = Block :: new ("purple_stained_glass_pane" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: EAST , & BlockStateProperties :: NORTH , & BlockStateProperties :: SOUTH , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: EAST => BlockStateProperties :: EAST . index_of (false) , BlockStateProperties :: NORTH => BlockStateProperties :: NORTH . index_of (false) , BlockStateProperties :: SOUTH => BlockStateProperties :: SOUTH . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST => BlockStateProperties :: WEST . index_of (false))) ;
pub const BLUE_STAINED_GLASS_PANE : Block = Block :: new ("blue_stained_glass_pane" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: EAST , & BlockStateProperties :: NORTH , & BlockStateProperties :: SOUTH , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: EAST => BlockStateProperties :: EAST . index_of (false) , BlockStateProperties :: NORTH => BlockStateProperties :: NORTH . index_of (false) , BlockStateProperties :: SOUTH => BlockStateProperties :: SOUTH . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST => BlockStateProperties :: WEST . index_of (false))) ;
pub const BROWN_STAINED_GLASS_PANE : Block = Block :: new ("brown_stained_glass_pane" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: EAST , & BlockStateProperties :: NORTH , & BlockStateProperties :: SOUTH , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: EAST => BlockStateProperties :: EAST . index_of (false) , BlockStateProperties :: NORTH => BlockStateProperties :: NORTH . index_of (false) , BlockStateProperties :: SOUTH => BlockStateProperties :: SOUTH . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST => BlockStateProperties :: WEST . index_of (false))) ;
pub const GREEN_STAINED_GLASS_PANE : Block = Block :: new ("green_stained_glass_pane" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: EAST , & BlockStateProperties :: NORTH , & BlockStateProperties :: SOUTH , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: EAST => BlockStateProperties :: EAST . index_of (false) , BlockStateProperties :: NORTH => BlockStateProperties :: NORTH . index_of (false) , BlockStateProperties :: SOUTH => BlockStateProperties :: SOUTH . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST => BlockStateProperties :: WEST . index_of (false))) ;
pub const RED_STAINED_GLASS_PANE : Block = Block :: new ("red_stained_glass_pane" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: EAST , & BlockStateProperties :: NORTH , & BlockStateProperties :: SOUTH , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: EAST => BlockStateProperties :: EAST . index_of (false) , BlockStateProperties :: NORTH => BlockStateProperties :: NORTH . index_of (false) , BlockStateProperties :: SOUTH => BlockStateProperties :: SOUTH . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST => BlockStateProperties :: WEST . index_of (false))) ;
pub const BLACK_STAINED_GLASS_PANE : Block = Block :: new ("black_stained_glass_pane" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: EAST , & BlockStateProperties :: NORTH , & BlockStateProperties :: SOUTH , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: EAST => BlockStateProperties :: EAST . index_of (false) , BlockStateProperties :: NORTH => BlockStateProperties :: NORTH . index_of (false) , BlockStateProperties :: SOUTH => BlockStateProperties :: SOUTH . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST => BlockStateProperties :: WEST . index_of (false))) ;
pub const ACACIA_STAIRS : Block = Block :: new ("acacia_stairs" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: STAIRS_SHAPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: HALF => crate :: properties :: Half :: Bottom as usize , BlockStateProperties :: STAIRS_SHAPE => crate :: properties :: StairsShape :: Straight as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const CHERRY_STAIRS : Block = Block :: new ("cherry_stairs" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: STAIRS_SHAPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: HALF => crate :: properties :: Half :: Bottom as usize , BlockStateProperties :: STAIRS_SHAPE => crate :: properties :: StairsShape :: Straight as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const DARK_OAK_STAIRS : Block = Block :: new ("dark_oak_stairs" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: STAIRS_SHAPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: HALF => crate :: properties :: Half :: Bottom as usize , BlockStateProperties :: STAIRS_SHAPE => crate :: properties :: StairsShape :: Straight as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const PALE_OAK_STAIRS : Block = Block :: new ("pale_oak_stairs" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: STAIRS_SHAPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: HALF => crate :: properties :: Half :: Bottom as usize , BlockStateProperties :: STAIRS_SHAPE => crate :: properties :: StairsShape :: Straight as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const MANGROVE_STAIRS : Block = Block :: new ("mangrove_stairs" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: STAIRS_SHAPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: HALF => crate :: properties :: Half :: Bottom as usize , BlockStateProperties :: STAIRS_SHAPE => crate :: properties :: StairsShape :: Straight as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const BAMBOO_STAIRS : Block = Block :: new ("bamboo_stairs" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: STAIRS_SHAPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: HALF => crate :: properties :: Half :: Bottom as usize , BlockStateProperties :: STAIRS_SHAPE => crate :: properties :: StairsShape :: Straight as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const BAMBOO_MOSAIC_STAIRS : Block = Block :: new ("bamboo_mosaic_stairs" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: STAIRS_SHAPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: HALF => crate :: properties :: Half :: Bottom as usize , BlockStateProperties :: STAIRS_SHAPE => crate :: properties :: StairsShape :: Straight as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const SLIME_BLOCK: Block = Block::new("slime_block", BlockBehaviourProperties::new(), &[]);
pub const BARRIER : Block = Block :: new ("barrier" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const LIGHT : Block = Block :: new ("light" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: LEVEL , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: LEVEL => 15usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const IRON_TRAPDOOR : Block = Block :: new ("iron_trapdoor" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: OPEN , & BlockStateProperties :: POWERED , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: HALF => crate :: properties :: Half :: Bottom as usize , BlockStateProperties :: OPEN => BlockStateProperties :: OPEN . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const PRISMARINE: Block = Block::new("prismarine", BlockBehaviourProperties::new(), &[]);
pub const PRISMARINE_BRICKS: Block =
    Block::new("prismarine_bricks", BlockBehaviourProperties::new(), &[]);
pub const DARK_PRISMARINE: Block =
    Block::new("dark_prismarine", BlockBehaviourProperties::new(), &[]);
pub const PRISMARINE_STAIRS : Block = Block :: new ("prismarine_stairs" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: STAIRS_SHAPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: HALF => crate :: properties :: Half :: Bottom as usize , BlockStateProperties :: STAIRS_SHAPE => crate :: properties :: StairsShape :: Straight as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const PRISMARINE_BRICK_STAIRS : Block = Block :: new ("prismarine_brick_stairs" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: STAIRS_SHAPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: HALF => crate :: properties :: Half :: Bottom as usize , BlockStateProperties :: STAIRS_SHAPE => crate :: properties :: StairsShape :: Straight as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const DARK_PRISMARINE_STAIRS : Block = Block :: new ("dark_prismarine_stairs" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: STAIRS_SHAPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: HALF => crate :: properties :: Half :: Bottom as usize , BlockStateProperties :: STAIRS_SHAPE => crate :: properties :: StairsShape :: Straight as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const PRISMARINE_SLAB : Block = Block :: new ("prismarine_slab" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: SLAB_TYPE => crate :: properties :: SlabType :: Bottom as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const PRISMARINE_BRICK_SLAB : Block = Block :: new ("prismarine_brick_slab" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: SLAB_TYPE => crate :: properties :: SlabType :: Bottom as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const DARK_PRISMARINE_SLAB : Block = Block :: new ("dark_prismarine_slab" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: SLAB_TYPE => crate :: properties :: SlabType :: Bottom as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const SEA_LANTERN: Block = Block::new("sea_lantern", BlockBehaviourProperties::new(), &[]);
pub const HAY_BLOCK : Block = Block :: new ("hay_block" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: AXIS] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: AXIS => crate :: properties :: Axis :: Y as usize)) ;
pub const WHITE_CARPET: Block = Block::new("white_carpet", BlockBehaviourProperties::new(), &[]);
pub const ORANGE_CARPET: Block = Block::new("orange_carpet", BlockBehaviourProperties::new(), &[]);
pub const MAGENTA_CARPET: Block =
    Block::new("magenta_carpet", BlockBehaviourProperties::new(), &[]);
pub const LIGHT_BLUE_CARPET: Block =
    Block::new("light_blue_carpet", BlockBehaviourProperties::new(), &[]);
pub const YELLOW_CARPET: Block = Block::new("yellow_carpet", BlockBehaviourProperties::new(), &[]);
pub const LIME_CARPET: Block = Block::new("lime_carpet", BlockBehaviourProperties::new(), &[]);
pub const PINK_CARPET: Block = Block::new("pink_carpet", BlockBehaviourProperties::new(), &[]);
pub const GRAY_CARPET: Block = Block::new("gray_carpet", BlockBehaviourProperties::new(), &[]);
pub const LIGHT_GRAY_CARPET: Block =
    Block::new("light_gray_carpet", BlockBehaviourProperties::new(), &[]);
pub const CYAN_CARPET: Block = Block::new("cyan_carpet", BlockBehaviourProperties::new(), &[]);
pub const PURPLE_CARPET: Block = Block::new("purple_carpet", BlockBehaviourProperties::new(), &[]);
pub const BLUE_CARPET: Block = Block::new("blue_carpet", BlockBehaviourProperties::new(), &[]);
pub const BROWN_CARPET: Block = Block::new("brown_carpet", BlockBehaviourProperties::new(), &[]);
pub const GREEN_CARPET: Block = Block::new("green_carpet", BlockBehaviourProperties::new(), &[]);
pub const RED_CARPET: Block = Block::new("red_carpet", BlockBehaviourProperties::new(), &[]);
pub const BLACK_CARPET: Block = Block::new("black_carpet", BlockBehaviourProperties::new(), &[]);
pub const TERRACOTTA: Block = Block::new("terracotta", BlockBehaviourProperties::new(), &[]);
pub const COAL_BLOCK: Block = Block::new("coal_block", BlockBehaviourProperties::new(), &[]);
pub const PACKED_ICE: Block = Block::new("packed_ice", BlockBehaviourProperties::new(), &[]);
pub const SUNFLOWER : Block = Block :: new ("sunflower" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: DOUBLE_BLOCK_HALF] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: DOUBLE_BLOCK_HALF => crate :: properties :: DoubleBlockHalf :: Lower as usize)) ;
pub const LILAC : Block = Block :: new ("lilac" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: DOUBLE_BLOCK_HALF] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: DOUBLE_BLOCK_HALF => crate :: properties :: DoubleBlockHalf :: Lower as usize)) ;
pub const ROSE_BUSH : Block = Block :: new ("rose_bush" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: DOUBLE_BLOCK_HALF] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: DOUBLE_BLOCK_HALF => crate :: properties :: DoubleBlockHalf :: Lower as usize)) ;
pub const PEONY : Block = Block :: new ("peony" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: DOUBLE_BLOCK_HALF] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: DOUBLE_BLOCK_HALF => crate :: properties :: DoubleBlockHalf :: Lower as usize)) ;
pub const TALL_GRASS : Block = Block :: new ("tall_grass" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: DOUBLE_BLOCK_HALF] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: DOUBLE_BLOCK_HALF => crate :: properties :: DoubleBlockHalf :: Lower as usize)) ;
pub const LARGE_FERN : Block = Block :: new ("large_fern" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: DOUBLE_BLOCK_HALF] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: DOUBLE_BLOCK_HALF => crate :: properties :: DoubleBlockHalf :: Lower as usize)) ;
pub const WHITE_BANNER: Block = Block::new(
    "white_banner",
    BlockBehaviourProperties::new(),
    &[&BlockStateProperties::ROTATION_16],
)
.with_default_state(crate :: blocks :: offset ! (BlockStateProperties :: ROTATION_16 => 0usize));
pub const ORANGE_BANNER: Block = Block::new(
    "orange_banner",
    BlockBehaviourProperties::new(),
    &[&BlockStateProperties::ROTATION_16],
)
.with_default_state(crate :: blocks :: offset ! (BlockStateProperties :: ROTATION_16 => 0usize));
pub const MAGENTA_BANNER: Block = Block::new(
    "magenta_banner",
    BlockBehaviourProperties::new(),
    &[&BlockStateProperties::ROTATION_16],
)
.with_default_state(crate :: blocks :: offset ! (BlockStateProperties :: ROTATION_16 => 0usize));
pub const LIGHT_BLUE_BANNER: Block = Block::new(
    "light_blue_banner",
    BlockBehaviourProperties::new(),
    &[&BlockStateProperties::ROTATION_16],
)
.with_default_state(crate :: blocks :: offset ! (BlockStateProperties :: ROTATION_16 => 0usize));
pub const YELLOW_BANNER: Block = Block::new(
    "yellow_banner",
    BlockBehaviourProperties::new(),
    &[&BlockStateProperties::ROTATION_16],
)
.with_default_state(crate :: blocks :: offset ! (BlockStateProperties :: ROTATION_16 => 0usize));
pub const LIME_BANNER: Block = Block::new(
    "lime_banner",
    BlockBehaviourProperties::new(),
    &[&BlockStateProperties::ROTATION_16],
)
.with_default_state(crate :: blocks :: offset ! (BlockStateProperties :: ROTATION_16 => 0usize));
pub const PINK_BANNER: Block = Block::new(
    "pink_banner",
    BlockBehaviourProperties::new(),
    &[&BlockStateProperties::ROTATION_16],
)
.with_default_state(crate :: blocks :: offset ! (BlockStateProperties :: ROTATION_16 => 0usize));
pub const GRAY_BANNER: Block = Block::new(
    "gray_banner",
    BlockBehaviourProperties::new(),
    &[&BlockStateProperties::ROTATION_16],
)
.with_default_state(crate :: blocks :: offset ! (BlockStateProperties :: ROTATION_16 => 0usize));
pub const LIGHT_GRAY_BANNER: Block = Block::new(
    "light_gray_banner",
    BlockBehaviourProperties::new(),
    &[&BlockStateProperties::ROTATION_16],
)
.with_default_state(crate :: blocks :: offset ! (BlockStateProperties :: ROTATION_16 => 0usize));
pub const CYAN_BANNER: Block = Block::new(
    "cyan_banner",
    BlockBehaviourProperties::new(),
    &[&BlockStateProperties::ROTATION_16],
)
.with_default_state(crate :: blocks :: offset ! (BlockStateProperties :: ROTATION_16 => 0usize));
pub const PURPLE_BANNER: Block = Block::new(
    "purple_banner",
    BlockBehaviourProperties::new(),
    &[&BlockStateProperties::ROTATION_16],
)
.with_default_state(crate :: blocks :: offset ! (BlockStateProperties :: ROTATION_16 => 0usize));
pub const BLUE_BANNER: Block = Block::new(
    "blue_banner",
    BlockBehaviourProperties::new(),
    &[&BlockStateProperties::ROTATION_16],
)
.with_default_state(crate :: blocks :: offset ! (BlockStateProperties :: ROTATION_16 => 0usize));
pub const BROWN_BANNER: Block = Block::new(
    "brown_banner",
    BlockBehaviourProperties::new(),
    &[&BlockStateProperties::ROTATION_16],
)
.with_default_state(crate :: blocks :: offset ! (BlockStateProperties :: ROTATION_16 => 0usize));
pub const GREEN_BANNER: Block = Block::new(
    "green_banner",
    BlockBehaviourProperties::new(),
    &[&BlockStateProperties::ROTATION_16],
)
.with_default_state(crate :: blocks :: offset ! (BlockStateProperties :: ROTATION_16 => 0usize));
pub const RED_BANNER: Block = Block::new(
    "red_banner",
    BlockBehaviourProperties::new(),
    &[&BlockStateProperties::ROTATION_16],
)
.with_default_state(crate :: blocks :: offset ! (BlockStateProperties :: ROTATION_16 => 0usize));
pub const BLACK_BANNER: Block = Block::new(
    "black_banner",
    BlockBehaviourProperties::new(),
    &[&BlockStateProperties::ROTATION_16],
)
.with_default_state(crate :: blocks :: offset ! (BlockStateProperties :: ROTATION_16 => 0usize));
pub const WHITE_WALL_BANNER : Block = Block :: new ("white_wall_banner" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize)) ;
pub const ORANGE_WALL_BANNER : Block = Block :: new ("orange_wall_banner" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize)) ;
pub const MAGENTA_WALL_BANNER : Block = Block :: new ("magenta_wall_banner" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize)) ;
pub const LIGHT_BLUE_WALL_BANNER : Block = Block :: new ("light_blue_wall_banner" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize)) ;
pub const YELLOW_WALL_BANNER : Block = Block :: new ("yellow_wall_banner" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize)) ;
pub const LIME_WALL_BANNER : Block = Block :: new ("lime_wall_banner" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize)) ;
pub const PINK_WALL_BANNER : Block = Block :: new ("pink_wall_banner" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize)) ;
pub const GRAY_WALL_BANNER : Block = Block :: new ("gray_wall_banner" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize)) ;
pub const LIGHT_GRAY_WALL_BANNER : Block = Block :: new ("light_gray_wall_banner" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize)) ;
pub const CYAN_WALL_BANNER : Block = Block :: new ("cyan_wall_banner" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize)) ;
pub const PURPLE_WALL_BANNER : Block = Block :: new ("purple_wall_banner" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize)) ;
pub const BLUE_WALL_BANNER : Block = Block :: new ("blue_wall_banner" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize)) ;
pub const BROWN_WALL_BANNER : Block = Block :: new ("brown_wall_banner" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize)) ;
pub const GREEN_WALL_BANNER : Block = Block :: new ("green_wall_banner" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize)) ;
pub const RED_WALL_BANNER : Block = Block :: new ("red_wall_banner" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize)) ;
pub const BLACK_WALL_BANNER : Block = Block :: new ("black_wall_banner" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize)) ;
pub const RED_SANDSTONE: Block = Block::new("red_sandstone", BlockBehaviourProperties::new(), &[]);
pub const CHISELED_RED_SANDSTONE: Block = Block::new(
    "chiseled_red_sandstone",
    BlockBehaviourProperties::new(),
    &[],
);
pub const CUT_RED_SANDSTONE: Block =
    Block::new("cut_red_sandstone", BlockBehaviourProperties::new(), &[]);
pub const RED_SANDSTONE_STAIRS : Block = Block :: new ("red_sandstone_stairs" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: STAIRS_SHAPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: HALF => crate :: properties :: Half :: Bottom as usize , BlockStateProperties :: STAIRS_SHAPE => crate :: properties :: StairsShape :: Straight as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const OAK_SLAB : Block = Block :: new ("oak_slab" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: SLAB_TYPE => crate :: properties :: SlabType :: Bottom as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const SPRUCE_SLAB : Block = Block :: new ("spruce_slab" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: SLAB_TYPE => crate :: properties :: SlabType :: Bottom as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const BIRCH_SLAB : Block = Block :: new ("birch_slab" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: SLAB_TYPE => crate :: properties :: SlabType :: Bottom as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const JUNGLE_SLAB : Block = Block :: new ("jungle_slab" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: SLAB_TYPE => crate :: properties :: SlabType :: Bottom as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const ACACIA_SLAB : Block = Block :: new ("acacia_slab" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: SLAB_TYPE => crate :: properties :: SlabType :: Bottom as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const CHERRY_SLAB : Block = Block :: new ("cherry_slab" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: SLAB_TYPE => crate :: properties :: SlabType :: Bottom as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const DARK_OAK_SLAB : Block = Block :: new ("dark_oak_slab" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: SLAB_TYPE => crate :: properties :: SlabType :: Bottom as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const PALE_OAK_SLAB : Block = Block :: new ("pale_oak_slab" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: SLAB_TYPE => crate :: properties :: SlabType :: Bottom as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const MANGROVE_SLAB : Block = Block :: new ("mangrove_slab" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: SLAB_TYPE => crate :: properties :: SlabType :: Bottom as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const BAMBOO_SLAB : Block = Block :: new ("bamboo_slab" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: SLAB_TYPE => crate :: properties :: SlabType :: Bottom as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const BAMBOO_MOSAIC_SLAB : Block = Block :: new ("bamboo_mosaic_slab" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: SLAB_TYPE => crate :: properties :: SlabType :: Bottom as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const STONE_SLAB : Block = Block :: new ("stone_slab" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: SLAB_TYPE => crate :: properties :: SlabType :: Bottom as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const SMOOTH_STONE_SLAB : Block = Block :: new ("smooth_stone_slab" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: SLAB_TYPE => crate :: properties :: SlabType :: Bottom as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const SANDSTONE_SLAB : Block = Block :: new ("sandstone_slab" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: SLAB_TYPE => crate :: properties :: SlabType :: Bottom as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const CUT_SANDSTONE_SLAB : Block = Block :: new ("cut_sandstone_slab" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: SLAB_TYPE => crate :: properties :: SlabType :: Bottom as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const PETRIFIED_OAK_SLAB : Block = Block :: new ("petrified_oak_slab" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: SLAB_TYPE => crate :: properties :: SlabType :: Bottom as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const COBBLESTONE_SLAB : Block = Block :: new ("cobblestone_slab" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: SLAB_TYPE => crate :: properties :: SlabType :: Bottom as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const BRICK_SLAB : Block = Block :: new ("brick_slab" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: SLAB_TYPE => crate :: properties :: SlabType :: Bottom as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const STONE_BRICK_SLAB : Block = Block :: new ("stone_brick_slab" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: SLAB_TYPE => crate :: properties :: SlabType :: Bottom as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const MUD_BRICK_SLAB : Block = Block :: new ("mud_brick_slab" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: SLAB_TYPE => crate :: properties :: SlabType :: Bottom as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const NETHER_BRICK_SLAB : Block = Block :: new ("nether_brick_slab" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: SLAB_TYPE => crate :: properties :: SlabType :: Bottom as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const QUARTZ_SLAB : Block = Block :: new ("quartz_slab" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: SLAB_TYPE => crate :: properties :: SlabType :: Bottom as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const RED_SANDSTONE_SLAB : Block = Block :: new ("red_sandstone_slab" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: SLAB_TYPE => crate :: properties :: SlabType :: Bottom as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const CUT_RED_SANDSTONE_SLAB : Block = Block :: new ("cut_red_sandstone_slab" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: SLAB_TYPE => crate :: properties :: SlabType :: Bottom as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const PURPUR_SLAB : Block = Block :: new ("purpur_slab" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: SLAB_TYPE => crate :: properties :: SlabType :: Bottom as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const SMOOTH_STONE: Block = Block::new("smooth_stone", BlockBehaviourProperties::new(), &[]);
pub const SMOOTH_SANDSTONE: Block =
    Block::new("smooth_sandstone", BlockBehaviourProperties::new(), &[]);
pub const SMOOTH_QUARTZ: Block = Block::new("smooth_quartz", BlockBehaviourProperties::new(), &[]);
pub const SMOOTH_RED_SANDSTONE: Block =
    Block::new("smooth_red_sandstone", BlockBehaviourProperties::new(), &[]);
pub const SPRUCE_FENCE_GATE : Block = Block :: new ("spruce_fence_gate" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: IN_WALL , & BlockStateProperties :: OPEN , & BlockStateProperties :: POWERED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: IN_WALL => BlockStateProperties :: IN_WALL . index_of (false) , BlockStateProperties :: OPEN => BlockStateProperties :: OPEN . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false))) ;
pub const BIRCH_FENCE_GATE : Block = Block :: new ("birch_fence_gate" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: IN_WALL , & BlockStateProperties :: OPEN , & BlockStateProperties :: POWERED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: IN_WALL => BlockStateProperties :: IN_WALL . index_of (false) , BlockStateProperties :: OPEN => BlockStateProperties :: OPEN . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false))) ;
pub const JUNGLE_FENCE_GATE : Block = Block :: new ("jungle_fence_gate" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: IN_WALL , & BlockStateProperties :: OPEN , & BlockStateProperties :: POWERED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: IN_WALL => BlockStateProperties :: IN_WALL . index_of (false) , BlockStateProperties :: OPEN => BlockStateProperties :: OPEN . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false))) ;
pub const ACACIA_FENCE_GATE : Block = Block :: new ("acacia_fence_gate" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: IN_WALL , & BlockStateProperties :: OPEN , & BlockStateProperties :: POWERED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: IN_WALL => BlockStateProperties :: IN_WALL . index_of (false) , BlockStateProperties :: OPEN => BlockStateProperties :: OPEN . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false))) ;
pub const CHERRY_FENCE_GATE : Block = Block :: new ("cherry_fence_gate" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: IN_WALL , & BlockStateProperties :: OPEN , & BlockStateProperties :: POWERED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: IN_WALL => BlockStateProperties :: IN_WALL . index_of (false) , BlockStateProperties :: OPEN => BlockStateProperties :: OPEN . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false))) ;
pub const DARK_OAK_FENCE_GATE : Block = Block :: new ("dark_oak_fence_gate" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: IN_WALL , & BlockStateProperties :: OPEN , & BlockStateProperties :: POWERED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: IN_WALL => BlockStateProperties :: IN_WALL . index_of (false) , BlockStateProperties :: OPEN => BlockStateProperties :: OPEN . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false))) ;
pub const PALE_OAK_FENCE_GATE : Block = Block :: new ("pale_oak_fence_gate" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: IN_WALL , & BlockStateProperties :: OPEN , & BlockStateProperties :: POWERED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: IN_WALL => BlockStateProperties :: IN_WALL . index_of (false) , BlockStateProperties :: OPEN => BlockStateProperties :: OPEN . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false))) ;
pub const MANGROVE_FENCE_GATE : Block = Block :: new ("mangrove_fence_gate" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: IN_WALL , & BlockStateProperties :: OPEN , & BlockStateProperties :: POWERED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: IN_WALL => BlockStateProperties :: IN_WALL . index_of (false) , BlockStateProperties :: OPEN => BlockStateProperties :: OPEN . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false))) ;
pub const BAMBOO_FENCE_GATE : Block = Block :: new ("bamboo_fence_gate" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: IN_WALL , & BlockStateProperties :: OPEN , & BlockStateProperties :: POWERED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: IN_WALL => BlockStateProperties :: IN_WALL . index_of (false) , BlockStateProperties :: OPEN => BlockStateProperties :: OPEN . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false))) ;
pub const SPRUCE_FENCE : Block = Block :: new ("spruce_fence" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: EAST , & BlockStateProperties :: NORTH , & BlockStateProperties :: SOUTH , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: EAST => BlockStateProperties :: EAST . index_of (false) , BlockStateProperties :: NORTH => BlockStateProperties :: NORTH . index_of (false) , BlockStateProperties :: SOUTH => BlockStateProperties :: SOUTH . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST => BlockStateProperties :: WEST . index_of (false))) ;
pub const BIRCH_FENCE : Block = Block :: new ("birch_fence" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: EAST , & BlockStateProperties :: NORTH , & BlockStateProperties :: SOUTH , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: EAST => BlockStateProperties :: EAST . index_of (false) , BlockStateProperties :: NORTH => BlockStateProperties :: NORTH . index_of (false) , BlockStateProperties :: SOUTH => BlockStateProperties :: SOUTH . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST => BlockStateProperties :: WEST . index_of (false))) ;
pub const JUNGLE_FENCE : Block = Block :: new ("jungle_fence" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: EAST , & BlockStateProperties :: NORTH , & BlockStateProperties :: SOUTH , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: EAST => BlockStateProperties :: EAST . index_of (false) , BlockStateProperties :: NORTH => BlockStateProperties :: NORTH . index_of (false) , BlockStateProperties :: SOUTH => BlockStateProperties :: SOUTH . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST => BlockStateProperties :: WEST . index_of (false))) ;
pub const ACACIA_FENCE : Block = Block :: new ("acacia_fence" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: EAST , & BlockStateProperties :: NORTH , & BlockStateProperties :: SOUTH , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: EAST => BlockStateProperties :: EAST . index_of (false) , BlockStateProperties :: NORTH => BlockStateProperties :: NORTH . index_of (false) , BlockStateProperties :: SOUTH => BlockStateProperties :: SOUTH . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST => BlockStateProperties :: WEST . index_of (false))) ;
pub const CHERRY_FENCE : Block = Block :: new ("cherry_fence" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: EAST , & BlockStateProperties :: NORTH , & BlockStateProperties :: SOUTH , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: EAST => BlockStateProperties :: EAST . index_of (false) , BlockStateProperties :: NORTH => BlockStateProperties :: NORTH . index_of (false) , BlockStateProperties :: SOUTH => BlockStateProperties :: SOUTH . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST => BlockStateProperties :: WEST . index_of (false))) ;
pub const DARK_OAK_FENCE : Block = Block :: new ("dark_oak_fence" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: EAST , & BlockStateProperties :: NORTH , & BlockStateProperties :: SOUTH , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: EAST => BlockStateProperties :: EAST . index_of (false) , BlockStateProperties :: NORTH => BlockStateProperties :: NORTH . index_of (false) , BlockStateProperties :: SOUTH => BlockStateProperties :: SOUTH . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST => BlockStateProperties :: WEST . index_of (false))) ;
pub const PALE_OAK_FENCE : Block = Block :: new ("pale_oak_fence" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: EAST , & BlockStateProperties :: NORTH , & BlockStateProperties :: SOUTH , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: EAST => BlockStateProperties :: EAST . index_of (false) , BlockStateProperties :: NORTH => BlockStateProperties :: NORTH . index_of (false) , BlockStateProperties :: SOUTH => BlockStateProperties :: SOUTH . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST => BlockStateProperties :: WEST . index_of (false))) ;
pub const MANGROVE_FENCE : Block = Block :: new ("mangrove_fence" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: EAST , & BlockStateProperties :: NORTH , & BlockStateProperties :: SOUTH , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: EAST => BlockStateProperties :: EAST . index_of (false) , BlockStateProperties :: NORTH => BlockStateProperties :: NORTH . index_of (false) , BlockStateProperties :: SOUTH => BlockStateProperties :: SOUTH . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST => BlockStateProperties :: WEST . index_of (false))) ;
pub const BAMBOO_FENCE : Block = Block :: new ("bamboo_fence" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: EAST , & BlockStateProperties :: NORTH , & BlockStateProperties :: SOUTH , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: EAST => BlockStateProperties :: EAST . index_of (false) , BlockStateProperties :: NORTH => BlockStateProperties :: NORTH . index_of (false) , BlockStateProperties :: SOUTH => BlockStateProperties :: SOUTH . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST => BlockStateProperties :: WEST . index_of (false))) ;
pub const SPRUCE_DOOR : Block = Block :: new ("spruce_door" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: DOUBLE_BLOCK_HALF , & BlockStateProperties :: DOOR_HINGE , & BlockStateProperties :: OPEN , & BlockStateProperties :: POWERED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: DOUBLE_BLOCK_HALF => crate :: properties :: DoubleBlockHalf :: Lower as usize , BlockStateProperties :: DOOR_HINGE => crate :: properties :: DoorHingeSide :: Left as usize , BlockStateProperties :: OPEN => BlockStateProperties :: OPEN . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false))) ;
pub const BIRCH_DOOR : Block = Block :: new ("birch_door" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: DOUBLE_BLOCK_HALF , & BlockStateProperties :: DOOR_HINGE , & BlockStateProperties :: OPEN , & BlockStateProperties :: POWERED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: DOUBLE_BLOCK_HALF => crate :: properties :: DoubleBlockHalf :: Lower as usize , BlockStateProperties :: DOOR_HINGE => crate :: properties :: DoorHingeSide :: Left as usize , BlockStateProperties :: OPEN => BlockStateProperties :: OPEN . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false))) ;
pub const JUNGLE_DOOR : Block = Block :: new ("jungle_door" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: DOUBLE_BLOCK_HALF , & BlockStateProperties :: DOOR_HINGE , & BlockStateProperties :: OPEN , & BlockStateProperties :: POWERED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: DOUBLE_BLOCK_HALF => crate :: properties :: DoubleBlockHalf :: Lower as usize , BlockStateProperties :: DOOR_HINGE => crate :: properties :: DoorHingeSide :: Left as usize , BlockStateProperties :: OPEN => BlockStateProperties :: OPEN . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false))) ;
pub const ACACIA_DOOR : Block = Block :: new ("acacia_door" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: DOUBLE_BLOCK_HALF , & BlockStateProperties :: DOOR_HINGE , & BlockStateProperties :: OPEN , & BlockStateProperties :: POWERED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: DOUBLE_BLOCK_HALF => crate :: properties :: DoubleBlockHalf :: Lower as usize , BlockStateProperties :: DOOR_HINGE => crate :: properties :: DoorHingeSide :: Left as usize , BlockStateProperties :: OPEN => BlockStateProperties :: OPEN . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false))) ;
pub const CHERRY_DOOR : Block = Block :: new ("cherry_door" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: DOUBLE_BLOCK_HALF , & BlockStateProperties :: DOOR_HINGE , & BlockStateProperties :: OPEN , & BlockStateProperties :: POWERED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: DOUBLE_BLOCK_HALF => crate :: properties :: DoubleBlockHalf :: Lower as usize , BlockStateProperties :: DOOR_HINGE => crate :: properties :: DoorHingeSide :: Left as usize , BlockStateProperties :: OPEN => BlockStateProperties :: OPEN . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false))) ;
pub const DARK_OAK_DOOR : Block = Block :: new ("dark_oak_door" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: DOUBLE_BLOCK_HALF , & BlockStateProperties :: DOOR_HINGE , & BlockStateProperties :: OPEN , & BlockStateProperties :: POWERED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: DOUBLE_BLOCK_HALF => crate :: properties :: DoubleBlockHalf :: Lower as usize , BlockStateProperties :: DOOR_HINGE => crate :: properties :: DoorHingeSide :: Left as usize , BlockStateProperties :: OPEN => BlockStateProperties :: OPEN . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false))) ;
pub const PALE_OAK_DOOR : Block = Block :: new ("pale_oak_door" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: DOUBLE_BLOCK_HALF , & BlockStateProperties :: DOOR_HINGE , & BlockStateProperties :: OPEN , & BlockStateProperties :: POWERED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: DOUBLE_BLOCK_HALF => crate :: properties :: DoubleBlockHalf :: Lower as usize , BlockStateProperties :: DOOR_HINGE => crate :: properties :: DoorHingeSide :: Left as usize , BlockStateProperties :: OPEN => BlockStateProperties :: OPEN . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false))) ;
pub const MANGROVE_DOOR : Block = Block :: new ("mangrove_door" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: DOUBLE_BLOCK_HALF , & BlockStateProperties :: DOOR_HINGE , & BlockStateProperties :: OPEN , & BlockStateProperties :: POWERED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: DOUBLE_BLOCK_HALF => crate :: properties :: DoubleBlockHalf :: Lower as usize , BlockStateProperties :: DOOR_HINGE => crate :: properties :: DoorHingeSide :: Left as usize , BlockStateProperties :: OPEN => BlockStateProperties :: OPEN . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false))) ;
pub const BAMBOO_DOOR : Block = Block :: new ("bamboo_door" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: DOUBLE_BLOCK_HALF , & BlockStateProperties :: DOOR_HINGE , & BlockStateProperties :: OPEN , & BlockStateProperties :: POWERED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: DOUBLE_BLOCK_HALF => crate :: properties :: DoubleBlockHalf :: Lower as usize , BlockStateProperties :: DOOR_HINGE => crate :: properties :: DoorHingeSide :: Left as usize , BlockStateProperties :: OPEN => BlockStateProperties :: OPEN . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false))) ;
pub const END_ROD : Block = Block :: new ("end_rod" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: FACING] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: FACING => crate :: properties :: Direction :: Up as usize)) ;
pub const CHORUS_PLANT : Block = Block :: new ("chorus_plant" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: DOWN , & BlockStateProperties :: EAST , & BlockStateProperties :: NORTH , & BlockStateProperties :: SOUTH , & BlockStateProperties :: UP , & BlockStateProperties :: WEST] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: DOWN => BlockStateProperties :: DOWN . index_of (false) , BlockStateProperties :: EAST => BlockStateProperties :: EAST . index_of (false) , BlockStateProperties :: NORTH => BlockStateProperties :: NORTH . index_of (false) , BlockStateProperties :: SOUTH => BlockStateProperties :: SOUTH . index_of (false) , BlockStateProperties :: UP => BlockStateProperties :: UP . index_of (false) , BlockStateProperties :: WEST => BlockStateProperties :: WEST . index_of (false))) ;
pub const CHORUS_FLOWER: Block = Block::new(
    "chorus_flower",
    BlockBehaviourProperties::new(),
    &[&BlockStateProperties::AGE_5],
)
.with_default_state(crate :: blocks :: offset ! (BlockStateProperties :: AGE_5 => 0usize));
pub const PURPUR_BLOCK: Block = Block::new("purpur_block", BlockBehaviourProperties::new(), &[]);
pub const PURPUR_PILLAR : Block = Block :: new ("purpur_pillar" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: AXIS] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: AXIS => crate :: properties :: Axis :: Y as usize)) ;
pub const PURPUR_STAIRS : Block = Block :: new ("purpur_stairs" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: STAIRS_SHAPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: HALF => crate :: properties :: Half :: Bottom as usize , BlockStateProperties :: STAIRS_SHAPE => crate :: properties :: StairsShape :: Straight as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const END_STONE_BRICKS: Block =
    Block::new("end_stone_bricks", BlockBehaviourProperties::new(), &[]);
pub const TORCHFLOWER_CROP: Block = Block::new(
    "torchflower_crop",
    BlockBehaviourProperties::new(),
    &[&BlockStateProperties::AGE_1],
)
.with_default_state(crate :: blocks :: offset ! (BlockStateProperties :: AGE_1 => 0usize));
pub const PITCHER_CROP : Block = Block :: new ("pitcher_crop" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: AGE_4 , & BlockStateProperties :: DOUBLE_BLOCK_HALF] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: AGE_4 => 0usize , BlockStateProperties :: DOUBLE_BLOCK_HALF => crate :: properties :: DoubleBlockHalf :: Lower as usize)) ;
pub const PITCHER_PLANT : Block = Block :: new ("pitcher_plant" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: DOUBLE_BLOCK_HALF] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: DOUBLE_BLOCK_HALF => crate :: properties :: DoubleBlockHalf :: Lower as usize)) ;
pub const BEETROOTS: Block = Block::new(
    "beetroots",
    BlockBehaviourProperties::new(),
    &[&BlockStateProperties::AGE_3],
)
.with_default_state(crate :: blocks :: offset ! (BlockStateProperties :: AGE_3 => 0usize));
pub const DIRT_PATH: Block = Block::new("dirt_path", BlockBehaviourProperties::new(), &[]);
pub const END_GATEWAY: Block = Block::new("end_gateway", BlockBehaviourProperties::new(), &[]);
pub const REPEATING_COMMAND_BLOCK : Block = Block :: new ("repeating_command_block" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: CONDITIONAL , & BlockStateProperties :: FACING] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: CONDITIONAL => BlockStateProperties :: CONDITIONAL . index_of (false) , BlockStateProperties :: FACING => crate :: properties :: Direction :: North as usize)) ;
pub const CHAIN_COMMAND_BLOCK : Block = Block :: new ("chain_command_block" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: CONDITIONAL , & BlockStateProperties :: FACING] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: CONDITIONAL => BlockStateProperties :: CONDITIONAL . index_of (false) , BlockStateProperties :: FACING => crate :: properties :: Direction :: North as usize)) ;
pub const FROSTED_ICE: Block = Block::new(
    "frosted_ice",
    BlockBehaviourProperties::new(),
    &[&BlockStateProperties::AGE_3],
)
.with_default_state(crate :: blocks :: offset ! (BlockStateProperties :: AGE_3 => 0usize));
pub const MAGMA_BLOCK: Block = Block::new("magma_block", BlockBehaviourProperties::new(), &[]);
pub const NETHER_WART_BLOCK: Block =
    Block::new("nether_wart_block", BlockBehaviourProperties::new(), &[]);
pub const RED_NETHER_BRICKS: Block =
    Block::new("red_nether_bricks", BlockBehaviourProperties::new(), &[]);
pub const BONE_BLOCK : Block = Block :: new ("bone_block" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: AXIS] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: AXIS => crate :: properties :: Axis :: Y as usize)) ;
pub const STRUCTURE_VOID: Block =
    Block::new("structure_void", BlockBehaviourProperties::new(), &[]);
pub const OBSERVER : Block = Block :: new ("observer" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: FACING , & BlockStateProperties :: POWERED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: FACING => crate :: properties :: Direction :: South as usize , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false))) ;
pub const SHULKER_BOX : Block = Block :: new ("shulker_box" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: FACING] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: FACING => crate :: properties :: Direction :: Up as usize)) ;
pub const WHITE_SHULKER_BOX : Block = Block :: new ("white_shulker_box" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: FACING] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: FACING => crate :: properties :: Direction :: Up as usize)) ;
pub const ORANGE_SHULKER_BOX : Block = Block :: new ("orange_shulker_box" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: FACING] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: FACING => crate :: properties :: Direction :: Up as usize)) ;
pub const MAGENTA_SHULKER_BOX : Block = Block :: new ("magenta_shulker_box" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: FACING] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: FACING => crate :: properties :: Direction :: Up as usize)) ;
pub const LIGHT_BLUE_SHULKER_BOX : Block = Block :: new ("light_blue_shulker_box" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: FACING] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: FACING => crate :: properties :: Direction :: Up as usize)) ;
pub const YELLOW_SHULKER_BOX : Block = Block :: new ("yellow_shulker_box" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: FACING] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: FACING => crate :: properties :: Direction :: Up as usize)) ;
pub const LIME_SHULKER_BOX : Block = Block :: new ("lime_shulker_box" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: FACING] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: FACING => crate :: properties :: Direction :: Up as usize)) ;
pub const PINK_SHULKER_BOX : Block = Block :: new ("pink_shulker_box" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: FACING] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: FACING => crate :: properties :: Direction :: Up as usize)) ;
pub const GRAY_SHULKER_BOX : Block = Block :: new ("gray_shulker_box" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: FACING] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: FACING => crate :: properties :: Direction :: Up as usize)) ;
pub const LIGHT_GRAY_SHULKER_BOX : Block = Block :: new ("light_gray_shulker_box" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: FACING] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: FACING => crate :: properties :: Direction :: Up as usize)) ;
pub const CYAN_SHULKER_BOX : Block = Block :: new ("cyan_shulker_box" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: FACING] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: FACING => crate :: properties :: Direction :: Up as usize)) ;
pub const PURPLE_SHULKER_BOX : Block = Block :: new ("purple_shulker_box" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: FACING] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: FACING => crate :: properties :: Direction :: Up as usize)) ;
pub const BLUE_SHULKER_BOX : Block = Block :: new ("blue_shulker_box" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: FACING] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: FACING => crate :: properties :: Direction :: Up as usize)) ;
pub const BROWN_SHULKER_BOX : Block = Block :: new ("brown_shulker_box" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: FACING] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: FACING => crate :: properties :: Direction :: Up as usize)) ;
pub const GREEN_SHULKER_BOX : Block = Block :: new ("green_shulker_box" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: FACING] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: FACING => crate :: properties :: Direction :: Up as usize)) ;
pub const RED_SHULKER_BOX : Block = Block :: new ("red_shulker_box" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: FACING] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: FACING => crate :: properties :: Direction :: Up as usize)) ;
pub const BLACK_SHULKER_BOX : Block = Block :: new ("black_shulker_box" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: FACING] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: FACING => crate :: properties :: Direction :: Up as usize)) ;
pub const WHITE_GLAZED_TERRACOTTA : Block = Block :: new ("white_glazed_terracotta" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize)) ;
pub const ORANGE_GLAZED_TERRACOTTA : Block = Block :: new ("orange_glazed_terracotta" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize)) ;
pub const MAGENTA_GLAZED_TERRACOTTA : Block = Block :: new ("magenta_glazed_terracotta" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize)) ;
pub const LIGHT_BLUE_GLAZED_TERRACOTTA : Block = Block :: new ("light_blue_glazed_terracotta" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize)) ;
pub const YELLOW_GLAZED_TERRACOTTA : Block = Block :: new ("yellow_glazed_terracotta" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize)) ;
pub const LIME_GLAZED_TERRACOTTA : Block = Block :: new ("lime_glazed_terracotta" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize)) ;
pub const PINK_GLAZED_TERRACOTTA : Block = Block :: new ("pink_glazed_terracotta" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize)) ;
pub const GRAY_GLAZED_TERRACOTTA : Block = Block :: new ("gray_glazed_terracotta" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize)) ;
pub const LIGHT_GRAY_GLAZED_TERRACOTTA : Block = Block :: new ("light_gray_glazed_terracotta" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize)) ;
pub const CYAN_GLAZED_TERRACOTTA : Block = Block :: new ("cyan_glazed_terracotta" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize)) ;
pub const PURPLE_GLAZED_TERRACOTTA : Block = Block :: new ("purple_glazed_terracotta" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize)) ;
pub const BLUE_GLAZED_TERRACOTTA : Block = Block :: new ("blue_glazed_terracotta" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize)) ;
pub const BROWN_GLAZED_TERRACOTTA : Block = Block :: new ("brown_glazed_terracotta" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize)) ;
pub const GREEN_GLAZED_TERRACOTTA : Block = Block :: new ("green_glazed_terracotta" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize)) ;
pub const RED_GLAZED_TERRACOTTA : Block = Block :: new ("red_glazed_terracotta" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize)) ;
pub const BLACK_GLAZED_TERRACOTTA : Block = Block :: new ("black_glazed_terracotta" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize)) ;
pub const WHITE_CONCRETE: Block =
    Block::new("white_concrete", BlockBehaviourProperties::new(), &[]);
pub const ORANGE_CONCRETE: Block =
    Block::new("orange_concrete", BlockBehaviourProperties::new(), &[]);
pub const MAGENTA_CONCRETE: Block =
    Block::new("magenta_concrete", BlockBehaviourProperties::new(), &[]);
pub const LIGHT_BLUE_CONCRETE: Block =
    Block::new("light_blue_concrete", BlockBehaviourProperties::new(), &[]);
pub const YELLOW_CONCRETE: Block =
    Block::new("yellow_concrete", BlockBehaviourProperties::new(), &[]);
pub const LIME_CONCRETE: Block = Block::new("lime_concrete", BlockBehaviourProperties::new(), &[]);
pub const PINK_CONCRETE: Block = Block::new("pink_concrete", BlockBehaviourProperties::new(), &[]);
pub const GRAY_CONCRETE: Block = Block::new("gray_concrete", BlockBehaviourProperties::new(), &[]);
pub const LIGHT_GRAY_CONCRETE: Block =
    Block::new("light_gray_concrete", BlockBehaviourProperties::new(), &[]);
pub const CYAN_CONCRETE: Block = Block::new("cyan_concrete", BlockBehaviourProperties::new(), &[]);
pub const PURPLE_CONCRETE: Block =
    Block::new("purple_concrete", BlockBehaviourProperties::new(), &[]);
pub const BLUE_CONCRETE: Block = Block::new("blue_concrete", BlockBehaviourProperties::new(), &[]);
pub const BROWN_CONCRETE: Block =
    Block::new("brown_concrete", BlockBehaviourProperties::new(), &[]);
pub const GREEN_CONCRETE: Block =
    Block::new("green_concrete", BlockBehaviourProperties::new(), &[]);
pub const RED_CONCRETE: Block = Block::new("red_concrete", BlockBehaviourProperties::new(), &[]);
pub const BLACK_CONCRETE: Block =
    Block::new("black_concrete", BlockBehaviourProperties::new(), &[]);
pub const WHITE_CONCRETE_POWDER: Block = Block::new(
    "white_concrete_powder",
    BlockBehaviourProperties::new(),
    &[],
);
pub const ORANGE_CONCRETE_POWDER: Block = Block::new(
    "orange_concrete_powder",
    BlockBehaviourProperties::new(),
    &[],
);
pub const MAGENTA_CONCRETE_POWDER: Block = Block::new(
    "magenta_concrete_powder",
    BlockBehaviourProperties::new(),
    &[],
);
pub const LIGHT_BLUE_CONCRETE_POWDER: Block = Block::new(
    "light_blue_concrete_powder",
    BlockBehaviourProperties::new(),
    &[],
);
pub const YELLOW_CONCRETE_POWDER: Block = Block::new(
    "yellow_concrete_powder",
    BlockBehaviourProperties::new(),
    &[],
);
pub const LIME_CONCRETE_POWDER: Block =
    Block::new("lime_concrete_powder", BlockBehaviourProperties::new(), &[]);
pub const PINK_CONCRETE_POWDER: Block =
    Block::new("pink_concrete_powder", BlockBehaviourProperties::new(), &[]);
pub const GRAY_CONCRETE_POWDER: Block =
    Block::new("gray_concrete_powder", BlockBehaviourProperties::new(), &[]);
pub const LIGHT_GRAY_CONCRETE_POWDER: Block = Block::new(
    "light_gray_concrete_powder",
    BlockBehaviourProperties::new(),
    &[],
);
pub const CYAN_CONCRETE_POWDER: Block =
    Block::new("cyan_concrete_powder", BlockBehaviourProperties::new(), &[]);
pub const PURPLE_CONCRETE_POWDER: Block = Block::new(
    "purple_concrete_powder",
    BlockBehaviourProperties::new(),
    &[],
);
pub const BLUE_CONCRETE_POWDER: Block =
    Block::new("blue_concrete_powder", BlockBehaviourProperties::new(), &[]);
pub const BROWN_CONCRETE_POWDER: Block = Block::new(
    "brown_concrete_powder",
    BlockBehaviourProperties::new(),
    &[],
);
pub const GREEN_CONCRETE_POWDER: Block = Block::new(
    "green_concrete_powder",
    BlockBehaviourProperties::new(),
    &[],
);
pub const RED_CONCRETE_POWDER: Block =
    Block::new("red_concrete_powder", BlockBehaviourProperties::new(), &[]);
pub const BLACK_CONCRETE_POWDER: Block = Block::new(
    "black_concrete_powder",
    BlockBehaviourProperties::new(),
    &[],
);
pub const KELP: Block = Block::new(
    "kelp",
    BlockBehaviourProperties::new(),
    &[&BlockStateProperties::AGE_25],
)
.with_default_state(crate :: blocks :: offset ! (BlockStateProperties :: AGE_25 => 0usize));
pub const KELP_PLANT: Block = Block::new("kelp_plant", BlockBehaviourProperties::new(), &[]);
pub const DRIED_KELP_BLOCK: Block =
    Block::new("dried_kelp_block", BlockBehaviourProperties::new(), &[]);
pub const TURTLE_EGG : Block = Block :: new ("turtle_egg" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: EGGS , & BlockStateProperties :: HATCH] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: EGGS => 1usize , BlockStateProperties :: HATCH => 0usize)) ;
pub const SNIFFER_EGG: Block = Block::new(
    "sniffer_egg",
    BlockBehaviourProperties::new(),
    &[&BlockStateProperties::HATCH],
)
.with_default_state(crate :: blocks :: offset ! (BlockStateProperties :: HATCH => 0usize));
pub const DRIED_GHAST : Block = Block :: new ("dried_ghast" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: DRIED_GHAST_HYDRATION_LEVELS , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: DRIED_GHAST_HYDRATION_LEVELS => 0usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const DEAD_TUBE_CORAL_BLOCK: Block = Block::new(
    "dead_tube_coral_block",
    BlockBehaviourProperties::new(),
    &[],
);
pub const DEAD_BRAIN_CORAL_BLOCK: Block = Block::new(
    "dead_brain_coral_block",
    BlockBehaviourProperties::new(),
    &[],
);
pub const DEAD_BUBBLE_CORAL_BLOCK: Block = Block::new(
    "dead_bubble_coral_block",
    BlockBehaviourProperties::new(),
    &[],
);
pub const DEAD_FIRE_CORAL_BLOCK: Block = Block::new(
    "dead_fire_coral_block",
    BlockBehaviourProperties::new(),
    &[],
);
pub const DEAD_HORN_CORAL_BLOCK: Block = Block::new(
    "dead_horn_coral_block",
    BlockBehaviourProperties::new(),
    &[],
);
pub const TUBE_CORAL_BLOCK: Block =
    Block::new("tube_coral_block", BlockBehaviourProperties::new(), &[]);
pub const BRAIN_CORAL_BLOCK: Block =
    Block::new("brain_coral_block", BlockBehaviourProperties::new(), &[]);
pub const BUBBLE_CORAL_BLOCK: Block =
    Block::new("bubble_coral_block", BlockBehaviourProperties::new(), &[]);
pub const FIRE_CORAL_BLOCK: Block =
    Block::new("fire_coral_block", BlockBehaviourProperties::new(), &[]);
pub const HORN_CORAL_BLOCK: Block =
    Block::new("horn_coral_block", BlockBehaviourProperties::new(), &[]);
pub const DEAD_TUBE_CORAL : Block = Block :: new ("dead_tube_coral" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (true))) ;
pub const DEAD_BRAIN_CORAL : Block = Block :: new ("dead_brain_coral" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (true))) ;
pub const DEAD_BUBBLE_CORAL : Block = Block :: new ("dead_bubble_coral" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (true))) ;
pub const DEAD_FIRE_CORAL : Block = Block :: new ("dead_fire_coral" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (true))) ;
pub const DEAD_HORN_CORAL : Block = Block :: new ("dead_horn_coral" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (true))) ;
pub const TUBE_CORAL : Block = Block :: new ("tube_coral" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (true))) ;
pub const BRAIN_CORAL : Block = Block :: new ("brain_coral" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (true))) ;
pub const BUBBLE_CORAL : Block = Block :: new ("bubble_coral" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (true))) ;
pub const FIRE_CORAL : Block = Block :: new ("fire_coral" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (true))) ;
pub const HORN_CORAL : Block = Block :: new ("horn_coral" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (true))) ;
pub const DEAD_TUBE_CORAL_FAN : Block = Block :: new ("dead_tube_coral_fan" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (true))) ;
pub const DEAD_BRAIN_CORAL_FAN : Block = Block :: new ("dead_brain_coral_fan" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (true))) ;
pub const DEAD_BUBBLE_CORAL_FAN : Block = Block :: new ("dead_bubble_coral_fan" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (true))) ;
pub const DEAD_FIRE_CORAL_FAN : Block = Block :: new ("dead_fire_coral_fan" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (true))) ;
pub const DEAD_HORN_CORAL_FAN : Block = Block :: new ("dead_horn_coral_fan" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (true))) ;
pub const TUBE_CORAL_FAN : Block = Block :: new ("tube_coral_fan" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (true))) ;
pub const BRAIN_CORAL_FAN : Block = Block :: new ("brain_coral_fan" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (true))) ;
pub const BUBBLE_CORAL_FAN : Block = Block :: new ("bubble_coral_fan" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (true))) ;
pub const FIRE_CORAL_FAN : Block = Block :: new ("fire_coral_fan" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (true))) ;
pub const HORN_CORAL_FAN : Block = Block :: new ("horn_coral_fan" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (true))) ;
pub const DEAD_TUBE_CORAL_WALL_FAN : Block = Block :: new ("dead_tube_coral_wall_fan" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (true))) ;
pub const DEAD_BRAIN_CORAL_WALL_FAN : Block = Block :: new ("dead_brain_coral_wall_fan" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (true))) ;
pub const DEAD_BUBBLE_CORAL_WALL_FAN : Block = Block :: new ("dead_bubble_coral_wall_fan" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (true))) ;
pub const DEAD_FIRE_CORAL_WALL_FAN : Block = Block :: new ("dead_fire_coral_wall_fan" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (true))) ;
pub const DEAD_HORN_CORAL_WALL_FAN : Block = Block :: new ("dead_horn_coral_wall_fan" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (true))) ;
pub const TUBE_CORAL_WALL_FAN : Block = Block :: new ("tube_coral_wall_fan" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (true))) ;
pub const BRAIN_CORAL_WALL_FAN : Block = Block :: new ("brain_coral_wall_fan" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (true))) ;
pub const BUBBLE_CORAL_WALL_FAN : Block = Block :: new ("bubble_coral_wall_fan" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (true))) ;
pub const FIRE_CORAL_WALL_FAN : Block = Block :: new ("fire_coral_wall_fan" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (true))) ;
pub const HORN_CORAL_WALL_FAN : Block = Block :: new ("horn_coral_wall_fan" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (true))) ;
pub const SEA_PICKLE : Block = Block :: new ("sea_pickle" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: PICKLES , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: PICKLES => 1usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (true))) ;
pub const BLUE_ICE: Block = Block::new("blue_ice", BlockBehaviourProperties::new(), &[]);
pub const CONDUIT : Block = Block :: new ("conduit" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (true))) ;
pub const BAMBOO_SAPLING: Block =
    Block::new("bamboo_sapling", BlockBehaviourProperties::new(), &[]);
pub const BAMBOO : Block = Block :: new ("bamboo" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: AGE_1 , & BlockStateProperties :: BAMBOO_LEAVES , & BlockStateProperties :: STAGE] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: AGE_1 => 0usize , BlockStateProperties :: BAMBOO_LEAVES => crate :: properties :: BambooLeaves :: None as usize , BlockStateProperties :: STAGE => 0usize)) ;
pub const POTTED_BAMBOO: Block = Block::new("potted_bamboo", BlockBehaviourProperties::new(), &[]);
pub const VOID_AIR: Block = Block::new("void_air", BlockBehaviourProperties::new(), &[]);
pub const CAVE_AIR: Block = Block::new("cave_air", BlockBehaviourProperties::new(), &[]);
pub const BUBBLE_COLUMN : Block = Block :: new ("bubble_column" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: DRAG] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: DRAG => BlockStateProperties :: DRAG . index_of (true))) ;
pub const POLISHED_GRANITE_STAIRS : Block = Block :: new ("polished_granite_stairs" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: STAIRS_SHAPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: HALF => crate :: properties :: Half :: Bottom as usize , BlockStateProperties :: STAIRS_SHAPE => crate :: properties :: StairsShape :: Straight as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const SMOOTH_RED_SANDSTONE_STAIRS : Block = Block :: new ("smooth_red_sandstone_stairs" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: STAIRS_SHAPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: HALF => crate :: properties :: Half :: Bottom as usize , BlockStateProperties :: STAIRS_SHAPE => crate :: properties :: StairsShape :: Straight as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const MOSSY_STONE_BRICK_STAIRS : Block = Block :: new ("mossy_stone_brick_stairs" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: STAIRS_SHAPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: HALF => crate :: properties :: Half :: Bottom as usize , BlockStateProperties :: STAIRS_SHAPE => crate :: properties :: StairsShape :: Straight as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const POLISHED_DIORITE_STAIRS : Block = Block :: new ("polished_diorite_stairs" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: STAIRS_SHAPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: HALF => crate :: properties :: Half :: Bottom as usize , BlockStateProperties :: STAIRS_SHAPE => crate :: properties :: StairsShape :: Straight as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const MOSSY_COBBLESTONE_STAIRS : Block = Block :: new ("mossy_cobblestone_stairs" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: STAIRS_SHAPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: HALF => crate :: properties :: Half :: Bottom as usize , BlockStateProperties :: STAIRS_SHAPE => crate :: properties :: StairsShape :: Straight as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const END_STONE_BRICK_STAIRS : Block = Block :: new ("end_stone_brick_stairs" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: STAIRS_SHAPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: HALF => crate :: properties :: Half :: Bottom as usize , BlockStateProperties :: STAIRS_SHAPE => crate :: properties :: StairsShape :: Straight as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const STONE_STAIRS : Block = Block :: new ("stone_stairs" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: STAIRS_SHAPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: HALF => crate :: properties :: Half :: Bottom as usize , BlockStateProperties :: STAIRS_SHAPE => crate :: properties :: StairsShape :: Straight as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const SMOOTH_SANDSTONE_STAIRS : Block = Block :: new ("smooth_sandstone_stairs" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: STAIRS_SHAPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: HALF => crate :: properties :: Half :: Bottom as usize , BlockStateProperties :: STAIRS_SHAPE => crate :: properties :: StairsShape :: Straight as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const SMOOTH_QUARTZ_STAIRS : Block = Block :: new ("smooth_quartz_stairs" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: STAIRS_SHAPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: HALF => crate :: properties :: Half :: Bottom as usize , BlockStateProperties :: STAIRS_SHAPE => crate :: properties :: StairsShape :: Straight as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const GRANITE_STAIRS : Block = Block :: new ("granite_stairs" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: STAIRS_SHAPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: HALF => crate :: properties :: Half :: Bottom as usize , BlockStateProperties :: STAIRS_SHAPE => crate :: properties :: StairsShape :: Straight as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const ANDESITE_STAIRS : Block = Block :: new ("andesite_stairs" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: STAIRS_SHAPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: HALF => crate :: properties :: Half :: Bottom as usize , BlockStateProperties :: STAIRS_SHAPE => crate :: properties :: StairsShape :: Straight as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const RED_NETHER_BRICK_STAIRS : Block = Block :: new ("red_nether_brick_stairs" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: STAIRS_SHAPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: HALF => crate :: properties :: Half :: Bottom as usize , BlockStateProperties :: STAIRS_SHAPE => crate :: properties :: StairsShape :: Straight as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const POLISHED_ANDESITE_STAIRS : Block = Block :: new ("polished_andesite_stairs" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: STAIRS_SHAPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: HALF => crate :: properties :: Half :: Bottom as usize , BlockStateProperties :: STAIRS_SHAPE => crate :: properties :: StairsShape :: Straight as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const DIORITE_STAIRS : Block = Block :: new ("diorite_stairs" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: STAIRS_SHAPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: HALF => crate :: properties :: Half :: Bottom as usize , BlockStateProperties :: STAIRS_SHAPE => crate :: properties :: StairsShape :: Straight as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const POLISHED_GRANITE_SLAB : Block = Block :: new ("polished_granite_slab" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: SLAB_TYPE => crate :: properties :: SlabType :: Bottom as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const SMOOTH_RED_SANDSTONE_SLAB : Block = Block :: new ("smooth_red_sandstone_slab" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: SLAB_TYPE => crate :: properties :: SlabType :: Bottom as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const MOSSY_STONE_BRICK_SLAB : Block = Block :: new ("mossy_stone_brick_slab" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: SLAB_TYPE => crate :: properties :: SlabType :: Bottom as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const POLISHED_DIORITE_SLAB : Block = Block :: new ("polished_diorite_slab" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: SLAB_TYPE => crate :: properties :: SlabType :: Bottom as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const MOSSY_COBBLESTONE_SLAB : Block = Block :: new ("mossy_cobblestone_slab" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: SLAB_TYPE => crate :: properties :: SlabType :: Bottom as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const END_STONE_BRICK_SLAB : Block = Block :: new ("end_stone_brick_slab" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: SLAB_TYPE => crate :: properties :: SlabType :: Bottom as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const SMOOTH_SANDSTONE_SLAB : Block = Block :: new ("smooth_sandstone_slab" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: SLAB_TYPE => crate :: properties :: SlabType :: Bottom as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const SMOOTH_QUARTZ_SLAB : Block = Block :: new ("smooth_quartz_slab" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: SLAB_TYPE => crate :: properties :: SlabType :: Bottom as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const GRANITE_SLAB : Block = Block :: new ("granite_slab" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: SLAB_TYPE => crate :: properties :: SlabType :: Bottom as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const ANDESITE_SLAB : Block = Block :: new ("andesite_slab" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: SLAB_TYPE => crate :: properties :: SlabType :: Bottom as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const RED_NETHER_BRICK_SLAB : Block = Block :: new ("red_nether_brick_slab" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: SLAB_TYPE => crate :: properties :: SlabType :: Bottom as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const POLISHED_ANDESITE_SLAB : Block = Block :: new ("polished_andesite_slab" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: SLAB_TYPE => crate :: properties :: SlabType :: Bottom as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const DIORITE_SLAB : Block = Block :: new ("diorite_slab" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: SLAB_TYPE => crate :: properties :: SlabType :: Bottom as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const BRICK_WALL : Block = Block :: new ("brick_wall" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: EAST_WALL , & BlockStateProperties :: NORTH_WALL , & BlockStateProperties :: SOUTH_WALL , & BlockStateProperties :: UP , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST_WALL] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: EAST_WALL => crate :: properties :: WallSide :: None as usize , BlockStateProperties :: NORTH_WALL => crate :: properties :: WallSide :: None as usize , BlockStateProperties :: SOUTH_WALL => crate :: properties :: WallSide :: None as usize , BlockStateProperties :: UP => BlockStateProperties :: UP . index_of (true) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST_WALL => crate :: properties :: WallSide :: None as usize)) ;
pub const PRISMARINE_WALL : Block = Block :: new ("prismarine_wall" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: EAST_WALL , & BlockStateProperties :: NORTH_WALL , & BlockStateProperties :: SOUTH_WALL , & BlockStateProperties :: UP , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST_WALL] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: EAST_WALL => crate :: properties :: WallSide :: None as usize , BlockStateProperties :: NORTH_WALL => crate :: properties :: WallSide :: None as usize , BlockStateProperties :: SOUTH_WALL => crate :: properties :: WallSide :: None as usize , BlockStateProperties :: UP => BlockStateProperties :: UP . index_of (true) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST_WALL => crate :: properties :: WallSide :: None as usize)) ;
pub const RED_SANDSTONE_WALL : Block = Block :: new ("red_sandstone_wall" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: EAST_WALL , & BlockStateProperties :: NORTH_WALL , & BlockStateProperties :: SOUTH_WALL , & BlockStateProperties :: UP , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST_WALL] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: EAST_WALL => crate :: properties :: WallSide :: None as usize , BlockStateProperties :: NORTH_WALL => crate :: properties :: WallSide :: None as usize , BlockStateProperties :: SOUTH_WALL => crate :: properties :: WallSide :: None as usize , BlockStateProperties :: UP => BlockStateProperties :: UP . index_of (true) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST_WALL => crate :: properties :: WallSide :: None as usize)) ;
pub const MOSSY_STONE_BRICK_WALL : Block = Block :: new ("mossy_stone_brick_wall" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: EAST_WALL , & BlockStateProperties :: NORTH_WALL , & BlockStateProperties :: SOUTH_WALL , & BlockStateProperties :: UP , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST_WALL] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: EAST_WALL => crate :: properties :: WallSide :: None as usize , BlockStateProperties :: NORTH_WALL => crate :: properties :: WallSide :: None as usize , BlockStateProperties :: SOUTH_WALL => crate :: properties :: WallSide :: None as usize , BlockStateProperties :: UP => BlockStateProperties :: UP . index_of (true) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST_WALL => crate :: properties :: WallSide :: None as usize)) ;
pub const GRANITE_WALL : Block = Block :: new ("granite_wall" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: EAST_WALL , & BlockStateProperties :: NORTH_WALL , & BlockStateProperties :: SOUTH_WALL , & BlockStateProperties :: UP , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST_WALL] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: EAST_WALL => crate :: properties :: WallSide :: None as usize , BlockStateProperties :: NORTH_WALL => crate :: properties :: WallSide :: None as usize , BlockStateProperties :: SOUTH_WALL => crate :: properties :: WallSide :: None as usize , BlockStateProperties :: UP => BlockStateProperties :: UP . index_of (true) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST_WALL => crate :: properties :: WallSide :: None as usize)) ;
pub const STONE_BRICK_WALL : Block = Block :: new ("stone_brick_wall" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: EAST_WALL , & BlockStateProperties :: NORTH_WALL , & BlockStateProperties :: SOUTH_WALL , & BlockStateProperties :: UP , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST_WALL] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: EAST_WALL => crate :: properties :: WallSide :: None as usize , BlockStateProperties :: NORTH_WALL => crate :: properties :: WallSide :: None as usize , BlockStateProperties :: SOUTH_WALL => crate :: properties :: WallSide :: None as usize , BlockStateProperties :: UP => BlockStateProperties :: UP . index_of (true) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST_WALL => crate :: properties :: WallSide :: None as usize)) ;
pub const MUD_BRICK_WALL : Block = Block :: new ("mud_brick_wall" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: EAST_WALL , & BlockStateProperties :: NORTH_WALL , & BlockStateProperties :: SOUTH_WALL , & BlockStateProperties :: UP , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST_WALL] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: EAST_WALL => crate :: properties :: WallSide :: None as usize , BlockStateProperties :: NORTH_WALL => crate :: properties :: WallSide :: None as usize , BlockStateProperties :: SOUTH_WALL => crate :: properties :: WallSide :: None as usize , BlockStateProperties :: UP => BlockStateProperties :: UP . index_of (true) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST_WALL => crate :: properties :: WallSide :: None as usize)) ;
pub const NETHER_BRICK_WALL : Block = Block :: new ("nether_brick_wall" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: EAST_WALL , & BlockStateProperties :: NORTH_WALL , & BlockStateProperties :: SOUTH_WALL , & BlockStateProperties :: UP , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST_WALL] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: EAST_WALL => crate :: properties :: WallSide :: None as usize , BlockStateProperties :: NORTH_WALL => crate :: properties :: WallSide :: None as usize , BlockStateProperties :: SOUTH_WALL => crate :: properties :: WallSide :: None as usize , BlockStateProperties :: UP => BlockStateProperties :: UP . index_of (true) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST_WALL => crate :: properties :: WallSide :: None as usize)) ;
pub const ANDESITE_WALL : Block = Block :: new ("andesite_wall" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: EAST_WALL , & BlockStateProperties :: NORTH_WALL , & BlockStateProperties :: SOUTH_WALL , & BlockStateProperties :: UP , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST_WALL] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: EAST_WALL => crate :: properties :: WallSide :: None as usize , BlockStateProperties :: NORTH_WALL => crate :: properties :: WallSide :: None as usize , BlockStateProperties :: SOUTH_WALL => crate :: properties :: WallSide :: None as usize , BlockStateProperties :: UP => BlockStateProperties :: UP . index_of (true) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST_WALL => crate :: properties :: WallSide :: None as usize)) ;
pub const RED_NETHER_BRICK_WALL : Block = Block :: new ("red_nether_brick_wall" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: EAST_WALL , & BlockStateProperties :: NORTH_WALL , & BlockStateProperties :: SOUTH_WALL , & BlockStateProperties :: UP , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST_WALL] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: EAST_WALL => crate :: properties :: WallSide :: None as usize , BlockStateProperties :: NORTH_WALL => crate :: properties :: WallSide :: None as usize , BlockStateProperties :: SOUTH_WALL => crate :: properties :: WallSide :: None as usize , BlockStateProperties :: UP => BlockStateProperties :: UP . index_of (true) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST_WALL => crate :: properties :: WallSide :: None as usize)) ;
pub const SANDSTONE_WALL : Block = Block :: new ("sandstone_wall" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: EAST_WALL , & BlockStateProperties :: NORTH_WALL , & BlockStateProperties :: SOUTH_WALL , & BlockStateProperties :: UP , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST_WALL] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: EAST_WALL => crate :: properties :: WallSide :: None as usize , BlockStateProperties :: NORTH_WALL => crate :: properties :: WallSide :: None as usize , BlockStateProperties :: SOUTH_WALL => crate :: properties :: WallSide :: None as usize , BlockStateProperties :: UP => BlockStateProperties :: UP . index_of (true) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST_WALL => crate :: properties :: WallSide :: None as usize)) ;
pub const END_STONE_BRICK_WALL : Block = Block :: new ("end_stone_brick_wall" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: EAST_WALL , & BlockStateProperties :: NORTH_WALL , & BlockStateProperties :: SOUTH_WALL , & BlockStateProperties :: UP , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST_WALL] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: EAST_WALL => crate :: properties :: WallSide :: None as usize , BlockStateProperties :: NORTH_WALL => crate :: properties :: WallSide :: None as usize , BlockStateProperties :: SOUTH_WALL => crate :: properties :: WallSide :: None as usize , BlockStateProperties :: UP => BlockStateProperties :: UP . index_of (true) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST_WALL => crate :: properties :: WallSide :: None as usize)) ;
pub const DIORITE_WALL : Block = Block :: new ("diorite_wall" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: EAST_WALL , & BlockStateProperties :: NORTH_WALL , & BlockStateProperties :: SOUTH_WALL , & BlockStateProperties :: UP , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST_WALL] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: EAST_WALL => crate :: properties :: WallSide :: None as usize , BlockStateProperties :: NORTH_WALL => crate :: properties :: WallSide :: None as usize , BlockStateProperties :: SOUTH_WALL => crate :: properties :: WallSide :: None as usize , BlockStateProperties :: UP => BlockStateProperties :: UP . index_of (true) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST_WALL => crate :: properties :: WallSide :: None as usize)) ;
pub const SCAFFOLDING : Block = Block :: new ("scaffolding" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: BOTTOM , & BlockStateProperties :: STABILITY_DISTANCE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: BOTTOM => BlockStateProperties :: BOTTOM . index_of (false) , BlockStateProperties :: STABILITY_DISTANCE => 7usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const LOOM : Block = Block :: new ("loom" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize)) ;
pub const BARREL : Block = Block :: new ("barrel" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: FACING , & BlockStateProperties :: OPEN] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: OPEN => BlockStateProperties :: OPEN . index_of (false))) ;
pub const SMOKER : Block = Block :: new ("smoker" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: LIT] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: LIT => BlockStateProperties :: LIT . index_of (false))) ;
pub const BLAST_FURNACE : Block = Block :: new ("blast_furnace" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: LIT] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: LIT => BlockStateProperties :: LIT . index_of (false))) ;
pub const CARTOGRAPHY_TABLE: Block =
    Block::new("cartography_table", BlockBehaviourProperties::new(), &[]);
pub const FLETCHING_TABLE: Block =
    Block::new("fletching_table", BlockBehaviourProperties::new(), &[]);
pub const GRINDSTONE : Block = Block :: new ("grindstone" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: ATTACH_FACE , & BlockStateProperties :: HORIZONTAL_FACING] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: ATTACH_FACE => crate :: properties :: AttachFace :: Wall as usize , BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize)) ;
pub const LECTERN : Block = Block :: new ("lectern" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HAS_BOOK , & BlockStateProperties :: POWERED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: HAS_BOOK => BlockStateProperties :: HAS_BOOK . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false))) ;
pub const SMITHING_TABLE: Block =
    Block::new("smithing_table", BlockBehaviourProperties::new(), &[]);
pub const STONECUTTER : Block = Block :: new ("stonecutter" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize)) ;
pub const BELL : Block = Block :: new ("bell" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: BELL_ATTACHMENT , & BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: POWERED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: BELL_ATTACHMENT => crate :: properties :: BellAttachType :: Floor as usize , BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false))) ;
pub const LANTERN : Block = Block :: new ("lantern" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HANGING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HANGING => BlockStateProperties :: HANGING . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const SOUL_LANTERN : Block = Block :: new ("soul_lantern" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HANGING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HANGING => BlockStateProperties :: HANGING . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const COPPER_LANTERN : Block = Block :: new ("copper_lantern" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HANGING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HANGING => BlockStateProperties :: HANGING . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const EXPOSED_COPPER_LANTERN : Block = Block :: new ("exposed_copper_lantern" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HANGING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HANGING => BlockStateProperties :: HANGING . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const WEATHERED_COPPER_LANTERN : Block = Block :: new ("weathered_copper_lantern" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HANGING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HANGING => BlockStateProperties :: HANGING . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const OXIDIZED_COPPER_LANTERN : Block = Block :: new ("oxidized_copper_lantern" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HANGING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HANGING => BlockStateProperties :: HANGING . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const WAXED_COPPER_LANTERN : Block = Block :: new ("waxed_copper_lantern" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HANGING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HANGING => BlockStateProperties :: HANGING . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const WAXED_EXPOSED_COPPER_LANTERN : Block = Block :: new ("waxed_exposed_copper_lantern" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HANGING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HANGING => BlockStateProperties :: HANGING . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const WAXED_WEATHERED_COPPER_LANTERN : Block = Block :: new ("waxed_weathered_copper_lantern" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HANGING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HANGING => BlockStateProperties :: HANGING . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const WAXED_OXIDIZED_COPPER_LANTERN : Block = Block :: new ("waxed_oxidized_copper_lantern" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HANGING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HANGING => BlockStateProperties :: HANGING . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const CAMPFIRE : Block = Block :: new ("campfire" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: LIT , & BlockStateProperties :: SIGNAL_FIRE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: LIT => BlockStateProperties :: LIT . index_of (true) , BlockStateProperties :: SIGNAL_FIRE => BlockStateProperties :: SIGNAL_FIRE . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const SOUL_CAMPFIRE : Block = Block :: new ("soul_campfire" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: LIT , & BlockStateProperties :: SIGNAL_FIRE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: LIT => BlockStateProperties :: LIT . index_of (true) , BlockStateProperties :: SIGNAL_FIRE => BlockStateProperties :: SIGNAL_FIRE . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const SWEET_BERRY_BUSH: Block = Block::new(
    "sweet_berry_bush",
    BlockBehaviourProperties::new(),
    &[&BlockStateProperties::AGE_3],
)
.with_default_state(crate :: blocks :: offset ! (BlockStateProperties :: AGE_3 => 0usize));
pub const WARPED_STEM : Block = Block :: new ("warped_stem" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: AXIS] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: AXIS => crate :: properties :: Axis :: Y as usize)) ;
pub const STRIPPED_WARPED_STEM : Block = Block :: new ("stripped_warped_stem" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: AXIS] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: AXIS => crate :: properties :: Axis :: Y as usize)) ;
pub const WARPED_HYPHAE : Block = Block :: new ("warped_hyphae" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: AXIS] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: AXIS => crate :: properties :: Axis :: Y as usize)) ;
pub const STRIPPED_WARPED_HYPHAE : Block = Block :: new ("stripped_warped_hyphae" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: AXIS] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: AXIS => crate :: properties :: Axis :: Y as usize)) ;
pub const WARPED_NYLIUM: Block = Block::new("warped_nylium", BlockBehaviourProperties::new(), &[]);
pub const WARPED_FUNGUS: Block = Block::new("warped_fungus", BlockBehaviourProperties::new(), &[]);
pub const WARPED_WART_BLOCK: Block =
    Block::new("warped_wart_block", BlockBehaviourProperties::new(), &[]);
pub const WARPED_ROOTS: Block = Block::new("warped_roots", BlockBehaviourProperties::new(), &[]);
pub const NETHER_SPROUTS: Block =
    Block::new("nether_sprouts", BlockBehaviourProperties::new(), &[]);
pub const CRIMSON_STEM : Block = Block :: new ("crimson_stem" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: AXIS] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: AXIS => crate :: properties :: Axis :: Y as usize)) ;
pub const STRIPPED_CRIMSON_STEM : Block = Block :: new ("stripped_crimson_stem" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: AXIS] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: AXIS => crate :: properties :: Axis :: Y as usize)) ;
pub const CRIMSON_HYPHAE : Block = Block :: new ("crimson_hyphae" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: AXIS] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: AXIS => crate :: properties :: Axis :: Y as usize)) ;
pub const STRIPPED_CRIMSON_HYPHAE : Block = Block :: new ("stripped_crimson_hyphae" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: AXIS] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: AXIS => crate :: properties :: Axis :: Y as usize)) ;
pub const CRIMSON_NYLIUM: Block =
    Block::new("crimson_nylium", BlockBehaviourProperties::new(), &[]);
pub const CRIMSON_FUNGUS: Block =
    Block::new("crimson_fungus", BlockBehaviourProperties::new(), &[]);
pub const SHROOMLIGHT: Block = Block::new("shroomlight", BlockBehaviourProperties::new(), &[]);
pub const WEEPING_VINES: Block = Block::new(
    "weeping_vines",
    BlockBehaviourProperties::new(),
    &[&BlockStateProperties::AGE_25],
)
.with_default_state(crate :: blocks :: offset ! (BlockStateProperties :: AGE_25 => 0usize));
pub const WEEPING_VINES_PLANT: Block =
    Block::new("weeping_vines_plant", BlockBehaviourProperties::new(), &[]);
pub const TWISTING_VINES: Block = Block::new(
    "twisting_vines",
    BlockBehaviourProperties::new(),
    &[&BlockStateProperties::AGE_25],
)
.with_default_state(crate :: blocks :: offset ! (BlockStateProperties :: AGE_25 => 0usize));
pub const TWISTING_VINES_PLANT: Block =
    Block::new("twisting_vines_plant", BlockBehaviourProperties::new(), &[]);
pub const CRIMSON_ROOTS: Block = Block::new("crimson_roots", BlockBehaviourProperties::new(), &[]);
pub const CRIMSON_PLANKS: Block =
    Block::new("crimson_planks", BlockBehaviourProperties::new(), &[]);
pub const WARPED_PLANKS: Block = Block::new("warped_planks", BlockBehaviourProperties::new(), &[]);
pub const CRIMSON_SLAB : Block = Block :: new ("crimson_slab" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: SLAB_TYPE => crate :: properties :: SlabType :: Bottom as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const WARPED_SLAB : Block = Block :: new ("warped_slab" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: SLAB_TYPE => crate :: properties :: SlabType :: Bottom as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const CRIMSON_PRESSURE_PLATE : Block = Block :: new ("crimson_pressure_plate" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: POWERED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false))) ;
pub const WARPED_PRESSURE_PLATE : Block = Block :: new ("warped_pressure_plate" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: POWERED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false))) ;
pub const CRIMSON_FENCE : Block = Block :: new ("crimson_fence" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: EAST , & BlockStateProperties :: NORTH , & BlockStateProperties :: SOUTH , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: EAST => BlockStateProperties :: EAST . index_of (false) , BlockStateProperties :: NORTH => BlockStateProperties :: NORTH . index_of (false) , BlockStateProperties :: SOUTH => BlockStateProperties :: SOUTH . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST => BlockStateProperties :: WEST . index_of (false))) ;
pub const WARPED_FENCE : Block = Block :: new ("warped_fence" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: EAST , & BlockStateProperties :: NORTH , & BlockStateProperties :: SOUTH , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: EAST => BlockStateProperties :: EAST . index_of (false) , BlockStateProperties :: NORTH => BlockStateProperties :: NORTH . index_of (false) , BlockStateProperties :: SOUTH => BlockStateProperties :: SOUTH . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST => BlockStateProperties :: WEST . index_of (false))) ;
pub const CRIMSON_TRAPDOOR : Block = Block :: new ("crimson_trapdoor" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: OPEN , & BlockStateProperties :: POWERED , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: HALF => crate :: properties :: Half :: Bottom as usize , BlockStateProperties :: OPEN => BlockStateProperties :: OPEN . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const WARPED_TRAPDOOR : Block = Block :: new ("warped_trapdoor" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: OPEN , & BlockStateProperties :: POWERED , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: HALF => crate :: properties :: Half :: Bottom as usize , BlockStateProperties :: OPEN => BlockStateProperties :: OPEN . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const CRIMSON_FENCE_GATE : Block = Block :: new ("crimson_fence_gate" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: IN_WALL , & BlockStateProperties :: OPEN , & BlockStateProperties :: POWERED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: IN_WALL => BlockStateProperties :: IN_WALL . index_of (false) , BlockStateProperties :: OPEN => BlockStateProperties :: OPEN . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false))) ;
pub const WARPED_FENCE_GATE : Block = Block :: new ("warped_fence_gate" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: IN_WALL , & BlockStateProperties :: OPEN , & BlockStateProperties :: POWERED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: IN_WALL => BlockStateProperties :: IN_WALL . index_of (false) , BlockStateProperties :: OPEN => BlockStateProperties :: OPEN . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false))) ;
pub const CRIMSON_STAIRS : Block = Block :: new ("crimson_stairs" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: STAIRS_SHAPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: HALF => crate :: properties :: Half :: Bottom as usize , BlockStateProperties :: STAIRS_SHAPE => crate :: properties :: StairsShape :: Straight as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const WARPED_STAIRS : Block = Block :: new ("warped_stairs" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: STAIRS_SHAPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: HALF => crate :: properties :: Half :: Bottom as usize , BlockStateProperties :: STAIRS_SHAPE => crate :: properties :: StairsShape :: Straight as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const CRIMSON_BUTTON : Block = Block :: new ("crimson_button" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: ATTACH_FACE , & BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: POWERED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: ATTACH_FACE => crate :: properties :: AttachFace :: Wall as usize , BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false))) ;
pub const WARPED_BUTTON : Block = Block :: new ("warped_button" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: ATTACH_FACE , & BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: POWERED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: ATTACH_FACE => crate :: properties :: AttachFace :: Wall as usize , BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false))) ;
pub const CRIMSON_DOOR : Block = Block :: new ("crimson_door" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: DOUBLE_BLOCK_HALF , & BlockStateProperties :: DOOR_HINGE , & BlockStateProperties :: OPEN , & BlockStateProperties :: POWERED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: DOUBLE_BLOCK_HALF => crate :: properties :: DoubleBlockHalf :: Lower as usize , BlockStateProperties :: DOOR_HINGE => crate :: properties :: DoorHingeSide :: Left as usize , BlockStateProperties :: OPEN => BlockStateProperties :: OPEN . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false))) ;
pub const WARPED_DOOR : Block = Block :: new ("warped_door" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: DOUBLE_BLOCK_HALF , & BlockStateProperties :: DOOR_HINGE , & BlockStateProperties :: OPEN , & BlockStateProperties :: POWERED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: DOUBLE_BLOCK_HALF => crate :: properties :: DoubleBlockHalf :: Lower as usize , BlockStateProperties :: DOOR_HINGE => crate :: properties :: DoorHingeSide :: Left as usize , BlockStateProperties :: OPEN => BlockStateProperties :: OPEN . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false))) ;
pub const CRIMSON_SIGN : Block = Block :: new ("crimson_sign" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: ROTATION_16 , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: ROTATION_16 => 0usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const WARPED_SIGN : Block = Block :: new ("warped_sign" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: ROTATION_16 , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: ROTATION_16 => 0usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const CRIMSON_WALL_SIGN : Block = Block :: new ("crimson_wall_sign" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const WARPED_WALL_SIGN : Block = Block :: new ("warped_wall_sign" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const STRUCTURE_BLOCK : Block = Block :: new ("structure_block" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: STRUCTUREBLOCK_MODE] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: STRUCTUREBLOCK_MODE => crate :: properties :: StructureMode :: Load as usize)) ;
pub const JIGSAW : Block = Block :: new ("jigsaw" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: ORIENTATION] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: ORIENTATION => crate :: properties :: FrontAndTop :: NorthUp as usize)) ;
pub const TEST_BLOCK : Block = Block :: new ("test_block" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: TEST_BLOCK_MODE] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: TEST_BLOCK_MODE => crate :: properties :: TestBlockMode :: Start as usize)) ;
pub const TEST_INSTANCE_BLOCK: Block =
    Block::new("test_instance_block", BlockBehaviourProperties::new(), &[]);
pub const COMPOSTER: Block = Block::new(
    "composter",
    BlockBehaviourProperties::new(),
    &[&BlockStateProperties::LEVEL_COMPOSTER],
)
.with_default_state(
    crate :: blocks :: offset ! (BlockStateProperties :: LEVEL_COMPOSTER => 0usize),
);
pub const TARGET: Block = Block::new(
    "target",
    BlockBehaviourProperties::new(),
    &[&BlockStateProperties::POWER],
)
.with_default_state(crate :: blocks :: offset ! (BlockStateProperties :: POWER => 0usize));
pub const BEE_NEST : Block = Block :: new ("bee_nest" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: LEVEL_HONEY] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: LEVEL_HONEY => 0usize)) ;
pub const BEEHIVE : Block = Block :: new ("beehive" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: LEVEL_HONEY] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: LEVEL_HONEY => 0usize)) ;
pub const HONEY_BLOCK: Block = Block::new("honey_block", BlockBehaviourProperties::new(), &[]);
pub const HONEYCOMB_BLOCK: Block =
    Block::new("honeycomb_block", BlockBehaviourProperties::new(), &[]);
pub const NETHERITE_BLOCK: Block =
    Block::new("netherite_block", BlockBehaviourProperties::new(), &[]);
pub const ANCIENT_DEBRIS: Block =
    Block::new("ancient_debris", BlockBehaviourProperties::new(), &[]);
pub const CRYING_OBSIDIAN: Block =
    Block::new("crying_obsidian", BlockBehaviourProperties::new(), &[]);
pub const RESPAWN_ANCHOR: Block = Block::new(
    "respawn_anchor",
    BlockBehaviourProperties::new(),
    &[&BlockStateProperties::RESPAWN_ANCHOR_CHARGES],
)
.with_default_state(
    crate :: blocks :: offset ! (BlockStateProperties :: RESPAWN_ANCHOR_CHARGES => 0usize),
);
pub const POTTED_CRIMSON_FUNGUS: Block = Block::new(
    "potted_crimson_fungus",
    BlockBehaviourProperties::new(),
    &[],
);
pub const POTTED_WARPED_FUNGUS: Block =
    Block::new("potted_warped_fungus", BlockBehaviourProperties::new(), &[]);
pub const POTTED_CRIMSON_ROOTS: Block =
    Block::new("potted_crimson_roots", BlockBehaviourProperties::new(), &[]);
pub const POTTED_WARPED_ROOTS: Block =
    Block::new("potted_warped_roots", BlockBehaviourProperties::new(), &[]);
pub const LODESTONE: Block = Block::new("lodestone", BlockBehaviourProperties::new(), &[]);
pub const BLACKSTONE: Block = Block::new("blackstone", BlockBehaviourProperties::new(), &[]);
pub const BLACKSTONE_STAIRS : Block = Block :: new ("blackstone_stairs" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: STAIRS_SHAPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: HALF => crate :: properties :: Half :: Bottom as usize , BlockStateProperties :: STAIRS_SHAPE => crate :: properties :: StairsShape :: Straight as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const BLACKSTONE_WALL : Block = Block :: new ("blackstone_wall" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: EAST_WALL , & BlockStateProperties :: NORTH_WALL , & BlockStateProperties :: SOUTH_WALL , & BlockStateProperties :: UP , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST_WALL] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: EAST_WALL => crate :: properties :: WallSide :: None as usize , BlockStateProperties :: NORTH_WALL => crate :: properties :: WallSide :: None as usize , BlockStateProperties :: SOUTH_WALL => crate :: properties :: WallSide :: None as usize , BlockStateProperties :: UP => BlockStateProperties :: UP . index_of (true) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST_WALL => crate :: properties :: WallSide :: None as usize)) ;
pub const BLACKSTONE_SLAB : Block = Block :: new ("blackstone_slab" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: SLAB_TYPE => crate :: properties :: SlabType :: Bottom as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const POLISHED_BLACKSTONE: Block =
    Block::new("polished_blackstone", BlockBehaviourProperties::new(), &[]);
pub const POLISHED_BLACKSTONE_BRICKS: Block = Block::new(
    "polished_blackstone_bricks",
    BlockBehaviourProperties::new(),
    &[],
);
pub const CRACKED_POLISHED_BLACKSTONE_BRICKS: Block = Block::new(
    "cracked_polished_blackstone_bricks",
    BlockBehaviourProperties::new(),
    &[],
);
pub const CHISELED_POLISHED_BLACKSTONE: Block = Block::new(
    "chiseled_polished_blackstone",
    BlockBehaviourProperties::new(),
    &[],
);
pub const POLISHED_BLACKSTONE_BRICK_SLAB : Block = Block :: new ("polished_blackstone_brick_slab" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: SLAB_TYPE => crate :: properties :: SlabType :: Bottom as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const POLISHED_BLACKSTONE_BRICK_STAIRS : Block = Block :: new ("polished_blackstone_brick_stairs" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: STAIRS_SHAPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: HALF => crate :: properties :: Half :: Bottom as usize , BlockStateProperties :: STAIRS_SHAPE => crate :: properties :: StairsShape :: Straight as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const POLISHED_BLACKSTONE_BRICK_WALL : Block = Block :: new ("polished_blackstone_brick_wall" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: EAST_WALL , & BlockStateProperties :: NORTH_WALL , & BlockStateProperties :: SOUTH_WALL , & BlockStateProperties :: UP , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST_WALL] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: EAST_WALL => crate :: properties :: WallSide :: None as usize , BlockStateProperties :: NORTH_WALL => crate :: properties :: WallSide :: None as usize , BlockStateProperties :: SOUTH_WALL => crate :: properties :: WallSide :: None as usize , BlockStateProperties :: UP => BlockStateProperties :: UP . index_of (true) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST_WALL => crate :: properties :: WallSide :: None as usize)) ;
pub const GILDED_BLACKSTONE: Block =
    Block::new("gilded_blackstone", BlockBehaviourProperties::new(), &[]);
pub const POLISHED_BLACKSTONE_STAIRS : Block = Block :: new ("polished_blackstone_stairs" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: STAIRS_SHAPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: HALF => crate :: properties :: Half :: Bottom as usize , BlockStateProperties :: STAIRS_SHAPE => crate :: properties :: StairsShape :: Straight as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const POLISHED_BLACKSTONE_SLAB : Block = Block :: new ("polished_blackstone_slab" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: SLAB_TYPE => crate :: properties :: SlabType :: Bottom as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const POLISHED_BLACKSTONE_PRESSURE_PLATE : Block = Block :: new ("polished_blackstone_pressure_plate" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: POWERED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false))) ;
pub const POLISHED_BLACKSTONE_BUTTON : Block = Block :: new ("polished_blackstone_button" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: ATTACH_FACE , & BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: POWERED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: ATTACH_FACE => crate :: properties :: AttachFace :: Wall as usize , BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false))) ;
pub const POLISHED_BLACKSTONE_WALL : Block = Block :: new ("polished_blackstone_wall" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: EAST_WALL , & BlockStateProperties :: NORTH_WALL , & BlockStateProperties :: SOUTH_WALL , & BlockStateProperties :: UP , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST_WALL] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: EAST_WALL => crate :: properties :: WallSide :: None as usize , BlockStateProperties :: NORTH_WALL => crate :: properties :: WallSide :: None as usize , BlockStateProperties :: SOUTH_WALL => crate :: properties :: WallSide :: None as usize , BlockStateProperties :: UP => BlockStateProperties :: UP . index_of (true) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST_WALL => crate :: properties :: WallSide :: None as usize)) ;
pub const CHISELED_NETHER_BRICKS: Block = Block::new(
    "chiseled_nether_bricks",
    BlockBehaviourProperties::new(),
    &[],
);
pub const CRACKED_NETHER_BRICKS: Block = Block::new(
    "cracked_nether_bricks",
    BlockBehaviourProperties::new(),
    &[],
);
pub const QUARTZ_BRICKS: Block = Block::new("quartz_bricks", BlockBehaviourProperties::new(), &[]);
pub const CANDLE : Block = Block :: new ("candle" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: CANDLES , & BlockStateProperties :: LIT , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: CANDLES => 1usize , BlockStateProperties :: LIT => BlockStateProperties :: LIT . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const WHITE_CANDLE : Block = Block :: new ("white_candle" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: CANDLES , & BlockStateProperties :: LIT , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: CANDLES => 1usize , BlockStateProperties :: LIT => BlockStateProperties :: LIT . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const ORANGE_CANDLE : Block = Block :: new ("orange_candle" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: CANDLES , & BlockStateProperties :: LIT , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: CANDLES => 1usize , BlockStateProperties :: LIT => BlockStateProperties :: LIT . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const MAGENTA_CANDLE : Block = Block :: new ("magenta_candle" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: CANDLES , & BlockStateProperties :: LIT , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: CANDLES => 1usize , BlockStateProperties :: LIT => BlockStateProperties :: LIT . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const LIGHT_BLUE_CANDLE : Block = Block :: new ("light_blue_candle" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: CANDLES , & BlockStateProperties :: LIT , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: CANDLES => 1usize , BlockStateProperties :: LIT => BlockStateProperties :: LIT . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const YELLOW_CANDLE : Block = Block :: new ("yellow_candle" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: CANDLES , & BlockStateProperties :: LIT , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: CANDLES => 1usize , BlockStateProperties :: LIT => BlockStateProperties :: LIT . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const LIME_CANDLE : Block = Block :: new ("lime_candle" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: CANDLES , & BlockStateProperties :: LIT , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: CANDLES => 1usize , BlockStateProperties :: LIT => BlockStateProperties :: LIT . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const PINK_CANDLE : Block = Block :: new ("pink_candle" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: CANDLES , & BlockStateProperties :: LIT , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: CANDLES => 1usize , BlockStateProperties :: LIT => BlockStateProperties :: LIT . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const GRAY_CANDLE : Block = Block :: new ("gray_candle" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: CANDLES , & BlockStateProperties :: LIT , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: CANDLES => 1usize , BlockStateProperties :: LIT => BlockStateProperties :: LIT . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const LIGHT_GRAY_CANDLE : Block = Block :: new ("light_gray_candle" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: CANDLES , & BlockStateProperties :: LIT , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: CANDLES => 1usize , BlockStateProperties :: LIT => BlockStateProperties :: LIT . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const CYAN_CANDLE : Block = Block :: new ("cyan_candle" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: CANDLES , & BlockStateProperties :: LIT , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: CANDLES => 1usize , BlockStateProperties :: LIT => BlockStateProperties :: LIT . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const PURPLE_CANDLE : Block = Block :: new ("purple_candle" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: CANDLES , & BlockStateProperties :: LIT , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: CANDLES => 1usize , BlockStateProperties :: LIT => BlockStateProperties :: LIT . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const BLUE_CANDLE : Block = Block :: new ("blue_candle" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: CANDLES , & BlockStateProperties :: LIT , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: CANDLES => 1usize , BlockStateProperties :: LIT => BlockStateProperties :: LIT . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const BROWN_CANDLE : Block = Block :: new ("brown_candle" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: CANDLES , & BlockStateProperties :: LIT , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: CANDLES => 1usize , BlockStateProperties :: LIT => BlockStateProperties :: LIT . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const GREEN_CANDLE : Block = Block :: new ("green_candle" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: CANDLES , & BlockStateProperties :: LIT , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: CANDLES => 1usize , BlockStateProperties :: LIT => BlockStateProperties :: LIT . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const RED_CANDLE : Block = Block :: new ("red_candle" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: CANDLES , & BlockStateProperties :: LIT , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: CANDLES => 1usize , BlockStateProperties :: LIT => BlockStateProperties :: LIT . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const BLACK_CANDLE : Block = Block :: new ("black_candle" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: CANDLES , & BlockStateProperties :: LIT , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: CANDLES => 1usize , BlockStateProperties :: LIT => BlockStateProperties :: LIT . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const CANDLE_CAKE : Block = Block :: new ("candle_cake" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: LIT] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: LIT => BlockStateProperties :: LIT . index_of (false))) ;
pub const WHITE_CANDLE_CAKE : Block = Block :: new ("white_candle_cake" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: LIT] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: LIT => BlockStateProperties :: LIT . index_of (false))) ;
pub const ORANGE_CANDLE_CAKE : Block = Block :: new ("orange_candle_cake" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: LIT] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: LIT => BlockStateProperties :: LIT . index_of (false))) ;
pub const MAGENTA_CANDLE_CAKE : Block = Block :: new ("magenta_candle_cake" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: LIT] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: LIT => BlockStateProperties :: LIT . index_of (false))) ;
pub const LIGHT_BLUE_CANDLE_CAKE : Block = Block :: new ("light_blue_candle_cake" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: LIT] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: LIT => BlockStateProperties :: LIT . index_of (false))) ;
pub const YELLOW_CANDLE_CAKE : Block = Block :: new ("yellow_candle_cake" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: LIT] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: LIT => BlockStateProperties :: LIT . index_of (false))) ;
pub const LIME_CANDLE_CAKE : Block = Block :: new ("lime_candle_cake" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: LIT] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: LIT => BlockStateProperties :: LIT . index_of (false))) ;
pub const PINK_CANDLE_CAKE : Block = Block :: new ("pink_candle_cake" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: LIT] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: LIT => BlockStateProperties :: LIT . index_of (false))) ;
pub const GRAY_CANDLE_CAKE : Block = Block :: new ("gray_candle_cake" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: LIT] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: LIT => BlockStateProperties :: LIT . index_of (false))) ;
pub const LIGHT_GRAY_CANDLE_CAKE : Block = Block :: new ("light_gray_candle_cake" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: LIT] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: LIT => BlockStateProperties :: LIT . index_of (false))) ;
pub const CYAN_CANDLE_CAKE : Block = Block :: new ("cyan_candle_cake" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: LIT] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: LIT => BlockStateProperties :: LIT . index_of (false))) ;
pub const PURPLE_CANDLE_CAKE : Block = Block :: new ("purple_candle_cake" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: LIT] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: LIT => BlockStateProperties :: LIT . index_of (false))) ;
pub const BLUE_CANDLE_CAKE : Block = Block :: new ("blue_candle_cake" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: LIT] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: LIT => BlockStateProperties :: LIT . index_of (false))) ;
pub const BROWN_CANDLE_CAKE : Block = Block :: new ("brown_candle_cake" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: LIT] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: LIT => BlockStateProperties :: LIT . index_of (false))) ;
pub const GREEN_CANDLE_CAKE : Block = Block :: new ("green_candle_cake" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: LIT] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: LIT => BlockStateProperties :: LIT . index_of (false))) ;
pub const RED_CANDLE_CAKE : Block = Block :: new ("red_candle_cake" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: LIT] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: LIT => BlockStateProperties :: LIT . index_of (false))) ;
pub const BLACK_CANDLE_CAKE : Block = Block :: new ("black_candle_cake" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: LIT] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: LIT => BlockStateProperties :: LIT . index_of (false))) ;
pub const AMETHYST_BLOCK: Block =
    Block::new("amethyst_block", BlockBehaviourProperties::new(), &[]);
pub const BUDDING_AMETHYST: Block =
    Block::new("budding_amethyst", BlockBehaviourProperties::new(), &[]);
pub const AMETHYST_CLUSTER : Block = Block :: new ("amethyst_cluster" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: FACING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: FACING => crate :: properties :: Direction :: Up as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const LARGE_AMETHYST_BUD : Block = Block :: new ("large_amethyst_bud" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: FACING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: FACING => crate :: properties :: Direction :: Up as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const MEDIUM_AMETHYST_BUD : Block = Block :: new ("medium_amethyst_bud" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: FACING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: FACING => crate :: properties :: Direction :: Up as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const SMALL_AMETHYST_BUD : Block = Block :: new ("small_amethyst_bud" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: FACING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: FACING => crate :: properties :: Direction :: Up as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const TUFF: Block = Block::new("tuff", BlockBehaviourProperties::new(), &[]);
pub const TUFF_SLAB : Block = Block :: new ("tuff_slab" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: SLAB_TYPE => crate :: properties :: SlabType :: Bottom as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const TUFF_STAIRS : Block = Block :: new ("tuff_stairs" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: STAIRS_SHAPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: HALF => crate :: properties :: Half :: Bottom as usize , BlockStateProperties :: STAIRS_SHAPE => crate :: properties :: StairsShape :: Straight as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const TUFF_WALL : Block = Block :: new ("tuff_wall" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: EAST_WALL , & BlockStateProperties :: NORTH_WALL , & BlockStateProperties :: SOUTH_WALL , & BlockStateProperties :: UP , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST_WALL] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: EAST_WALL => crate :: properties :: WallSide :: None as usize , BlockStateProperties :: NORTH_WALL => crate :: properties :: WallSide :: None as usize , BlockStateProperties :: SOUTH_WALL => crate :: properties :: WallSide :: None as usize , BlockStateProperties :: UP => BlockStateProperties :: UP . index_of (true) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST_WALL => crate :: properties :: WallSide :: None as usize)) ;
pub const POLISHED_TUFF: Block = Block::new("polished_tuff", BlockBehaviourProperties::new(), &[]);
pub const POLISHED_TUFF_SLAB : Block = Block :: new ("polished_tuff_slab" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: SLAB_TYPE => crate :: properties :: SlabType :: Bottom as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const POLISHED_TUFF_STAIRS : Block = Block :: new ("polished_tuff_stairs" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: STAIRS_SHAPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: HALF => crate :: properties :: Half :: Bottom as usize , BlockStateProperties :: STAIRS_SHAPE => crate :: properties :: StairsShape :: Straight as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const POLISHED_TUFF_WALL : Block = Block :: new ("polished_tuff_wall" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: EAST_WALL , & BlockStateProperties :: NORTH_WALL , & BlockStateProperties :: SOUTH_WALL , & BlockStateProperties :: UP , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST_WALL] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: EAST_WALL => crate :: properties :: WallSide :: None as usize , BlockStateProperties :: NORTH_WALL => crate :: properties :: WallSide :: None as usize , BlockStateProperties :: SOUTH_WALL => crate :: properties :: WallSide :: None as usize , BlockStateProperties :: UP => BlockStateProperties :: UP . index_of (true) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST_WALL => crate :: properties :: WallSide :: None as usize)) ;
pub const CHISELED_TUFF: Block = Block::new("chiseled_tuff", BlockBehaviourProperties::new(), &[]);
pub const TUFF_BRICKS: Block = Block::new("tuff_bricks", BlockBehaviourProperties::new(), &[]);
pub const TUFF_BRICK_SLAB : Block = Block :: new ("tuff_brick_slab" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: SLAB_TYPE => crate :: properties :: SlabType :: Bottom as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const TUFF_BRICK_STAIRS : Block = Block :: new ("tuff_brick_stairs" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: STAIRS_SHAPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: HALF => crate :: properties :: Half :: Bottom as usize , BlockStateProperties :: STAIRS_SHAPE => crate :: properties :: StairsShape :: Straight as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const TUFF_BRICK_WALL : Block = Block :: new ("tuff_brick_wall" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: EAST_WALL , & BlockStateProperties :: NORTH_WALL , & BlockStateProperties :: SOUTH_WALL , & BlockStateProperties :: UP , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST_WALL] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: EAST_WALL => crate :: properties :: WallSide :: None as usize , BlockStateProperties :: NORTH_WALL => crate :: properties :: WallSide :: None as usize , BlockStateProperties :: SOUTH_WALL => crate :: properties :: WallSide :: None as usize , BlockStateProperties :: UP => BlockStateProperties :: UP . index_of (true) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST_WALL => crate :: properties :: WallSide :: None as usize)) ;
pub const CHISELED_TUFF_BRICKS: Block =
    Block::new("chiseled_tuff_bricks", BlockBehaviourProperties::new(), &[]);
pub const CALCITE: Block = Block::new("calcite", BlockBehaviourProperties::new(), &[]);
pub const TINTED_GLASS: Block = Block::new("tinted_glass", BlockBehaviourProperties::new(), &[]);
pub const POWDER_SNOW: Block = Block::new("powder_snow", BlockBehaviourProperties::new(), &[]);
pub const SCULK_SENSOR : Block = Block :: new ("sculk_sensor" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: POWER , & BlockStateProperties :: SCULK_SENSOR_PHASE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: POWER => 0usize , BlockStateProperties :: SCULK_SENSOR_PHASE => crate :: properties :: SculkSensorPhase :: Inactive as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const CALIBRATED_SCULK_SENSOR : Block = Block :: new ("calibrated_sculk_sensor" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: POWER , & BlockStateProperties :: SCULK_SENSOR_PHASE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: POWER => 0usize , BlockStateProperties :: SCULK_SENSOR_PHASE => crate :: properties :: SculkSensorPhase :: Inactive as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const SCULK: Block = Block::new("sculk", BlockBehaviourProperties::new(), &[]);
pub const SCULK_VEIN : Block = Block :: new ("sculk_vein" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: DOWN , & BlockStateProperties :: EAST , & BlockStateProperties :: NORTH , & BlockStateProperties :: SOUTH , & BlockStateProperties :: UP , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: DOWN => BlockStateProperties :: DOWN . index_of (false) , BlockStateProperties :: EAST => BlockStateProperties :: EAST . index_of (false) , BlockStateProperties :: NORTH => BlockStateProperties :: NORTH . index_of (false) , BlockStateProperties :: SOUTH => BlockStateProperties :: SOUTH . index_of (false) , BlockStateProperties :: UP => BlockStateProperties :: UP . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST => BlockStateProperties :: WEST . index_of (false))) ;
pub const SCULK_CATALYST : Block = Block :: new ("sculk_catalyst" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: BLOOM] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: BLOOM => BlockStateProperties :: BLOOM . index_of (false))) ;
pub const SCULK_SHRIEKER : Block = Block :: new ("sculk_shrieker" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: CAN_SUMMON , & BlockStateProperties :: SHRIEKING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: CAN_SUMMON => BlockStateProperties :: CAN_SUMMON . index_of (false) , BlockStateProperties :: SHRIEKING => BlockStateProperties :: SHRIEKING . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const COPPER_BLOCK: Block = Block::new("copper_block", BlockBehaviourProperties::new(), &[]);
pub const EXPOSED_COPPER: Block =
    Block::new("exposed_copper", BlockBehaviourProperties::new(), &[]);
pub const WEATHERED_COPPER: Block =
    Block::new("weathered_copper", BlockBehaviourProperties::new(), &[]);
pub const OXIDIZED_COPPER: Block =
    Block::new("oxidized_copper", BlockBehaviourProperties::new(), &[]);
pub const COPPER_ORE: Block = Block::new("copper_ore", BlockBehaviourProperties::new(), &[]);
pub const DEEPSLATE_COPPER_ORE: Block =
    Block::new("deepslate_copper_ore", BlockBehaviourProperties::new(), &[]);
pub const OXIDIZED_CUT_COPPER: Block =
    Block::new("oxidized_cut_copper", BlockBehaviourProperties::new(), &[]);
pub const WEATHERED_CUT_COPPER: Block =
    Block::new("weathered_cut_copper", BlockBehaviourProperties::new(), &[]);
pub const EXPOSED_CUT_COPPER: Block =
    Block::new("exposed_cut_copper", BlockBehaviourProperties::new(), &[]);
pub const CUT_COPPER: Block = Block::new("cut_copper", BlockBehaviourProperties::new(), &[]);
pub const OXIDIZED_CHISELED_COPPER: Block = Block::new(
    "oxidized_chiseled_copper",
    BlockBehaviourProperties::new(),
    &[],
);
pub const WEATHERED_CHISELED_COPPER: Block = Block::new(
    "weathered_chiseled_copper",
    BlockBehaviourProperties::new(),
    &[],
);
pub const EXPOSED_CHISELED_COPPER: Block = Block::new(
    "exposed_chiseled_copper",
    BlockBehaviourProperties::new(),
    &[],
);
pub const CHISELED_COPPER: Block =
    Block::new("chiseled_copper", BlockBehaviourProperties::new(), &[]);
pub const WAXED_OXIDIZED_CHISELED_COPPER: Block = Block::new(
    "waxed_oxidized_chiseled_copper",
    BlockBehaviourProperties::new(),
    &[],
);
pub const WAXED_WEATHERED_CHISELED_COPPER: Block = Block::new(
    "waxed_weathered_chiseled_copper",
    BlockBehaviourProperties::new(),
    &[],
);
pub const WAXED_EXPOSED_CHISELED_COPPER: Block = Block::new(
    "waxed_exposed_chiseled_copper",
    BlockBehaviourProperties::new(),
    &[],
);
pub const WAXED_CHISELED_COPPER: Block = Block::new(
    "waxed_chiseled_copper",
    BlockBehaviourProperties::new(),
    &[],
);
pub const OXIDIZED_CUT_COPPER_STAIRS : Block = Block :: new ("oxidized_cut_copper_stairs" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: STAIRS_SHAPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: HALF => crate :: properties :: Half :: Bottom as usize , BlockStateProperties :: STAIRS_SHAPE => crate :: properties :: StairsShape :: Straight as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const WEATHERED_CUT_COPPER_STAIRS : Block = Block :: new ("weathered_cut_copper_stairs" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: STAIRS_SHAPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: HALF => crate :: properties :: Half :: Bottom as usize , BlockStateProperties :: STAIRS_SHAPE => crate :: properties :: StairsShape :: Straight as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const EXPOSED_CUT_COPPER_STAIRS : Block = Block :: new ("exposed_cut_copper_stairs" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: STAIRS_SHAPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: HALF => crate :: properties :: Half :: Bottom as usize , BlockStateProperties :: STAIRS_SHAPE => crate :: properties :: StairsShape :: Straight as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const CUT_COPPER_STAIRS : Block = Block :: new ("cut_copper_stairs" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: STAIRS_SHAPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: HALF => crate :: properties :: Half :: Bottom as usize , BlockStateProperties :: STAIRS_SHAPE => crate :: properties :: StairsShape :: Straight as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const OXIDIZED_CUT_COPPER_SLAB : Block = Block :: new ("oxidized_cut_copper_slab" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: SLAB_TYPE => crate :: properties :: SlabType :: Bottom as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const WEATHERED_CUT_COPPER_SLAB : Block = Block :: new ("weathered_cut_copper_slab" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: SLAB_TYPE => crate :: properties :: SlabType :: Bottom as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const EXPOSED_CUT_COPPER_SLAB : Block = Block :: new ("exposed_cut_copper_slab" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: SLAB_TYPE => crate :: properties :: SlabType :: Bottom as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const CUT_COPPER_SLAB : Block = Block :: new ("cut_copper_slab" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: SLAB_TYPE => crate :: properties :: SlabType :: Bottom as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const WAXED_COPPER_BLOCK: Block =
    Block::new("waxed_copper_block", BlockBehaviourProperties::new(), &[]);
pub const WAXED_WEATHERED_COPPER: Block = Block::new(
    "waxed_weathered_copper",
    BlockBehaviourProperties::new(),
    &[],
);
pub const WAXED_EXPOSED_COPPER: Block =
    Block::new("waxed_exposed_copper", BlockBehaviourProperties::new(), &[]);
pub const WAXED_OXIDIZED_COPPER: Block = Block::new(
    "waxed_oxidized_copper",
    BlockBehaviourProperties::new(),
    &[],
);
pub const WAXED_OXIDIZED_CUT_COPPER: Block = Block::new(
    "waxed_oxidized_cut_copper",
    BlockBehaviourProperties::new(),
    &[],
);
pub const WAXED_WEATHERED_CUT_COPPER: Block = Block::new(
    "waxed_weathered_cut_copper",
    BlockBehaviourProperties::new(),
    &[],
);
pub const WAXED_EXPOSED_CUT_COPPER: Block = Block::new(
    "waxed_exposed_cut_copper",
    BlockBehaviourProperties::new(),
    &[],
);
pub const WAXED_CUT_COPPER: Block =
    Block::new("waxed_cut_copper", BlockBehaviourProperties::new(), &[]);
pub const WAXED_OXIDIZED_CUT_COPPER_STAIRS : Block = Block :: new ("waxed_oxidized_cut_copper_stairs" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: STAIRS_SHAPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: HALF => crate :: properties :: Half :: Bottom as usize , BlockStateProperties :: STAIRS_SHAPE => crate :: properties :: StairsShape :: Straight as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const WAXED_WEATHERED_CUT_COPPER_STAIRS : Block = Block :: new ("waxed_weathered_cut_copper_stairs" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: STAIRS_SHAPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: HALF => crate :: properties :: Half :: Bottom as usize , BlockStateProperties :: STAIRS_SHAPE => crate :: properties :: StairsShape :: Straight as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const WAXED_EXPOSED_CUT_COPPER_STAIRS : Block = Block :: new ("waxed_exposed_cut_copper_stairs" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: STAIRS_SHAPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: HALF => crate :: properties :: Half :: Bottom as usize , BlockStateProperties :: STAIRS_SHAPE => crate :: properties :: StairsShape :: Straight as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const WAXED_CUT_COPPER_STAIRS : Block = Block :: new ("waxed_cut_copper_stairs" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: STAIRS_SHAPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: HALF => crate :: properties :: Half :: Bottom as usize , BlockStateProperties :: STAIRS_SHAPE => crate :: properties :: StairsShape :: Straight as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const WAXED_OXIDIZED_CUT_COPPER_SLAB : Block = Block :: new ("waxed_oxidized_cut_copper_slab" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: SLAB_TYPE => crate :: properties :: SlabType :: Bottom as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const WAXED_WEATHERED_CUT_COPPER_SLAB : Block = Block :: new ("waxed_weathered_cut_copper_slab" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: SLAB_TYPE => crate :: properties :: SlabType :: Bottom as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const WAXED_EXPOSED_CUT_COPPER_SLAB : Block = Block :: new ("waxed_exposed_cut_copper_slab" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: SLAB_TYPE => crate :: properties :: SlabType :: Bottom as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const WAXED_CUT_COPPER_SLAB : Block = Block :: new ("waxed_cut_copper_slab" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: SLAB_TYPE => crate :: properties :: SlabType :: Bottom as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const COPPER_DOOR : Block = Block :: new ("copper_door" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: DOUBLE_BLOCK_HALF , & BlockStateProperties :: DOOR_HINGE , & BlockStateProperties :: OPEN , & BlockStateProperties :: POWERED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: DOUBLE_BLOCK_HALF => crate :: properties :: DoubleBlockHalf :: Lower as usize , BlockStateProperties :: DOOR_HINGE => crate :: properties :: DoorHingeSide :: Left as usize , BlockStateProperties :: OPEN => BlockStateProperties :: OPEN . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false))) ;
pub const EXPOSED_COPPER_DOOR : Block = Block :: new ("exposed_copper_door" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: DOUBLE_BLOCK_HALF , & BlockStateProperties :: DOOR_HINGE , & BlockStateProperties :: OPEN , & BlockStateProperties :: POWERED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: DOUBLE_BLOCK_HALF => crate :: properties :: DoubleBlockHalf :: Lower as usize , BlockStateProperties :: DOOR_HINGE => crate :: properties :: DoorHingeSide :: Left as usize , BlockStateProperties :: OPEN => BlockStateProperties :: OPEN . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false))) ;
pub const OXIDIZED_COPPER_DOOR : Block = Block :: new ("oxidized_copper_door" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: DOUBLE_BLOCK_HALF , & BlockStateProperties :: DOOR_HINGE , & BlockStateProperties :: OPEN , & BlockStateProperties :: POWERED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: DOUBLE_BLOCK_HALF => crate :: properties :: DoubleBlockHalf :: Lower as usize , BlockStateProperties :: DOOR_HINGE => crate :: properties :: DoorHingeSide :: Left as usize , BlockStateProperties :: OPEN => BlockStateProperties :: OPEN . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false))) ;
pub const WEATHERED_COPPER_DOOR : Block = Block :: new ("weathered_copper_door" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: DOUBLE_BLOCK_HALF , & BlockStateProperties :: DOOR_HINGE , & BlockStateProperties :: OPEN , & BlockStateProperties :: POWERED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: DOUBLE_BLOCK_HALF => crate :: properties :: DoubleBlockHalf :: Lower as usize , BlockStateProperties :: DOOR_HINGE => crate :: properties :: DoorHingeSide :: Left as usize , BlockStateProperties :: OPEN => BlockStateProperties :: OPEN . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false))) ;
pub const WAXED_COPPER_DOOR : Block = Block :: new ("waxed_copper_door" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: DOUBLE_BLOCK_HALF , & BlockStateProperties :: DOOR_HINGE , & BlockStateProperties :: OPEN , & BlockStateProperties :: POWERED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: DOUBLE_BLOCK_HALF => crate :: properties :: DoubleBlockHalf :: Lower as usize , BlockStateProperties :: DOOR_HINGE => crate :: properties :: DoorHingeSide :: Left as usize , BlockStateProperties :: OPEN => BlockStateProperties :: OPEN . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false))) ;
pub const WAXED_EXPOSED_COPPER_DOOR : Block = Block :: new ("waxed_exposed_copper_door" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: DOUBLE_BLOCK_HALF , & BlockStateProperties :: DOOR_HINGE , & BlockStateProperties :: OPEN , & BlockStateProperties :: POWERED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: DOUBLE_BLOCK_HALF => crate :: properties :: DoubleBlockHalf :: Lower as usize , BlockStateProperties :: DOOR_HINGE => crate :: properties :: DoorHingeSide :: Left as usize , BlockStateProperties :: OPEN => BlockStateProperties :: OPEN . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false))) ;
pub const WAXED_OXIDIZED_COPPER_DOOR : Block = Block :: new ("waxed_oxidized_copper_door" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: DOUBLE_BLOCK_HALF , & BlockStateProperties :: DOOR_HINGE , & BlockStateProperties :: OPEN , & BlockStateProperties :: POWERED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: DOUBLE_BLOCK_HALF => crate :: properties :: DoubleBlockHalf :: Lower as usize , BlockStateProperties :: DOOR_HINGE => crate :: properties :: DoorHingeSide :: Left as usize , BlockStateProperties :: OPEN => BlockStateProperties :: OPEN . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false))) ;
pub const WAXED_WEATHERED_COPPER_DOOR : Block = Block :: new ("waxed_weathered_copper_door" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: DOUBLE_BLOCK_HALF , & BlockStateProperties :: DOOR_HINGE , & BlockStateProperties :: OPEN , & BlockStateProperties :: POWERED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: DOUBLE_BLOCK_HALF => crate :: properties :: DoubleBlockHalf :: Lower as usize , BlockStateProperties :: DOOR_HINGE => crate :: properties :: DoorHingeSide :: Left as usize , BlockStateProperties :: OPEN => BlockStateProperties :: OPEN . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false))) ;
pub const COPPER_TRAPDOOR : Block = Block :: new ("copper_trapdoor" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: OPEN , & BlockStateProperties :: POWERED , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: HALF => crate :: properties :: Half :: Bottom as usize , BlockStateProperties :: OPEN => BlockStateProperties :: OPEN . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const EXPOSED_COPPER_TRAPDOOR : Block = Block :: new ("exposed_copper_trapdoor" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: OPEN , & BlockStateProperties :: POWERED , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: HALF => crate :: properties :: Half :: Bottom as usize , BlockStateProperties :: OPEN => BlockStateProperties :: OPEN . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const OXIDIZED_COPPER_TRAPDOOR : Block = Block :: new ("oxidized_copper_trapdoor" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: OPEN , & BlockStateProperties :: POWERED , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: HALF => crate :: properties :: Half :: Bottom as usize , BlockStateProperties :: OPEN => BlockStateProperties :: OPEN . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const WEATHERED_COPPER_TRAPDOOR : Block = Block :: new ("weathered_copper_trapdoor" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: OPEN , & BlockStateProperties :: POWERED , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: HALF => crate :: properties :: Half :: Bottom as usize , BlockStateProperties :: OPEN => BlockStateProperties :: OPEN . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const WAXED_COPPER_TRAPDOOR : Block = Block :: new ("waxed_copper_trapdoor" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: OPEN , & BlockStateProperties :: POWERED , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: HALF => crate :: properties :: Half :: Bottom as usize , BlockStateProperties :: OPEN => BlockStateProperties :: OPEN . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const WAXED_EXPOSED_COPPER_TRAPDOOR : Block = Block :: new ("waxed_exposed_copper_trapdoor" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: OPEN , & BlockStateProperties :: POWERED , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: HALF => crate :: properties :: Half :: Bottom as usize , BlockStateProperties :: OPEN => BlockStateProperties :: OPEN . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const WAXED_OXIDIZED_COPPER_TRAPDOOR : Block = Block :: new ("waxed_oxidized_copper_trapdoor" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: OPEN , & BlockStateProperties :: POWERED , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: HALF => crate :: properties :: Half :: Bottom as usize , BlockStateProperties :: OPEN => BlockStateProperties :: OPEN . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const WAXED_WEATHERED_COPPER_TRAPDOOR : Block = Block :: new ("waxed_weathered_copper_trapdoor" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: OPEN , & BlockStateProperties :: POWERED , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: HALF => crate :: properties :: Half :: Bottom as usize , BlockStateProperties :: OPEN => BlockStateProperties :: OPEN . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const COPPER_GRATE : Block = Block :: new ("copper_grate" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const EXPOSED_COPPER_GRATE : Block = Block :: new ("exposed_copper_grate" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const WEATHERED_COPPER_GRATE : Block = Block :: new ("weathered_copper_grate" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const OXIDIZED_COPPER_GRATE : Block = Block :: new ("oxidized_copper_grate" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const WAXED_COPPER_GRATE : Block = Block :: new ("waxed_copper_grate" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const WAXED_EXPOSED_COPPER_GRATE : Block = Block :: new ("waxed_exposed_copper_grate" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const WAXED_WEATHERED_COPPER_GRATE : Block = Block :: new ("waxed_weathered_copper_grate" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const WAXED_OXIDIZED_COPPER_GRATE : Block = Block :: new ("waxed_oxidized_copper_grate" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const COPPER_BULB : Block = Block :: new ("copper_bulb" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: LIT , & BlockStateProperties :: POWERED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: LIT => BlockStateProperties :: LIT . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false))) ;
pub const EXPOSED_COPPER_BULB : Block = Block :: new ("exposed_copper_bulb" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: LIT , & BlockStateProperties :: POWERED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: LIT => BlockStateProperties :: LIT . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false))) ;
pub const WEATHERED_COPPER_BULB : Block = Block :: new ("weathered_copper_bulb" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: LIT , & BlockStateProperties :: POWERED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: LIT => BlockStateProperties :: LIT . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false))) ;
pub const OXIDIZED_COPPER_BULB : Block = Block :: new ("oxidized_copper_bulb" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: LIT , & BlockStateProperties :: POWERED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: LIT => BlockStateProperties :: LIT . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false))) ;
pub const WAXED_COPPER_BULB : Block = Block :: new ("waxed_copper_bulb" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: LIT , & BlockStateProperties :: POWERED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: LIT => BlockStateProperties :: LIT . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false))) ;
pub const WAXED_EXPOSED_COPPER_BULB : Block = Block :: new ("waxed_exposed_copper_bulb" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: LIT , & BlockStateProperties :: POWERED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: LIT => BlockStateProperties :: LIT . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false))) ;
pub const WAXED_WEATHERED_COPPER_BULB : Block = Block :: new ("waxed_weathered_copper_bulb" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: LIT , & BlockStateProperties :: POWERED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: LIT => BlockStateProperties :: LIT . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false))) ;
pub const WAXED_OXIDIZED_COPPER_BULB : Block = Block :: new ("waxed_oxidized_copper_bulb" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: LIT , & BlockStateProperties :: POWERED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: LIT => BlockStateProperties :: LIT . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false))) ;
pub const COPPER_CHEST : Block = Block :: new ("copper_chest" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: CHEST_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: CHEST_TYPE => crate :: properties :: ChestType :: Single as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const EXPOSED_COPPER_CHEST : Block = Block :: new ("exposed_copper_chest" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: CHEST_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: CHEST_TYPE => crate :: properties :: ChestType :: Single as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const WEATHERED_COPPER_CHEST : Block = Block :: new ("weathered_copper_chest" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: CHEST_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: CHEST_TYPE => crate :: properties :: ChestType :: Single as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const OXIDIZED_COPPER_CHEST : Block = Block :: new ("oxidized_copper_chest" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: CHEST_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: CHEST_TYPE => crate :: properties :: ChestType :: Single as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const WAXED_COPPER_CHEST : Block = Block :: new ("waxed_copper_chest" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: CHEST_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: CHEST_TYPE => crate :: properties :: ChestType :: Single as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const WAXED_EXPOSED_COPPER_CHEST : Block = Block :: new ("waxed_exposed_copper_chest" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: CHEST_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: CHEST_TYPE => crate :: properties :: ChestType :: Single as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const WAXED_WEATHERED_COPPER_CHEST : Block = Block :: new ("waxed_weathered_copper_chest" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: CHEST_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: CHEST_TYPE => crate :: properties :: ChestType :: Single as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const WAXED_OXIDIZED_COPPER_CHEST : Block = Block :: new ("waxed_oxidized_copper_chest" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: CHEST_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: CHEST_TYPE => crate :: properties :: ChestType :: Single as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const COPPER_GOLEM_STATUE : Block = Block :: new ("copper_golem_statue" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: COPPER_GOLEM_POSE , & BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: COPPER_GOLEM_POSE => crate :: properties :: Pose :: Standing as usize , BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const EXPOSED_COPPER_GOLEM_STATUE : Block = Block :: new ("exposed_copper_golem_statue" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: COPPER_GOLEM_POSE , & BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: COPPER_GOLEM_POSE => crate :: properties :: Pose :: Standing as usize , BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const WEATHERED_COPPER_GOLEM_STATUE : Block = Block :: new ("weathered_copper_golem_statue" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: COPPER_GOLEM_POSE , & BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: COPPER_GOLEM_POSE => crate :: properties :: Pose :: Standing as usize , BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const OXIDIZED_COPPER_GOLEM_STATUE : Block = Block :: new ("oxidized_copper_golem_statue" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: COPPER_GOLEM_POSE , & BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: COPPER_GOLEM_POSE => crate :: properties :: Pose :: Standing as usize , BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const WAXED_COPPER_GOLEM_STATUE : Block = Block :: new ("waxed_copper_golem_statue" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: COPPER_GOLEM_POSE , & BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: COPPER_GOLEM_POSE => crate :: properties :: Pose :: Standing as usize , BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const WAXED_EXPOSED_COPPER_GOLEM_STATUE : Block = Block :: new ("waxed_exposed_copper_golem_statue" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: COPPER_GOLEM_POSE , & BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: COPPER_GOLEM_POSE => crate :: properties :: Pose :: Standing as usize , BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const WAXED_WEATHERED_COPPER_GOLEM_STATUE : Block = Block :: new ("waxed_weathered_copper_golem_statue" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: COPPER_GOLEM_POSE , & BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: COPPER_GOLEM_POSE => crate :: properties :: Pose :: Standing as usize , BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const WAXED_OXIDIZED_COPPER_GOLEM_STATUE : Block = Block :: new ("waxed_oxidized_copper_golem_statue" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: COPPER_GOLEM_POSE , & BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: COPPER_GOLEM_POSE => crate :: properties :: Pose :: Standing as usize , BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const LIGHTNING_ROD : Block = Block :: new ("lightning_rod" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: FACING , & BlockStateProperties :: POWERED , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: FACING => crate :: properties :: Direction :: Up as usize , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const EXPOSED_LIGHTNING_ROD : Block = Block :: new ("exposed_lightning_rod" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: FACING , & BlockStateProperties :: POWERED , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: FACING => crate :: properties :: Direction :: Up as usize , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const WEATHERED_LIGHTNING_ROD : Block = Block :: new ("weathered_lightning_rod" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: FACING , & BlockStateProperties :: POWERED , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: FACING => crate :: properties :: Direction :: Up as usize , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const OXIDIZED_LIGHTNING_ROD : Block = Block :: new ("oxidized_lightning_rod" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: FACING , & BlockStateProperties :: POWERED , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: FACING => crate :: properties :: Direction :: Up as usize , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const WAXED_LIGHTNING_ROD : Block = Block :: new ("waxed_lightning_rod" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: FACING , & BlockStateProperties :: POWERED , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: FACING => crate :: properties :: Direction :: Up as usize , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const WAXED_EXPOSED_LIGHTNING_ROD : Block = Block :: new ("waxed_exposed_lightning_rod" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: FACING , & BlockStateProperties :: POWERED , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: FACING => crate :: properties :: Direction :: Up as usize , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const WAXED_WEATHERED_LIGHTNING_ROD : Block = Block :: new ("waxed_weathered_lightning_rod" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: FACING , & BlockStateProperties :: POWERED , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: FACING => crate :: properties :: Direction :: Up as usize , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const WAXED_OXIDIZED_LIGHTNING_ROD : Block = Block :: new ("waxed_oxidized_lightning_rod" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: FACING , & BlockStateProperties :: POWERED , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: FACING => crate :: properties :: Direction :: Up as usize , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const POINTED_DRIPSTONE : Block = Block :: new ("pointed_dripstone" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: DRIPSTONE_THICKNESS , & BlockStateProperties :: VERTICAL_DIRECTION , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: DRIPSTONE_THICKNESS => crate :: properties :: DripstoneThickness :: Tip as usize , BlockStateProperties :: VERTICAL_DIRECTION => crate :: properties :: Direction :: Up as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const DRIPSTONE_BLOCK: Block =
    Block::new("dripstone_block", BlockBehaviourProperties::new(), &[]);
pub const CAVE_VINES : Block = Block :: new ("cave_vines" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: AGE_25 , & BlockStateProperties :: BERRIES] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: AGE_25 => 0usize , BlockStateProperties :: BERRIES => BlockStateProperties :: BERRIES . index_of (false))) ;
pub const CAVE_VINES_PLANT : Block = Block :: new ("cave_vines_plant" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: BERRIES] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: BERRIES => BlockStateProperties :: BERRIES . index_of (false))) ;
pub const SPORE_BLOSSOM: Block = Block::new("spore_blossom", BlockBehaviourProperties::new(), &[]);
pub const AZALEA: Block = Block::new("azalea", BlockBehaviourProperties::new(), &[]);
pub const FLOWERING_AZALEA: Block =
    Block::new("flowering_azalea", BlockBehaviourProperties::new(), &[]);
pub const MOSS_CARPET: Block = Block::new("moss_carpet", BlockBehaviourProperties::new(), &[]);
pub const PINK_PETALS : Block = Block :: new ("pink_petals" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: FLOWER_AMOUNT] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: FLOWER_AMOUNT => 1usize)) ;
pub const WILDFLOWERS : Block = Block :: new ("wildflowers" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: FLOWER_AMOUNT] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: FLOWER_AMOUNT => 1usize)) ;
pub const LEAF_LITTER : Block = Block :: new ("leaf_litter" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: SEGMENT_AMOUNT] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: SEGMENT_AMOUNT => 1usize)) ;
pub const MOSS_BLOCK: Block = Block::new("moss_block", BlockBehaviourProperties::new(), &[]);
pub const BIG_DRIPLEAF : Block = Block :: new ("big_dripleaf" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: TILT , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: TILT => crate :: properties :: Tilt :: None as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const BIG_DRIPLEAF_STEM : Block = Block :: new ("big_dripleaf_stem" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const SMALL_DRIPLEAF : Block = Block :: new ("small_dripleaf" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: DOUBLE_BLOCK_HALF , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: DOUBLE_BLOCK_HALF => crate :: properties :: DoubleBlockHalf :: Lower as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const HANGING_ROOTS : Block = Block :: new ("hanging_roots" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const ROOTED_DIRT: Block = Block::new("rooted_dirt", BlockBehaviourProperties::new(), &[]);
pub const MUD: Block = Block::new("mud", BlockBehaviourProperties::new(), &[]);
pub const DEEPSLATE : Block = Block :: new ("deepslate" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: AXIS] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: AXIS => crate :: properties :: Axis :: Y as usize)) ;
pub const COBBLED_DEEPSLATE: Block =
    Block::new("cobbled_deepslate", BlockBehaviourProperties::new(), &[]);
pub const COBBLED_DEEPSLATE_STAIRS : Block = Block :: new ("cobbled_deepslate_stairs" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: STAIRS_SHAPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: HALF => crate :: properties :: Half :: Bottom as usize , BlockStateProperties :: STAIRS_SHAPE => crate :: properties :: StairsShape :: Straight as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const COBBLED_DEEPSLATE_SLAB : Block = Block :: new ("cobbled_deepslate_slab" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: SLAB_TYPE => crate :: properties :: SlabType :: Bottom as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const COBBLED_DEEPSLATE_WALL : Block = Block :: new ("cobbled_deepslate_wall" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: EAST_WALL , & BlockStateProperties :: NORTH_WALL , & BlockStateProperties :: SOUTH_WALL , & BlockStateProperties :: UP , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST_WALL] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: EAST_WALL => crate :: properties :: WallSide :: None as usize , BlockStateProperties :: NORTH_WALL => crate :: properties :: WallSide :: None as usize , BlockStateProperties :: SOUTH_WALL => crate :: properties :: WallSide :: None as usize , BlockStateProperties :: UP => BlockStateProperties :: UP . index_of (true) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST_WALL => crate :: properties :: WallSide :: None as usize)) ;
pub const POLISHED_DEEPSLATE: Block =
    Block::new("polished_deepslate", BlockBehaviourProperties::new(), &[]);
pub const POLISHED_DEEPSLATE_STAIRS : Block = Block :: new ("polished_deepslate_stairs" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: STAIRS_SHAPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: HALF => crate :: properties :: Half :: Bottom as usize , BlockStateProperties :: STAIRS_SHAPE => crate :: properties :: StairsShape :: Straight as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const POLISHED_DEEPSLATE_SLAB : Block = Block :: new ("polished_deepslate_slab" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: SLAB_TYPE => crate :: properties :: SlabType :: Bottom as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const POLISHED_DEEPSLATE_WALL : Block = Block :: new ("polished_deepslate_wall" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: EAST_WALL , & BlockStateProperties :: NORTH_WALL , & BlockStateProperties :: SOUTH_WALL , & BlockStateProperties :: UP , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST_WALL] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: EAST_WALL => crate :: properties :: WallSide :: None as usize , BlockStateProperties :: NORTH_WALL => crate :: properties :: WallSide :: None as usize , BlockStateProperties :: SOUTH_WALL => crate :: properties :: WallSide :: None as usize , BlockStateProperties :: UP => BlockStateProperties :: UP . index_of (true) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST_WALL => crate :: properties :: WallSide :: None as usize)) ;
pub const DEEPSLATE_TILES: Block =
    Block::new("deepslate_tiles", BlockBehaviourProperties::new(), &[]);
pub const DEEPSLATE_TILE_STAIRS : Block = Block :: new ("deepslate_tile_stairs" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: STAIRS_SHAPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: HALF => crate :: properties :: Half :: Bottom as usize , BlockStateProperties :: STAIRS_SHAPE => crate :: properties :: StairsShape :: Straight as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const DEEPSLATE_TILE_SLAB : Block = Block :: new ("deepslate_tile_slab" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: SLAB_TYPE => crate :: properties :: SlabType :: Bottom as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const DEEPSLATE_TILE_WALL : Block = Block :: new ("deepslate_tile_wall" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: EAST_WALL , & BlockStateProperties :: NORTH_WALL , & BlockStateProperties :: SOUTH_WALL , & BlockStateProperties :: UP , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST_WALL] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: EAST_WALL => crate :: properties :: WallSide :: None as usize , BlockStateProperties :: NORTH_WALL => crate :: properties :: WallSide :: None as usize , BlockStateProperties :: SOUTH_WALL => crate :: properties :: WallSide :: None as usize , BlockStateProperties :: UP => BlockStateProperties :: UP . index_of (true) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST_WALL => crate :: properties :: WallSide :: None as usize)) ;
pub const DEEPSLATE_BRICKS: Block =
    Block::new("deepslate_bricks", BlockBehaviourProperties::new(), &[]);
pub const DEEPSLATE_BRICK_STAIRS : Block = Block :: new ("deepslate_brick_stairs" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: STAIRS_SHAPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: HALF => crate :: properties :: Half :: Bottom as usize , BlockStateProperties :: STAIRS_SHAPE => crate :: properties :: StairsShape :: Straight as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const DEEPSLATE_BRICK_SLAB : Block = Block :: new ("deepslate_brick_slab" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: SLAB_TYPE => crate :: properties :: SlabType :: Bottom as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const DEEPSLATE_BRICK_WALL : Block = Block :: new ("deepslate_brick_wall" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: EAST_WALL , & BlockStateProperties :: NORTH_WALL , & BlockStateProperties :: SOUTH_WALL , & BlockStateProperties :: UP , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST_WALL] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: EAST_WALL => crate :: properties :: WallSide :: None as usize , BlockStateProperties :: NORTH_WALL => crate :: properties :: WallSide :: None as usize , BlockStateProperties :: SOUTH_WALL => crate :: properties :: WallSide :: None as usize , BlockStateProperties :: UP => BlockStateProperties :: UP . index_of (true) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST_WALL => crate :: properties :: WallSide :: None as usize)) ;
pub const CHISELED_DEEPSLATE: Block =
    Block::new("chiseled_deepslate", BlockBehaviourProperties::new(), &[]);
pub const CRACKED_DEEPSLATE_BRICKS: Block = Block::new(
    "cracked_deepslate_bricks",
    BlockBehaviourProperties::new(),
    &[],
);
pub const CRACKED_DEEPSLATE_TILES: Block = Block::new(
    "cracked_deepslate_tiles",
    BlockBehaviourProperties::new(),
    &[],
);
pub const INFESTED_DEEPSLATE : Block = Block :: new ("infested_deepslate" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: AXIS] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: AXIS => crate :: properties :: Axis :: Y as usize)) ;
pub const SMOOTH_BASALT: Block = Block::new("smooth_basalt", BlockBehaviourProperties::new(), &[]);
pub const RAW_IRON_BLOCK: Block =
    Block::new("raw_iron_block", BlockBehaviourProperties::new(), &[]);
pub const RAW_COPPER_BLOCK: Block =
    Block::new("raw_copper_block", BlockBehaviourProperties::new(), &[]);
pub const RAW_GOLD_BLOCK: Block =
    Block::new("raw_gold_block", BlockBehaviourProperties::new(), &[]);
pub const POTTED_AZALEA_BUSH: Block =
    Block::new("potted_azalea_bush", BlockBehaviourProperties::new(), &[]);
pub const POTTED_FLOWERING_AZALEA_BUSH: Block = Block::new(
    "potted_flowering_azalea_bush",
    BlockBehaviourProperties::new(),
    &[],
);
pub const OCHRE_FROGLIGHT : Block = Block :: new ("ochre_froglight" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: AXIS] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: AXIS => crate :: properties :: Axis :: Y as usize)) ;
pub const VERDANT_FROGLIGHT : Block = Block :: new ("verdant_froglight" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: AXIS] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: AXIS => crate :: properties :: Axis :: Y as usize)) ;
pub const PEARLESCENT_FROGLIGHT : Block = Block :: new ("pearlescent_froglight" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: AXIS] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: AXIS => crate :: properties :: Axis :: Y as usize)) ;
pub const FROGSPAWN: Block = Block::new("frogspawn", BlockBehaviourProperties::new(), &[]);
pub const REINFORCED_DEEPSLATE: Block =
    Block::new("reinforced_deepslate", BlockBehaviourProperties::new(), &[]);
pub const DECORATED_POT : Block = Block :: new ("decorated_pot" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: CRACKED , & BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: CRACKED => BlockStateProperties :: CRACKED . index_of (false) , BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const CRAFTER : Block = Block :: new ("crafter" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: CRAFTING , & BlockStateProperties :: ORIENTATION , & BlockStateProperties :: TRIGGERED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: CRAFTING => BlockStateProperties :: CRAFTING . index_of (false) , BlockStateProperties :: ORIENTATION => crate :: properties :: FrontAndTop :: NorthUp as usize , BlockStateProperties :: TRIGGERED => BlockStateProperties :: TRIGGERED . index_of (false))) ;
pub const TRIAL_SPAWNER : Block = Block :: new ("trial_spawner" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: OMINOUS , & BlockStateProperties :: TRIAL_SPAWNER_STATE] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: OMINOUS => BlockStateProperties :: OMINOUS . index_of (false) , BlockStateProperties :: TRIAL_SPAWNER_STATE => crate :: properties :: TrialSpawnerState :: Inactive as usize)) ;
pub const VAULT : Block = Block :: new ("vault" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: OMINOUS , & BlockStateProperties :: VAULT_STATE] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: HORIZONTAL_FACING => crate :: properties :: Direction :: North as usize , BlockStateProperties :: OMINOUS => BlockStateProperties :: OMINOUS . index_of (false) , BlockStateProperties :: VAULT_STATE => crate :: properties :: VaultState :: Inactive as usize)) ;
pub const HEAVY_CORE : Block = Block :: new ("heavy_core" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: WATERLOGGED] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false))) ;
pub const PALE_MOSS_BLOCK: Block =
    Block::new("pale_moss_block", BlockBehaviourProperties::new(), &[]);
pub const PALE_MOSS_CARPET : Block = Block :: new ("pale_moss_carpet" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: BOTTOM , & BlockStateProperties :: EAST_WALL , & BlockStateProperties :: NORTH_WALL , & BlockStateProperties :: SOUTH_WALL , & BlockStateProperties :: WEST_WALL] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: BOTTOM => BlockStateProperties :: BOTTOM . index_of (true) , BlockStateProperties :: EAST_WALL => crate :: properties :: WallSide :: None as usize , BlockStateProperties :: NORTH_WALL => crate :: properties :: WallSide :: None as usize , BlockStateProperties :: SOUTH_WALL => crate :: properties :: WallSide :: None as usize , BlockStateProperties :: WEST_WALL => crate :: properties :: WallSide :: None as usize)) ;
pub const PALE_HANGING_MOSS : Block = Block :: new ("pale_hanging_moss" , BlockBehaviourProperties :: new () , & [& BlockStateProperties :: TIP] ,) . with_default_state (crate :: blocks :: offset ! (BlockStateProperties :: TIP => BlockStateProperties :: TIP . index_of (true))) ;
pub const OPEN_EYEBLOSSOM: Block =
    Block::new("open_eyeblossom", BlockBehaviourProperties::new(), &[]);
pub const CLOSED_EYEBLOSSOM: Block =
    Block::new("closed_eyeblossom", BlockBehaviourProperties::new(), &[]);
pub const POTTED_OPEN_EYEBLOSSOM: Block = Block::new(
    "potted_open_eyeblossom",
    BlockBehaviourProperties::new(),
    &[],
);
pub const POTTED_CLOSED_EYEBLOSSOM: Block = Block::new(
    "potted_closed_eyeblossom",
    BlockBehaviourProperties::new(),
    &[],
);
pub const FIREFLY_BUSH: Block = Block::new("firefly_bush", BlockBehaviourProperties::new(), &[]);
pub fn register_blocks(registry: &mut BlockRegistry) {
    registry.register(&AIR);
    registry.register(&STONE);
    registry.register(&GRANITE);
    registry.register(&POLISHED_GRANITE);
    registry.register(&DIORITE);
    registry.register(&POLISHED_DIORITE);
    registry.register(&ANDESITE);
    registry.register(&POLISHED_ANDESITE);
    registry.register(&GRASS_BLOCK);
    registry.register(&DIRT);
    registry.register(&COARSE_DIRT);
    registry.register(&PODZOL);
    registry.register(&COBBLESTONE);
    registry.register(&OAK_PLANKS);
    registry.register(&SPRUCE_PLANKS);
    registry.register(&BIRCH_PLANKS);
    registry.register(&JUNGLE_PLANKS);
    registry.register(&ACACIA_PLANKS);
    registry.register(&CHERRY_PLANKS);
    registry.register(&DARK_OAK_PLANKS);
    registry.register(&PALE_OAK_WOOD);
    registry.register(&PALE_OAK_PLANKS);
    registry.register(&MANGROVE_PLANKS);
    registry.register(&BAMBOO_PLANKS);
    registry.register(&BAMBOO_MOSAIC);
    registry.register(&OAK_SAPLING);
    registry.register(&SPRUCE_SAPLING);
    registry.register(&BIRCH_SAPLING);
    registry.register(&JUNGLE_SAPLING);
    registry.register(&ACACIA_SAPLING);
    registry.register(&CHERRY_SAPLING);
    registry.register(&DARK_OAK_SAPLING);
    registry.register(&PALE_OAK_SAPLING);
    registry.register(&MANGROVE_PROPAGULE);
    registry.register(&BEDROCK);
    registry.register(&WATER);
    registry.register(&LAVA);
    registry.register(&SAND);
    registry.register(&SUSPICIOUS_SAND);
    registry.register(&RED_SAND);
    registry.register(&GRAVEL);
    registry.register(&SUSPICIOUS_GRAVEL);
    registry.register(&GOLD_ORE);
    registry.register(&DEEPSLATE_GOLD_ORE);
    registry.register(&IRON_ORE);
    registry.register(&DEEPSLATE_IRON_ORE);
    registry.register(&COAL_ORE);
    registry.register(&DEEPSLATE_COAL_ORE);
    registry.register(&NETHER_GOLD_ORE);
    registry.register(&OAK_LOG);
    registry.register(&SPRUCE_LOG);
    registry.register(&BIRCH_LOG);
    registry.register(&JUNGLE_LOG);
    registry.register(&ACACIA_LOG);
    registry.register(&CHERRY_LOG);
    registry.register(&DARK_OAK_LOG);
    registry.register(&PALE_OAK_LOG);
    registry.register(&MANGROVE_LOG);
    registry.register(&MANGROVE_ROOTS);
    registry.register(&MUDDY_MANGROVE_ROOTS);
    registry.register(&BAMBOO_BLOCK);
    registry.register(&STRIPPED_SPRUCE_LOG);
    registry.register(&STRIPPED_BIRCH_LOG);
    registry.register(&STRIPPED_JUNGLE_LOG);
    registry.register(&STRIPPED_ACACIA_LOG);
    registry.register(&STRIPPED_CHERRY_LOG);
    registry.register(&STRIPPED_DARK_OAK_LOG);
    registry.register(&STRIPPED_PALE_OAK_LOG);
    registry.register(&STRIPPED_OAK_LOG);
    registry.register(&STRIPPED_MANGROVE_LOG);
    registry.register(&STRIPPED_BAMBOO_BLOCK);
    registry.register(&OAK_WOOD);
    registry.register(&SPRUCE_WOOD);
    registry.register(&BIRCH_WOOD);
    registry.register(&JUNGLE_WOOD);
    registry.register(&ACACIA_WOOD);
    registry.register(&CHERRY_WOOD);
    registry.register(&DARK_OAK_WOOD);
    registry.register(&MANGROVE_WOOD);
    registry.register(&STRIPPED_OAK_WOOD);
    registry.register(&STRIPPED_SPRUCE_WOOD);
    registry.register(&STRIPPED_BIRCH_WOOD);
    registry.register(&STRIPPED_JUNGLE_WOOD);
    registry.register(&STRIPPED_ACACIA_WOOD);
    registry.register(&STRIPPED_CHERRY_WOOD);
    registry.register(&STRIPPED_DARK_OAK_WOOD);
    registry.register(&STRIPPED_PALE_OAK_WOOD);
    registry.register(&STRIPPED_MANGROVE_WOOD);
    registry.register(&OAK_LEAVES);
    registry.register(&SPRUCE_LEAVES);
    registry.register(&BIRCH_LEAVES);
    registry.register(&JUNGLE_LEAVES);
    registry.register(&ACACIA_LEAVES);
    registry.register(&CHERRY_LEAVES);
    registry.register(&DARK_OAK_LEAVES);
    registry.register(&PALE_OAK_LEAVES);
    registry.register(&MANGROVE_LEAVES);
    registry.register(&AZALEA_LEAVES);
    registry.register(&FLOWERING_AZALEA_LEAVES);
    registry.register(&SPONGE);
    registry.register(&WET_SPONGE);
    registry.register(&GLASS);
    registry.register(&LAPIS_ORE);
    registry.register(&DEEPSLATE_LAPIS_ORE);
    registry.register(&LAPIS_BLOCK);
    registry.register(&DISPENSER);
    registry.register(&SANDSTONE);
    registry.register(&CHISELED_SANDSTONE);
    registry.register(&CUT_SANDSTONE);
    registry.register(&NOTE_BLOCK);
    registry.register(&WHITE_BED);
    registry.register(&ORANGE_BED);
    registry.register(&MAGENTA_BED);
    registry.register(&LIGHT_BLUE_BED);
    registry.register(&YELLOW_BED);
    registry.register(&LIME_BED);
    registry.register(&PINK_BED);
    registry.register(&GRAY_BED);
    registry.register(&LIGHT_GRAY_BED);
    registry.register(&CYAN_BED);
    registry.register(&PURPLE_BED);
    registry.register(&BLUE_BED);
    registry.register(&BROWN_BED);
    registry.register(&GREEN_BED);
    registry.register(&RED_BED);
    registry.register(&BLACK_BED);
    registry.register(&POWERED_RAIL);
    registry.register(&DETECTOR_RAIL);
    registry.register(&STICKY_PISTON);
    registry.register(&COBWEB);
    registry.register(&SHORT_GRASS);
    registry.register(&FERN);
    registry.register(&DEAD_BUSH);
    registry.register(&BUSH);
    registry.register(&SHORT_DRY_GRASS);
    registry.register(&TALL_DRY_GRASS);
    registry.register(&SEAGRASS);
    registry.register(&TALL_SEAGRASS);
    registry.register(&PISTON);
    registry.register(&PISTON_HEAD);
    registry.register(&WHITE_WOOL);
    registry.register(&ORANGE_WOOL);
    registry.register(&MAGENTA_WOOL);
    registry.register(&LIGHT_BLUE_WOOL);
    registry.register(&YELLOW_WOOL);
    registry.register(&LIME_WOOL);
    registry.register(&PINK_WOOL);
    registry.register(&GRAY_WOOL);
    registry.register(&LIGHT_GRAY_WOOL);
    registry.register(&CYAN_WOOL);
    registry.register(&PURPLE_WOOL);
    registry.register(&BLUE_WOOL);
    registry.register(&BROWN_WOOL);
    registry.register(&GREEN_WOOL);
    registry.register(&RED_WOOL);
    registry.register(&BLACK_WOOL);
    registry.register(&MOVING_PISTON);
    registry.register(&DANDELION);
    registry.register(&TORCHFLOWER);
    registry.register(&POPPY);
    registry.register(&BLUE_ORCHID);
    registry.register(&ALLIUM);
    registry.register(&AZURE_BLUET);
    registry.register(&RED_TULIP);
    registry.register(&ORANGE_TULIP);
    registry.register(&WHITE_TULIP);
    registry.register(&PINK_TULIP);
    registry.register(&OXEYE_DAISY);
    registry.register(&CORNFLOWER);
    registry.register(&WITHER_ROSE);
    registry.register(&LILY_OF_THE_VALLEY);
    registry.register(&BROWN_MUSHROOM);
    registry.register(&RED_MUSHROOM);
    registry.register(&GOLD_BLOCK);
    registry.register(&IRON_BLOCK);
    registry.register(&BRICKS);
    registry.register(&TNT);
    registry.register(&BOOKSHELF);
    registry.register(&CHISELED_BOOKSHELF);
    registry.register(&ACACIA_SHELF);
    registry.register(&BAMBOO_SHELF);
    registry.register(&BIRCH_SHELF);
    registry.register(&CHERRY_SHELF);
    registry.register(&CRIMSON_SHELF);
    registry.register(&DARK_OAK_SHELF);
    registry.register(&JUNGLE_SHELF);
    registry.register(&MANGROVE_SHELF);
    registry.register(&OAK_SHELF);
    registry.register(&PALE_OAK_SHELF);
    registry.register(&SPRUCE_SHELF);
    registry.register(&WARPED_SHELF);
    registry.register(&MOSSY_COBBLESTONE);
    registry.register(&OBSIDIAN);
    registry.register(&TORCH);
    registry.register(&WALL_TORCH);
    registry.register(&FIRE);
    registry.register(&SOUL_FIRE);
    registry.register(&SPAWNER);
    registry.register(&CREAKING_HEART);
    registry.register(&OAK_STAIRS);
    registry.register(&CHEST);
    registry.register(&REDSTONE_WIRE);
    registry.register(&DIAMOND_ORE);
    registry.register(&DEEPSLATE_DIAMOND_ORE);
    registry.register(&DIAMOND_BLOCK);
    registry.register(&CRAFTING_TABLE);
    registry.register(&WHEAT);
    registry.register(&FARMLAND);
    registry.register(&FURNACE);
    registry.register(&OAK_SIGN);
    registry.register(&SPRUCE_SIGN);
    registry.register(&BIRCH_SIGN);
    registry.register(&ACACIA_SIGN);
    registry.register(&CHERRY_SIGN);
    registry.register(&JUNGLE_SIGN);
    registry.register(&DARK_OAK_SIGN);
    registry.register(&PALE_OAK_SIGN);
    registry.register(&MANGROVE_SIGN);
    registry.register(&BAMBOO_SIGN);
    registry.register(&OAK_DOOR);
    registry.register(&LADDER);
    registry.register(&RAIL);
    registry.register(&COBBLESTONE_STAIRS);
    registry.register(&OAK_WALL_SIGN);
    registry.register(&SPRUCE_WALL_SIGN);
    registry.register(&BIRCH_WALL_SIGN);
    registry.register(&ACACIA_WALL_SIGN);
    registry.register(&CHERRY_WALL_SIGN);
    registry.register(&JUNGLE_WALL_SIGN);
    registry.register(&DARK_OAK_WALL_SIGN);
    registry.register(&PALE_OAK_WALL_SIGN);
    registry.register(&MANGROVE_WALL_SIGN);
    registry.register(&BAMBOO_WALL_SIGN);
    registry.register(&OAK_HANGING_SIGN);
    registry.register(&SPRUCE_HANGING_SIGN);
    registry.register(&BIRCH_HANGING_SIGN);
    registry.register(&ACACIA_HANGING_SIGN);
    registry.register(&CHERRY_HANGING_SIGN);
    registry.register(&JUNGLE_HANGING_SIGN);
    registry.register(&DARK_OAK_HANGING_SIGN);
    registry.register(&PALE_OAK_HANGING_SIGN);
    registry.register(&CRIMSON_HANGING_SIGN);
    registry.register(&WARPED_HANGING_SIGN);
    registry.register(&MANGROVE_HANGING_SIGN);
    registry.register(&BAMBOO_HANGING_SIGN);
    registry.register(&OAK_WALL_HANGING_SIGN);
    registry.register(&SPRUCE_WALL_HANGING_SIGN);
    registry.register(&BIRCH_WALL_HANGING_SIGN);
    registry.register(&ACACIA_WALL_HANGING_SIGN);
    registry.register(&CHERRY_WALL_HANGING_SIGN);
    registry.register(&JUNGLE_WALL_HANGING_SIGN);
    registry.register(&DARK_OAK_WALL_HANGING_SIGN);
    registry.register(&PALE_OAK_WALL_HANGING_SIGN);
    registry.register(&MANGROVE_WALL_HANGING_SIGN);
    registry.register(&CRIMSON_WALL_HANGING_SIGN);
    registry.register(&WARPED_WALL_HANGING_SIGN);
    registry.register(&BAMBOO_WALL_HANGING_SIGN);
    registry.register(&LEVER);
    registry.register(&STONE_PRESSURE_PLATE);
    registry.register(&IRON_DOOR);
    registry.register(&OAK_PRESSURE_PLATE);
    registry.register(&SPRUCE_PRESSURE_PLATE);
    registry.register(&BIRCH_PRESSURE_PLATE);
    registry.register(&JUNGLE_PRESSURE_PLATE);
    registry.register(&ACACIA_PRESSURE_PLATE);
    registry.register(&CHERRY_PRESSURE_PLATE);
    registry.register(&DARK_OAK_PRESSURE_PLATE);
    registry.register(&PALE_OAK_PRESSURE_PLATE);
    registry.register(&MANGROVE_PRESSURE_PLATE);
    registry.register(&BAMBOO_PRESSURE_PLATE);
    registry.register(&REDSTONE_ORE);
    registry.register(&DEEPSLATE_REDSTONE_ORE);
    registry.register(&REDSTONE_TORCH);
    registry.register(&REDSTONE_WALL_TORCH);
    registry.register(&STONE_BUTTON);
    registry.register(&SNOW);
    registry.register(&ICE);
    registry.register(&SNOW_BLOCK);
    registry.register(&CACTUS);
    registry.register(&CACTUS_FLOWER);
    registry.register(&CLAY);
    registry.register(&SUGAR_CANE);
    registry.register(&JUKEBOX);
    registry.register(&OAK_FENCE);
    registry.register(&NETHERRACK);
    registry.register(&SOUL_SAND);
    registry.register(&SOUL_SOIL);
    registry.register(&BASALT);
    registry.register(&POLISHED_BASALT);
    registry.register(&SOUL_TORCH);
    registry.register(&SOUL_WALL_TORCH);
    registry.register(&COPPER_TORCH);
    registry.register(&COPPER_WALL_TORCH);
    registry.register(&GLOWSTONE);
    registry.register(&NETHER_PORTAL);
    registry.register(&CARVED_PUMPKIN);
    registry.register(&JACK_O_LANTERN);
    registry.register(&CAKE);
    registry.register(&REPEATER);
    registry.register(&WHITE_STAINED_GLASS);
    registry.register(&ORANGE_STAINED_GLASS);
    registry.register(&MAGENTA_STAINED_GLASS);
    registry.register(&LIGHT_BLUE_STAINED_GLASS);
    registry.register(&YELLOW_STAINED_GLASS);
    registry.register(&LIME_STAINED_GLASS);
    registry.register(&PINK_STAINED_GLASS);
    registry.register(&GRAY_STAINED_GLASS);
    registry.register(&LIGHT_GRAY_STAINED_GLASS);
    registry.register(&CYAN_STAINED_GLASS);
    registry.register(&PURPLE_STAINED_GLASS);
    registry.register(&BLUE_STAINED_GLASS);
    registry.register(&BROWN_STAINED_GLASS);
    registry.register(&GREEN_STAINED_GLASS);
    registry.register(&RED_STAINED_GLASS);
    registry.register(&BLACK_STAINED_GLASS);
    registry.register(&OAK_TRAPDOOR);
    registry.register(&SPRUCE_TRAPDOOR);
    registry.register(&BIRCH_TRAPDOOR);
    registry.register(&JUNGLE_TRAPDOOR);
    registry.register(&ACACIA_TRAPDOOR);
    registry.register(&CHERRY_TRAPDOOR);
    registry.register(&DARK_OAK_TRAPDOOR);
    registry.register(&PALE_OAK_TRAPDOOR);
    registry.register(&MANGROVE_TRAPDOOR);
    registry.register(&BAMBOO_TRAPDOOR);
    registry.register(&STONE_BRICKS);
    registry.register(&MOSSY_STONE_BRICKS);
    registry.register(&CRACKED_STONE_BRICKS);
    registry.register(&CHISELED_STONE_BRICKS);
    registry.register(&PACKED_MUD);
    registry.register(&MUD_BRICKS);
    registry.register(&INFESTED_STONE);
    registry.register(&INFESTED_COBBLESTONE);
    registry.register(&INFESTED_STONE_BRICKS);
    registry.register(&INFESTED_MOSSY_STONE_BRICKS);
    registry.register(&INFESTED_CRACKED_STONE_BRICKS);
    registry.register(&INFESTED_CHISELED_STONE_BRICKS);
    registry.register(&BROWN_MUSHROOM_BLOCK);
    registry.register(&RED_MUSHROOM_BLOCK);
    registry.register(&MUSHROOM_STEM);
    registry.register(&IRON_BARS);
    registry.register(&COPPER_BARS);
    registry.register(&EXPOSED_COPPER_BARS);
    registry.register(&WEATHERED_COPPER_BARS);
    registry.register(&OXIDIZED_COPPER_BARS);
    registry.register(&WAXED_COPPER_BARS);
    registry.register(&WAXED_EXPOSED_COPPER_BARS);
    registry.register(&WAXED_WEATHERED_COPPER_BARS);
    registry.register(&WAXED_OXIDIZED_COPPER_BARS);
    registry.register(&IRON_CHAIN);
    registry.register(&COPPER_CHAIN);
    registry.register(&EXPOSED_COPPER_CHAIN);
    registry.register(&WEATHERED_COPPER_CHAIN);
    registry.register(&OXIDIZED_COPPER_CHAIN);
    registry.register(&WAXED_COPPER_CHAIN);
    registry.register(&WAXED_EXPOSED_COPPER_CHAIN);
    registry.register(&WAXED_WEATHERED_COPPER_CHAIN);
    registry.register(&WAXED_OXIDIZED_COPPER_CHAIN);
    registry.register(&GLASS_PANE);
    registry.register(&PUMPKIN);
    registry.register(&MELON);
    registry.register(&ATTACHED_PUMPKIN_STEM);
    registry.register(&ATTACHED_MELON_STEM);
    registry.register(&PUMPKIN_STEM);
    registry.register(&MELON_STEM);
    registry.register(&VINE);
    registry.register(&GLOW_LICHEN);
    registry.register(&RESIN_CLUMP);
    registry.register(&OAK_FENCE_GATE);
    registry.register(&BRICK_STAIRS);
    registry.register(&STONE_BRICK_STAIRS);
    registry.register(&MUD_BRICK_STAIRS);
    registry.register(&MYCELIUM);
    registry.register(&LILY_PAD);
    registry.register(&RESIN_BLOCK);
    registry.register(&RESIN_BRICKS);
    registry.register(&RESIN_BRICK_STAIRS);
    registry.register(&RESIN_BRICK_SLAB);
    registry.register(&RESIN_BRICK_WALL);
    registry.register(&CHISELED_RESIN_BRICKS);
    registry.register(&NETHER_BRICKS);
    registry.register(&NETHER_BRICK_FENCE);
    registry.register(&NETHER_BRICK_STAIRS);
    registry.register(&NETHER_WART);
    registry.register(&ENCHANTING_TABLE);
    registry.register(&BREWING_STAND);
    registry.register(&CAULDRON);
    registry.register(&WATER_CAULDRON);
    registry.register(&LAVA_CAULDRON);
    registry.register(&POWDER_SNOW_CAULDRON);
    registry.register(&END_PORTAL);
    registry.register(&END_PORTAL_FRAME);
    registry.register(&END_STONE);
    registry.register(&DRAGON_EGG);
    registry.register(&REDSTONE_LAMP);
    registry.register(&COCOA);
    registry.register(&SANDSTONE_STAIRS);
    registry.register(&EMERALD_ORE);
    registry.register(&DEEPSLATE_EMERALD_ORE);
    registry.register(&ENDER_CHEST);
    registry.register(&TRIPWIRE_HOOK);
    registry.register(&TRIPWIRE);
    registry.register(&EMERALD_BLOCK);
    registry.register(&SPRUCE_STAIRS);
    registry.register(&BIRCH_STAIRS);
    registry.register(&JUNGLE_STAIRS);
    registry.register(&COMMAND_BLOCK);
    registry.register(&BEACON);
    registry.register(&COBBLESTONE_WALL);
    registry.register(&MOSSY_COBBLESTONE_WALL);
    registry.register(&FLOWER_POT);
    registry.register(&POTTED_TORCHFLOWER);
    registry.register(&POTTED_OAK_SAPLING);
    registry.register(&POTTED_SPRUCE_SAPLING);
    registry.register(&POTTED_BIRCH_SAPLING);
    registry.register(&POTTED_JUNGLE_SAPLING);
    registry.register(&POTTED_ACACIA_SAPLING);
    registry.register(&POTTED_CHERRY_SAPLING);
    registry.register(&POTTED_DARK_OAK_SAPLING);
    registry.register(&POTTED_PALE_OAK_SAPLING);
    registry.register(&POTTED_MANGROVE_PROPAGULE);
    registry.register(&POTTED_FERN);
    registry.register(&POTTED_DANDELION);
    registry.register(&POTTED_POPPY);
    registry.register(&POTTED_BLUE_ORCHID);
    registry.register(&POTTED_ALLIUM);
    registry.register(&POTTED_AZURE_BLUET);
    registry.register(&POTTED_RED_TULIP);
    registry.register(&POTTED_ORANGE_TULIP);
    registry.register(&POTTED_WHITE_TULIP);
    registry.register(&POTTED_PINK_TULIP);
    registry.register(&POTTED_OXEYE_DAISY);
    registry.register(&POTTED_CORNFLOWER);
    registry.register(&POTTED_LILY_OF_THE_VALLEY);
    registry.register(&POTTED_WITHER_ROSE);
    registry.register(&POTTED_RED_MUSHROOM);
    registry.register(&POTTED_BROWN_MUSHROOM);
    registry.register(&POTTED_DEAD_BUSH);
    registry.register(&POTTED_CACTUS);
    registry.register(&CARROTS);
    registry.register(&POTATOES);
    registry.register(&OAK_BUTTON);
    registry.register(&SPRUCE_BUTTON);
    registry.register(&BIRCH_BUTTON);
    registry.register(&JUNGLE_BUTTON);
    registry.register(&ACACIA_BUTTON);
    registry.register(&CHERRY_BUTTON);
    registry.register(&DARK_OAK_BUTTON);
    registry.register(&PALE_OAK_BUTTON);
    registry.register(&MANGROVE_BUTTON);
    registry.register(&BAMBOO_BUTTON);
    registry.register(&SKELETON_SKULL);
    registry.register(&SKELETON_WALL_SKULL);
    registry.register(&WITHER_SKELETON_SKULL);
    registry.register(&WITHER_SKELETON_WALL_SKULL);
    registry.register(&ZOMBIE_HEAD);
    registry.register(&ZOMBIE_WALL_HEAD);
    registry.register(&PLAYER_HEAD);
    registry.register(&PLAYER_WALL_HEAD);
    registry.register(&CREEPER_HEAD);
    registry.register(&CREEPER_WALL_HEAD);
    registry.register(&DRAGON_HEAD);
    registry.register(&DRAGON_WALL_HEAD);
    registry.register(&PIGLIN_HEAD);
    registry.register(&PIGLIN_WALL_HEAD);
    registry.register(&ANVIL);
    registry.register(&CHIPPED_ANVIL);
    registry.register(&DAMAGED_ANVIL);
    registry.register(&TRAPPED_CHEST);
    registry.register(&LIGHT_WEIGHTED_PRESSURE_PLATE);
    registry.register(&HEAVY_WEIGHTED_PRESSURE_PLATE);
    registry.register(&COMPARATOR);
    registry.register(&DAYLIGHT_DETECTOR);
    registry.register(&REDSTONE_BLOCK);
    registry.register(&NETHER_QUARTZ_ORE);
    registry.register(&HOPPER);
    registry.register(&QUARTZ_BLOCK);
    registry.register(&CHISELED_QUARTZ_BLOCK);
    registry.register(&QUARTZ_PILLAR);
    registry.register(&QUARTZ_STAIRS);
    registry.register(&ACTIVATOR_RAIL);
    registry.register(&DROPPER);
    registry.register(&WHITE_TERRACOTTA);
    registry.register(&ORANGE_TERRACOTTA);
    registry.register(&MAGENTA_TERRACOTTA);
    registry.register(&LIGHT_BLUE_TERRACOTTA);
    registry.register(&YELLOW_TERRACOTTA);
    registry.register(&LIME_TERRACOTTA);
    registry.register(&PINK_TERRACOTTA);
    registry.register(&GRAY_TERRACOTTA);
    registry.register(&LIGHT_GRAY_TERRACOTTA);
    registry.register(&CYAN_TERRACOTTA);
    registry.register(&PURPLE_TERRACOTTA);
    registry.register(&BLUE_TERRACOTTA);
    registry.register(&BROWN_TERRACOTTA);
    registry.register(&GREEN_TERRACOTTA);
    registry.register(&RED_TERRACOTTA);
    registry.register(&BLACK_TERRACOTTA);
    registry.register(&WHITE_STAINED_GLASS_PANE);
    registry.register(&ORANGE_STAINED_GLASS_PANE);
    registry.register(&MAGENTA_STAINED_GLASS_PANE);
    registry.register(&LIGHT_BLUE_STAINED_GLASS_PANE);
    registry.register(&YELLOW_STAINED_GLASS_PANE);
    registry.register(&LIME_STAINED_GLASS_PANE);
    registry.register(&PINK_STAINED_GLASS_PANE);
    registry.register(&GRAY_STAINED_GLASS_PANE);
    registry.register(&LIGHT_GRAY_STAINED_GLASS_PANE);
    registry.register(&CYAN_STAINED_GLASS_PANE);
    registry.register(&PURPLE_STAINED_GLASS_PANE);
    registry.register(&BLUE_STAINED_GLASS_PANE);
    registry.register(&BROWN_STAINED_GLASS_PANE);
    registry.register(&GREEN_STAINED_GLASS_PANE);
    registry.register(&RED_STAINED_GLASS_PANE);
    registry.register(&BLACK_STAINED_GLASS_PANE);
    registry.register(&ACACIA_STAIRS);
    registry.register(&CHERRY_STAIRS);
    registry.register(&DARK_OAK_STAIRS);
    registry.register(&PALE_OAK_STAIRS);
    registry.register(&MANGROVE_STAIRS);
    registry.register(&BAMBOO_STAIRS);
    registry.register(&BAMBOO_MOSAIC_STAIRS);
    registry.register(&SLIME_BLOCK);
    registry.register(&BARRIER);
    registry.register(&LIGHT);
    registry.register(&IRON_TRAPDOOR);
    registry.register(&PRISMARINE);
    registry.register(&PRISMARINE_BRICKS);
    registry.register(&DARK_PRISMARINE);
    registry.register(&PRISMARINE_STAIRS);
    registry.register(&PRISMARINE_BRICK_STAIRS);
    registry.register(&DARK_PRISMARINE_STAIRS);
    registry.register(&PRISMARINE_SLAB);
    registry.register(&PRISMARINE_BRICK_SLAB);
    registry.register(&DARK_PRISMARINE_SLAB);
    registry.register(&SEA_LANTERN);
    registry.register(&HAY_BLOCK);
    registry.register(&WHITE_CARPET);
    registry.register(&ORANGE_CARPET);
    registry.register(&MAGENTA_CARPET);
    registry.register(&LIGHT_BLUE_CARPET);
    registry.register(&YELLOW_CARPET);
    registry.register(&LIME_CARPET);
    registry.register(&PINK_CARPET);
    registry.register(&GRAY_CARPET);
    registry.register(&LIGHT_GRAY_CARPET);
    registry.register(&CYAN_CARPET);
    registry.register(&PURPLE_CARPET);
    registry.register(&BLUE_CARPET);
    registry.register(&BROWN_CARPET);
    registry.register(&GREEN_CARPET);
    registry.register(&RED_CARPET);
    registry.register(&BLACK_CARPET);
    registry.register(&TERRACOTTA);
    registry.register(&COAL_BLOCK);
    registry.register(&PACKED_ICE);
    registry.register(&SUNFLOWER);
    registry.register(&LILAC);
    registry.register(&ROSE_BUSH);
    registry.register(&PEONY);
    registry.register(&TALL_GRASS);
    registry.register(&LARGE_FERN);
    registry.register(&WHITE_BANNER);
    registry.register(&ORANGE_BANNER);
    registry.register(&MAGENTA_BANNER);
    registry.register(&LIGHT_BLUE_BANNER);
    registry.register(&YELLOW_BANNER);
    registry.register(&LIME_BANNER);
    registry.register(&PINK_BANNER);
    registry.register(&GRAY_BANNER);
    registry.register(&LIGHT_GRAY_BANNER);
    registry.register(&CYAN_BANNER);
    registry.register(&PURPLE_BANNER);
    registry.register(&BLUE_BANNER);
    registry.register(&BROWN_BANNER);
    registry.register(&GREEN_BANNER);
    registry.register(&RED_BANNER);
    registry.register(&BLACK_BANNER);
    registry.register(&WHITE_WALL_BANNER);
    registry.register(&ORANGE_WALL_BANNER);
    registry.register(&MAGENTA_WALL_BANNER);
    registry.register(&LIGHT_BLUE_WALL_BANNER);
    registry.register(&YELLOW_WALL_BANNER);
    registry.register(&LIME_WALL_BANNER);
    registry.register(&PINK_WALL_BANNER);
    registry.register(&GRAY_WALL_BANNER);
    registry.register(&LIGHT_GRAY_WALL_BANNER);
    registry.register(&CYAN_WALL_BANNER);
    registry.register(&PURPLE_WALL_BANNER);
    registry.register(&BLUE_WALL_BANNER);
    registry.register(&BROWN_WALL_BANNER);
    registry.register(&GREEN_WALL_BANNER);
    registry.register(&RED_WALL_BANNER);
    registry.register(&BLACK_WALL_BANNER);
    registry.register(&RED_SANDSTONE);
    registry.register(&CHISELED_RED_SANDSTONE);
    registry.register(&CUT_RED_SANDSTONE);
    registry.register(&RED_SANDSTONE_STAIRS);
    registry.register(&OAK_SLAB);
    registry.register(&SPRUCE_SLAB);
    registry.register(&BIRCH_SLAB);
    registry.register(&JUNGLE_SLAB);
    registry.register(&ACACIA_SLAB);
    registry.register(&CHERRY_SLAB);
    registry.register(&DARK_OAK_SLAB);
    registry.register(&PALE_OAK_SLAB);
    registry.register(&MANGROVE_SLAB);
    registry.register(&BAMBOO_SLAB);
    registry.register(&BAMBOO_MOSAIC_SLAB);
    registry.register(&STONE_SLAB);
    registry.register(&SMOOTH_STONE_SLAB);
    registry.register(&SANDSTONE_SLAB);
    registry.register(&CUT_SANDSTONE_SLAB);
    registry.register(&PETRIFIED_OAK_SLAB);
    registry.register(&COBBLESTONE_SLAB);
    registry.register(&BRICK_SLAB);
    registry.register(&STONE_BRICK_SLAB);
    registry.register(&MUD_BRICK_SLAB);
    registry.register(&NETHER_BRICK_SLAB);
    registry.register(&QUARTZ_SLAB);
    registry.register(&RED_SANDSTONE_SLAB);
    registry.register(&CUT_RED_SANDSTONE_SLAB);
    registry.register(&PURPUR_SLAB);
    registry.register(&SMOOTH_STONE);
    registry.register(&SMOOTH_SANDSTONE);
    registry.register(&SMOOTH_QUARTZ);
    registry.register(&SMOOTH_RED_SANDSTONE);
    registry.register(&SPRUCE_FENCE_GATE);
    registry.register(&BIRCH_FENCE_GATE);
    registry.register(&JUNGLE_FENCE_GATE);
    registry.register(&ACACIA_FENCE_GATE);
    registry.register(&CHERRY_FENCE_GATE);
    registry.register(&DARK_OAK_FENCE_GATE);
    registry.register(&PALE_OAK_FENCE_GATE);
    registry.register(&MANGROVE_FENCE_GATE);
    registry.register(&BAMBOO_FENCE_GATE);
    registry.register(&SPRUCE_FENCE);
    registry.register(&BIRCH_FENCE);
    registry.register(&JUNGLE_FENCE);
    registry.register(&ACACIA_FENCE);
    registry.register(&CHERRY_FENCE);
    registry.register(&DARK_OAK_FENCE);
    registry.register(&PALE_OAK_FENCE);
    registry.register(&MANGROVE_FENCE);
    registry.register(&BAMBOO_FENCE);
    registry.register(&SPRUCE_DOOR);
    registry.register(&BIRCH_DOOR);
    registry.register(&JUNGLE_DOOR);
    registry.register(&ACACIA_DOOR);
    registry.register(&CHERRY_DOOR);
    registry.register(&DARK_OAK_DOOR);
    registry.register(&PALE_OAK_DOOR);
    registry.register(&MANGROVE_DOOR);
    registry.register(&BAMBOO_DOOR);
    registry.register(&END_ROD);
    registry.register(&CHORUS_PLANT);
    registry.register(&CHORUS_FLOWER);
    registry.register(&PURPUR_BLOCK);
    registry.register(&PURPUR_PILLAR);
    registry.register(&PURPUR_STAIRS);
    registry.register(&END_STONE_BRICKS);
    registry.register(&TORCHFLOWER_CROP);
    registry.register(&PITCHER_CROP);
    registry.register(&PITCHER_PLANT);
    registry.register(&BEETROOTS);
    registry.register(&DIRT_PATH);
    registry.register(&END_GATEWAY);
    registry.register(&REPEATING_COMMAND_BLOCK);
    registry.register(&CHAIN_COMMAND_BLOCK);
    registry.register(&FROSTED_ICE);
    registry.register(&MAGMA_BLOCK);
    registry.register(&NETHER_WART_BLOCK);
    registry.register(&RED_NETHER_BRICKS);
    registry.register(&BONE_BLOCK);
    registry.register(&STRUCTURE_VOID);
    registry.register(&OBSERVER);
    registry.register(&SHULKER_BOX);
    registry.register(&WHITE_SHULKER_BOX);
    registry.register(&ORANGE_SHULKER_BOX);
    registry.register(&MAGENTA_SHULKER_BOX);
    registry.register(&LIGHT_BLUE_SHULKER_BOX);
    registry.register(&YELLOW_SHULKER_BOX);
    registry.register(&LIME_SHULKER_BOX);
    registry.register(&PINK_SHULKER_BOX);
    registry.register(&GRAY_SHULKER_BOX);
    registry.register(&LIGHT_GRAY_SHULKER_BOX);
    registry.register(&CYAN_SHULKER_BOX);
    registry.register(&PURPLE_SHULKER_BOX);
    registry.register(&BLUE_SHULKER_BOX);
    registry.register(&BROWN_SHULKER_BOX);
    registry.register(&GREEN_SHULKER_BOX);
    registry.register(&RED_SHULKER_BOX);
    registry.register(&BLACK_SHULKER_BOX);
    registry.register(&WHITE_GLAZED_TERRACOTTA);
    registry.register(&ORANGE_GLAZED_TERRACOTTA);
    registry.register(&MAGENTA_GLAZED_TERRACOTTA);
    registry.register(&LIGHT_BLUE_GLAZED_TERRACOTTA);
    registry.register(&YELLOW_GLAZED_TERRACOTTA);
    registry.register(&LIME_GLAZED_TERRACOTTA);
    registry.register(&PINK_GLAZED_TERRACOTTA);
    registry.register(&GRAY_GLAZED_TERRACOTTA);
    registry.register(&LIGHT_GRAY_GLAZED_TERRACOTTA);
    registry.register(&CYAN_GLAZED_TERRACOTTA);
    registry.register(&PURPLE_GLAZED_TERRACOTTA);
    registry.register(&BLUE_GLAZED_TERRACOTTA);
    registry.register(&BROWN_GLAZED_TERRACOTTA);
    registry.register(&GREEN_GLAZED_TERRACOTTA);
    registry.register(&RED_GLAZED_TERRACOTTA);
    registry.register(&BLACK_GLAZED_TERRACOTTA);
    registry.register(&WHITE_CONCRETE);
    registry.register(&ORANGE_CONCRETE);
    registry.register(&MAGENTA_CONCRETE);
    registry.register(&LIGHT_BLUE_CONCRETE);
    registry.register(&YELLOW_CONCRETE);
    registry.register(&LIME_CONCRETE);
    registry.register(&PINK_CONCRETE);
    registry.register(&GRAY_CONCRETE);
    registry.register(&LIGHT_GRAY_CONCRETE);
    registry.register(&CYAN_CONCRETE);
    registry.register(&PURPLE_CONCRETE);
    registry.register(&BLUE_CONCRETE);
    registry.register(&BROWN_CONCRETE);
    registry.register(&GREEN_CONCRETE);
    registry.register(&RED_CONCRETE);
    registry.register(&BLACK_CONCRETE);
    registry.register(&WHITE_CONCRETE_POWDER);
    registry.register(&ORANGE_CONCRETE_POWDER);
    registry.register(&MAGENTA_CONCRETE_POWDER);
    registry.register(&LIGHT_BLUE_CONCRETE_POWDER);
    registry.register(&YELLOW_CONCRETE_POWDER);
    registry.register(&LIME_CONCRETE_POWDER);
    registry.register(&PINK_CONCRETE_POWDER);
    registry.register(&GRAY_CONCRETE_POWDER);
    registry.register(&LIGHT_GRAY_CONCRETE_POWDER);
    registry.register(&CYAN_CONCRETE_POWDER);
    registry.register(&PURPLE_CONCRETE_POWDER);
    registry.register(&BLUE_CONCRETE_POWDER);
    registry.register(&BROWN_CONCRETE_POWDER);
    registry.register(&GREEN_CONCRETE_POWDER);
    registry.register(&RED_CONCRETE_POWDER);
    registry.register(&BLACK_CONCRETE_POWDER);
    registry.register(&KELP);
    registry.register(&KELP_PLANT);
    registry.register(&DRIED_KELP_BLOCK);
    registry.register(&TURTLE_EGG);
    registry.register(&SNIFFER_EGG);
    registry.register(&DRIED_GHAST);
    registry.register(&DEAD_TUBE_CORAL_BLOCK);
    registry.register(&DEAD_BRAIN_CORAL_BLOCK);
    registry.register(&DEAD_BUBBLE_CORAL_BLOCK);
    registry.register(&DEAD_FIRE_CORAL_BLOCK);
    registry.register(&DEAD_HORN_CORAL_BLOCK);
    registry.register(&TUBE_CORAL_BLOCK);
    registry.register(&BRAIN_CORAL_BLOCK);
    registry.register(&BUBBLE_CORAL_BLOCK);
    registry.register(&FIRE_CORAL_BLOCK);
    registry.register(&HORN_CORAL_BLOCK);
    registry.register(&DEAD_TUBE_CORAL);
    registry.register(&DEAD_BRAIN_CORAL);
    registry.register(&DEAD_BUBBLE_CORAL);
    registry.register(&DEAD_FIRE_CORAL);
    registry.register(&DEAD_HORN_CORAL);
    registry.register(&TUBE_CORAL);
    registry.register(&BRAIN_CORAL);
    registry.register(&BUBBLE_CORAL);
    registry.register(&FIRE_CORAL);
    registry.register(&HORN_CORAL);
    registry.register(&DEAD_TUBE_CORAL_FAN);
    registry.register(&DEAD_BRAIN_CORAL_FAN);
    registry.register(&DEAD_BUBBLE_CORAL_FAN);
    registry.register(&DEAD_FIRE_CORAL_FAN);
    registry.register(&DEAD_HORN_CORAL_FAN);
    registry.register(&TUBE_CORAL_FAN);
    registry.register(&BRAIN_CORAL_FAN);
    registry.register(&BUBBLE_CORAL_FAN);
    registry.register(&FIRE_CORAL_FAN);
    registry.register(&HORN_CORAL_FAN);
    registry.register(&DEAD_TUBE_CORAL_WALL_FAN);
    registry.register(&DEAD_BRAIN_CORAL_WALL_FAN);
    registry.register(&DEAD_BUBBLE_CORAL_WALL_FAN);
    registry.register(&DEAD_FIRE_CORAL_WALL_FAN);
    registry.register(&DEAD_HORN_CORAL_WALL_FAN);
    registry.register(&TUBE_CORAL_WALL_FAN);
    registry.register(&BRAIN_CORAL_WALL_FAN);
    registry.register(&BUBBLE_CORAL_WALL_FAN);
    registry.register(&FIRE_CORAL_WALL_FAN);
    registry.register(&HORN_CORAL_WALL_FAN);
    registry.register(&SEA_PICKLE);
    registry.register(&BLUE_ICE);
    registry.register(&CONDUIT);
    registry.register(&BAMBOO_SAPLING);
    registry.register(&BAMBOO);
    registry.register(&POTTED_BAMBOO);
    registry.register(&VOID_AIR);
    registry.register(&CAVE_AIR);
    registry.register(&BUBBLE_COLUMN);
    registry.register(&POLISHED_GRANITE_STAIRS);
    registry.register(&SMOOTH_RED_SANDSTONE_STAIRS);
    registry.register(&MOSSY_STONE_BRICK_STAIRS);
    registry.register(&POLISHED_DIORITE_STAIRS);
    registry.register(&MOSSY_COBBLESTONE_STAIRS);
    registry.register(&END_STONE_BRICK_STAIRS);
    registry.register(&STONE_STAIRS);
    registry.register(&SMOOTH_SANDSTONE_STAIRS);
    registry.register(&SMOOTH_QUARTZ_STAIRS);
    registry.register(&GRANITE_STAIRS);
    registry.register(&ANDESITE_STAIRS);
    registry.register(&RED_NETHER_BRICK_STAIRS);
    registry.register(&POLISHED_ANDESITE_STAIRS);
    registry.register(&DIORITE_STAIRS);
    registry.register(&POLISHED_GRANITE_SLAB);
    registry.register(&SMOOTH_RED_SANDSTONE_SLAB);
    registry.register(&MOSSY_STONE_BRICK_SLAB);
    registry.register(&POLISHED_DIORITE_SLAB);
    registry.register(&MOSSY_COBBLESTONE_SLAB);
    registry.register(&END_STONE_BRICK_SLAB);
    registry.register(&SMOOTH_SANDSTONE_SLAB);
    registry.register(&SMOOTH_QUARTZ_SLAB);
    registry.register(&GRANITE_SLAB);
    registry.register(&ANDESITE_SLAB);
    registry.register(&RED_NETHER_BRICK_SLAB);
    registry.register(&POLISHED_ANDESITE_SLAB);
    registry.register(&DIORITE_SLAB);
    registry.register(&BRICK_WALL);
    registry.register(&PRISMARINE_WALL);
    registry.register(&RED_SANDSTONE_WALL);
    registry.register(&MOSSY_STONE_BRICK_WALL);
    registry.register(&GRANITE_WALL);
    registry.register(&STONE_BRICK_WALL);
    registry.register(&MUD_BRICK_WALL);
    registry.register(&NETHER_BRICK_WALL);
    registry.register(&ANDESITE_WALL);
    registry.register(&RED_NETHER_BRICK_WALL);
    registry.register(&SANDSTONE_WALL);
    registry.register(&END_STONE_BRICK_WALL);
    registry.register(&DIORITE_WALL);
    registry.register(&SCAFFOLDING);
    registry.register(&LOOM);
    registry.register(&BARREL);
    registry.register(&SMOKER);
    registry.register(&BLAST_FURNACE);
    registry.register(&CARTOGRAPHY_TABLE);
    registry.register(&FLETCHING_TABLE);
    registry.register(&GRINDSTONE);
    registry.register(&LECTERN);
    registry.register(&SMITHING_TABLE);
    registry.register(&STONECUTTER);
    registry.register(&BELL);
    registry.register(&LANTERN);
    registry.register(&SOUL_LANTERN);
    registry.register(&COPPER_LANTERN);
    registry.register(&EXPOSED_COPPER_LANTERN);
    registry.register(&WEATHERED_COPPER_LANTERN);
    registry.register(&OXIDIZED_COPPER_LANTERN);
    registry.register(&WAXED_COPPER_LANTERN);
    registry.register(&WAXED_EXPOSED_COPPER_LANTERN);
    registry.register(&WAXED_WEATHERED_COPPER_LANTERN);
    registry.register(&WAXED_OXIDIZED_COPPER_LANTERN);
    registry.register(&CAMPFIRE);
    registry.register(&SOUL_CAMPFIRE);
    registry.register(&SWEET_BERRY_BUSH);
    registry.register(&WARPED_STEM);
    registry.register(&STRIPPED_WARPED_STEM);
    registry.register(&WARPED_HYPHAE);
    registry.register(&STRIPPED_WARPED_HYPHAE);
    registry.register(&WARPED_NYLIUM);
    registry.register(&WARPED_FUNGUS);
    registry.register(&WARPED_WART_BLOCK);
    registry.register(&WARPED_ROOTS);
    registry.register(&NETHER_SPROUTS);
    registry.register(&CRIMSON_STEM);
    registry.register(&STRIPPED_CRIMSON_STEM);
    registry.register(&CRIMSON_HYPHAE);
    registry.register(&STRIPPED_CRIMSON_HYPHAE);
    registry.register(&CRIMSON_NYLIUM);
    registry.register(&CRIMSON_FUNGUS);
    registry.register(&SHROOMLIGHT);
    registry.register(&WEEPING_VINES);
    registry.register(&WEEPING_VINES_PLANT);
    registry.register(&TWISTING_VINES);
    registry.register(&TWISTING_VINES_PLANT);
    registry.register(&CRIMSON_ROOTS);
    registry.register(&CRIMSON_PLANKS);
    registry.register(&WARPED_PLANKS);
    registry.register(&CRIMSON_SLAB);
    registry.register(&WARPED_SLAB);
    registry.register(&CRIMSON_PRESSURE_PLATE);
    registry.register(&WARPED_PRESSURE_PLATE);
    registry.register(&CRIMSON_FENCE);
    registry.register(&WARPED_FENCE);
    registry.register(&CRIMSON_TRAPDOOR);
    registry.register(&WARPED_TRAPDOOR);
    registry.register(&CRIMSON_FENCE_GATE);
    registry.register(&WARPED_FENCE_GATE);
    registry.register(&CRIMSON_STAIRS);
    registry.register(&WARPED_STAIRS);
    registry.register(&CRIMSON_BUTTON);
    registry.register(&WARPED_BUTTON);
    registry.register(&CRIMSON_DOOR);
    registry.register(&WARPED_DOOR);
    registry.register(&CRIMSON_SIGN);
    registry.register(&WARPED_SIGN);
    registry.register(&CRIMSON_WALL_SIGN);
    registry.register(&WARPED_WALL_SIGN);
    registry.register(&STRUCTURE_BLOCK);
    registry.register(&JIGSAW);
    registry.register(&TEST_BLOCK);
    registry.register(&TEST_INSTANCE_BLOCK);
    registry.register(&COMPOSTER);
    registry.register(&TARGET);
    registry.register(&BEE_NEST);
    registry.register(&BEEHIVE);
    registry.register(&HONEY_BLOCK);
    registry.register(&HONEYCOMB_BLOCK);
    registry.register(&NETHERITE_BLOCK);
    registry.register(&ANCIENT_DEBRIS);
    registry.register(&CRYING_OBSIDIAN);
    registry.register(&RESPAWN_ANCHOR);
    registry.register(&POTTED_CRIMSON_FUNGUS);
    registry.register(&POTTED_WARPED_FUNGUS);
    registry.register(&POTTED_CRIMSON_ROOTS);
    registry.register(&POTTED_WARPED_ROOTS);
    registry.register(&LODESTONE);
    registry.register(&BLACKSTONE);
    registry.register(&BLACKSTONE_STAIRS);
    registry.register(&BLACKSTONE_WALL);
    registry.register(&BLACKSTONE_SLAB);
    registry.register(&POLISHED_BLACKSTONE);
    registry.register(&POLISHED_BLACKSTONE_BRICKS);
    registry.register(&CRACKED_POLISHED_BLACKSTONE_BRICKS);
    registry.register(&CHISELED_POLISHED_BLACKSTONE);
    registry.register(&POLISHED_BLACKSTONE_BRICK_SLAB);
    registry.register(&POLISHED_BLACKSTONE_BRICK_STAIRS);
    registry.register(&POLISHED_BLACKSTONE_BRICK_WALL);
    registry.register(&GILDED_BLACKSTONE);
    registry.register(&POLISHED_BLACKSTONE_STAIRS);
    registry.register(&POLISHED_BLACKSTONE_SLAB);
    registry.register(&POLISHED_BLACKSTONE_PRESSURE_PLATE);
    registry.register(&POLISHED_BLACKSTONE_BUTTON);
    registry.register(&POLISHED_BLACKSTONE_WALL);
    registry.register(&CHISELED_NETHER_BRICKS);
    registry.register(&CRACKED_NETHER_BRICKS);
    registry.register(&QUARTZ_BRICKS);
    registry.register(&CANDLE);
    registry.register(&WHITE_CANDLE);
    registry.register(&ORANGE_CANDLE);
    registry.register(&MAGENTA_CANDLE);
    registry.register(&LIGHT_BLUE_CANDLE);
    registry.register(&YELLOW_CANDLE);
    registry.register(&LIME_CANDLE);
    registry.register(&PINK_CANDLE);
    registry.register(&GRAY_CANDLE);
    registry.register(&LIGHT_GRAY_CANDLE);
    registry.register(&CYAN_CANDLE);
    registry.register(&PURPLE_CANDLE);
    registry.register(&BLUE_CANDLE);
    registry.register(&BROWN_CANDLE);
    registry.register(&GREEN_CANDLE);
    registry.register(&RED_CANDLE);
    registry.register(&BLACK_CANDLE);
    registry.register(&CANDLE_CAKE);
    registry.register(&WHITE_CANDLE_CAKE);
    registry.register(&ORANGE_CANDLE_CAKE);
    registry.register(&MAGENTA_CANDLE_CAKE);
    registry.register(&LIGHT_BLUE_CANDLE_CAKE);
    registry.register(&YELLOW_CANDLE_CAKE);
    registry.register(&LIME_CANDLE_CAKE);
    registry.register(&PINK_CANDLE_CAKE);
    registry.register(&GRAY_CANDLE_CAKE);
    registry.register(&LIGHT_GRAY_CANDLE_CAKE);
    registry.register(&CYAN_CANDLE_CAKE);
    registry.register(&PURPLE_CANDLE_CAKE);
    registry.register(&BLUE_CANDLE_CAKE);
    registry.register(&BROWN_CANDLE_CAKE);
    registry.register(&GREEN_CANDLE_CAKE);
    registry.register(&RED_CANDLE_CAKE);
    registry.register(&BLACK_CANDLE_CAKE);
    registry.register(&AMETHYST_BLOCK);
    registry.register(&BUDDING_AMETHYST);
    registry.register(&AMETHYST_CLUSTER);
    registry.register(&LARGE_AMETHYST_BUD);
    registry.register(&MEDIUM_AMETHYST_BUD);
    registry.register(&SMALL_AMETHYST_BUD);
    registry.register(&TUFF);
    registry.register(&TUFF_SLAB);
    registry.register(&TUFF_STAIRS);
    registry.register(&TUFF_WALL);
    registry.register(&POLISHED_TUFF);
    registry.register(&POLISHED_TUFF_SLAB);
    registry.register(&POLISHED_TUFF_STAIRS);
    registry.register(&POLISHED_TUFF_WALL);
    registry.register(&CHISELED_TUFF);
    registry.register(&TUFF_BRICKS);
    registry.register(&TUFF_BRICK_SLAB);
    registry.register(&TUFF_BRICK_STAIRS);
    registry.register(&TUFF_BRICK_WALL);
    registry.register(&CHISELED_TUFF_BRICKS);
    registry.register(&CALCITE);
    registry.register(&TINTED_GLASS);
    registry.register(&POWDER_SNOW);
    registry.register(&SCULK_SENSOR);
    registry.register(&CALIBRATED_SCULK_SENSOR);
    registry.register(&SCULK);
    registry.register(&SCULK_VEIN);
    registry.register(&SCULK_CATALYST);
    registry.register(&SCULK_SHRIEKER);
    registry.register(&COPPER_BLOCK);
    registry.register(&EXPOSED_COPPER);
    registry.register(&WEATHERED_COPPER);
    registry.register(&OXIDIZED_COPPER);
    registry.register(&COPPER_ORE);
    registry.register(&DEEPSLATE_COPPER_ORE);
    registry.register(&OXIDIZED_CUT_COPPER);
    registry.register(&WEATHERED_CUT_COPPER);
    registry.register(&EXPOSED_CUT_COPPER);
    registry.register(&CUT_COPPER);
    registry.register(&OXIDIZED_CHISELED_COPPER);
    registry.register(&WEATHERED_CHISELED_COPPER);
    registry.register(&EXPOSED_CHISELED_COPPER);
    registry.register(&CHISELED_COPPER);
    registry.register(&WAXED_OXIDIZED_CHISELED_COPPER);
    registry.register(&WAXED_WEATHERED_CHISELED_COPPER);
    registry.register(&WAXED_EXPOSED_CHISELED_COPPER);
    registry.register(&WAXED_CHISELED_COPPER);
    registry.register(&OXIDIZED_CUT_COPPER_STAIRS);
    registry.register(&WEATHERED_CUT_COPPER_STAIRS);
    registry.register(&EXPOSED_CUT_COPPER_STAIRS);
    registry.register(&CUT_COPPER_STAIRS);
    registry.register(&OXIDIZED_CUT_COPPER_SLAB);
    registry.register(&WEATHERED_CUT_COPPER_SLAB);
    registry.register(&EXPOSED_CUT_COPPER_SLAB);
    registry.register(&CUT_COPPER_SLAB);
    registry.register(&WAXED_COPPER_BLOCK);
    registry.register(&WAXED_WEATHERED_COPPER);
    registry.register(&WAXED_EXPOSED_COPPER);
    registry.register(&WAXED_OXIDIZED_COPPER);
    registry.register(&WAXED_OXIDIZED_CUT_COPPER);
    registry.register(&WAXED_WEATHERED_CUT_COPPER);
    registry.register(&WAXED_EXPOSED_CUT_COPPER);
    registry.register(&WAXED_CUT_COPPER);
    registry.register(&WAXED_OXIDIZED_CUT_COPPER_STAIRS);
    registry.register(&WAXED_WEATHERED_CUT_COPPER_STAIRS);
    registry.register(&WAXED_EXPOSED_CUT_COPPER_STAIRS);
    registry.register(&WAXED_CUT_COPPER_STAIRS);
    registry.register(&WAXED_OXIDIZED_CUT_COPPER_SLAB);
    registry.register(&WAXED_WEATHERED_CUT_COPPER_SLAB);
    registry.register(&WAXED_EXPOSED_CUT_COPPER_SLAB);
    registry.register(&WAXED_CUT_COPPER_SLAB);
    registry.register(&COPPER_DOOR);
    registry.register(&EXPOSED_COPPER_DOOR);
    registry.register(&OXIDIZED_COPPER_DOOR);
    registry.register(&WEATHERED_COPPER_DOOR);
    registry.register(&WAXED_COPPER_DOOR);
    registry.register(&WAXED_EXPOSED_COPPER_DOOR);
    registry.register(&WAXED_OXIDIZED_COPPER_DOOR);
    registry.register(&WAXED_WEATHERED_COPPER_DOOR);
    registry.register(&COPPER_TRAPDOOR);
    registry.register(&EXPOSED_COPPER_TRAPDOOR);
    registry.register(&OXIDIZED_COPPER_TRAPDOOR);
    registry.register(&WEATHERED_COPPER_TRAPDOOR);
    registry.register(&WAXED_COPPER_TRAPDOOR);
    registry.register(&WAXED_EXPOSED_COPPER_TRAPDOOR);
    registry.register(&WAXED_OXIDIZED_COPPER_TRAPDOOR);
    registry.register(&WAXED_WEATHERED_COPPER_TRAPDOOR);
    registry.register(&COPPER_GRATE);
    registry.register(&EXPOSED_COPPER_GRATE);
    registry.register(&WEATHERED_COPPER_GRATE);
    registry.register(&OXIDIZED_COPPER_GRATE);
    registry.register(&WAXED_COPPER_GRATE);
    registry.register(&WAXED_EXPOSED_COPPER_GRATE);
    registry.register(&WAXED_WEATHERED_COPPER_GRATE);
    registry.register(&WAXED_OXIDIZED_COPPER_GRATE);
    registry.register(&COPPER_BULB);
    registry.register(&EXPOSED_COPPER_BULB);
    registry.register(&WEATHERED_COPPER_BULB);
    registry.register(&OXIDIZED_COPPER_BULB);
    registry.register(&WAXED_COPPER_BULB);
    registry.register(&WAXED_EXPOSED_COPPER_BULB);
    registry.register(&WAXED_WEATHERED_COPPER_BULB);
    registry.register(&WAXED_OXIDIZED_COPPER_BULB);
    registry.register(&COPPER_CHEST);
    registry.register(&EXPOSED_COPPER_CHEST);
    registry.register(&WEATHERED_COPPER_CHEST);
    registry.register(&OXIDIZED_COPPER_CHEST);
    registry.register(&WAXED_COPPER_CHEST);
    registry.register(&WAXED_EXPOSED_COPPER_CHEST);
    registry.register(&WAXED_WEATHERED_COPPER_CHEST);
    registry.register(&WAXED_OXIDIZED_COPPER_CHEST);
    registry.register(&COPPER_GOLEM_STATUE);
    registry.register(&EXPOSED_COPPER_GOLEM_STATUE);
    registry.register(&WEATHERED_COPPER_GOLEM_STATUE);
    registry.register(&OXIDIZED_COPPER_GOLEM_STATUE);
    registry.register(&WAXED_COPPER_GOLEM_STATUE);
    registry.register(&WAXED_EXPOSED_COPPER_GOLEM_STATUE);
    registry.register(&WAXED_WEATHERED_COPPER_GOLEM_STATUE);
    registry.register(&WAXED_OXIDIZED_COPPER_GOLEM_STATUE);
    registry.register(&LIGHTNING_ROD);
    registry.register(&EXPOSED_LIGHTNING_ROD);
    registry.register(&WEATHERED_LIGHTNING_ROD);
    registry.register(&OXIDIZED_LIGHTNING_ROD);
    registry.register(&WAXED_LIGHTNING_ROD);
    registry.register(&WAXED_EXPOSED_LIGHTNING_ROD);
    registry.register(&WAXED_WEATHERED_LIGHTNING_ROD);
    registry.register(&WAXED_OXIDIZED_LIGHTNING_ROD);
    registry.register(&POINTED_DRIPSTONE);
    registry.register(&DRIPSTONE_BLOCK);
    registry.register(&CAVE_VINES);
    registry.register(&CAVE_VINES_PLANT);
    registry.register(&SPORE_BLOSSOM);
    registry.register(&AZALEA);
    registry.register(&FLOWERING_AZALEA);
    registry.register(&MOSS_CARPET);
    registry.register(&PINK_PETALS);
    registry.register(&WILDFLOWERS);
    registry.register(&LEAF_LITTER);
    registry.register(&MOSS_BLOCK);
    registry.register(&BIG_DRIPLEAF);
    registry.register(&BIG_DRIPLEAF_STEM);
    registry.register(&SMALL_DRIPLEAF);
    registry.register(&HANGING_ROOTS);
    registry.register(&ROOTED_DIRT);
    registry.register(&MUD);
    registry.register(&DEEPSLATE);
    registry.register(&COBBLED_DEEPSLATE);
    registry.register(&COBBLED_DEEPSLATE_STAIRS);
    registry.register(&COBBLED_DEEPSLATE_SLAB);
    registry.register(&COBBLED_DEEPSLATE_WALL);
    registry.register(&POLISHED_DEEPSLATE);
    registry.register(&POLISHED_DEEPSLATE_STAIRS);
    registry.register(&POLISHED_DEEPSLATE_SLAB);
    registry.register(&POLISHED_DEEPSLATE_WALL);
    registry.register(&DEEPSLATE_TILES);
    registry.register(&DEEPSLATE_TILE_STAIRS);
    registry.register(&DEEPSLATE_TILE_SLAB);
    registry.register(&DEEPSLATE_TILE_WALL);
    registry.register(&DEEPSLATE_BRICKS);
    registry.register(&DEEPSLATE_BRICK_STAIRS);
    registry.register(&DEEPSLATE_BRICK_SLAB);
    registry.register(&DEEPSLATE_BRICK_WALL);
    registry.register(&CHISELED_DEEPSLATE);
    registry.register(&CRACKED_DEEPSLATE_BRICKS);
    registry.register(&CRACKED_DEEPSLATE_TILES);
    registry.register(&INFESTED_DEEPSLATE);
    registry.register(&SMOOTH_BASALT);
    registry.register(&RAW_IRON_BLOCK);
    registry.register(&RAW_COPPER_BLOCK);
    registry.register(&RAW_GOLD_BLOCK);
    registry.register(&POTTED_AZALEA_BUSH);
    registry.register(&POTTED_FLOWERING_AZALEA_BUSH);
    registry.register(&OCHRE_FROGLIGHT);
    registry.register(&VERDANT_FROGLIGHT);
    registry.register(&PEARLESCENT_FROGLIGHT);
    registry.register(&FROGSPAWN);
    registry.register(&REINFORCED_DEEPSLATE);
    registry.register(&DECORATED_POT);
    registry.register(&CRAFTER);
    registry.register(&TRIAL_SPAWNER);
    registry.register(&VAULT);
    registry.register(&HEAVY_CORE);
    registry.register(&PALE_MOSS_BLOCK);
    registry.register(&PALE_MOSS_CARPET);
    registry.register(&PALE_HANGING_MOSS);
    registry.register(&OPEN_EYEBLOSSOM);
    registry.register(&CLOSED_EYEBLOSSOM);
    registry.register(&POTTED_OPEN_EYEBLOSSOM);
    registry.register(&POTTED_CLOSED_EYEBLOSSOM);
    registry.register(&FIREFLY_BUSH);
}
