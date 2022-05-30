use crate::models::{PipelineNotifierSettings, PipelineTriggerSettings, Task};
use crate::proto;
use serde::{Deserialize, Serialize};

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
    pub tasks: Vec<Task>,
    /// A mapping of pipeline owned triggers to their settings.
    pub triggers: Vec<PipelineTriggerSettings>,
    /// A mapping of pipeline owned notifiers to their settings.
    pub notifiers: Vec<PipelineNotifierSettings>,
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

    pub fn tasks(mut self, tasks: Vec<Task>) -> Self {
        self.tasks = tasks;
        self
    }

    pub fn triggers(mut self, triggers: Vec<PipelineTriggerSettings>) -> Self {
        self.triggers = triggers;
        self
    }

    pub fn notifiers(mut self, notifiers: Vec<PipelineNotifierSettings>) -> Self {
        self.notifiers = notifiers;
        self
    }

    pub fn finish(self) {
        println!("{}", serde_json::to_string(&self).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pipeline() {
        let pipeline = PipelineConfig::new("simple_pipeline", "Simple Pipeline")
            .description("Test Description")
            .tasks(vec![])
            .finish();
    }
}
