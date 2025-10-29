use std::{
    net::SocketAddr,
    sync::{Arc, atomic::AtomicBool},
};

use crossbeam::atomic::AtomicCell;
use steel_protocol::{
    packet_reader::TCPNetworkDecoder,
    packet_traits::{CBoundPacket, CompressionInfo, EncodedPacket},
    packet_writer::TCPNetworkEncoder,
    packets::{
        common::c_disconnect_packet::CDisconnectPacket,
        handshake::ClientIntent,
        login::c_login_disconnect_packet::CLoginDisconnectPacket,
        serverbound::{
            SBoundConfiguration, SBoundHandshake, SBoundLogin, SBoundPacket, SBoundPlay,
            SBoundStatus,
        },
    },
    utils::{ConnectionProtocol, PacketError},
};
use steel_utils::{FrontVec, text::TextComponent};
use steel_world::player::game_profile::GameProfile;
use thiserror::Error;
use tokio::{
    io::{BufReader, BufWriter},
    net::{
        TcpStream,
        tcp::{OwnedReadHalf, OwnedWriteHalf},
    },
    sync::{
        Mutex, Notify,
        broadcast::{self, Receiver, Sender},
        mpsc,
    },
};
use tokio_util::{sync::CancellationToken, task::TaskTracker};

use crate::{
    network::{
        config,
        login::{self},
        status::{handle_ping_request, handle_status_request},
    },
    server::server::Server,
};

#[derive(Error, Debug)]
pub enum EncryptionError {
    #[error("failed to decrypt shared secret")]
    FailedDecrypt,
    #[error("shared secret has the wrong length")]
    SharedWrongLength,
}

pub enum EnqueuedPacket {
    RawData(FrontVec),
    EncodedPacket(EncodedPacket),
}

#[derive(Clone, Debug)]
pub enum ConnectionUpdate {
    EnableEncryption([u8; 16]),
    EnableCompression(CompressionInfo),
}

pub struct JavaTcpClient {
    pub id: u64,
    /// The client's game profile information.
    pub gameprofile: Mutex<Option<GameProfile>>,
    /// The current connection state of the client (e.g., Handshaking, Status, Play).
    pub connection_protocol: Arc<AtomicCell<ConnectionProtocol>>,
    /// The client's IP address.
    pub address: Mutex<SocketAddr>,
    /// A collection of tasks associated with this client. The tasks await completion when removing the client.
    tasks: TaskTracker,
    /// A token to cancel the client's operations. Called when the connection is closed. Or client is removed.
    cancel_token: CancellationToken,

    packet_receiver: Mutex<Option<Receiver<Arc<SBoundPacket>>>>,
    pub packet_recv_sender: Arc<Sender<Arc<SBoundPacket>>>,

    /// A queue of serialized packets to send to the network
    outgoing_queue: mpsc::UnboundedSender<EnqueuedPacket>,
    /// A queue of serialized packets to send to the network
    outgoing_queue_recv: Option<mpsc::UnboundedReceiver<EnqueuedPacket>>,
    /// The packet encoder for outgoing packets.
    network_writer: Arc<Mutex<TCPNetworkEncoder<BufWriter<OwnedWriteHalf>>>>,
    /// The packet decoder for incoming packets.
    network_reader: Option<TCPNetworkDecoder<BufReader<OwnedReadHalf>>>,
    pub(crate) compression_info: Arc<AtomicCell<Option<CompressionInfo>>>,
    pub(crate) has_requested_status: AtomicBool,
    pub server: Arc<Server>,
    pub challenge: AtomicCell<Option<[u8; 4]>>,

    pub(crate) connection_updates: Arc<Sender<ConnectionUpdate>>,
    pub(crate) connection_update_enabled: Arc<Notify>,

    pub(crate) can_process_next_packet: Arc<Notify>,
}

