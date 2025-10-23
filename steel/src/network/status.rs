use std::sync::{LazyLock, atomic::Ordering};

use steel_protocol::packets::{
    clientbound::{CBoundPacket, CBoundStatus},
    status::{
        c_pong_response_packet::CPongResponsePacket,
        c_status_response_packet::{CStatusResponsePacket, Players, Status, Version},
        s_ping_request_packet::SPingRequestPacket,
        s_status_request_packet::SStatusRequestPacket,
    },
};
use steel_utils::text::TextComponent;

use crate::network::java_tcp_client::JavaTcpClient;

pub async fn handle_status_request(tcp_client: &JavaTcpClient, packet: SStatusRequestPacket) {
    // Checks if this funciton has already been called this connection. If not it sets has_requested_status to true. If it has been called before compare_exchange fails.
    if let Err(_) = tcp_client.has_requested_status.compare_exchange(
        false,
        true,
        Ordering::Relaxed,
        Ordering::Relaxed,
    ) {
        tcp_client.close();
        return;
    }

    let res_packet = CStatusResponsePacket::new(Status {
        description: "Hello World!".to_string(),
        players: Some(Players {
            max: 10,
            online: 5,
            sample: vec![],
        }),
        enforce_secure_chat: false,
        favicon: None,
        version: Some(Version {
            name: "1.21.10".to_string(),
            protocol: steel_registry::packets::CURRENT_MC_PROTOCOL as i32,
        }),
    });
    tcp_client
        .send_packet_now(&CBoundPacket::Status(CBoundStatus::StatusResponse(
            res_packet,
        )))
        .await;
}

pub async fn handle_ping_request(tcp_client: &JavaTcpClient, packet: SPingRequestPacket) {
    let res_packet = CPongResponsePacket::new(packet.time);
    tcp_client
        .send_packet_now(&CBoundPacket::Status(CBoundStatus::Pong(res_packet)))
        .await;
    tcp_client.close();
}
