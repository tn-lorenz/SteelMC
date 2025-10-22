use std::{io::Write, sync::Arc};

use bytes::Bytes;
use crossbeam::atomic::AtomicCell;
use steel_protocol::{
    packet_reader::TCPNetworkDecoder,
    packet_writer::TCPNetworkEncoder,
    packets::{
        clientbound::{
            ClientBoundConfiguration, ClientBoundLogin, ClientBoundPacket, ClientBoundPlay,
        },
        common::clientbound_disconnect_packet::ClientboundDisconnectPacket,
        login::clientbound_login_disconnect_packet::ClientboundLoginDisconnectPacket,
        serverbound::ServerboundPacket,
    },
    ser::NetworkWriteExt,
    utils::{ConnectionProtocol, PacketReadError, PacketWriteError, RawPacket},
};
use steel_utils::text::TextComponent;
use thiserror::Error;
use tokio::{
    io::{BufReader, BufWriter},
    net::{
        TcpStream,
        tcp::{OwnedReadHalf, OwnedWriteHalf},
        unix::SocketAddr,
    },
    sync::{
        Mutex,
        broadcast::{self, Receiver, Sender},
    },
};
use tokio_util::{sync::CancellationToken, task::TaskTracker};

use crate::network::game_profile::GameProfile;

#[derive(Error, Debug)]
pub enum EncryptionError {
    #[error("failed to decrypt shared secret")]
    FailedDecrypt,
    #[error("shared secret has the wrong length")]
    SharedWrongLength,
}

#[derive(Clone, Debug)]
pub struct CompressionInfo {
    /// The compression threshold used when compression is enabled.
    pub threshold: u32,
    /// A value between `0..9`.
    /// `1` = Optimize for the best speed of encoding.
    /// `9` = Optimize for the size of data being encoded.
    pub level: u32,
}

impl Default for CompressionInfo {
    fn default() -> Self {
        Self {
            threshold: 256,
            level: 4,
        }
    }
}

pub struct JavaTcpClient {
    pub id: u64,
    /// The client's game profile information.
    pub gameprofile: Mutex<Option<GameProfile>>,
    /// The current connection state of the client (e.g., Handshaking, Status, Play).
    pub connection_protocol: AtomicCell<ConnectionProtocol>,
    /// The client's IP address.
    pub address: Mutex<SocketAddr>,
    /// A collection of tasks associated with this client. The tasks await completion when removing the client.
    tasks: TaskTracker,
    /// A token to cancel the client's operations. Called when the connection is closed. Or client is removed.
    cancellation_token: CancellationToken,

    pub packet_receiver: Receiver<ServerboundPacket>,
    packet_sender: Sender<ServerboundPacket>,

    /// A queue of serialized packets to send to the network
    outgoing_packet_queue: Sender<Bytes>,
    /// A queue of serialized packets to send to the network
    outgoing_packet_queue_recv: Option<Receiver<Bytes>>,
    /// The packet encoder for outgoing packets.
    network_writer: Arc<Mutex<TCPNetworkEncoder<BufWriter<OwnedWriteHalf>>>>,
    /// The packet decoder for incoming packets.
    network_reader: Mutex<TCPNetworkDecoder<BufReader<OwnedReadHalf>>>,
}

impl JavaTcpClient {
    pub fn new(
        tcp_stream: TcpStream,
        address: SocketAddr,
        id: u64,
        cancellation_token: CancellationToken,
    ) -> Self {
        let (read, write) = tcp_stream.into_split();
        let (send, recv) = broadcast::channel(128);

        let (packet_sender, packet_receiver) = broadcast::channel(128);

        Self {
            id,
            gameprofile: Mutex::new(None),
            address: Mutex::new(address),
            connection_protocol: AtomicCell::new(ConnectionProtocol::HANDSHAKING),
            tasks: TaskTracker::new(),
            cancellation_token,

            packet_receiver,
            packet_sender,
            outgoing_packet_queue: send,
            outgoing_packet_queue_recv: Some(recv),
            network_writer: Arc::new(Mutex::new(TCPNetworkEncoder::new(BufWriter::new(write)))),
            network_reader: Mutex::new(TCPNetworkDecoder::new(BufReader::new(read))),
        }
    }

    async fn set_encryption(
        &self,
        shared_secret: &[u8], // decrypted
    ) -> Result<(), EncryptionError> {
        let crypt_key: [u8; 16] = shared_secret
            .try_into()
            .map_err(|_| EncryptionError::SharedWrongLength)?;
        self.network_reader.lock().await.set_encryption(&crypt_key);
        self.network_writer.lock().await.set_encryption(&crypt_key);
        Ok(())
    }

