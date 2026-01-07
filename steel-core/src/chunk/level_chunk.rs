//! This module contains the `LevelChunk` struct, which is a chunk that is ready to be sent to the client.
use std::{
    io::Cursor,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

use steel_protocol::packets::game::{
    ChunkPacketData, HeightmapType as ProtocolHeightmapType, Heightmaps, LightUpdatePacketData,
};
use steel_registry::{vanilla_blocks, BlockStateExt, REGISTRY};
use steel_utils::{
    codec::BitSet, locks::SyncRwLock, types::UpdateFlags, BlockPos, BlockStateId, ChunkPos,
};

use crate::chunk::{
    heightmap::{ChunkHeightmaps, HeightmapType},
    proto_chunk::ProtoChunk,
    section::Sections,
};

/// A chunk that is ready to be sent to the client.
#[derive(Debug)]
pub struct LevelChunk {
    /// The sections of the chunk.
    pub sections: Sections,
    /// The position of the chunk.
    pub pos: ChunkPos,
    /// Whether the chunk has been modified since last save.
    pub dirty: Arc<AtomicBool>,
    /// The heightmaps for this chunk (wrapped in `RwLock` for interior mutability).
    pub heightmaps: Arc<SyncRwLock<ChunkHeightmaps>>,
    /// The minimum Y coordinate of the world this chunk belongs to.
    min_y: i32,
    /// The total height of the world.
    height: i32,
}

impl LevelChunk {
    /// Creates a new `LevelChunk` from a `ProtoChunk`.
    ///
    /// Transfers final heightmaps from the proto chunk if available.
    #[must_use]
    pub fn from_proto(proto_chunk: ProtoChunk, min_y: i32, height: i32) -> Self {
        // Transfer final heightmaps from proto chunk if available
        let proto_heightmaps = proto_chunk.heightmaps.read();
        let mut chunk_heightmaps = ChunkHeightmaps::new(min_y, height);

        // Copy final heightmap data if available in proto chunk
        for &hm_type in HeightmapType::final_types() {
            if let Some(proto_hm) = proto_heightmaps.get(&hm_type) {
                chunk_heightmaps
                    .get_mut(hm_type)
                    .set_raw_data(&proto_hm.get_raw_data());
            }
        }
        drop(proto_heightmaps);

        Self {
            sections: proto_chunk.sections,
            pos: proto_chunk.pos,
            dirty: proto_chunk.dirty.clone(),
            heightmaps: Arc::new(SyncRwLock::new(chunk_heightmaps)),
            min_y,
            height,
        }
    }

    /// Creates a new `LevelChunk` that was loaded from disk (not dirty).
    #[must_use]
    pub fn from_disk(sections: Sections, pos: ChunkPos, min_y: i32, height: i32) -> Self {
        Self {
            sections,
            pos,
            dirty: Arc::new(AtomicBool::new(false)),
            heightmaps: Arc::new(SyncRwLock::new(ChunkHeightmaps::new(min_y, height))),
            min_y,
            height,
        }
    }

    /// Returns the minimum Y coordinate of the world.
    #[must_use]
    pub const fn min_y(&self) -> i32 {
        self.min_y
    }

    /// Returns the total height of the world.
    #[must_use]
    pub const fn height(&self) -> i32 {
        self.height
    }

    /// Gets the section index for a given Y coordinate.
    #[must_use]
    fn get_section_index(&self, y: i32) -> usize {
        ((y - self.min_y) / 16) as usize
    }

    /// Marks the chunk as unsaved.
    fn mark_unsaved(&self) {
        self.dirty.store(true, Ordering::Release);
    }

    /// Sets a block state at the given position.
    ///
    /// Returns the old block state, or `None` if nothing changed.
    ///
    /// # Arguments
    /// * `pos` - The absolute block position
    /// * `state` - The new block state to set
    /// * `flags` - Update flags controlling behavior
    #[must_use]
    pub fn set_block_state(
        &self,
        pos: BlockPos,
        state: BlockStateId,
        flags: UpdateFlags,
    ) -> Option<BlockStateId> {
        let y = pos.0.y;

        // Bounds check - return None if out of range (vanilla behavior)
        // Java: LevelChunk assumes valid bounds, but we add safety
        if y < self.min_y || y >= self.min_y + self.height {
            return None;
        }

        let section_index = self.get_section_index(y);

        // Safety check for section index
        if section_index >= self.sections.sections.len() {
            return None;
        }

        let section = &self.sections.sections[section_index];

        let was_empty = section.read().states.has_only_air();
        if was_empty && state.is_air() {
            return None;
        }

        let local_x = (pos.0.x & 15) as usize;
        let local_y = (y & 15) as usize;
        let local_z = (pos.0.z & 15) as usize;

        let old_state = section.write().states.set(local_x, local_y, local_z, state);

        if old_state == state {
            return None;
        }

        // Update heightmaps
        let min_y = self.min_y;
        let sections = &self.sections;
        self.heightmaps
            .write()
            .update(local_x, y, local_z, state, |lx, scan_y, lz| {
                let scan_section_index = ((scan_y - min_y) / 16) as usize;
                let scan_local_y = ((scan_y - min_y) % 16) as usize;
                sections.sections[scan_section_index]
                    .read()
                    .states
                    .get(lx, scan_local_y, lz)
            });

        // Check if section emptiness changed
        let is_empty = section.read().states.has_only_air();
        if was_empty != is_empty {
            // TODO: Light engine updates
            // self.level.get_chunk_source().get_light_engine().update_section_status(pos, is_empty);
            // self.level.get_chunk_source().on_section_emptiness_changed(
            //     self.pos.0.x,
            //     SectionPos::block_to_section_coord(y),
            //     self.pos.0.y,
            //     is_empty
            // );
        }

        // TODO: Light property changes
        // if LightEngine::has_different_light_properties(old_state, state) {
        //     ProfilerFiller profiler = Profiler::get();
        //     profiler.push("updateSkyLightSources");
        //     self.sky_light_sources.update(self, local_x, y, local_z);
        //     profiler.pop_push("queueCheckLight");
        //     self.level.get_chunk_source().get_light_engine().check_block(pos);
        //     profiler.pop();
        // }

        let old_block = old_state.get_block();
        let new_block = state.get_block();
        let _block_changed = !std::ptr::eq(old_block, new_block);
        let _moved_by_piston = flags.contains(UpdateFlags::UPDATE_MOVE_BY_PISTON);
        let _side_effects = !flags.contains(UpdateFlags::UPDATE_SKIP_BLOCK_ENTITY_SIDEEFFECTS);

        // TODO: Block entity handling
        // Handle block entity removal when block type changes
        // if block_changed && old_state.has_block_entity() && !state.should_changed_state_keep_block_entity(old_state) {
        //     if !self.level.is_client_side() && side_effects {
        //         let block_entity = self.level.get_block_entity(pos);
        //         if let Some(be) = block_entity {
        //             be.pre_remove_side_effects(pos, old_state);
        //         }
        //     }
        //     self.remove_block_entity(pos);
        // }

        // LevelChunk.java:327-329: Re-read to verify the block placement "stuck"
        // This prevents running updates if a callback already changed the block
        let current_block = section
            .read()
            .states
            .get(local_x, local_y, local_z)
            .get_block();
        if !std::ptr::eq(current_block, new_block) {
            return None;
        }

        // TODO: Block entity creation/update
        // if state.has_block_entity() {
        //     let block_entity = self.get_block_entity(pos, EntityCreationType::CHECK);
        //     if let Some(be) = block_entity {
        //         if !be.is_valid_block_state(state) {
        //             log::warn!("Found mismatched block entity @ {:?}: type = {:?}, state = {:?}",
        //                        pos, be.get_type(), state);
        //             self.remove_block_entity(pos);
        //             block_entity = None;
        //         }
        //     }
        //
        //     if block_entity.is_none() {
        //         block_entity = new_block.new_block_entity(pos, state);
        //         if let Some(be) = block_entity {
        //             self.add_and_register_block_entity(be);
        //         }
        //     } else {
        //         block_entity.set_block_state(state);
        //         self.update_block_entity_ticker(block_entity);
        //     }
        // }

        self.mark_unsaved();
        Some(old_state)
    }

    /// Gets a block state at the given position.
    #[must_use]
    pub fn get_block_state(&self, pos: BlockPos) -> BlockStateId {
        let y = pos.0.y;
        let section_index = self.get_section_index(y);

        // Bounds check - return air if out of range
        if section_index >= self.sections.sections.len() {
            return REGISTRY.blocks.get_base_state_id(vanilla_blocks::VOID_AIR);
        }

        let section = &self.sections.sections[section_index];
        let section_guard = section.read();

        if section_guard.states.has_only_air() {
            return REGISTRY.blocks.get_base_state_id(vanilla_blocks::VOID_AIR);
        }

        let local_x = (pos.0.x & 15) as usize;
        let local_y = (y & 15) as usize;
        let local_z = (pos.0.z & 15) as usize;

        section_guard.states.get(local_x, local_y, local_z)
    }

    /// Extracts the chunk data for sending to the client.
    #[must_use]
    pub fn extract_chunk_data(&self) -> ChunkPacketData {
        let data = Vec::new();

        let mut cursor = Cursor::new(data);
        self.sections.sections.iter().for_each(|section| {
            section.read().write(&mut cursor);
        });

        let heightmaps_guard = self.heightmaps.read();
        ChunkPacketData {
            heightmaps: Heightmaps {
                heightmaps: vec![
                    (
                        ProtocolHeightmapType::WorldSurface,
                        heightmaps_guard
                            .get(HeightmapType::WorldSurface)
                            .get_raw_data(),
                    ),
                    (
                        ProtocolHeightmapType::MotionBlocking,
                        heightmaps_guard
                            .get(HeightmapType::MotionBlocking)
                            .get_raw_data(),
                    ),
                    (
                        ProtocolHeightmapType::MotionBlockingNoLeaves,
                        heightmaps_guard
                            .get(HeightmapType::MotionBlockingNoLeaves)
                            .get_raw_data(),
                    ),
                ],
            },
            data: cursor.into_inner(),
            block_entities: Vec::new(),
        }
    }

    /// Extracts the light data for sending to the client.
    #[must_use]
    pub fn extract_light_data(&self) -> LightUpdatePacketData {
        let section_count = self.sections.sections.len();
        let mut sky_y_mask = BitSet(vec![0; section_count.div_ceil(64)].into_boxed_slice());
        let mut block_y_mask = BitSet(vec![0; section_count.div_ceil(64)].into_boxed_slice());
        let empty_sky_y_mask = BitSet(vec![0; section_count.div_ceil(64)].into_boxed_slice());
        let empty_block_y_mask = BitSet(vec![0; section_count.div_ceil(64)].into_boxed_slice());

        let mut sky_updates = Vec::new();
        let mut block_updates = Vec::new();

        for i in 0..section_count {
            sky_y_mask.set(i, true);
            block_y_mask.set(i, true);
            sky_updates.push(vec![0xFF; 2048]);
            block_updates.push(vec![0xFF; 2048]);
        }

        LightUpdatePacketData {
            sky_y_mask,
            block_y_mask,
            empty_sky_y_mask,
            empty_block_y_mask,
            sky_updates,
            block_updates,
        }
    }
}
