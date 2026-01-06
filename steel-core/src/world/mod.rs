//! This module contains the `World` struct, which represents a world.
use std::sync::{Arc, Weak};
use std::time::Duration;

use scc::HashMap;
use steel_protocol::packets::game::{CPlayerChat, CSystemChat};
use steel_registry::{
    BlockStateExt, REGISTRY, compat_traits::RegistryWorld, dimension_type::DimensionTypeRef,
};
use steel_utils::{BlockPos, BlockStateId, ChunkPos, SectionPos, types::UpdateFlags};
use tokio::runtime::Runtime;
use uuid::Uuid;

use crate::{
    ChunkMap,
    chunk::chunk_access::ChunkAccess,
    player::{LastSeen, Player},
};

mod world_entities;

/// A struct that represents a world.
pub struct World {
    /// The chunk map of the world.
    pub chunk_map: Arc<ChunkMap>,
    /// A map of all the players in the world.
    pub players: HashMap<Uuid, Arc<Player>>,
    /// The dimension of the world.
    pub dimension: DimensionTypeRef,
}

impl World {
    /// Creates a new world.
    ///
    /// Uses `Arc::new_cyclic` to create a cyclic reference between
    /// the World and its `ChunkMap`'s `WorldGenContext`.
    #[allow(clippy::new_without_default)]
    #[must_use]
    pub fn new(chunk_runtime: Arc<Runtime>, dimension: DimensionTypeRef) -> Arc<Self> {
        Arc::new_cyclic(|weak_self: &Weak<World>| Self {
            chunk_map: Arc::new(ChunkMap::new(chunk_runtime, weak_self.clone(), &dimension)),
            players: HashMap::new(),
            dimension,
        })
    }

    /// Returns the total height of the world in blocks.
    pub fn get_height(&self) -> i32 {
        self.dimension.height
    }

    /// Returns the minimum Y coordinate of the world.
    pub fn get_min_y(&self) -> i32 {
        self.dimension.min_y
    }

    /// Returns the maximum Y coordinate of the world.
    pub fn get_max_y(&self) -> i32 {
        self.get_min_y() + self.get_height() - 1
    }

    /// Returns whether the given Y coordinate is outside the build height.
    pub fn is_outside_build_height(&self, block_y: i32) -> bool {
        block_y < self.get_min_y() || block_y > self.get_max_y()
    }

    /// Returns whether the block position is within valid horizontal bounds.
    pub fn is_in_valid_bounds_horizontal(&self, block_pos: &BlockPos) -> bool {
        let chunk_x = SectionPos::block_to_section_coord(block_pos.0.x);
        let chunk_z = SectionPos::block_to_section_coord(block_pos.0.z);
        ChunkPos::is_valid(chunk_x, chunk_z)
    }

    /// Returns whether the block position is within valid world bounds.
    pub fn is_in_valid_bounds(&self, block_pos: &BlockPos) -> bool {
        !self.is_outside_build_height(block_pos.0.y)
            && self.is_in_valid_bounds_horizontal(block_pos)
    }

    /// Sets a block at the given position.
    ///
    /// Returns `true` if the block was successfully set, `false` otherwise.
    pub fn set_block(&self, pos: BlockPos, block_state: BlockStateId, flags: UpdateFlags) -> bool {
        if !self.is_in_valid_bounds(&pos) {
            return false;
        }

        let Some(chunk) = self.get_chunk_at(&pos) else {
            return false;
        };

        let Some(old_state) = chunk.set_block_state(pos, block_state, flags) else {
            // Nothing changed
            return false;
        };

        let old_block = old_state.get_block();
        let new_block = block_state.get_block();
        let block_changed = !std::ptr::eq(old_block, new_block);
        let moved_by_piston = flags.contains(UpdateFlags::UPDATE_MOVE_BY_PISTON);

        // Call affect_neighbors_after_removal when UPDATE_NEIGHBORS is set and block changed
        if block_changed && flags.contains(UpdateFlags::UPDATE_NEIGHBORS) {
            let behavior = REGISTRY.blocks.get_behavior(old_block);
            behavior.affect_neighbors_after_removal(old_state, self, pos, moved_by_piston);
        }

        // Call on_place unless UPDATE_SKIP_ON_PLACE is set
        if !flags.contains(UpdateFlags::UPDATE_SKIP_ON_PLACE) {
            let behavior = REGISTRY.blocks.get_behavior(new_block);
            behavior.on_place(block_state, self, pos, old_state, moved_by_piston);
        }

        true
    }

