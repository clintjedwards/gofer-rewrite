use crate::models::{epoch, Task};
use crate::proto;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use strum::{Display, EnumString};

/// The current state of the pipeline. Pipelines can be disabled to stop execution.
#[derive(Debug, Display, EnumString, Serialize, Deserialize)]
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

impl From<proto::pipeline::PipelineState> for PipelineState {
    fn from(p: proto::pipeline::PipelineState) -> Self {
        match p {
            proto::pipeline::PipelineState::Unknown => PipelineState::Unknown,
            proto::pipeline::PipelineState::Active => PipelineState::Active,
            proto::pipeline::PipelineState::Disabled => PipelineState::Disabled,
        }
    }
}

impl From<PipelineState> for proto::pipeline::PipelineState {
    fn from(p: PipelineState) -> Self {
        match p {
            PipelineState::Unknown => proto::pipeline::PipelineState::Unknown,
            PipelineState::Active => proto::pipeline::PipelineState::Active,
            PipelineState::Disabled => proto::pipeline::PipelineState::Disabled,
        }
    }
}

/// A collection of logically grouped tasks. A task is a unit of work wrapped in a docker container.
/// Pipeline is a secondary level unit being contained within namespaces and containing runs.
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

impl From<Pipeline> for proto::Pipeline {
    fn from(p: Pipeline) -> Self {
        proto::Pipeline {
            namespace: p.namespace,
            id: p.id,
            name: p.name,
            description: p.description,
            last_run_id: p.last_run_id,
            last_run_time: p.last_run_time,
            parallelism: p.parallelism,
            created: p.created,
            modified: p.modified,
            state: proto::pipeline::PipelineState::from(p.state) as i32,
            tasks: p
                .tasks
                .into_iter()
                .map(|(key, value)| (key, value.into()))
                .collect(),
            triggers: p
                .triggers
                .into_iter()
                .map(|(key, value)| (key, value.into()))
                .collect(),
            notifiers: p
                .notifiers
                .into_iter()
                .map(|(key, value)| (key, value.into()))
                .collect(),
            store_keys: p.store_keys,
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

/// Every time a pipeline attempts to subscribe to a trigger, it passes certain
/// values back to that trigger for certain functionality. Since triggers keep no
/// permanent state, these settings are kept here so that when triggers are restarted
/// they can be restored with proper settings.
#[derive(Debug)]
pub struct PipelineTriggerSettings {
    /// A global unique identifier for the trigger type.
    pub kind: String,
    /// A user defined identifier for the trigger so that a pipeline with
    /// multiple notifiers can be differentiated.
    pub label: String,
    /// The settings for pertaining to that specific trigger.
    pub settings: HashMap<String, String>,
    /// If the trigger could not be set up for the pipeline we return an error on why that might be.
    pub error: Option<String>,
}

impl From<proto::PipelineTriggerSettings> for PipelineTriggerSettings {
    fn from(p: proto::PipelineTriggerSettings) -> Self {
        PipelineTriggerSettings {
            kind: p.kind,
            label: p.label,
            settings: p.settings,
            error: {
                if p.error.is_empty() {
                    None
                } else {
                    Some(p.error)
                }
            },
        }
    }
}

impl From<PipelineTriggerSettings> for proto::PipelineTriggerSettings {
    fn from(p: PipelineTriggerSettings) -> Self {
        proto::PipelineTriggerSettings {
            kind: p.kind,
            label: p.label,
            settings: p.settings,
            error: match p.error {
                Some(error) => error,
                None => "".to_string(),
            },
        }
    }
}

#[derive(Debug)]
pub struct PipelineNotifierSettings {
    /// A global unique identifier for the notifier type.
    pub kind: String,
    /// A user defined identifier for the notifier so that a pipeline with
    /// multiple notifiers can be differentiated.
    pub label: String,
    /// The settings for pertaining to that specific notifier.
    pub settings: HashMap<String, String>,
    /// If the notifier could not be set up for the pipeline we return an error on why that might be.
    pub error: Option<String>,
}

impl From<proto::PipelineNotifierSettings> for PipelineNotifierSettings {
    fn from(p: proto::PipelineNotifierSettings) -> Self {
        PipelineNotifierSettings {
            kind: p.kind,
            label: p.label,
            settings: p.settings,
            error: {
                if p.error.is_empty() {
                    None
                } else {
                    Some(p.error)
                }
            },
        }
    }
}

impl From<PipelineNotifierSettings> for proto::PipelineNotifierSettings {
    fn from(p: PipelineNotifierSettings) -> Self {
        proto::PipelineNotifierSettings {
            kind: p.kind,
            label: p.label,
            settings: p.settings,
            error: match p.error {
                Some(error) => error,
                None => "".to_string(),
            },
        }
    }
}