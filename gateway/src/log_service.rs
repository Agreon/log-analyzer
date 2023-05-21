use actix_web::{
    error::{self},
    post,
    web::{self},
    HttpResponse,
};
use bytes::{Buf, Bytes};
use common::log::Log;
use rdkafka::{producer::FutureRecord, util::Timeout};

use crate::{app_config::AppConfig, AppData};

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
) -> actix_web::Result<HttpResponse> {
    check_auth_header(&req, &data.app_config)?;

    let log = Log::from_bytes(&req_body).map_err(|err| error::ErrorBadRequest(err.message))?;

    match data.kafka_client.lock() {
        // TODO: Log real errors in middleware + sentry
        Err(_err) => Err(error::ErrorInternalServerError("Internal Server Error")),
        Ok(client) => {
            let record: FutureRecord<String, [u8]> =
                FutureRecord::to(&data.app_config.kafka_log_topic)
                    .payload(log.original_data.chunk());

            match client.send(record, Timeout::Never).await {
                Err(err) => {
                    println!("{:?}", err);
                    Err(error::ErrorInternalServerError("Internal Server Error"))
                }
                Ok(_) => Ok(HttpResponse::Ok().finish()),
            }
        }
    }
}
