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

    let mut server = SteelServer::new().await;

    log::info!(
        "{:?}",
        server
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

    server.start().await;
}
