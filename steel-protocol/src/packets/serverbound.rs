use std::io::Cursor;

use steel_registry::packets::serverbound::{config, handshake, login, status};

use crate::{
    packet_traits::PacketRead,
    packets::{
        common::s_custom_payload_packet::SCustomPayloadPacket,
        handshake::ClientIntentionPacket,
        login::{
            s_hello_packet::SHelloPacket, s_key_packet::SKeyPacket,
            s_login_acknowledged_packet::SLoginAcknowledgedPacket,
        },
        status::{
            s_ping_request_packet::SPingRequestPacket,
            s_status_request_packet::SStatusRequestPacket,
        },
    },
    utils::{ConnectionProtocol, PacketError, RawPacket},
};

/*
When adding a common packet search up .addPacket(CommonPacketTypes.CLIENTBOUND_DISCONNECT) for example to see all it's usages.
*/

// Serverbound packets

#[derive(Clone, Debug)]
pub enum SBoundHandshake {
    Intention(ClientIntentionPacket),
}

impl SBoundHandshake {
    pub fn from_raw_packet(raw_packet: RawPacket) -> Result<Self, PacketError> {
        let mut data = Cursor::new(raw_packet.payload);

        match raw_packet.id {
            handshake::SERVERBOUND_INTENTION => {
                let packet = ClientIntentionPacket::read_packet(&mut data)?;
                Ok(Self::Intention(packet))
            }
            _ => Err(PacketError::MalformedValue(format!(
                "Invalid packet id: {}",
                raw_packet.id
            ))),
        }
    }
}

#[derive(Clone, Debug)]
pub enum SBoundLogin {
    Hello(SHelloPacket),
    Key(SKeyPacket),
    LoginAcknowledged(SLoginAcknowledgedPacket),
}

impl SBoundLogin {
    pub fn from_raw_packet(raw_packet: RawPacket) -> Result<Self, PacketError> {
        let mut data = Cursor::new(raw_packet.payload);

        match raw_packet.id {
            login::SERVERBOUND_HELLO => {
                let packet = SHelloPacket::read_packet(&mut data)?;
                Ok(Self::Hello(packet))
            }
            login::SERVERBOUND_KEY => {
                let packet = SKeyPacket::read_packet(&mut data)?;
                Ok(Self::Key(packet))
            }
            login::SERVERBOUND_LOGIN_ACKNOWLEDGED => {
                let packet = SLoginAcknowledgedPacket::read_packet(&mut data)?;
                Ok(Self::LoginAcknowledged(packet))
            }
            _ => Err(PacketError::MalformedValue(format!(
                "Invalid packet id: {}",
                raw_packet.id
            ))),
        }
    }
}

#[derive(Clone, Debug)]
pub enum SBoundConfiguration {
    CustomPayload(SCustomPayloadPacket),
}

impl SBoundConfiguration {
    pub fn from_raw_packet(raw_packet: RawPacket) -> Result<Self, PacketError> {
        let mut data = Cursor::new(raw_packet.payload);

        match raw_packet.id {
            config::SERVERBOUND_CUSTOM_PAYLOAD => {
                let packet = SCustomPayloadPacket::read_packet(&mut data)?;
                Ok(Self::CustomPayload(packet))
            }
            _ => Err(PacketError::MalformedValue(format!(
                "Invalid packet id: {}",
                raw_packet.id
            ))),
        }
    }
}

#[derive(Clone, Debug)]
pub enum SBoundStatus {
    StatusRequest(SStatusRequestPacket),
    PingRequest(SPingRequestPacket),
}

impl SBoundStatus {
    pub fn from_raw_packet(raw_packet: RawPacket) -> Result<Self, PacketError> {
        let mut data = Cursor::new(raw_packet.payload);

        match raw_packet.id {
            status::SERVERBOUND_STATUS_REQUEST => {
                let packet = SStatusRequestPacket::read_packet(&mut data)?;
                Ok(Self::StatusRequest(packet))
            }
            status::SERVERBOUND_PING_REQUEST => {
                let packet = SPingRequestPacket::read_packet(&mut data)?;
                Ok(Self::PingRequest(packet))
            }
            _ => Err(PacketError::MalformedValue(format!(
                "Invalid packet id: {}",
                raw_packet.id
            ))),
        }
    }
}

#[derive(Clone, Debug)]
pub enum SBoundPlay {}

impl SBoundPlay {
    pub fn from_raw_packet(raw_packet: RawPacket) -> Result<Self, PacketError> {
        Err(PacketError::MalformedValue(format!(
            "Invalid packet id: {}",
            raw_packet.id
        )))
    }
}

#[derive(Clone, Debug)]
pub enum SBoundPacket {
    Handshake(SBoundHandshake),
    Status(SBoundStatus),
    Login(SBoundLogin),
    Configuration(SBoundConfiguration),
    Play(SBoundPlay),
}

impl SBoundPacket {
    pub fn from_raw_packet(
        raw_packet: RawPacket,
        connection_protocol: ConnectionProtocol,
    ) -> Result<Self, PacketError> {
        match connection_protocol {
            ConnectionProtocol::HANDSHAKING => {
                let packet = SBoundHandshake::from_raw_packet(raw_packet)?;
                Ok(Self::Handshake(packet))
            }
            ConnectionProtocol::STATUS => {
                let packet = SBoundStatus::from_raw_packet(raw_packet)?;
                Ok(Self::Status(packet))
            }
            ConnectionProtocol::LOGIN => {
                let packet = SBoundLogin::from_raw_packet(raw_packet)?;
                Ok(Self::Login(packet))
            }
            ConnectionProtocol::CONFIGURATION => {
                let packet = SBoundConfiguration::from_raw_packet(raw_packet)?;
                Ok(Self::Configuration(packet))
            }
            ConnectionProtocol::PLAY => {
                let packet = SBoundPlay::from_raw_packet(raw_packet)?;
                Ok(Self::Play(packet))
            }
        }
    }
}
