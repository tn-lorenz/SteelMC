use super::*;
use crate::player::ClientInformation;

#[test]
fn light_changed_does_not_broadcast_unloading_full_chunk() {
    let chunk_map = test_chunk_map();
    let pos = ChunkPos::new(2, 3);
    let holder = unloaded_full_holder(pos);
    let _ = chunk_map
        .unloading_chunks
        .insert_sync(pos, Arc::clone(&holder));

    let chunk = holder
        .try_chunk(ChunkStatus::Full)
        .expect("test holder should contain a full chunk");
    chunk.clear_dirty();

    chunk_map.light_changed(LightLayer::Block, SectionPos::new(pos.0.x, 0, pos.0.y));

    let chunk = holder
        .try_chunk(ChunkStatus::Full)
        .expect("test holder should still contain a full chunk");
    assert!(chunk.is_dirty());

    assert!(chunk_map.chunks_to_broadcast.lock().is_empty());
    assert!(!holder.has_changes_to_broadcast());
}

#[test]
fn broadcast_changed_chunks_does_not_defer_blocks_while_light_work_is_blocked() {
    init_vanilla_registry();
    init_behaviors();
    let world = fresh_test_world("blocked_light_block_publication");
    let center = ChunkPos::new(0, 0);
    let holder = insert_ready_full_chunk(&world, center);
    for z in -LIGHT_CACHE_RADIUS..=LIGHT_CACHE_RADIUS {
        for x in -LIGHT_CACHE_RADIUS..=LIGHT_CACHE_RADIUS {
            if x != 0 || z != 0 {
                insert_ready_full_chunk(&world, ChunkPos::new(x, z));
            }
        }
    }
    let pos = BlockPos::new(1, 2, 3);
    assert!(world.set_block(
        pos,
        vanilla_blocks::STONE.default_state(),
        UpdateFlags::UPDATE_ALL,
    ));
    world.chunk_map.broadcast_changed_chunks();
    assert!(!world.chunk_map.light_update_touches_chunk(center));

    let (player, packets) = recording_player(&world);
    assert!(world.add_player(Arc::clone(&player), ResetReason::InitialJoin));
    let _ = player.mark_joined_world();
    player.set_client_loaded(true);
    player.chunk_sender.lock().mark_chunk_sent_for_test(center);
    packets.lock().clear();

    let Some(reservation) = world
        .chunk_map
        .light_work_window_gate
        .try_reserve_centered(center)
    else {
        panic!("test should reserve the light work window");
    };

    assert!(world.set_block(
        pos,
        vanilla_blocks::AIR.default_state(),
        UpdateFlags::UPDATE_ALL,
    ));
    assert!(holder.has_changes_to_broadcast());
    assert!(world.chunk_map.light_update_touches_chunk(center));
    player.ack_block_changes_up_to(1);

    world.tick_game(1, true);

    assert!(world.chunk_map.chunks_to_broadcast.lock().is_empty());
    assert!(!holder.has_changes_to_broadcast());
    assert_eq!(holder.take_changed_blocks().len(), 0);
    assert!(world.chunk_map.light_update_touches_chunk(center));
    let relevant_packet_ids = packets
        .lock()
        .iter()
        .map(packet_id)
        .filter(|id| matches!(*id, C_BLOCK_UPDATE | C_BLOCK_CHANGED_ACK))
        .collect::<Vec<_>>();
    assert_eq!(relevant_packet_ids, [C_BLOCK_UPDATE, C_BLOCK_CHANGED_ACK]);

    drop(reservation);
    world.chunk_map.broadcast_changed_chunks();

    assert!(!world.chunk_map.light_update_touches_chunk(center));
    assert!(world.chunk_map.chunks_to_broadcast.lock().is_empty());
    world.remove_player_for_world_change(&player);
}

#[test]
fn frozen_tick_broadcasts_block_changes_before_acknowledging_them() {
    init_vanilla_registry();
    init_behaviors();
    let world = fresh_test_world("frozen_block_change_publication");
    let chunk_pos = ChunkPos::new(0, 0);
    let holder = insert_ready_full_chunk(&world, chunk_pos);
    let pos = BlockPos::new(1, 64, 1);
    assert!(world.set_block(
        pos,
        vanilla_blocks::STONE.default_state(),
        UpdateFlags::UPDATE_ALL,
    ));
    world.chunk_map.broadcast_changed_chunks();

    let (player, packets) = recording_player(&world);
    assert!(world.add_player(Arc::clone(&player), ResetReason::InitialJoin));
    let _ = player.mark_joined_world();
    player.set_client_loaded(true);
    player
        .chunk_sender
        .lock()
        .mark_chunk_sent_for_test(chunk_pos);
    packets.lock().clear();

    assert!(world.set_block(
        pos,
        vanilla_blocks::AIR.default_state(),
        UpdateFlags::UPDATE_ALL,
    ));
    assert!(holder.has_changes_to_broadcast());
    player.ack_block_changes_up_to(1);

    world.tick_game(1, false);

    assert!(!holder.has_changes_to_broadcast());
    let relevant_packet_ids = packets
        .lock()
        .iter()
        .map(packet_id)
        .filter(|id| matches!(*id, C_BLOCK_UPDATE | C_BLOCK_CHANGED_ACK))
        .collect::<Vec<_>>();
    assert_eq!(relevant_packet_ids, [C_BLOCK_UPDATE, C_BLOCK_CHANGED_ACK]);
    world.remove_player_for_world_change(&player);
}

