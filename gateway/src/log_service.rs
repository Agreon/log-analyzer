use actix_web::{
    error::{self},
    post,
    web::{self},
    HttpResponse, Responder,
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
) -> impl Responder {
    check_auth_header(&req, &data.app_config)?;

    let log = Log::try_from(req_body.chunk()).map_err(error::ErrorBadRequest)?;

    match data.kafka_client.lock() {
        // TODO: Log real errors in middleware + sentry
        Err(_err) => Err(error::ErrorInternalServerError("Internal Server Error")),
        Ok(client) => {
            let record: FutureRecord<String, [u8]> =
                FutureRecord::to(&data.app_config.kafka_log_topic)
                    .payload(log.original_data.chunk());

            client
                .send(record, Timeout::Never)
                .await
                .map(|_| HttpResponse::Ok())
                .map_err(|err| {
                    println!("{:?}", err);
                    error::ErrorInternalServerError("Internal Server Error")
                })
        }
    }
}
