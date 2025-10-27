use log::LevelFilter;
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode};
use steel::SteelServer;

#[tokio::main]
async fn main() {
    TermLogger::init(
        LevelFilter::Info,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )
    .unwrap();

    let mut server = SteelServer::new().await;

    println!(
        "{:?}",
        steel_registry::MINECRAFT_CORE_DIR
            .get_file("minecraft/pack.mcmeta")
            .unwrap()
            .contents_utf8()
    );

    server.start().await;
}
