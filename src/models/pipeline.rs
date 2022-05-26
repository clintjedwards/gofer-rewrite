use crate::models::{
    epoch, validate_id, ModelError, PipelineNotifierSettings, PipelineTriggerSettings, Task,
};
use std::collections::HashMap;
use std::str::FromStr;

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

impl FromStr for PipelineState {
    type Err = ModelError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.to_lowercase().as_str() {
            "unknown" => Ok(PipelineState::Unknown),
            "active" => Ok(PipelineState::Active),
            "disabled" => Ok(PipelineState::Disabled),
            _ => Err(ModelError::EnumMismatch(input.to_string())),
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
    /// Controls how many runs can be active at any single time.
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

// #[derive(Debug)]
// pub struct NewPipeline {
//     /// Unique identifier for the namespace that this pipeline belongs to.
//     pub namespace: String,
//     /// Unique user defined identifier.
//     pub id: String,
//     /// Humanized name, meant for display.
//     pub name: String,
//     /// Short description of what the pipeline is used for.
//     pub description: String,
//     /// Controls how many runs can be active at any single time.
//     pub parallelism: u8,
//     /// A mapping of pipeline owned tasks.
//     pub tasks: HashMap<String, Task>,
//     /// A mapping of pipeline owned triggers to their settings.
//     pub triggers: HashMap<String, PipelineTriggerSettings>,
//     /// A mapping of pipeline owned notifiers to their settings.
//     pub notifiers: HashMap<String, PipelineNotifierSettings>,
// }

// impl Pipeline {
//     pub fn new(settings: &NewPipeline) -> Result<Self, ModelError> {
//         validate_id(&settings.id)?;

//         Ok(Pipeline {
//             namespace: settings.namespace.clone(),
//             id: settings.id.clone(),
//             name: settings.name.clone(),
//             description: settings.description.clone(),
//             last_run_id: 0,
//             last_run_time: 0,
//             parallelism: settings.parallelism,
//             created: epoch(),
//             modified: epoch(),
//             state: PipelineState::Active,
//             tasks: settings.tasks.clone(),
//             triggers: settings.triggers.clone(),
//             notifiers: settings.notifiers.clone(),
//             store_keys: vec![],
//         })
//     }
// }
