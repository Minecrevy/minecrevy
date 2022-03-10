use std::{env, fs, io};
use std::path::Path;

use bevy::app::{App, Plugin};
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct Config {
    pub view_distance: u32,
    pub network: NetworkConfig,
}

impl Config {
    pub fn load<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        Ok(toml::from_str::<Self>(&fs::read_to_string(path)?)?)
    }
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct NetworkConfig {
    pub address: String,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            address: "127.0.0.1:25565".to_owned(),
        }
    }
}

#[derive(Default)]
pub struct ConfigPlugin;

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        const MINECREVY_CONFIG_PATH: &str = "MINECREVY_CONFIG_PATH";
        const DEFAULT_CONFIG_PATH: &str = "config.toml";

        let env_config_path = env::var(MINECREVY_CONFIG_PATH);
        let config_path = env_config_path.as_ref()
            .map(|p| p.as_str())
            .unwrap_or(DEFAULT_CONFIG_PATH);

        let config = match Config::load(config_path) {
            Ok(config) => config,
            Err(e) if e.kind() == io::ErrorKind::NotFound => {
                let config = Config::default();

                let config_str = toml::to_string_pretty(&config)
                    .expect("failed to serialize default config");
                fs::write(config_path, config_str)
                    .expect("Failed to save default config");

                config
            }
            Err(e) => {
                tracing::error!("failed to load config, using default: {}", e);

                Config::default()
            }
        };

        app.insert_resource(config);
    }
}
