use steel_protocol::packets::{common::SCustomPayload, game::SClientTickEnd};

use crate::network::java_tcp_client::JavaTcpClient;

impl JavaTcpClient {
    pub fn handle_custom_payload(&self, packet: SCustomPayload) {
        if let Some(player) = self.player.get()
            && let Some(player) = player.upgrade()
        {
            player.handle_custom_payload(packet);
        }
    }

    pub fn handle_client_tick_end(&self, packet: SClientTickEnd) {
        if let Some(player) = self.player.get()
            && let Some(player) = player.upgrade()
        {
            player.handle_client_tick_end(packet);
        }
    }
}
