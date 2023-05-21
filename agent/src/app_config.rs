use config::{Config, ConfigError};
use dotenv::dotenv;
use serde::Deserialize;
use std::default::Default;

#[derive(Debug, Default, Deserialize, PartialEq, Eq)]
pub struct AppConfig {
    pub api_url: String,
    pub api_token: String,
    pub container_selector: String,
    pub message_buffer_size: usize,
    pub docker_container_poll_interval_in_seconds: u64,
}

impl AppConfig {
    pub fn initialize() -> Result<AppConfig, ConfigError> {
        let _ = dotenv();
        env_logger::init();

        Config::builder()
            .set_default("message_buffer_size", 32)?
            .set_default("docker_container_poll_interval_in_seconds", 2)?
            .add_source(config::Environment::default())
            .build()?
            .try_deserialize()
    }
}
