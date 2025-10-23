use serde::Deserialize;
use std::{fs, net::SocketAddr, path::Path, sync::LazyLock};

const DEFAULT_CONFIG_STR: &str = include_str!("default_config.json5");

#[derive(Debug, Clone, Deserialize)]
pub struct SteelConfig {
    pub server_address: SocketAddr,
    pub seed: String,
    pub max_players: u32,
    pub view_distance: u8,
    pub simulation_distance: u8,
    pub online_mode: bool,
    pub encryption: bool,
    pub motd: String,
    pub use_favicon: bool,
    pub favicon: String,
}

impl SteelConfig {
    pub fn load_or_create(path: &Path) -> Self {
        if path.exists() {
            let config_str = fs::read_to_string(path).unwrap();
            let config: SteelConfig = serde_json5::from_str(config_str.as_str()).unwrap();
            config.validate().unwrap();
            config
        } else {
            fs::create_dir_all(path.parent().unwrap()).unwrap();
            fs::write(path, DEFAULT_CONFIG_STR).unwrap();
            let config: SteelConfig = serde_json5::from_str(DEFAULT_CONFIG_STR).unwrap();
            config.validate().unwrap();
            config
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.server_address.is_ipv4() {
            return Err("Server address must be a valid IPv4 address".to_string());
        }
        if self.view_distance == 0 {
            return Err("View distance must be greater than 0".to_string());
        }
        if self.view_distance > 32 {
            return Err("View distance must be less than or equal to 32".to_string());
        }
        if self.simulation_distance == 0 {
            return Err("Simulation distance must be greater than 0".to_string());
        }
        if self.simulation_distance > 32 {
            return Err("Simulation distance must be less than or equal to 32".to_string());
        }
        Ok(())
    }
}
