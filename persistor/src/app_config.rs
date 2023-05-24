use config::{Config, ConfigError};
use dotenv::dotenv;
use serde::Deserialize;
use std::default::Default;

#[derive(Debug, Clone, Default, Deserialize, PartialEq, Eq)]
pub struct AppConfig {
    pub kafka_host: String,
    pub kafka_log_topic: String,
}

impl AppConfig {
    pub fn initialize() -> Result<AppConfig, ConfigError> {
        let _ = dotenv();
        env_logger::init();

        Config::builder()
            .add_source(config::Environment::default())
            .build()?
            .try_deserialize()
    }
}
