use log::LevelFilter;
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode};
use steel::SteelServer;
use steel_utils::ResourceLocation;

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

    log::info!(
        "{:?}",
        server
            .server
            .registry
            .items
            .get_tag(&ResourceLocation::vanilla_static("swords"))
            .unwrap()
            .iter()
            .map(|b| b.key.path.to_string())
            .collect::<Vec<String>>()
    );

    server.start().await;
}
