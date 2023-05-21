use api_service::{ApiService, HttpApiService};
use app_config::AppConfig;
use bollard::Docker;
use common::log::Log;
use container_log_extractor::ContainerLogExtractor;
use tokio::fs::File;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::sync::mpsc;

mod api_service;
mod app_config;
mod container;
mod container_log_extractor;

#[macro_use]
extern crate log;

static LATEST_LOG_SENT_TIMESTAMP_FILE: &str = "latest_log_sent_timestamp";

async fn persist_latest_log_sent(time: u64) -> io::Result<()> {
    let mut file = File::create(LATEST_LOG_SENT_TIMESTAMP_FILE).await?;

    file.write_all(&time.to_ne_bytes()).await?;
    file.flush().await?;

    Ok(())
}

async fn retrieve_latest_log_sent_timestamp() -> io::Result<i64> {
    let file = File::open(LATEST_LOG_SENT_TIMESTAMP_FILE).await;

    if let Ok(mut file) = file {
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).await?;

        return Ok(i64::from_ne_bytes(buffer.try_into().unwrap()));
    }

    Ok(0)
}

// TODO: Test wether we are allowed to write log-timestamp file
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let app_config = AppConfig::initialize()?;
    let api_service = HttpApiService::new(&app_config.api_url, &app_config.api_token);

    let latest_log_sent_timestamp = retrieve_latest_log_sent_timestamp().await?;

    let docker = Docker::connect_with_local_defaults()?;

    let (log_channel_sender, mut log_channel_receiver) =
        mpsc::channel::<Log>(app_config.message_buffer_size);

    tokio::spawn(async move {
        while let Some(log) = log_channel_receiver.recv().await {
            let time = log.time;

            api_service.send_log(log).await;

            persist_latest_log_sent(time).await.unwrap();
        }
    });

    ContainerLogExtractor::new(docker, &app_config, latest_log_sent_timestamp)
        .start_polling(&log_channel_sender)
        .await?;

    Ok(())
}
