use anyhow::anyhow;
use anyhow::Result;
use app_config::AppConfig;
use common::log::{Log, ParseLogErr};
use futures_util::{StreamExt, TryStreamExt};
use rdkafka::error::KafkaResult;
use rdkafka::message::{Headers, Message};
use rdkafka::{
    consumer::{CommitMode, Consumer, StreamConsumer},
    message::BorrowedMessage,
    ClientConfig,
};

mod app_config;

async fn handle_message(consumer: &StreamConsumer, message: &BorrowedMessage<'_>) -> Result<()> {
    let log = message
        .payload()
        .map(Log::try_from)
        .ok_or(anyhow!("No payload received"))??;

    // TODO: Write log to fs

    consumer.commit_message(message, CommitMode::Sync)?;

    Ok(())
}

#[tokio::main]
async fn main() {
    let app_config = AppConfig::initialize().unwrap();

    let consumer: StreamConsumer = ClientConfig::new()
        .set("bootstrap.servers", &app_config.kafka_host)
        .set("enable.auto.commit", "false")
        // Fail fast
        .set("request.timeout.ms", "3000")
        // .set_log_level(RDKafkaLogLevel::Debug)
        .create()
        .expect("Consumer creation failed");

    consumer
        .subscribe(&[&app_config.kafka_log_topic])
        .expect("Can't subscribe to specified topics");

    loop {
        match consumer.recv().await {
            Err(err) => panic!("{}", err),
            Ok(message) => {
                if let Err(err) = handle_message(&consumer, &message).await {
                    println!("{:?}", err);
                }
            }
        }
    }
}