#[test]
fn player_simulation_removal_applies_before_the_next_world_tick() {
    init_vanilla_registry();
    init_behaviors();
    let world = fresh_test_world("synchronous_player_simulation_removal");
    let center = ChunkPos::new(0, 0);
    let holder = insert_ready_full_chunk(&world, center);
    holder.set_non_player_simulation_level(None);
    holder.swap_load_level(ChunkTicketLevel::ENTITY_TICKING_CHUNK);
    holder.transition_ticking_readiness(TickingReadiness::EntityTicking);
    world.chunk_map.rebuild_ticking_chunk_snapshot();
    assert_eq!(world.chunk_map.tickable_full_chunk_positions().len(), 0);

    let ChunkSchedulingBoundaryStep::Start { .. } = world.chunk_map.scheduling.take_boundary_step()
    else {
        panic!("fresh scheduling coordinator should start an epoch");
    };

    let player = TestPlayerBuilder::new(Arc::clone(&world), "SimulationPlayer", 1).build();
    assert!(world.add_player(Arc::clone(&player), ResetReason::InitialJoin));
    world.chunk_map.advance_scheduling();
    assert_eq!(world.chunk_map.tickable_full_chunk_positions(), [center]);

    world.remove_player_for_world_change(&player);
    world.chunk_map.flush_player_simulation();
    assert_eq!(world.chunk_map.tickable_full_chunk_positions().len(), 0);

    world.chunk_map.stop_generation_refill_loop();
    world.chunk_map.task_tracker.close();
    world
        .chunk_map
        .chunk_runtime
        .block_on(world.chunk_map.task_tracker.wait());
}

#[test]
fn removing_a_player_preserves_non_player_simulation() {
    init_vanilla_registry();
    init_behaviors();
    let world = fresh_test_world("non_player_simulation_overlap");
    let center = ChunkPos::new(0, 0);
    let holder = insert_ready_full_chunk(&world, center);
    holder.swap_load_level(ChunkTicketLevel::ENTITY_TICKING_CHUNK);
    holder.set_non_player_simulation_level(Some(ChunkTicketLevel::ENTITY_TICKING_CHUNK));
    holder.transition_ticking_readiness(TickingReadiness::EntityTicking);
    world.chunk_map.rebuild_ticking_chunk_snapshot();

    let ChunkSchedulingBoundaryStep::Start { .. } = world.chunk_map.scheduling.take_boundary_step()
    else {
        panic!("fresh scheduling coordinator should start an epoch");
    };

    let player = TestPlayerBuilder::new(Arc::clone(&world), "OverlapPlayer", 1).build();
    assert!(world.add_player(Arc::clone(&player), ResetReason::InitialJoin));
    world.chunk_map.flush_player_simulation();
    world.remove_player_for_world_change(&player);
    world.chunk_map.flush_player_simulation();

    assert_eq!(
        holder.simulation_level(),
        Some(ChunkTicketLevel::ENTITY_TICKING_CHUNK)
    );
    assert_eq!(world.chunk_map.tickable_full_chunk_positions(), [center]);

    world.chunk_map.stop_generation_refill_loop();
    world.chunk_map.task_tracker.close();
    world
        .chunk_map
        .chunk_runtime
        .block_on(world.chunk_map.task_tracker.wait());
}

#[test]
fn player_loading_uses_the_server_view_distance() {
    init_vanilla_registry();
    init_behaviors();
    let world = fresh_test_world_with_distances("server_player_loading_distance", 4, 2);
    let client_information = ClientInformation {
        view_distance: 2,
        ..ClientInformation::default()
    };
    let player = TestPlayerBuilder::new(Arc::clone(&world), "ShortViewPlayer", 1)
        .client_information(client_information)
        .build();
    assert!(world.add_player(Arc::clone(&player), ResetReason::InitialJoin));

    let barrier_pos = ChunkPos::new(100, 100);
    let barrier_ticket = ChunkTicket::loading(ChunkTicketLevel::MAX);
    let barrier_revision = world
        .chunk_map
        .add_chunk_ticket(barrier_pos, barrier_ticket);
    advance_until_revision(&world.chunk_map, barrier_revision);

    let outer_entity_ticking_pos = ChunkPos::new(4, 0);
    let outer_level = world
        .chunk_map
        .chunks
        .read_sync(&outer_entity_ticking_pos, |_, holder| holder.load_level())
        .flatten();
    assert_eq!(outer_level, Some(ChunkTicketLevel::ENTITY_TICKING_CHUNK));

    let simulated_pos = ChunkPos::new(2, 0);
    let simulation_level = world
        .chunk_map
        .chunks
        .read_sync(&simulated_pos, |_, holder| holder.simulation_level())
        .flatten();
    assert_eq!(
        simulation_level,
        Some(ChunkTicketLevel::ENTITY_TICKING_CHUNK)
    );

    world.remove_player_for_world_change(&player);
    let cleanup_revision = world
        .chunk_map
        .remove_chunk_ticket(barrier_pos, barrier_ticket);
    advance_until_revision(&world.chunk_map, cleanup_revision);
    world.chunk_map.stop_generation_refill_loop();
    world.chunk_map.task_tracker.close();
    world
        .chunk_map
        .chunk_runtime
        .block_on(world.chunk_map.task_tracker.wait());
}
