use super::{
    Arc, CSetChunkCenter, ChunkMap, ChunkPos, ChunkStatus, ChunkTicket, Entity, Player,
    PlayerChunkView,
};
use crate::player::chunk_sender::ChunkSender;

impl ChunkMap {
    /// Updates the player's status in the chunk map.
    pub fn update_player_status(&self, player: &Player) {
        let current_chunk_pos = ChunkPos::from_entity_pos(player.position());
        *player.last_chunk_pos.lock() = current_chunk_pos;
        let view_distance = player.view_distance();
        let world = self.world_gen_context.world();
        let player_loading_ticket = ChunkTicket::player_loading(world.view_distance);

        let new_view = PlayerChunkView::new(current_chunk_pos, view_distance);
        let mut last_view_guard = player.last_tracking_view.lock();

        if last_view_guard.as_ref() != Some(&new_view) {
            if let Some(last_view) = last_view_guard.as_ref() {
                if last_view.center != new_view.center {
                    self.replace_chunk_ticket(
                        last_view.center,
                        player_loading_ticket,
                        new_view.center,
                        player_loading_ticket,
                    );
                    self.player_simulation
                        .lock()
                        .move_player(last_view.center, new_view.center);

                    player.send_packet(CSetChunkCenter {
                        x: new_view.center.0.x,
                        y: new_view.center.0.y,
                    });
                }

                // Track chunks for PlayerAreaMap update
                let mut added_chunks = Vec::new();
                let mut removed_chunks = Vec::new();

                // We lock here to ensure we have unique access for the duration of the diff
                let mut chunk_sender = player.chunk_sender.lock();
                let connection = &*player.connection;
                PlayerChunkView::difference(
                    last_view,
                    &new_view,
                    |pos, ctx: &mut (&mut _, &mut Vec<_>, &mut Vec<_>)| {
                        ctx.0.mark_chunk_pending_to_send(pos);
                        ctx.1.push(pos);
                    },
                    |pos, ctx: &mut (&mut _, &mut Vec<_>, &mut Vec<_>)| {
                        ctx.0.drop_chunk(connection, pos);
                        ctx.2.push(pos);
                    },
                    &mut (&mut chunk_sender, &mut added_chunks, &mut removed_chunks),
                );
                drop(chunk_sender);

                // Update the player area map with the diff
                world.player_area_map.on_player_view_change(
                    player.id(),
                    &added_chunks,
                    &removed_chunks,
                );
            } else {
                self.add_chunk_ticket(new_view.center, player_loading_ticket);
                self.player_simulation.lock().add_player(new_view.center);

                // Send initial chunk cache center to client
                player.send_packet(CSetChunkCenter {
                    x: new_view.center.0.x,
                    y: new_view.center.0.y,
                });

                let mut chunk_sender = player.chunk_sender.lock();
                new_view.for_each(|pos| {
                    chunk_sender.mark_chunk_pending_to_send(pos);
                });
                drop(chunk_sender);

                // First time - add all chunks in view to player area map
                world.player_area_map.on_player_join(player, &new_view);
            }

            *last_view_guard = Some(new_view);
        }
        drop(last_view_guard);

        // Entity visibility also depends on exact player position, not only
        // chunk-view changes. Vanilla refreshes tracked entities for accepted
        // movement within the same chunk as well.
        let sent_chunks = player.chunk_sender.lock().sent_chunks_snapshot();
        world
            .entity_tracker()
            .update_player(player, &new_view, |chunk| sent_chunks.contains(&chunk));
    }

    /// Removes a player from the chunk map.
    pub fn remove_player(&self, player: &Player) {
        let last_view = {
            // Keep the same view -> sender lock order as `update_player_status`.
            // The independent chunk-sending loop holds the view through its commit,
            // making this the linearization point for detaching chunk state.
            let mut last_view = player.last_tracking_view.lock();
            let mut chunk_sender = player.chunk_sender.lock();
            let mut chunk_send_epoch = player.chunk_send_epoch.lock();
            *chunk_send_epoch = chunk_send_epoch.wrapping_add(1);
            *chunk_sender = ChunkSender::default();
            *player.last_chunk_pos.lock() = ChunkPos::new(i32::MAX, i32::MAX);
            last_view.take()
        };

        if let Some(last_view) = last_view {
            let world = self.world_gen_context.world();
            let ticket = ChunkTicket::player_loading(world.view_distance);
            self.remove_chunk_ticket(last_view.center, ticket);
            self.player_simulation
                .lock()
                .remove_player(last_view.center);
        }
    }

    /// Applies queued player simulation changes before the world's entity phase.
    pub(super) fn apply_player_simulation_updates(&self) -> bool {
        let changes = {
            let mut player_simulation = self.player_simulation.lock();
            if player_simulation.is_inactive() {
                return false;
            }
            let simulation_distance = self.world_gen_context.world().simulation_distance;
            player_simulation.run_all_updates(simulation_distance)
        };
        if changes.is_empty() {
            return false;
        }

        let world = self.world_gen_context.world();
        let mut rebuild_ticking_snapshot = false;
        for change in changes {
            let Some(holder) = self
                .chunks
                .read_sync(&change.pos, |_, holder| Arc::clone(holder))
            else {
                continue;
            };
            let old_level = holder.simulation_level();
            let readiness = holder.ticking_readiness_snapshot().readiness();
            holder.set_player_simulation_level(change.new_level);
            let new_level = holder.simulation_level();
            if old_level == new_level {
                continue;
            }

            rebuild_ticking_snapshot |= Self::ticking_snapshot_membership(readiness, old_level)
                != Self::ticking_snapshot_membership(readiness, new_level);
            if holder.try_chunk(ChunkStatus::Empty).is_some() {
                world.update_entity_chunk_visibility(change.pos, holder.entity_visibility());
            }
        }

        rebuild_ticking_snapshot
    }
}
