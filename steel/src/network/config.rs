use steel_protocol::packets::clientbound::{CBoundConfiguration, CBoundPacket};
use steel_protocol::packets::common::c_custom_payload_packet::CCustomPayloadPacket;
use steel_protocol::packets::common::{
    s_client_information_packet::SClientInformationPacket,
    s_custom_payload_packet::SCustomPayloadPacket,
};
use steel_utils::ResourceLocation;

use crate::network::java_tcp_client::JavaTcpClient;

pub async fn handle_custom_payload(tcp_client: &JavaTcpClient, packet: &SCustomPayloadPacket) {
    println!("Custom payload packet: {:?}", packet);
}

pub async fn handle_client_information(
    tcp_client: &JavaTcpClient,
    packet: &SClientInformationPacket,
) {
    println!("Client information packet: {:?}", packet);
}

const BRAND_PAYLOAD: &[u8; 5] = b"Steel";

pub async fn start_configuration(tcp_client: &JavaTcpClient) {
    tcp_client
        .send_packet_now(CBoundPacket::Configuration(
            CBoundConfiguration::CustomPayload(CCustomPayloadPacket::new(
                ResourceLocation::vanilla_static("brand"),
                Box::new(*BRAND_PAYLOAD),
            )),
        ))
        .await;
}
