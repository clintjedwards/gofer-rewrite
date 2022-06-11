mod docker;

use crate::models::TaskRunState;
use std::collections::HashMap;
use std::io::BufRead;
use std::time::Duration;
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum SchedulerError {
    #[error("could not connect to scheduler; {0}")]
    Connection(String),

    #[error("container not found")]
    NoSuchContainer,

    #[error("docker image not found")]
    NoSuchImage,

    #[error("unexpected scheduler error occurred; {0}")]
    Unknown(String),
}

pub struct Exec {
    pub shell: String,
    pub script: String,
}

pub struct StartContainerRequest {
    /// A unique identifier passed by the caller to name the container; this usually maps back to a scheduler
    /// specific unique identifier, returned when the container starts.
    pub name: String,
    /// The docker image repository and docker image name; tag can be included.
    pub image_name: String,
    /// Environment variables to be passed to the container.
    pub variables: HashMap<String, String>,
    /// Username for docker auth registry.
    pub registry_user: String,
    /// Password for docker auth registry.
    pub registry_pass: String,
    /// Attempt to pull the container from the upstream repository even if it exists already locally.
    /// This is useful if your containers don't use proper tagging or versioning.
    pub always_pull: bool,
    /// Only needed by triggers; used to spin the container up with networking on so that Gofer can tal
    /// to it.
    pub enable_networking: bool,
    /// Replaces the container's entrypoint with a custom passed in script.
    pub exec: Option<Exec>,
}

pub struct StartContainerResponse {
    /// A unique way for the scheduler to identify the container. Is only obtained upon the container successfully starting.
    pub scheduler_id: String,
    /// An endpoint that only is returned for containers with networking set to on.
    pub url: Option<String>,
}

pub struct StopContainerRequest {
    /// Unique identifier for container to stop.
    pub scheduler_id: String,
    /// The total time the scheduler should wait for a graceful stop before issuing a SIGKILL.
    pub timeout: Duration,
}

pub struct GetStateRequest {
    /// Unique identifier for container to stop.
    pub scheduler_id: String,
}

pub struct GetStateResponse {
    /// In the event that the container is in a "complete" state; the exit code of that container.
    pub exit_code: Option<u8>,
    /// The current state of the container, state referencing how complete the container process of running is.
    pub state: TaskRunState,
}

pub struct GetLogsRequest {
    /// Unique identifier for container to stop.
    pub scheduler_id: String,
}

pub trait Scheduler {
    fn start_container(
        req: StartContainerRequest,
    ) -> Result<StartContainerResponse, SchedulerError>;
    fn stop_container(req: StopContainerRequest) -> Result<(), SchedulerError>;
    fn get_state(req: GetStateRequest) -> Result<GetStateResponse, SchedulerError>;
    fn get_logs(req: GetLogsRequest) -> Result<Box<dyn BufRead>, SchedulerError>;
}