    async fn set_compression(&self, compression: CompressionInfo) {
        if compression.level > 9 {
            log::error!("Invalid compression level! Clients will not be able to read this!");
        }

        self.network_reader
            .lock()
            .await
            .set_compression(compression.threshold as usize);

        self.network_writer
            .lock()
            .await
            .set_compression((compression.threshold as usize, compression.level));
    }

    async fn get_packet(&self) -> Option<RawPacket> {
        let mut network_reader = self.network_reader.lock().await;
        tokio::select! {
            () = self.cancellation_token.cancelled() => {
                log::debug!("Canceling player packet processing");
                None
            },
            packet_result = network_reader.get_raw_packet() => {
                match packet_result {
                    Ok(packet) => Some(packet),
                    Err(err) => {
                        if !matches!(err, PacketReadError::ConnectionClosed) {
                            log::warn!("Failed to decode packet from client {}: {}", self.id, err);
                            let text = format!("Error while reading incoming packet {err}");
                            self.kick(TextComponent::text(text)).await;
                        }
                        None
                    }
                }
            }
        }
    }

    pub fn close(&self) {
        self.cancellation_token.cancel();
    }

    pub async fn send_packet_now(&self, packet: &ClientBoundPacket) {
        let mut packet_buf = Vec::new();
        let writer = &mut packet_buf;
        self.write_prefixed_packet(packet, writer).unwrap();
        if let Err(err) = self
            .network_writer
            .lock()
            .await
            .write_packet(Bytes::from(packet_buf))
            .await
        {
            // It is expected that the packet will fail if we are cancelled
            if !self.cancellation_token.is_cancelled() {
                log::warn!("Failed to send packet to client {}: {}", self.id, err);
                // We now need to close the connection to the client since the stream is in an
                // unknown state
                self.close();
            }
        }
    }

    pub async fn enqueue_packet(&self, packet: &ClientBoundPacket) -> Result<(), PacketWriteError> {
        let mut packet_buf = Vec::new();
        let writer = &mut packet_buf;
        self.write_prefixed_packet(packet, writer)?;
        self.outgoing_packet_queue
            .send(Bytes::from(packet_buf))
            .map_err(|err| {
                PacketWriteError::Message(format!(
                    "Failed to send packet to client {}: {}",
                    self.id, err
                ))
            })?;
        Ok(())
    }

    pub fn write_prefixed_packet(
        &self,
        packet: &ClientBoundPacket,
        writer: &mut impl Write,
    ) -> Result<(), PacketWriteError> {
        let packet_id = packet.get_id();
        writer.write_var_int(packet_id)?;
        packet.write_packet(writer)?;
        Ok(())
    }

    /// Starts a task that will send packets to the client from the outgoing packet queue.
    /// This task will run until the client is closed or the cancellation token is cancelled.
    fn start_outgoing_packet_task(&mut self) {
        let mut packet_receiver = self
            .outgoing_packet_queue_recv
            .take()
            .expect("This was set in the new fn");
        let cancellation_token = self.cancellation_token.clone();
        let writer = self.network_writer.clone();
        let id = self.id;

        self.tasks.spawn(async move {
            let cancellation_token_clone = cancellation_token.clone();

            cancellation_token
                .run_until_cancelled(async move {
                    loop {
                        match packet_receiver.recv().await {
                            Ok(packet) => {
                                if let Err(err) = writer.lock().await.write_packet(packet).await {
                                    log::warn!("Failed to send packet to client {}: {}", id, err);
                                    cancellation_token_clone.cancel();
                                }
                            }
                            Err(err) => {
                                log::warn!(
                                    "Internal packet_receiver channel closed for client {}: {}",
                                    id,
                                    err
                                );
                                cancellation_token_clone.cancel();
                            }
                        }
                    }
                })
                .await;
        });
    }

    pub async fn kick(&self, reason: TextComponent) {
        match self.connection_protocol.load() {
            ConnectionProtocol::LOGIN => {
                let packet = ClientboundLoginDisconnectPacket::new(reason.0);
                self.send_packet_now(&ClientBoundPacket::Login(
                    ClientBoundLogin::LoginDisconnectPacket(packet),
                ))
                .await;
            }
            ConnectionProtocol::CONFIGURATION => {
                let packet = ClientboundDisconnectPacket::new(reason.0);
                self.send_packet_now(&ClientBoundPacket::Configuration(
                    ClientBoundConfiguration::Disconnect(packet),
                ))
                .await;
            }
            ConnectionProtocol::PLAY => {
                let packet = ClientboundDisconnectPacket::new(reason.0);
                self.send_packet_now(&ClientBoundPacket::Play(ClientBoundPlay::Disconnect(
                    packet,
                )))
                .await;
            }
            _ => {}
        }
        log::debug!("Closing connection for {}", self.id);
        self.close();
    }
}

pub fn is_valid_player_name(name: &str) -> bool {
    name.len() >= 3
        && name.len() <= 16
        && name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
}
