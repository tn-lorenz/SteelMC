use steel_macros::{CBoundPacket, PacketWrite};
use steel_registry::packets::clientbound::play::CLIENTBOUND_LOGIN;
use steel_utils::{BlockPos, ResourceLocation, types::GameType};

use crate::packet_traits::WriteTo;

#[derive(Clone, Debug, PacketWrite)]
pub struct CommonPlayerSpawnInfo {
    #[write_as(as = "var_int")]
    pub dimension_type: i32,
    pub dimension: ResourceLocation,
    pub seed: i64,
    #[write_as(as = "byte")]
    pub game_type: GameType,
    #[write_as(as = "option_byte")]
    pub previous_game_type: Option<GameType>,
    pub is_debug: bool,
    pub is_flat: bool,
    pub last_death_location: Option<(ResourceLocation, BlockPos)>,
    #[write_as(as = "var_int")]
    pub portal_cooldown: i32,
    #[write_as(as = "var_int")]
    pub sea_level: i32,
}

impl WriteTo for BlockPos {
    fn write(&self, writer: &mut impl std::io::Write) -> Result<(), std::io::Error> {
        self.as_i64().write(writer)?;
        Ok(())
    }
}

impl WriteTo for (ResourceLocation, BlockPos) {
    fn write(&self, writer: &mut impl std::io::Write) -> Result<(), std::io::Error> {
        self.0.write(writer)?;
        self.1.write(writer)?;
        Ok(())
    }
}

#[derive(PacketWrite, CBoundPacket, Clone, Debug)]
#[packet_id(PLAY = "CLIENTBOUND_LOGIN")]
pub struct CLoginPacket {
    pub player_id: i32,
    pub hardcore: bool,
    #[write_as(as = "vec")]
    pub levels: Vec<ResourceLocation>,
    #[write_as(as = "var_int")]
    pub max_players: i32,
    #[write_as(as = "var_int")]
    pub chunk_radius: i32,
    #[write_as(as = "var_int")]
    pub simulation_distance: i32,
    pub reduced_debug_info: bool,
    pub show_death_screen: bool,
    pub do_limited_crafting: bool,
    pub common_player_spawn_info: CommonPlayerSpawnInfo,
    pub enforces_secure_chat: bool,
}

impl CommonPlayerSpawnInfo {
    pub fn new(
        dimension_type: i32,
        dimension: ResourceLocation,
        seed: i64,
        game_type: GameType,
        previous_game_type: Option<GameType>,
        is_debug: bool,
        is_flat: bool,
        last_death_location: Option<(ResourceLocation, BlockPos)>,
        portal_cooldown: i32,
        sea_level: i32,
    ) -> Self {
        Self {
            dimension_type,
            dimension,
            seed,
            game_type,
            previous_game_type,
            is_debug,
            is_flat,
            last_death_location,
            portal_cooldown,
            sea_level,
        }
    }
}

impl CLoginPacket {
    pub fn new(
        player_id: i32,
        hardcore: bool,
        levels: Vec<ResourceLocation>,
        max_players: i32,
        chunk_radius: i32,
        simulation_distance: i32,
        reduced_debug_info: bool,
        show_death_screen: bool,
        do_limited_crafting: bool,
        common_player_spawn_info: CommonPlayerSpawnInfo,
        enforces_secure_chat: bool,
    ) -> Self {
        Self {
            player_id,
            hardcore,
            levels,
            max_players,
            chunk_radius,
            simulation_distance,
            reduced_debug_info,
            show_death_screen,
            do_limited_crafting,
            common_player_spawn_info,
            enforces_secure_chat,
        }
    }
}
