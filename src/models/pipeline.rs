use crate::models::{epoch, Task};
use crate::proto;
use std::collections::HashMap;

/// The current state of the pipeline. Pipelines can be disabled to stop execution.
#[derive(Debug)]
pub enum PipelineState {
    /// The state of the pipeline is unknown. This should never happen.
    #[allow(dead_code)]
    Unknown,
    /// Pipeline is enabled and able to start runs.
    Active,
    /// Pipeline is disabled and not able to start runs. Any triggers will be removed and any manually started runs
    /// will automatically fail.
    Disabled,
}

impl From<proto::PipelineState> for PipelineState {
    fn from(p: proto::PipelineState) -> Self {
        match p {
            proto::PipelineState::Unknown => PipelineState::Unknown,
            proto::PipelineState::Active => PipelineState::Active,
            proto::PipelineState::Disabled => PipelineState::Disabled,
        }
    }
}

/// A collection of logically grouped tasks. A task is a unit of work wrapped in a docker container.
/// Pipeline is a secondary level unit being contained within namespaces and containing tasks.
#[derive(Debug)]
pub struct Pipeline {
    /// Unique identifier for the namespace that this pipeline belongs to.
    pub namespace: String,
    /// Unique user defined identifier.
    pub id: String,
    /// Humanized name, meant for display.
    pub name: String,
    /// Short description of what the pipeline is used for.
    pub description: String,
    /// The identifier for the last run that the pipeline executed.
    pub last_run_id: u64,
    /// The time in epoch milli that the last run started. 0 indicates that this was never run.
    pub last_run_time: u64,
    /// Controls how many runs can be active at any single time. 0 indicates unbounded with respect to bounds
    /// enforced by Gofer.
    pub parallelism: u64,
    /// The creation time in epoch milli.
    pub created: u64,
    /// The last modified time in epoch milli. Only updates on changes to the pipeline attributes, not tangential
    /// things like last run time.
    pub modified: u64,
    /// The current state of the pipeline. Pipelines can be disabled to stop execution of runs/tasks.
    pub state: PipelineState,
    /// A mapping of pipeline owned tasks.
    pub tasks: HashMap<String, Task>,
    /// A mapping of pipeline owned triggers to their settings.
    pub triggers: HashMap<String, PipelineTriggerSettings>,
    /// A mapping of pipeline owned notifiers to their settings.
    pub notifiers: HashMap<String, PipelineNotifierSettings>,
    /// A listing pipeline owned keys that are stored in Gofer's object store.
    pub store_keys: Vec<String>,
}

impl Pipeline {
    pub fn new(namespace: String, config: PipelineConfig) -> Self {
        Pipeline {
            namespace,
            id: config.id,
            name: config.name,
            description: config.description,
            last_run_id: 0,
            last_run_time: 0,
            parallelism: config.parallelism,
            created: epoch(),
            modified: epoch(),
            state: PipelineState::Active,
            tasks: config.tasks,
            triggers: config.triggers,
            notifiers: config.notifiers,
            store_keys: vec![],
        }
    }
}

#[derive(Debug)]
pub struct PipelineConfig {
    /// Unique user defined identifier.
    pub id: String,
    /// Humanized name, meant for display.
    pub name: String,
    /// Short description of what the pipeline is used for.
    pub description: String,
    /// Controls how many runs can be active at any single time.
    pub parallelism: u64,
    /// A mapping of pipeline owned tasks.
    pub tasks: HashMap<String, Task>,
    /// A mapping of pipeline owned triggers to their settings.
    pub triggers: HashMap<String, PipelineTriggerSettings>,
    /// A mapping of pipeline owned notifiers to their settings.
    pub notifiers: HashMap<String, PipelineNotifierSettings>,
}

impl From<proto::PipelineConfig> for PipelineConfig {
    fn from(ns: proto::PipelineConfig) -> Self {
        PipelineConfig {
            id: ns.id,
            name: ns.name,
            description: ns.description,
            parallelism: ns.parallelism,
            tasks: ns
                .tasks
                .into_iter()
                .map(|(key, value)| (key, value.into()))
                .collect(),
            triggers: ns
                .triggers
                .into_iter()
                .map(|(key, value)| (key, value.into()))
                .collect(),
            notifiers: ns
                .notifiers
                .into_iter()
                .map(|(key, value)| (key, value.into()))
                .collect(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PipelineTriggerSettings {}

impl From<proto::PipelineTriggerSettings> for PipelineTriggerSettings {
    fn from(ns: proto::PipelineTriggerSettings) -> Self {
        PipelineTriggerSettings {}
    }
}

#[derive(Debug, Clone)]
pub struct PipelineNotifierSettings {}

impl From<proto::PipelineNotifierSettings> for PipelineNotifierSettings {
    fn from(ns: proto::PipelineNotifierSettings) -> Self {
        PipelineNotifierSettings {}
    }
}
