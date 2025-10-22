use std::io::Write;

use crate::{
    packet_traits::PacketWrite,
    packets::{
        common::clientbound_disconnect_packet::ClientboundDisconnectPacket,
        handshake::ClientIntentionPacket,
        login::clientbound_login_disconnect_packet::ClientboundLoginDisconnectPacket,
    },
    utils::{ConnectionProtocol, PacketWriteError},
};
use steel_registry::packets::clientbound::{config, login, play, status};

/*
When adding a common packet search up .addPacket(CommonPacketTypes.CLIENTBOUND_DISCONNECT) for example to see all it's usages.
*/

// Clientbound packets

#[derive(Clone)]
pub enum ClientBoundLogin {
    LoginDisconnectPacket(ClientboundLoginDisconnectPacket),
}

impl ClientBoundLogin {
    pub fn get_id(&self) -> i32 {
        match self {
            Self::LoginDisconnectPacket(_) => login::CLIENTBOUND_LOGIN_DISCONNECT,
        }
    }

    pub fn write_packet(&self, writer: &mut impl Write) -> Result<(), PacketWriteError> {
        match self {
            Self::LoginDisconnectPacket(packet) => packet.write_packet(writer),
        }
    }
}

#[derive(Clone)]
pub enum ClientBoundConfiguration {
    Disconnect(ClientboundDisconnectPacket),
}

impl ClientBoundConfiguration {
    pub fn get_id(&self) -> i32 {
        match self {
            Self::Disconnect(_) => config::CLIENTBOUND_DISCONNECT,
        }
    }

    pub fn write_packet(&self, writer: &mut impl Write) -> Result<(), PacketWriteError> {
        match self {
            Self::Disconnect(packet) => packet.write_packet(writer),
        }
    }
}

#[derive(Clone)]
pub enum ClientBoundStatus {}

impl ClientBoundStatus {
    pub fn get_id(&self) -> i32 {
        unimplemented!("Not implemented")
    }

    pub fn write_packet(&self, writer: &mut impl Write) -> Result<(), PacketWriteError> {
        unimplemented!("Not implemented")
    }
}

#[derive(Clone)]
pub enum ClientBoundPlay {
    Disconnect(ClientboundDisconnectPacket),
}

impl ClientBoundPlay {
    pub fn get_id(&self) -> i32 {
        match self {
            Self::Disconnect(_) => play::CLIENTBOUND_DISCONNECT,
        }
    }

    pub fn write_packet(&self, writer: &mut impl Write) -> Result<(), PacketWriteError> {
        match self {
            Self::Disconnect(packet) => packet.write_packet(writer),
        }
    }
}

#[derive(Clone)]
pub enum ClientBoundPacket {
    Status(ClientBoundStatus),
    Login(ClientBoundLogin),
    Configuration(ClientBoundConfiguration),
    Play(ClientBoundPlay),
}

impl ClientBoundPacket {
    pub fn get_id(&self) -> i32 {
        match self {
            Self::Status(status) => status.get_id(),
            Self::Login(login) => login.get_id(),
            Self::Configuration(configuration) => configuration.get_id(),
            Self::Play(play) => play.get_id(),
        }
    }

    pub fn write_packet(&self, writer: &mut impl Write) -> Result<(), PacketWriteError> {
        match self {
            Self::Status(status) => status.write_packet(writer),
            Self::Login(login) => login.write_packet(writer),
            Self::Configuration(configuration) => configuration.write_packet(writer),
            Self::Play(play) => play.write_packet(writer),
        }
    }
}
