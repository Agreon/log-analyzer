use actix_web::{error, Error, FromRequest};
use config::{Config, ConfigError};
use dotenv::dotenv;
use futures_util::future::{err, ok, Ready};
use serde::Deserialize;
use std::default::Default;

use crate::AppData;

// TODO: Wrap in Arc
#[derive(Debug, Clone, Default, Deserialize, PartialEq, Eq)]
pub struct AppConfig {
    pub server_port: u16,
    pub api_token: String,
    pub kafka_host: String,
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

impl FromRequest for AppConfig {
    type Error = Error;
    type Future = Ready<Result<Self, Error>>;

    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        if let Some(state) = req.app_data::<AppData>() {
            ok(state.app_config.clone())
        } else {
            err(error::ErrorInternalServerError(
                "Requested application data is not configured correctly. \
                View/enable debug logs for more details.",
            ))
        }
    }
}
