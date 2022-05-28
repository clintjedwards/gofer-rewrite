use crate::models::Variable;
use crate::proto;

use std::str::FromStr;

/// Represents the current state of the run as it progresses through the steps
/// involved to completion.
#[derive(Debug)]
pub enum RunState {
    /// Could not determine current state of the run. Should never happen.
    Unknown,
    /// Before a run is sent to a scheduler it must complete various steps like
    /// validation checking, this state represents that step.
    Pending,
    /// The run is currently being executed on the scheduler.
    Running,
    /// All tasks have been resolved and the run is no longer being executed.
    Complete,
}

impl From<proto::run::RunState> for RunState {
    fn from(r: proto::run::RunState) -> Self {
        match r {
            proto::run::RunState::Unknown => RunState::Unknown,
            proto::run::RunState::Pending => RunState::Pending,
            proto::run::RunState::Running => RunState::Running,
            proto::run::RunState::Complete => RunState::Complete,
        }
    }
}

impl FromStr for RunState {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.to_lowercase().as_str() {
            "unknown" => Ok(RunState::Unknown),
            "pending" => Ok(RunState::Pending),
            "running" => Ok(RunState::Running),
            "complete" => Ok(RunState::Complete),
            _ => Err(()),
        }
    }
}

/// Represents the current status of a completed run.
#[derive(Debug)]
pub enum RunStatus {
    /// Could not determine current state of the status. Should only be in this state if
    /// the run has not yet completed.
    Unknown,
    /// All tasks in run have completed with a non-failure state.
    Successful,
    /// One or more tasks in run have failed.
    Failed,
    /// One or more tasks in a run have been cancelled.
    Cancelled,
}

impl From<proto::run::RunStatus> for RunStatus {
    fn from(r: proto::run::RunStatus) -> Self {
        match r {
            proto::run::RunStatus::Unknown => RunStatus::Unknown,
            proto::run::RunStatus::Successful => RunStatus::Successful,
            proto::run::RunStatus::Failed => RunStatus::Failed,
            proto::run::RunStatus::Cancelled => RunStatus::Cancelled,
        }
    }
}

impl FromStr for RunStatus {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.to_lowercase().as_str() {
            "unknown" => Ok(RunStatus::Unknown),
            "successful" => Ok(RunStatus::Successful),
            "failed" => Ok(RunStatus::Failed),
            "cancelled" => Ok(RunStatus::Cancelled),
            _ => Err(()),
        }
    }
}

/// Explains in more detail why a particular run might have failed.
#[derive(Debug)]
pub enum RunFailureReason {
    /// Could not determine failure reason for current run. Should never happen.
    Unknown,
    /// While executing the run one or more tasks exited with an abnormal exit code.
    AbnormalExit,
    /// While executing the run one or more tasks could not be scheduled.
    SchedulerError,
    /// The run could not be executed as requested due to user defined attributes given.
    FailedPrecondition,
    /// One or more tasks could not be completed due to a user cancelling the run.
    UserCancelled,
    /// One or more tasks could not be completed due to the system or admin cancelling the run.
    AdminCancelled,
}

impl From<proto::run_failure_info::RunFailureReason> for RunFailureReason {
    fn from(r: proto::run_failure_info::RunFailureReason) -> Self {
        match r {
            proto::run_failure_info::RunFailureReason::Unknown => RunFailureReason::Unknown,
            proto::run_failure_info::RunFailureReason::AbnormalExit => {
                RunFailureReason::AbnormalExit
            }
            proto::run_failure_info::RunFailureReason::SchedulerError => {
                RunFailureReason::SchedulerError
            }
            proto::run_failure_info::RunFailureReason::FailedPrecondition => {
                RunFailureReason::FailedPrecondition
            }
            proto::run_failure_info::RunFailureReason::UserCancelled => {
                RunFailureReason::UserCancelled
            }
            proto::run_failure_info::RunFailureReason::AdminCancelled => {
                RunFailureReason::AdminCancelled
            }
        }
    }
}

impl FromStr for RunFailureReason {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.to_lowercase().as_str() {
            "unknown" => Ok(RunFailureReason::Unknown),
            "abnormal_exit" => Ok(RunFailureReason::AbnormalExit),
            "scheduler_error" => Ok(RunFailureReason::SchedulerError),
            "failed_precondition" => Ok(RunFailureReason::FailedPrecondition),
            "user_cancelled" => Ok(RunFailureReason::UserCancelled),
            "admin_cancelled" => Ok(RunFailureReason::AdminCancelled),
            _ => Err(()),
        }
    }
}

/// Information about a run's failure. Does not get populated before a run is finished.
#[derive(Debug)]
pub struct RunFailureInfo {
    /// Why the run might have failed.
    pub reason: RunFailureReason,
    /// A more exact description on what happened.
    pub description: String,
}

/// Information about which trigger was responsible for the run's execution.
#[derive(Debug)]
pub struct RunTriggerInfo {
    /// The trigger kind responsible for starting the run.
    pub kind: String,
    /// The trigger label responsible for starting the run. The label is a user chosen name
    /// for the trigger to differentiate it from other pipeline triggers of the same kind.
    pub label: String,
}

/// Information about the run's store keys as they pertain to Gofer's object store.
#[derive(Debug)]
pub struct RunStoreInfo {
    /// After a certain number of runs Gofer's run objects are removed.
    pub is_expired: bool,
    /// They keys specific to this run.
    pub keys: Vec<String>,
}

/// A run is one or more tasks being executed on behalf of some trigger.
/// Run is a third level unit containing tasks and being contained in a pipeline.
#[derive(Debug)]
pub struct Run {
    /// Identifier for the namespace that this run belongs to.
    pub namespace: String,
    /// Identifier for the pipeline that this run belongs to.
    pub pipeline: String,
    /// Unique numeric auto-incrementing identifier.
    pub id: u64,
    /// Time run started in epoch milli.
    pub started: u64,
    /// Time run ended in epoch milli.
    pub ended: u64,
    /// Used to describe the current stage in the process of the run.
    pub state: RunState,
    /// Used to describe the final outcome of the run (success/fail).
    pub status: RunStatus,
    /// On a failed run, contains more information about the run's status.
    pub failure_info: Option<RunFailureInfo>,
    /// The unique identifier for each task run.
    pub task_runs: Vec<String>,
    /// Information about which trigger was responsible for the run's execution.
    pub trigger: RunTriggerInfo,
    /// Environment variables to be injected into each child task run. These are usually injected by the trigger.
    pub variables: Vec<Variable>,
    /// Information about the object keys that were stored in Gofer's run object store for this run.
    pub store_info: Option<RunStoreInfo>,
}
