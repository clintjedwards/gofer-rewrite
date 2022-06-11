use crate::scheduler::SchedulerError;
use slog_scope::debug;

fn format_env_var(key: &str, value: &str) -> String {
    return format!("{}={}", key, value);
}

pub struct Engine {
    client: bollard::Docker,
}

impl Engine {
    pub async fn new() -> Result<Self, SchedulerError> {
        let client = bollard::Docker::connect_with_socket_defaults().map_err(|e| {
            SchedulerError::Connection(format!(
                "{}; Make sure the Docker daemon is installed and running.",
                e
            ))
        })?;

        // Check that we can actually get a connection.
        let version = client.version().await.map_err(|e| {
            SchedulerError::Connection(format!(
                "{}; Make sure the Docker daemon is installed and running.",
                e
            ))
        })?;

        debug!("local docker scheduler successfully connected"; "version" => format!("{:?}", version));

        Ok(Self { client })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn hello() {
        Engine::new().await.unwrap();
    }
}