impl JavaTcpClient {
    pub fn new(
        tcp_stream: TcpStream,
        address: SocketAddr,
        id: u64,
        cancel_token: CancellationToken,
        server: Arc<Server>,
    ) -> Self {
        let (read, write) = tcp_stream.into_split();
        let (send, recv) = tokio::sync::mpsc::unbounded_channel();
        let (connection_updates_send, _) = broadcast::channel(128);

        let (packet_recv_sender, packet_receiver) = broadcast::channel(128);

        Self {
            id,
            gameprofile: Mutex::new(None),
            address: Mutex::new(address),
            connection_protocol: Arc::new(AtomicCell::new(ConnectionProtocol::HANDSHAKING)),
            tasks: TaskTracker::new(),
            cancel_token,

            packet_receiver: Mutex::new(Some(packet_receiver)),
            packet_recv_sender: Arc::new(packet_recv_sender),
            outgoing_queue: send,
            outgoing_queue_recv: Some(recv),
            network_writer: Arc::new(Mutex::new(TCPNetworkEncoder::new(BufWriter::new(write)))),
            network_reader: Some(TCPNetworkDecoder::new(BufReader::new(read))),
            has_requested_status: AtomicBool::new(false),
            compression_info: Arc::new(AtomicCell::new(None)),
            server,
            challenge: AtomicCell::new(None),
            connection_updates: Arc::new(connection_updates_send),
            connection_update_enabled: Arc::new(Notify::new()),
            can_process_next_packet: Arc::new(Notify::new()),
        }
    }

    pub fn close(&self) {
        self.cancel_token.cancel();
    }

    pub async fn await_tasks(&self) {
        self.tasks.close();
        self.tasks.wait().await;
    }

    pub async fn send_packet_now<P: CBoundPacket>(&self, packet: P) {
        let compression_info = self.compression_info.load();
        let connection_protocol = self.connection_protocol.load();
        let encoded_packet =
            EncodedPacket::from_packet(&packet, compression_info, connection_protocol)
                .await
                .unwrap();
        if let Err(err) = self
            .network_writer
            .lock()
            .await
            .write_encoded_packet(&encoded_packet)
            .await
        {
            // It is expected that the packet will fail if we are cancelled
            if !self.cancel_token.is_cancelled() {
                log::warn!("Failed to send packet to client {}: {}", self.id, err);
                // We now need to close the connection to the client since the stream is in an
                // unknown state
                self.close();
            }
        }
    }

    pub fn enqueue_packet<P: CBoundPacket>(&self, packet: P) -> Result<(), PacketError> {
        let connection_protocol = self.connection_protocol.load();
        let buf = EncodedPacket::data_from_packet(&packet, connection_protocol)?;
        self.outgoing_queue
            .send(EnqueuedPacket::RawData(buf))
            .map_err(|e| {
                PacketError::SendError(format!(
                    "Failed to send packet to client {}: {}",
                    self.id, e
                ))
            })?;
        Ok(())
    }

    pub fn enqueue_encoded_packet(&self, packet: EncodedPacket) -> Result<(), PacketError> {
        self.outgoing_queue
            .send(EnqueuedPacket::EncodedPacket(packet))
            .map_err(|e| {
                PacketError::SendError(format!(
                    "Failed to send packet to client {}: {}",
                    self.id, e
                ))
            })?;
        Ok(())
    }

