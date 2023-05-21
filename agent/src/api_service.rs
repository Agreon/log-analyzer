use async_trait::async_trait;
use common::log::Log;
use log::{debug, info};
use reqwest::{header, Client, ClientBuilder};

// TODO: Buffering + What if too many logs are incoming?
#[async_trait]
pub trait ApiService {
    async fn send_log(&self, log: Log);
}

#[derive(Debug, Clone)]
pub struct HttpApiService {
    api_url: String,
    client: Client,
}

// TODO: Support batching
// => with config value "Max time until log sent" => e.g. 1000ms intervals
// => Only send if max-size of req payload reached
// => But makes write-log implementation harder
// => Maybe also solve with channel => or just some kind of callback
#[async_trait]
impl ApiService for HttpApiService {
    // TODO: Retry mechanism?
    // => Implicit through mpsc-queueing?
    // => But the current log is lost
    // => Need to check sent timestamps
    // TODO: Return result
    async fn send_log(&self, log: Log) {
        info!(
            "Send: [{:?}] [{}] {}",
            log.time,
            log.size_in_bytes,
            String::from_utf8_lossy(&log.original_data)
        );

        self.client
            .post(format!("{}/log", self.api_url))
            .body(log.original_data)
            .send()
            .await
            .unwrap();
    }
}

impl HttpApiService {
    pub fn new(api_url: &str, auth_token: &str) -> Self {
        let mut default_headers = header::HeaderMap::new();
        let mut auth_value = header::HeaderValue::from_str(auth_token).unwrap();
        auth_value.set_sensitive(true);
        default_headers.insert(header::AUTHORIZATION, auth_value);

        let client = ClientBuilder::new()
            .default_headers(default_headers)
            .build()
            .unwrap();

        HttpApiService {
            api_url: String::from(api_url),
            client,
        }
    }
}
