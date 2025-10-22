use bytes::Buf;
use steel_registry::packets::serverbound::{config, handshake, login, play, status};

use crate::{
    packet_traits::PacketRead,
    packets::handshake::ClientIntentionPacket,
    utils::{ConnectionProtocol, PacketReadError, RawPacket},
};

/*
When adding a common packet search up .addPacket(CommonPacketTypes.CLIENTBOUND_DISCONNECT) for example to see all it's usages.
*/

// Serverbound packets

#[derive(Clone, Debug)]
pub enum ServerBoundHandshake {
    Intention(ClientIntentionPacket),
}

impl ServerBoundHandshake {
    pub fn from_raw_packet(raw_packet: RawPacket) -> Result<Self, PacketReadError> {
        match raw_packet.id {
            handshake::SERVERBOUND_INTENTION => {
                let packet = ClientIntentionPacket::read_packet(&mut raw_packet.payload.reader())?;
                Ok(Self::Intention(packet))
            }
            _ => Err(PacketReadError::MalformedValue(format!(
                "Invalid packet id: {}",
                raw_packet.id
            ))),
        }
    }
}

#[derive(Clone, Debug)]
pub enum ServerBoundLogin {}

impl ServerBoundLogin {
    pub fn from_raw_packet(raw_packet: RawPacket) -> Result<Self, PacketReadError> {
        match raw_packet.id {
            _ => Err(PacketReadError::MalformedValue(format!(
                "Invalid packet id: {}",
                raw_packet.id
            ))),
        }
    }
}

#[derive(Clone, Debug)]
pub enum ServerBoundConfiguration {}

impl ServerBoundConfiguration {
    pub fn from_raw_packet(raw_packet: RawPacket) -> Result<Self, PacketReadError> {
        match raw_packet.id {
            _ => Err(PacketReadError::MalformedValue(format!(
                "Invalid packet id: {}",
                raw_packet.id
            ))),
        }
    }
}

#[derive(Clone, Debug)]
pub enum ServerBoundStatus {}

impl ServerBoundStatus {
    pub fn from_raw_packet(raw_packet: RawPacket) -> Result<Self, PacketReadError> {
        match raw_packet.id {
            _ => Err(PacketReadError::MalformedValue(format!(
                "Invalid packet id: {}",
                raw_packet.id
            ))),
        }
    }
}

#[derive(Clone, Debug)]
pub enum ServerBoundPlay {}

impl ServerBoundPlay {
    pub fn from_raw_packet(raw_packet: RawPacket) -> Result<Self, PacketReadError> {
        match raw_packet.id {
            _ => Err(PacketReadError::MalformedValue(format!(
                "Invalid packet id: {}",
                raw_packet.id
            ))),
        }
    }
}

#[derive(Clone, Debug)]
pub enum ServerboundPacket {
    Handshake(ServerBoundHandshake),
    Status(ServerBoundStatus),
    Login(ServerBoundLogin),
    Configuration(ServerBoundConfiguration),
    Play(ServerBoundPlay),
}

impl ServerboundPacket {
    pub fn from_raw_packet(
        raw_packet: RawPacket,
        connection_protocol: ConnectionProtocol,
    ) -> Result<Self, PacketReadError> {
        match connection_protocol {
            ConnectionProtocol::HANDSHAKING => {
                let packet = ServerBoundHandshake::from_raw_packet(raw_packet)?;
                Ok(Self::Handshake(packet))
            }
            ConnectionProtocol::STATUS => {
                let packet = ServerBoundStatus::from_raw_packet(raw_packet)?;
                Ok(Self::Status(packet))
            }
            ConnectionProtocol::LOGIN => {
                let packet = ServerBoundLogin::from_raw_packet(raw_packet)?;
                Ok(Self::Login(packet))
            }
            ConnectionProtocol::CONFIGURATION => {
                let packet = ServerBoundConfiguration::from_raw_packet(raw_packet)?;
                Ok(Self::Configuration(packet))
            }
            ConnectionProtocol::PLAY => {
                let packet = ServerBoundPlay::from_raw_packet(raw_packet)?;
                Ok(Self::Play(packet))
            }
        }
    }
}