    /// Starts a task that will send packets to the client from the outgoing packet queue.
    /// This task will run until the client is closed or the cancellation token is cancelled.
    pub fn start_outgoing_packet_task(&mut self) {
        let mut sender_recv = self
            .outgoing_queue_recv
            .take()
            .expect("This was set in the new fn");
        let cancel_token = self.cancel_token.clone();
        let network_writer = self.network_writer.clone();
        let id = self.id;
        let compression_info = self.compression_info.clone();
        let mut connection_updates_recv = self.connection_updates.subscribe();
        let connection_update_enabled = self.connection_update_enabled.clone();

        self.tasks.spawn(async move {
            let cancel_token_clone = cancel_token.clone();

            loop {
                tokio::select! {
                    _ = cancel_token_clone.cancelled() => {
                        break;
                    }
                    packet = sender_recv.recv() => {
                        match packet {
                            Some(packet) => {
                                let encoded_packet = match packet {
                                    EnqueuedPacket::EncodedPacket(packet) => packet,
                                    EnqueuedPacket::RawData(packet) => {
                                        EncodedPacket::from_data(packet, compression_info.load())
                                            .await
                                            .unwrap()
                                    }
                                };
                                if let Err(err) = network_writer.lock().await.write_encoded_packet(&encoded_packet).await
                                {
                                    log::warn!("Failed to send packet to client {}: {}", id, err);
                                    cancel_token_clone.cancel();
                                }
                            }
                            None => {
                                log::warn!(
                                    "Internal packet_sender_recv channel closed for client {}",
                                    id,
                                );
                                cancel_token_clone.cancel();
                            }

                        }
                    }
                    connection_update = connection_updates_recv.recv() => {
                        match connection_update {
                            Ok(connection_update) => {
                                if let ConnectionUpdate::EnableEncryption(key) = connection_update {
                                    network_writer.lock().await.set_encryption(&key);
                                    connection_update_enabled.notify_waiters();
                                }
                            }
                            Err(err) => {
                                log::warn!("Internal connection_updates_recv channel closed for client {}: {}", id, err);
                                cancel_token_clone.cancel();
                            }
                        }
                    }
                }
            }
        });
    }

    pub fn start_incoming_packet_task(&mut self) {
        let mut network_reader = self
            .network_reader
            .take()
            .expect("This was set in the new fn");
        let cancel_token = self.cancel_token.clone();
        let id = self.id;
        let packet_recv_sender = self.packet_recv_sender.clone();
        let connection_protocol = self.connection_protocol.clone();
        let mut connection_updates_recv = self.connection_updates.subscribe();
        let can_process_next_packet = self.can_process_next_packet.clone();
        let connection_update_enabled = self.connection_update_enabled.clone();

        self.tasks.spawn(async move {
            let cancel_token_clone = cancel_token.clone();
            loop {
                tokio::select! {
                    _ = cancel_token_clone.cancelled() => {
                        break;
                    }
                    packet = network_reader.get_raw_packet() => {
                        match packet {
                            Ok(packet) => {
                                log::info!("Received packet: {:?}, protocol: {:?}", packet.id, connection_protocol.load());
                                match SBoundPacket::from_raw_packet(
                                    packet,
                                    connection_protocol.load(),
                                ) {
                                    Ok(packet) => {
                                        packet_recv_sender.send(Arc::new(packet)).unwrap();
                                        can_process_next_packet.notified().await;
                                    }
                                    Err(err) => {
                                        log::warn!(
                                            "Failed to get packet from client {}: {}",
                                            id,
                                            err,
                                        );
                                        cancel_token_clone.cancel();
                                    }
                                }
                            }
                            Err(err) => {
                                if cancel_token_clone.is_cancelled() {
                                    break;
                                }
                                log::info!("Failed to get raw packet from client {}: {}", id, err);
                                cancel_token_clone.cancel();
                            }
                        }
                    }
                    connection_update = connection_updates_recv.recv() => {

                        match connection_update {
                            Ok(connection_update) => {
                                match connection_update {
                                    ConnectionUpdate::EnableEncryption(key) => {
                                        network_reader.set_encryption(&key);
                                    }
                                    ConnectionUpdate::EnableCompression(compression) => {
                                        network_reader.set_compression(compression.threshold);
                                        connection_update_enabled.notify_waiters();
                                    }
                                }
                            }
                            Err(err) => {
                                log::warn!("Internal connection_updates_recv channel closed for client {}: {}", id, err);
                                cancel_token_clone.cancel();
                            }
                        }
                    }
                }
            }
        });
    }

