use std::{collections::HashMap, time::Duration};

use bollard::{
    container::{ListContainersOptions, LogOutput, LogsOptions},
    errors::Error,
    Docker,
};
use common::log::Log;
use futures_core::Stream;
use futures_util::pin_mut;
use futures_util::StreamExt;
use log::{debug, info};
use tokio::{sync::mpsc::Sender, task::JoinHandle, time};

use crate::{app_config::AppConfig, container::Container};

pub struct ContainerLogExtractor {
    docker: Docker,
    log_handle_tasks: HashMap<String, JoinHandle<()>>,
    container_poll_interval: time::Interval,
    container_selector: String,
    latest_log_sent_timestamp: i64,
}

impl ContainerLogExtractor {
    pub fn new(
        docker: Docker,
        app_config: &AppConfig,
        latest_log_sent_timestamp: i64,
    ) -> ContainerLogExtractor {
        ContainerLogExtractor {
            docker,
            log_handle_tasks: HashMap::new(),
            container_poll_interval: time::interval(Duration::from_secs(
                app_config.docker_container_poll_interval_in_seconds,
            )),
            container_selector: app_config.container_selector.clone(),
            latest_log_sent_timestamp,
        }
    }

    pub async fn start_polling(&mut self, log_channel_sender: &Sender<Log>) -> Result<(), Error> {
        loop {
            self.container_poll_interval.tick().await;

            let running_containers = self
                .docker
                .list_containers(None::<ListContainersOptions<String>>)
                .await?;

            let relevant_containers = running_containers
                .iter()
                .filter_map(Container::from_summary)
                .filter(|container| container.name.starts_with(&self.container_selector));

            let mut containers_to_start = vec![];
            for container in relevant_containers {
                match self.log_handle_tasks.get(&container.name) {
                    None => containers_to_start.push(container),
                    Some(handle) => {
                        if handle.is_finished() {
                            debug!("Remove finished handle for {}", container.name);
                            self.log_handle_tasks.remove(&container.name);
                            containers_to_start.push(container);
                        }
                    }
                }
            }

            let created_log_streams = containers_to_start.iter().map(|container| {
                (
                    container.name.clone(),
                    self.docker.logs(
                        &container.name,
                        Some(LogsOptions::<String> {
                            follow: true,
                            stdout: true,
                            stderr: true,
                            // Currently the command doesn't support a higher resolution.
                            since: self.latest_log_sent_timestamp / 1000,
                            ..Default::default()
                        }),
                    ),
                )
            });

            for (container, stream) in created_log_streams {
                debug!("Capturing log stream of {}", container);
                let log_channel_sender = log_channel_sender.clone();
                let handle = tokio::spawn(async move {
                    ContainerLogExtractor::handle_container_logs(stream, &log_channel_sender).await;
                });
                self.log_handle_tasks.insert(container, handle);
            }
        }
    }

    async fn handle_container_logs(
        stream: impl Stream<Item = Result<LogOutput, Error>>,
        tx: &Sender<Log>,
    ) {
        pin_mut!(stream);

        while let Some(res) = stream.next().await {
            match res {
                Err(err) => panic!("{}", err),
                Ok(log_output) => match Log::from_bytes(&log_output.into_bytes()) {
                    Ok(log) => tx.send(log).await.unwrap(),
                    Err(error) => info!("Warn: No valid log found ({})", error),
                },
            }
        }
    }
}
