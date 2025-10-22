use steel_registry::packets::serverbound::{config, handshake, login, play, status};

use crate::packets::handshake::ClientIntentionPacket;

/*
When adding a common packet search up .addPacket(CommonPacketTypes.CLIENTBOUND_DISCONNECT) for example to see all it's usages.
*/

// Serverbound packets

#[derive(Clone)]
pub enum ServerBoundHandshake {
    Intention(ClientIntentionPacket),
}

impl ServerBoundHandshake {
    pub fn get_id(&self) -> i32 {
        match self {
            Self::Intention(_) => handshake::SERVERBOUND_INTENTION,
        }
    }
}

#[derive(Clone)]
pub enum ServerBoundLogin {}

impl ServerBoundLogin {
    pub fn get_id(&self) -> i32 {
        unimplemented!()
    }
}

#[derive(Clone)]
pub enum ServerBoundConfiguration {}

impl ServerBoundConfiguration {
    pub fn get_id(&self) -> i32 {
        unimplemented!()
    }
}

#[derive(Clone)]
pub enum ServerBoundStatus {}

impl ServerBoundStatus {
    pub fn get_id(&self) -> i32 {
        unimplemented!()
    }
}

#[derive(Clone)]
pub enum ServerBoundPlay {}

impl ServerBoundPlay {
    pub fn get_id(&self) -> i32 {
        unimplemented!()
    }
}

#[derive(Clone)]
pub enum ServerboundPacket {
    Handshake(ServerBoundHandshake),
    Status(ServerBoundStatus),
    Login(ServerBoundLogin),
    Configuration(ServerBoundConfiguration),
    Play(ServerBoundPlay),
}

impl ServerboundPacket {
    pub fn get_id(&self) -> i32 {
        match self {
            Self::Handshake(handshake) => handshake.get_id(),
            Self::Status(status) => status.get_id(),
            Self::Login(login) => login.get_id(),
            Self::Configuration(configuration) => configuration.get_id(),
            Self::Play(play) => play.get_id(),
        }
    }
}
