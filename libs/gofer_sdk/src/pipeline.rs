use crate::TaskConfig;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct PipelineConfig {
    /// Unique user defined identifier.
    pub id: String,
    /// Humanized name, meant for display.
    pub name: String,
    /// Short description of what the pipeline is used for.
    pub description: Option<String>,
    /// Controls how many runs can be active at any single time.
    /// 0 defaults to whatever the global Gofer setting is.
    pub parallelism: u64,
    /// A mapping of pipeline owned tasks.
    pub tasks: Vec<TaskConfig>,
    /// A mapping of pipeline owned triggers to their settings.
    pub triggers: Vec<PipelineTriggerConfig>,
    /// A mapping of pipeline owned notifiers to their settings.
    pub notifiers: Vec<PipelineNotifierConfig>,
}

impl From<proto::PipelineConfig> for PipelineConfig {
    fn from(p: proto::PipelineConfig) -> Self {
        PipelineConfig {
            id: p.id,
            name: p.name,
            description: {
                if p.description.is_empty() {
                    None
                } else {
                    Some(p.description)
                }
            },
            parallelism: p.parallelism,
            tasks: p.tasks.into_iter().map(|value| value.into()).collect(),
            triggers: p.triggers.into_iter().map(|value| value.into()).collect(),
            notifiers: p.notifiers.into_iter().map(|value| value.into()).collect(),
        }
    }
}

impl From<PipelineConfig> for proto::PipelineConfig {
    fn from(p: PipelineConfig) -> Self {
        proto::PipelineConfig {
            id: p.id,
            name: p.name,
            description: p.description.unwrap_or_default(),
            parallelism: p.parallelism,
            tasks: p.tasks.into_iter().map(|value| value.into()).collect(),
            triggers: p.triggers.into_iter().map(|value| value.into()).collect(),
            notifiers: p.notifiers.into_iter().map(|value| value.into()).collect(),
        }
    }
}

impl PipelineConfig {
    pub fn new(id: &str, name: &str) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            description: None,
            parallelism: 0,
            tasks: Vec::new(),
            triggers: Vec::new(),
            notifiers: Vec::new(),
        }
    }

    pub fn description(mut self, description: &str) -> Self {
        self.description = Some(description.to_string());
        self
    }

    pub fn parallelism(mut self, parallelism: u64) -> Self {
        self.parallelism = parallelism;
        self
    }

    pub fn tasks(mut self, tasks: Vec<TaskConfig>) -> Self {
        self.tasks = tasks;
        self
    }

    pub fn triggers(mut self, triggers: Vec<PipelineTriggerConfig>) -> Self {
        self.triggers = triggers;
        self
    }

    pub fn notifiers(mut self, notifiers: Vec<PipelineNotifierConfig>) -> Self {
        self.notifiers = notifiers;
        self
    }

    pub fn finish(self) {
        println!("{}", serde_json::to_string(&self).unwrap())
    }
}

/// Every time a pipeline attempts to subscribe to a trigger, it passes certain
/// values back to that trigger for certain functionality. Since triggers keep no
/// permanent state, these settings are kept here so that when triggers are restarted
/// they can be restored with proper settings.
#[derive(Debug, Serialize, Deserialize)]
pub struct PipelineTriggerConfig {
    /// A global unique identifier for the trigger type.
    pub kind: String,
    /// A user defined identifier for the trigger so that a pipeline with
    /// multiple notifiers can be differentiated.
    pub label: String,
    /// The settings for pertaining to that specific trigger.
    pub settings: HashMap<String, String>,
}

impl PipelineTriggerConfig {
    pub fn new(kind: &str, label: &str) -> Self {
        PipelineTriggerConfig {
            kind: kind.to_string(),
            label: label.to_string(),
            settings: HashMap::new(),
        }
    }

    pub fn settings(mut self, settings: HashMap<String, String>) -> Self {
        self.settings = settings;
        self
    }
}

impl From<proto::PipelineTriggerConfig> for PipelineTriggerConfig {
    fn from(p: proto::PipelineTriggerConfig) -> Self {
        PipelineTriggerConfig {
            kind: p.kind,
            label: p.label,
            settings: p.settings,
        }
    }
}

impl From<PipelineTriggerConfig> for proto::PipelineTriggerConfig {
    fn from(p: PipelineTriggerConfig) -> Self {
        proto::PipelineTriggerConfig {
            kind: p.kind,
            label: p.label,
            settings: p.settings,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PipelineNotifierConfig {
    /// A global unique identifier for the notifier type.
    pub kind: String,
    /// A user defined identifier for the notifier so that a pipeline with
    /// multiple notifiers can be differentiated.
    pub label: String,
    /// The settings for pertaining to that specific notifier.
    pub settings: HashMap<String, String>,
}

impl PipelineNotifierConfig {
    pub fn new(kind: &str, label: &str) -> Self {
        PipelineNotifierConfig {
            kind: kind.to_string(),
            label: label.to_string(),
            settings: HashMap::new(),
        }
    }

    pub fn settings(mut self, settings: HashMap<String, String>) -> Self {
        self.settings = settings;
        self
    }
}

impl From<proto::PipelineNotifierConfig> for PipelineNotifierConfig {
    fn from(p: proto::PipelineNotifierConfig) -> Self {
        PipelineNotifierConfig {
            kind: p.kind,
            label: p.label,
            settings: p.settings,
        }
    }
}

impl From<PipelineNotifierConfig> for proto::PipelineNotifierConfig {
    fn from(p: PipelineNotifierConfig) -> Self {
        proto::PipelineNotifierConfig {
            kind: p.kind,
            label: p.label,
            settings: p.settings,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pipeline() {
        PipelineConfig::new("simple_pipeline", "Simple Pipeline")
            .description("Test Description")
            .tasks(vec![])
            .finish();
    }
}
