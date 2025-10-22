use std::sync::Arc;

use steel_registry::{
    blocks::blocks::BlockRegistry,
    data_components::{DataComponentRegistry, vanilla_components},
    items::items::ItemRegistry,
    vanilla_blocks, vanilla_items,
};
use tokio::{net::TcpListener, select};
use tokio_util::{sync::CancellationToken, task::TaskTracker};

use crate::network::java_tcp_client::JavaTcpClient;

pub mod network;

pub struct SteelServer {
    pub tcp_listener: TcpListener,
    pub cancellation_token: CancellationToken,
    pub client_id: u64,
}

impl SteelServer {
    pub async fn new() -> Self {
        log::info!("Starting Steel Server");

        let mut registry = BlockRegistry::new();
        vanilla_blocks::register_blocks(&mut registry);
        registry.freeze();

        let mut data_component_registry = DataComponentRegistry::new();
        vanilla_components::register_vanilla_data_components(&mut data_component_registry);
        data_component_registry.freeze();

        let mut item_registry = ItemRegistry::new();
        vanilla_items::register_items(&mut item_registry);
        item_registry.freeze();

        Self {
            tcp_listener: TcpListener::bind("0.0.0.0:25565").await.unwrap(),
            cancellation_token: CancellationToken::new(),
            client_id: 0,
        }
    }

    pub async fn start(&mut self) {
        log::info!("Started Steel Server");
        let tasks = TaskTracker::new();

        loop {
            let cancellation_token_clone = self.cancellation_token.clone();

            select! {
                _ = cancellation_token_clone.cancelled() => {
                    break;
                }
                accept_result = self.tcp_listener.accept() => {
                    let (connection, address) = accept_result.unwrap();
                    if let Err(e) = connection.set_nodelay(true) {
                        log::warn!("Failed to set TCP_NODELAY: {e}");
                    }
                    let mut java_client = JavaTcpClient::new(connection, address, self.client_id, cancellation_token_clone.child_token());
                    self.client_id += 1;
                    log::info!("Accepted connection from Java Edition: {address} (id {})", self.client_id);
                    java_client.start_incoming_packet_task();
                    java_client.start_outgoing_packet_task();
                    let java_client = Arc::new(java_client);
                    tasks.spawn(async move {
                        java_client.process_packets().await;
                        java_client.close();
                        java_client.await_tasks().await;
                    });
                }
            }
        }
    }
}