    pub async fn process_packets(self: &Arc<Self>) {
        let mut packet_receiver = self
            .packet_receiver
            .lock()
            .await
            .take()
            .expect("This was set in the new fn or the function was called twice");
        let can_process_next_packet = self.can_process_next_packet.clone();

        self.cancel_token
            .run_until_cancelled(async move {
                loop {
                    let packet = packet_receiver.recv().await.unwrap();
                    match &*packet {
                        SBoundPacket::Handshake(packet) => self.handle_handshake(packet).await,
                        SBoundPacket::Status(packet) => self.handle_status(packet).await,
                        SBoundPacket::Login(packet) => self.handle_login(packet).await,
                        SBoundPacket::Configuration(packet) => {
                            self.handle_configuration(packet).await
                        }
                        SBoundPacket::Play(packet) => self.handle_play(packet).await,
                    }
                    can_process_next_packet.notify_waiters();
                }
            })
            .await;
    }

    fn assert_protocol(&self, protocol: ConnectionProtocol) -> bool {
        if self.connection_protocol.load() != protocol {
            self.close();
            return false;
        }
        true
    }

    pub async fn handle_handshake(&self, packet: &SBoundHandshake) {
        if !self.assert_protocol(ConnectionProtocol::HANDSHAKING) {
            return;
        }
        match packet {
            SBoundHandshake::Intention(packet) => {
                let intent = match packet.intention {
                    ClientIntent::LOGIN => ConnectionProtocol::LOGIN,
                    ClientIntent::STATUS => ConnectionProtocol::STATUS,
                    ClientIntent::TRANSFER => ConnectionProtocol::LOGIN,
                };
                self.connection_protocol.store(intent);

                if intent != ConnectionProtocol::STATUS {
                    //TODO: Handle client version being too low or high
                }
            }
        }
    }

    pub async fn handle_status(&self, packet: &SBoundStatus) {
        if !self.assert_protocol(ConnectionProtocol::STATUS) {
            return;
        }

        match packet {
            SBoundStatus::StatusRequest(packet) => handle_status_request(self, packet).await,
            SBoundStatus::PingRequest(packet) => handle_ping_request(self, packet).await,
        }
    }

    pub async fn handle_login(&self, packet: &SBoundLogin) {
        if !self.assert_protocol(ConnectionProtocol::LOGIN) {
            return;
        }

        match packet {
            SBoundLogin::Hello(packet) => login::handle_hello(self, packet).await,
            SBoundLogin::Key(packet) => login::handle_key(self, packet).await,
            SBoundLogin::LoginAcknowledged(packet) => {
                login::handle_login_acknowledged(self, packet).await
            }
        }
    }

    pub async fn handle_configuration(&self, packet: &SBoundConfiguration) {
        if !self.assert_protocol(ConnectionProtocol::CONFIGURATION) {
            return;
        }
        match packet {
            SBoundConfiguration::CustomPayload(packet) => {
                config::handle_custom_payload(self, packet).await
            }
            SBoundConfiguration::ClientInformation(packet) => {
                config::handle_client_information(self, packet).await
            }
            SBoundConfiguration::SelectKnownPacks(packet) => {
                config::handle_select_known_packs(self, packet).await
            }
            SBoundConfiguration::FinishConfiguration(packet) => {
                config::handle_finish_configuration(self, packet).await
            }
        }
    }

    pub async fn handle_play(&self, _packet: &SBoundPlay) {
        if !self.assert_protocol(ConnectionProtocol::PLAY) {
            return;
        }
    }
}

impl JavaTcpClient {
    pub async fn kick(&self, reason: TextComponent) {
        match self.connection_protocol.load() {
            ConnectionProtocol::LOGIN => {
                let packet = CLoginDisconnectPacket::new(reason);
                self.send_packet_now(packet).await;
            }
            ConnectionProtocol::CONFIGURATION => {
                let packet = CDisconnectPacket::new(reason);
                self.send_packet_now(packet).await;
            }
            ConnectionProtocol::PLAY => {
                let packet = CDisconnectPacket::new(reason);
                self.send_packet_now(packet).await;
            }
            _ => {}
        }
        log::debug!("Closing connection for {}", self.id);
        self.close();
    }
}
