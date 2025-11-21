use log::LevelFilter;
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode};
use steel::SteelServer;
use steel_utils::{Identifier, translations};

#[tokio::main]
async fn main() {
    TermLogger::init(
        LevelFilter::Info,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )
    .unwrap();

    let mut steel = SteelServer::new().await;

    log::info!(
        "{:?}",
        steel
            .server
            .registry
            .items
            .get_tag(&Identifier::vanilla_static("swords"))
            .unwrap()
            .iter()
            .map(|b| b.key.path.to_string())
            .collect::<Vec<String>>()
    );

    log::info!(
        "{}",
        translations::DEATH_ATTACK_ANVIL_PLAYER
            .message(["4LVE", "Borrow Checker"])
            .format()
    );

    /*
    steel.server.worlds[0]
        .chunk_map
        .distance_manager
        .lock()
        .await
        .add_player(ChunkPos::new(0, 0), 10);

    let _start = Instant::now();

    println!("{:?}", steel.server.worlds[0].chunk_map.chunks.len()); */

    /*
    steel.server.worlds[0]
        .chunk_map
        .chunks
        .get_async(&ChunkPos::new(0, 10))
        .await
        .unwrap()
        .get()
        .await_chunk_and_then(ChunkStatus::Full, |chunk| match chunk {
            ChunkAccess::Full(chunk) => {
                log::info!(
                    "Lesgo {:?} in {:?}",
                    chunk.sections.sections.len(),
                    start.elapsed()
                );
            }
            _ => unreachable!(),
        })
        .await;
     */

    let server = steel.server.clone();
    let cancel_token = steel.cancel_token.clone();

    tokio::spawn(async move {
        if tokio::signal::ctrl_c().await.is_ok() {
            log::info!("Shutdown signal received");
            cancel_token.cancel();
        }
    });

    steel.start().await;

    log::info!("Waiting for pending tasks...");
    for world in &server.worlds {
        world.chunk_map.task_tracker.close();
        world.chunk_map.task_tracker.wait().await;
    }

    log::info!("Server stopped");
}
