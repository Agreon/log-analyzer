use actix_web::{
    error::{self},
    post,
    web::{self},
};
use bytes::{Buf, Bytes};
use common::log::Log;
use rdkafka::{producer::FutureRecord, util::Timeout};

use crate::{app_config::AppConfig, AppData};

// TODO: We need some global error handler for internalServerError
fn check_auth_header(
    req: &actix_web::HttpRequest,
    app_config: &AppConfig,
) -> Result<(), actix_web::Error> {
    if let Some(header) = req.headers().get("Authorization") {
        if header.eq(&app_config.api_token) {
            return Ok(());
        }

        return Err(error::ErrorUnauthorized("Wrong Authorization token"));
    }

    Err(error::ErrorUnauthorized("Missing Authorization header"))
}

#[post("/log")]
pub async fn add_log(
    req: actix_web::HttpRequest,
    req_body: Bytes,
    data: web::Data<AppData>,
) -> actix_web::Result<String> {
    check_auth_header(&req, &data.app_config)?;

    let log = Log::from_bytes(&req_body).map_err(|err| error::ErrorBadRequest(err.message))?;

    println!("{:?}", log);

    let res = data
        .kafka_client
        .lock()
        .unwrap()
        .send(
            // TODO: Topic
            FutureRecord::to("topic")
                // TODO: key
                .key("test")
                .payload(log.original_data.chunk()),
            Timeout::Never,
        )
        .await;

    println!("{:?}", res);
    Ok(String::from(""))
}
