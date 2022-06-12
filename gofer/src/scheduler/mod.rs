mod docker;

use crate::conf;
use crate::models::TaskRunState;
use async_trait::async_trait;
use std::collections::HashMap;
use std::io::BufRead;
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum SchedulerError {
    #[error("could not init scheduler; {0}")]
    FailedPrecondition(String),

    #[error("could not connect to scheduler; {0}")]
    Connection(String),

    #[error("container not found")]
    NoSuchContainer,

    #[error("docker image not found; {0}")]
    NoSuchImage(String),

    #[error("unexpected scheduler error occurred; {0}")]
    Unknown(String),
}

#[derive(Debug)]
pub struct Exec {
    pub shell: String,
    pub script: String,
}

#[derive(Debug)]
pub struct RegistryAuth {
    pub user: String,
    pub pass: String,
}

#[derive(Debug)]
pub struct StartContainerRequest {
    /// A unique identifier to identify the container with.
    pub name: String,
    /// The docker image repository and docker image name; tag can be included.
    pub image_name: String,
    /// Environment variables to be passed to the container.
    pub variables: HashMap<String, String>,
    /// Registry authentication details.
    pub registry_auth: Option<RegistryAuth>,
    /// Attempt to pull the container from the upstream repository even if it exists already locally.
    /// This is useful if your containers don't use proper tagging or versioning.
    pub always_pull: bool,
    /// Only needed by triggers; used to spin the container up with networking on so that Gofer can tal
    /// to it.
    pub enable_networking: bool,
    /// Replaces the container's entrypoint with a custom passed in script.
    pub exec: Option<Exec>,
}

#[derive(Debug)]
pub struct StartContainerResponse {
    /// An optional, unique way for the scheduler to identify the container. Sometimes the scheduler
    /// will not be able to use the client provided container name as a unique identifier and will
    /// return it's own identifier. In these cases the client will have to store the scheduler's id
    /// for further use.
    pub scheduler_id: Option<String>,
    /// An endpoint that only is returned for containers with networking set to on.
    pub url: Option<String>,
}

#[derive(Debug)]
pub struct StopContainerRequest {
    /// A unique identifier to identify the container with.
    pub name: String,
    /// The total time the scheduler should wait for a graceful stop before issuing a SIGKILL.
    pub timeout: i64,
}

#[derive(Debug)]
pub struct GetStateRequest {
    /// Unique identifier for container to stop.
    pub name: String,
}

#[derive(Debug)]
pub struct GetStateResponse {
    /// In the event that the container is in a "complete" state; the exit code of that container.
    pub exit_code: Option<u8>,
    /// The current state of the container, state referencing how complete the container process of running is.
    pub state: TaskRunState,
}

#[derive(Debug)]
pub struct GetLogsRequest {
    /// Unique identifier for container to stop.
    pub name: String,
}

#[async_trait]
pub trait Scheduler {
    async fn start_container(
        &self,
        req: StartContainerRequest,
    ) -> Result<StartContainerResponse, SchedulerError>;
    async fn stop_container(&self, req: StopContainerRequest) -> Result<(), SchedulerError>;
    async fn get_state(&self, req: GetStateRequest) -> Result<GetStateResponse, SchedulerError>;
    async fn get_logs(&self, req: GetLogsRequest) -> Result<Box<dyn BufRead>, SchedulerError>;
}

pub enum SchedulerEngine {
    Docker,
}

pub async fn init_scheduler(
    engine: SchedulerEngine,
    config: conf::api::Config,
) -> Result<Box<dyn Scheduler>, SchedulerError> {
    #[allow(clippy::match_single_binding)]
    match engine {
        SchedulerEngine::Docker => {
            if let Some(config) = config.scheduler.docker {
                let engine = docker::Engine::new(config.prune, config.prune_interval).await?;
                Ok(Box::new(engine))
            } else {
                Err(SchedulerError::FailedPrecondition(
                    "docker engine settings not found in config".into(),
                ))
            }
        }
    }
}
