use crate::scheduler::{Scheduler, SchedulerError};
use slog_scope::{debug, error};
use std::sync::Arc;
use std::time::Duration;

fn format_env_var(key: &str, value: &str) -> String {
    return format!("{}={}", key, value);
}

pub struct Engine {
    client: Arc<bollard::Docker>,
}

impl Engine {
    pub async fn new(prune: bool, prune_interval: Duration) -> Result<Self, SchedulerError> {
        let client = bollard::Docker::connect_with_socket_defaults().map_err(|e| {
            SchedulerError::Connection(format!(
                "{}; Make sure the Docker daemon is installed and running.",
                e
            ))
        })?;
        let client = Arc::new(client);
        let prune_client = Arc::clone(&client);

        // Check that we can actually get a connection.
        let version = client.version().await.map_err(|e| {
            SchedulerError::Connection(format!(
                "{}; Make sure the Docker daemon is installed and running.",
                e
            ))
        })?;

        // We periodically need to clean up docker assets so we don't run out of disk space.
        // We perform it very infrequently though, in order to give operators time to diagnose
        // any potential issues they might be having with a particular container.
        if prune {
            tokio::spawn(async move {
                match prune_client.prune_containers::<String>(None).await {
                    Ok(response) => {
                        debug!("pruned containers";
                               "containers_deleted" => format!("{:?}", response.containers_deleted),
                               "space_reclaimed" => response.space_reclaimed);
                    }
                    Err(e) => {
                        error!("could not successfully prune containers"; "error" => e.to_string())
                    }
                };

                tokio::time::sleep(prune_interval).await;
            });
        }

        debug!("local docker scheduler successfully connected"; "version" => format!("{:?}", version));

        Ok(Self { client })
    }
}

impl Scheduler for Engine {
    fn start_container(
        &self,
        req: super::StartContainerRequest,
    ) -> Result<super::StartContainerResponse, SchedulerError> {
        unimplemented!()
    }
    fn stop_container(&self, req: super::StopContainerRequest) -> Result<(), SchedulerError> {
        unimplemented!()
    }
    fn get_logs(
        &self,
        req: super::GetLogsRequest,
    ) -> Result<Box<dyn std::io::BufRead>, SchedulerError> {
        unimplemented!()
    }
    fn get_state(
        &self,
        req: super::GetStateRequest,
    ) -> Result<super::GetStateResponse, SchedulerError> {
        unimplemented!()
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[tokio::test]
//     async fn hello() {
//         Engine::new().await.unwrap();
//     }
// }
