//! This module is responsible for sending chunks to the client.
use std::collections::HashSet;

use steel_protocol::packets::game::{
    CChunkBatchFinished, CChunkBatchStart, CForgetLevelChunk, CLevelChunkWithLight,
};
use steel_utils::ChunkPos;

use crate::{chunk::level_chunk::LevelChunk, player::networking::JavaConnection, world::World};

/// This struct is responsible for sending chunks to the client.
#[derive(Debug)]
pub struct ChunkSender {
    /// A list of chunks that are waiting to be sent to the client.
    pub pending_chunks: HashSet<ChunkPos>,
    /// The number of batches that have been sent to the client but have not been acknowledged yet.
    pub unacknowledged_batches: u16,
    /// The number of chunks that should be sent to the client per tick.
    pub desired_chunks_per_tick: f32,
    /// The number of chunks that can be sent to the client in the current batch.
    pub batch_quota: f32,
    /// The maximum number of unacknowledged batches allowed.
    pub max_unacknowledged_batches: u16,
}

impl ChunkSender {
    /// Marks a chunk as pending to be sent to the client.
    pub fn mark_chunk_pending_to_send(&mut self, pos: ChunkPos) {
        self.pending_chunks.insert(pos);
    }

    /// Drops a chunk from the client's view.
    pub fn drop_chunk(&mut self, connection: &JavaConnection, pos: ChunkPos) {
        if !self.pending_chunks.remove(&pos) && !connection.closed() {
            connection.send_packet(CForgetLevelChunk {
                z: pos.0.y,
                x: pos.0.x,
            });
        }
    }

    /// Sends the next batch of chunks to the client.
    pub fn send_next_chunks(
        &mut self,
        connection: &JavaConnection,
        world: &World,
        player_chunk_pos: ChunkPos,
    ) {
        if self.unacknowledged_batches < self.max_unacknowledged_batches {
            let max_batch_size = self.desired_chunks_per_tick.max(1.0);
            self.batch_quota =
                (self.batch_quota + self.desired_chunks_per_tick).min(max_batch_size);

            if self.batch_quota >= 1.0 && !self.pending_chunks.is_empty() {
                let chunks_to_send = self.collect_chunks_to_send(world, player_chunk_pos);

                if !chunks_to_send.is_empty() {
                    log::info!("Sending {} chunks", chunks_to_send.len());
                    self.unacknowledged_batches += 1;
                    connection.send_packet(CChunkBatchStart {});

                    for chunk in &chunks_to_send {
                        Self::send_chunk(chunk, connection);
                    }

                    connection.send_packet(CChunkBatchFinished {
                        batch_size: chunks_to_send.len() as i32,
                    });

                    self.batch_quota -= chunks_to_send.len() as f32;
                }
            }
        }
    }

    fn collect_chunks_to_send(
        &mut self,
        world: &World,
        player_chunk_pos: ChunkPos,
    ) -> Vec<LevelChunk> {
        let max_batch_size = self.batch_quota.floor() as usize;
        let mut candidates: Vec<ChunkPos> = self.pending_chunks.iter().copied().collect();

        // Sort by distance to player
        candidates.sort_by_key(|pos| {
            let dx = pos.0.x - player_chunk_pos.0.x;
            let dz = pos.0.y - player_chunk_pos.0.y;
            dx * dx + dz * dz
        });

        let mut chunks_to_send = Vec::new();

        for pos in candidates {
            if chunks_to_send.len() >= max_batch_size {
                break;
            }

            if let Some(holder) = world.chunk_map.chunks.get_sync(&pos) {
                // Check if chunk is full and get it
                if let Some(chunk) = holder.get().with_full_chunk(std::clone::Clone::clone) {
                    chunks_to_send.push(chunk);
                    self.pending_chunks.remove(&pos);
                }
            }
        }

        chunks_to_send
    }

    /// Sends a chunk to the client.
    pub fn send_chunk(chunk: &LevelChunk, connection: &JavaConnection) {
        connection.send_packet(CLevelChunkWithLight {
            pos: chunk.pos,
            chunk_data: chunk.extract_chunk_data(),
            light_data: chunk.extract_light_data(),
        });
    }

    /// Handles the acknowledgement of a chunk batch from the client.
    pub fn on_chunk_batch_received_by_client(&mut self, _batch_size: f32) {
        if self.unacknowledged_batches > 0 {
            self.unacknowledged_batches -= 1;
        }

        // Java implementation logic for desired_chunks_per_tick update:
        // this.desiredChunksPerTick = Double.isNaN(desiredChunksPerTick) ? 0.01F : Mth.clamp(desiredChunksPerTick, 0.01F, 64.0F);
        // For now we keep it simple or use the passed batch_size if it acts as desired rate?
        // Wait, the packet is ChunkBatchReceived, the field is `desiredChunksPerTick` in Java's method `onChunkBatchReceivedByClient(float desiredChunksPerTick)`.
        // The packet `CChunkBatchFinished` in my implementation has `batch_size`.
        // The name `CChunkBatchFinished` suggests `ClientboundChunkBatchFinished`?
        // No, `ServerboundChunkBatchReceived` is the packet FROM client TO server.
        // In `PlayerChunkSender.java`: `onChunkBatchReceivedByClient` is called when `ServerboundChunkBatchReceivedPacket` is received.
        // My packet name `CChunkBatchFinished` seems to correspond to `ClientboundChunkBatchFinishedPacket` (sent TO client).
        // But I need to handle the packet FROM client.
        // The plan said: "In `process_packet`, add case for `C_CHUNK_BATCH_FINISHED`".
        // Wait, `CChunkBatchFinished` is clientbound. I need the Serverbound packet.
        // The packet from client is `ChunkBatchReceived`.
        // `steel-protocol` seems to have `CChunkBatchFinished` (Clientbound).
        // I need to find the Serverbound packet. `ChunkBatchReceived`?
    }
}

impl Default for ChunkSender {
    fn default() -> Self {
        Self {
            pending_chunks: HashSet::new(),
            unacknowledged_batches: 0,
            desired_chunks_per_tick: 9.0,
            batch_quota: 0.0,
            max_unacknowledged_batches: 1,
        }
    }
}
