use std::sync::Arc;
use std::time::Duration;

use glam::DVec3;
use steel_registry::game_rules::GameRuleValue;
use steel_registry::vanilla_entities;
use steel_registry::vanilla_game_rules::RESPAWN_RADIUS;
use steel_utils::{BlockPos, ChunkPos, SectionPos, WorldAabb, types::GameType};
use tokio::time::sleep;

use crate::behavior::BlockCollisionContext;
use crate::chunk::chunk_access::ChunkStatus;
use crate::chunk::chunk_request::{ChunkRequestHandle, ChunkRequestState, ChunkTicketKind};
use crate::fluid::get_fluid_state;
use crate::physics::{CollisionWorld as _, WorldCollisionProvider};
use crate::world::World;

const ABSOLUTE_MAX_ATTEMPTS: i32 = 1024;
const PLAYER_SPAWN_CHUNK_RADIUS: u8 = 3;
const CHUNK_REQUEST_POLL_DELAY: Duration = Duration::from_millis(10);

impl World {
    /// Finds the adjusted shared spawn position used for players entering this world's default spawn.
    pub async fn find_adjusted_shared_spawn_pos(
        self: &Arc<Self>,
        spawn_suggestion: BlockPos,
        game_type: GameType,
    ) -> Result<DVec3, String> {
        if game_type == GameType::Adventure {
            let _chunk_request = self
                .load_spawn_candidate_chunk(spawn_suggestion.x(), spawn_suggestion.z())
                .await?;
            return Ok(self.fixup_spawn_height(spawn_suggestion));
        }

        let mut radius = match self.get_game_rule(&RESPAWN_RADIUS) {
            GameRuleValue::Int(radius) => radius.max(0),
            value @ GameRuleValue::Bool(_) => {
                return Err(format!(
                    "gamerule {} should be an integer, got {value:?}",
                    RESPAWN_RADIUS.key
                ));
            }
        };
        let border_distance = self
            .world_border_snapshot()
            .distance_to_border(
                f64::from(spawn_suggestion.x()),
                f64::from(spawn_suggestion.z()),
            )
            .floor() as i32;
        if border_distance < radius {
            radius = border_distance;
        }
        if border_distance <= 1 {
            radius = 1;
        }

        let square_side = i64::from(radius) * 2 + 1;
        let candidate_count =
            i32::try_from(i64::from(ABSOLUTE_MAX_ATTEMPTS).min(square_side * square_side))
                .map_err(|e| format!("invalid spawn candidate count: {e}"))?;
        let coprime = get_coprime(candidate_count);
        let offset = rand::random_range(0..candidate_count);

        for candidate_index in 0..candidate_count {
            let value = (offset + coprime * candidate_index).rem_euclid(candidate_count);
            let delta_x = value % (radius * 2 + 1);
            let delta_z = value / (radius * 2 + 1);
            let target_x = spawn_suggestion.x() + delta_x - radius;
            let target_z = spawn_suggestion.z() + delta_z - radius;

            let _chunk_request = self.load_spawn_candidate_chunk(target_x, target_z).await?;
            let Some(spawn_pos) = self.level_respawn_pos(target_x, target_z) else {
                continue;
            };
            if self.no_collision_no_liquid(spawn_pos) {
                return Ok(block_bottom_center(spawn_pos));
            }
        }

        let _chunk_request = self
            .load_spawn_candidate_chunk(spawn_suggestion.x(), spawn_suggestion.z())
            .await?;
        Ok(self.fixup_spawn_height(spawn_suggestion))
    }

    /// Loads the vanilla radius-3 full chunk square around a prepared player spawn.
    pub async fn prepare_player_spawn_chunks(
        self: &Arc<Self>,
        spawn_position: DVec3,
    ) -> Result<ChunkRequestHandle, String> {
        let spawn_pos = BlockPos::containing(spawn_position.x, spawn_position.y, spawn_position.z);
        let center = ChunkPos::new(
            SectionPos::block_to_section_coord(spawn_pos.x()),
            SectionPos::block_to_section_coord(spawn_pos.z()),
        );
        let request = self.chunk_map.request_square(
            center,
            PLAYER_SPAWN_CHUNK_RADIUS,
            ChunkStatus::Full,
            ChunkTicketKind::PlayerSpawn,
        );
        Self::wait_for_chunk_request(&request).await?;
        Ok(request)
    }

    async fn load_spawn_candidate_chunk(
        self: &Arc<Self>,
        x: i32,
        z: i32,
    ) -> Result<ChunkRequestHandle, String> {
        let chunk = ChunkPos::new(
            SectionPos::block_to_section_coord(x),
            SectionPos::block_to_section_coord(z),
        );
        let request =
            self.chunk_map
                .request_chunk(chunk, ChunkStatus::Full, ChunkTicketKind::SpawnSearch);
        Self::wait_for_chunk_request(&request).await?;
        Ok(request)
    }

    async fn wait_for_chunk_request(request: &ChunkRequestHandle) -> Result<(), String> {
        loop {
            match request.poll() {
                ChunkRequestState::Ready => return Ok(()),
                ChunkRequestState::Cancelled => {
                    return Err("chunk request was cancelled".to_owned());
                }
                ChunkRequestState::Pending { .. } => {
                    sleep(CHUNK_REQUEST_POLL_DELAY).await;
                }
            }
        }
    }

    fn fixup_spawn_height(self: &Arc<Self>, spawn_pos: BlockPos) -> DVec3 {
        let mut pos = spawn_pos;

        while !self.no_collision_no_liquid(pos) && pos.y() < self.get_max_y() {
            pos = pos.above();
        }

        pos = pos.below();

        while self.no_collision_no_liquid(pos) && pos.y() > self.get_min_y() {
            pos = pos.below();
        }

        block_bottom_center(pos.above())
    }

    fn no_collision_no_liquid(self: &Arc<Self>, pos: BlockPos) -> bool {
        let dimensions = vanilla_entities::PLAYER.dimensions;
        let aabb = WorldAabb::entity_box(
            f64::from(pos.x()) + 0.5,
            f64::from(pos.y()),
            f64::from(pos.z()) + 0.5,
            f64::from(dimensions.half_width()),
            f64::from(dimensions.height),
        );
        let collision_world = WorldCollisionProvider::new(self);

        !collision_world.has_entity_collision(&aabb)
            && !collision_world
                .has_block_collision_with_context(&aabb, BlockCollisionContext::empty())
            && !aabb_contains_any_liquid(self, aabb)
    }
}

fn aabb_contains_any_liquid(world: &Arc<World>, aabb: WorldAabb) -> bool {
    let min_x = aabb.min_x().floor() as i32;
    let max_x = aabb.max_x().ceil() as i32;
    let min_y = aabb.min_y().floor() as i32;
    let max_y = aabb.max_y().ceil() as i32;
    let min_z = aabb.min_z().floor() as i32;
    let max_z = aabb.max_z().ceil() as i32;

    for x in min_x..max_x {
        for y in min_y..max_y {
            for z in min_z..max_z {
                if !get_fluid_state(world, BlockPos::new(x, y, z)).is_empty() {
                    return true;
                }
            }
        }
    }

    false
}

const fn get_coprime(possible_origins: i32) -> i32 {
    if possible_origins <= 16 {
        possible_origins - 1
    } else {
        17
    }
}

fn block_bottom_center(pos: BlockPos) -> DVec3 {
    let (x, y, z) = pos.get_bottom_center();
    DVec3::new(x, y, z)
}
