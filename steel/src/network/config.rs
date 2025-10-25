use steel_protocol::packets::common::s_custom_payload_packet::SCustomPayloadPacket;

use crate::network::java_tcp_client::JavaTcpClient;

pub async fn handle_custom_payload(tcp_client: &JavaTcpClient, packet: &SCustomPayloadPacket) {
    println!("Custom payload packet: {:?}", packet);
}
