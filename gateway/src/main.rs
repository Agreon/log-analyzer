use std::sync::Mutex;

use actix_web::{App, HttpServer};
use app_config::AppConfig;
use log_service::add_log;
use rdkafka::{producer::FutureProducer, ClientConfig};

#[macro_use]
extern crate log;

mod app_config;
mod log_service;

pub struct AppData {
    kafka_client: Mutex<FutureProducer>,
    app_config: AppConfig,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_config = AppConfig::initialize().unwrap();

    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", &app_config.kafka_host)
        .set("message.timeout.ms", "5000")
        .create()
        .unwrap();

    let state = actix_web::web::Data::new(AppData {
        app_config: app_config.clone(),
        kafka_client: Mutex::new(producer),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(actix_web::web::Data::clone(&state))
            .service(add_log)
    })
    .bind(("127.0.0.1", app_config.server_port))?
    .run()
    .await
}
