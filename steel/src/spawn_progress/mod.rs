//! Spawn chunk generation.
//!
//! During server startup, generates chunks around the spawn position until
//! the 7×7 Full area is complete.
//!
//! Set `PREGEN_RADIUS` environment variable to generate a larger area (e.g., 128).

use std::sync::Arc;
use std::time::{Duration, Instant};

use tokio::time::sleep;

use steel_core::chunk::chunk_access::ChunkStatus;
use steel_core::chunk::chunk_map::GenerationTaskCap;
use steel_core::chunk::chunk_ticket_manager::ChunkTicket;
use steel_core::server::Server;
use steel_core::world::World;
use steel_utils::{ChunkPos, SectionPos};

#[cfg(feature = "slow_chunk_gen")]
use std::sync::atomic::Ordering;
#[cfg(feature = "slow_chunk_gen")]
use steel_core::chunk::chunk_holder::SLOW_CHUNK_GEN;

/// Vanilla spawn chunk radius — chunks within this radius reach Full status.
const SPAWN_RADIUS: i32 = 3;

/// Gets the pregeneration radius from environment variable, or returns default spawn radius.
fn get_pregen_radius() -> i32 {
    use std::env;
    env::var("PREGEN_RADIUS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(SPAWN_RADIUS)
}

/// Generates spawn chunks.
///
/// Adds a ticket at the world spawn position so that a 7×7 area of chunks
/// reaches `Full` status. The generation system is pumped in a loop until
/// completion.
///
/// Set `PREGEN_RADIUS` environment variable to generate a larger area.
pub async fn generate_spawn_chunks(server: &Arc<Server>) {
    let overworld = server.overworld();
    let pregen_radius = get_pregen_radius();

    // For large pregeneration, use center at 0,0; otherwise use spawn position
    let center_chunk = if pregen_radius > SPAWN_RADIUS {
        ChunkPos::new(0, 0)
    } else {
        let spawn_pos = overworld.level_data.read().data().spawn_pos();
        ChunkPos::new(
            SectionPos::block_to_section_coord(spawn_pos.0.x),
            SectionPos::block_to_section_coord(spawn_pos.0.z),
        )
    };

    pregen_overworld(overworld, center_chunk, pregen_radius).await;
}

async fn pregen_overworld(world: &Arc<World>, center_chunk: ChunkPos, pregen_radius: i32) {
    let total_chunks = ((pregen_radius * 2 + 1) * (pregen_radius * 2 + 1)) as usize;

    log::info!(
        "Preparing spawn area: {} chunks (radius {}) around chunk ({}, {})",
        total_chunks,
        pregen_radius,
        center_chunk.0.x,
        center_chunk.0.y,
    );

    let ticket = ChunkTicket::full_chunks(3);
    let ticket_positions = build_ticket_positions(center_chunk, pregen_radius);

    {
        let mut tickets = world.chunk_map.chunk_tickets.lock();
        for pos in &ticket_positions {
            tickets.add_ticket(*pos, ticket);
        }
    }

    #[cfg(feature = "slow_chunk_gen")]
    SLOW_CHUNK_GEN.store(true, Ordering::Relaxed);

    let elapsed = {
        let start = Instant::now();
        generate_pregen(world, center_chunk, pregen_radius).await;
        start.elapsed()
    };

    #[cfg(feature = "slow_chunk_gen")]
    SLOW_CHUNK_GEN.store(false, Ordering::Relaxed);

    {
        let mut tickets = world.chunk_map.chunk_tickets.lock();
        for pos in &ticket_positions {
            tickets.remove_ticket(*pos, ticket);
        }
    }

    log::info!(
        "Spawn area prepared: {} chunks in {:.2}s ({:.1} chunks/s)",
        total_chunks,
        elapsed.as_secs_f64(),
        total_chunks as f64 / elapsed.as_secs_f64(),
    );
}

fn build_ticket_positions(center_chunk: ChunkPos, pregen_radius: i32) -> Vec<ChunkPos> {
    if pregen_radius > SPAWN_RADIUS {
        let total = ((pregen_radius * 2 + 1) * (pregen_radius * 2 + 1)) as usize;
        let mut positions = Vec::with_capacity(total);
        for z in -pregen_radius..=pregen_radius {
            for x in -pregen_radius..=pregen_radius {
                positions.push(ChunkPos::new(center_chunk.0.x + x, center_chunk.0.y + z));
            }
        }
        positions
    } else {
        vec![center_chunk]
    }
}

/// Generates chunks with progress reporting for pregeneration.
async fn generate_pregen(world: &Arc<World>, center_chunk: ChunkPos, radius: i32) {
    let total_chunks = ((radius * 2 + 1) * (radius * 2 + 1)) as usize;
    let mut last_report = Instant::now();
    let mut last_completed = 0usize;
    let start = Instant::now();

    loop {
        world
            .chunk_map
            .tick_scheduling(GenerationTaskCap::RespectMaxCap);

        // Count completed chunks
        let completed = count_full_chunks(world, center_chunk, radius);

        // Report progress every 5 seconds for large pregen
        if radius > SPAWN_RADIUS && last_report.elapsed() >= Duration::from_secs(5) {
            let elapsed = start.elapsed().as_secs_f64();
            let chunks_per_sec = if elapsed > 0.0 {
                (completed.saturating_sub(last_completed)) as f64 / 5.0
            } else {
                0.0
            };
            let percent = (completed as f64 / total_chunks as f64) * 100.0;
            let eta = if chunks_per_sec > 0.0 {
                (total_chunks - completed) as f64 / chunks_per_sec
            } else {
                0.0
            };
            log::info!(
                "Progress: {completed}/{total_chunks} ({percent:.1}%), {chunks_per_sec:.1} chunks/s, ETA: {eta:.0}s",
            );
            last_report = Instant::now();
            last_completed = completed;
        }

        if completed == total_chunks {
            break;
        }

        sleep(Duration::from_millis(10)).await;
    }
}

/// Counts how many chunks in the area have reached Full status.
fn count_full_chunks(world: &Arc<World>, center_chunk: ChunkPos, radius: i32) -> usize {
    let mut completed = 0;
    for dz in -radius..=radius {
        for dx in -radius..=radius {
            let pos = ChunkPos::new(center_chunk.0.x + dx, center_chunk.0.y + dz);
            let status = world
                .chunk_map
                .chunks
                .read_sync(&pos, |_, holder| holder.persisted_status())
                .flatten();
            if status == Some(ChunkStatus::Full) {
                completed += 1;
            }
        }
    }
    completed
}