    fn get_chunk_at(&self, pos: &BlockPos) -> Option<Arc<ChunkAccess>> {
        let chunk_pos = ChunkPos::new(
            SectionPos::block_to_section_coord(pos.0.x),
            SectionPos::block_to_section_coord(pos.0.z),
        );
        self.chunk_map.get_full_chunk(&chunk_pos)
    }

    /// Ticks the world.
    pub fn tick_b(&self, tick_count: u64) {
        self.chunk_map.tick_b(tick_count);

        // Tick players
        let start = tokio::time::Instant::now();
        self.players.iter_sync(|_uuid, player| {
            player.tick();

            true
        });
        let player_tick_elapsed = start.elapsed();
        if player_tick_elapsed >= Duration::from_millis(100) {
            log::warn!("Player tick slow: {player_tick_elapsed:?}");
        }
    }

    /// Broadcasts a signed chat message to all players in the world.
    ///
    /// # Panics
    /// Panics if `message_signature` is `None` after checking `is_some()` (should never happen).
    pub fn broadcast_chat(
        &self,
        mut packet: CPlayerChat,
        _sender: Arc<Player>,
        sender_last_seen: LastSeen,
        message_signature: Option<[u8; 256]>,
    ) {
        log::debug!(
            "broadcast_chat: sender_last_seen has {} signatures, message_signature present: {}",
            sender_last_seen.len(),
            message_signature.is_some()
        );

        self.players.iter_sync(|_, recipient| {
            let messages_received = recipient.get_and_increment_messages_received();
            packet.global_index = messages_received;

            log::debug!(
                "Broadcasting to player {} (UUID: {}), global_index={}",
                recipient.gameprofile.name,
                recipient.gameprofile.id,
                messages_received
            );

            // IMPORTANT: Index previous messages BEFORE updating the cache
            // This matches vanilla's order: pack() then push()
            let previous_messages = {
                let recipient_cache = recipient.signature_cache.lock();
                recipient_cache.index_previous_messages(&sender_last_seen)
            };

            log::debug!(
                "  Indexed {} previous messages for recipient",
                previous_messages.len()
            );

            packet.previous_messages.clone_from(&previous_messages);

            // Send the packet
            recipient.connection.send_packet(packet.clone());

            // AFTER sending, update the recipient's cache using vanilla's push algorithm
            // This adds all lastSeen signatures + current signature to the cache
            if let Some(signature) = message_signature {
                recipient
                    .signature_cache
                    .lock()
                    .push(&sender_last_seen, Some(&signature));

                log::debug!("  Added signature to recipient's cache and pending list");

                // Add to pending messages for acknowledgment tracking
                recipient
                    .message_validator
                    .lock()
                    .add_pending(Some(Box::new(signature) as Box<[u8]>));
            } else {
                // Even unsigned messages update the pending tracker
                recipient.message_validator.lock().add_pending(None);
                log::debug!("  Added unsigned message to pending list");
            }

            true
        });
    }

    /// Broadcasts a system chat message to all players.
    pub fn broadcast_system_chat(&self, packet: CSystemChat) {
        self.players.iter_sync(|_, player| {
            player.connection.send_packet(packet.clone());
            true
        });
    }

    /// Broadcasts an unsigned player chat message to all players.
    pub fn broadcast_unsigned_chat(
        &self,
        mut packet: CPlayerChat,
        sender_name: &str,
        message: &str,
    ) {
        log::info!("<{sender_name}> {message}");

        self.players.iter_sync(|_, recipient| {
            let messages_received = recipient.get_and_increment_messages_received();
            packet.global_index = messages_received;

            recipient.connection.send_packet(packet.clone());
            true
        });
    }

    /// Saves all dirty chunks in this world to disk.
    ///
    /// This should be called during graceful shutdown.
    /// Returns the number of chunks saved.
    pub async fn save_all_chunks(&self) -> std::io::Result<usize> {
        self.chunk_map.save_all_chunks().await
    }
}

impl RegistryWorld for World {}
