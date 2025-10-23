use steel_protocol::packets::login::s_hello_packet::SHelloPacket;
use steel_utils::text::TextComponent;

use crate::network::java_tcp_client::JavaTcpClient;

pub fn is_valid_player_name(name: &str) -> bool {
    name.len() >= 3
        && name.len() <= 16
        && name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
}

pub async fn handle_hello(tcp_client: &JavaTcpClient, packet: SHelloPacket) {
    if !is_valid_player_name(&packet.name) {
        tcp_client
            .kick(TextComponent::text("Invalid player name"))
            .await;
        return;
    }
}
