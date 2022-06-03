use super::super::CliHarness;
use crate::cli::{humanize_duration, DEFAULT_NAMESPACE};
use colored::Colorize;
use std::process;

impl CliHarness {
    pub async fn pipeline_get(&self, id: &str) {
        let mut client = match self.connect().await {
            Ok(client) => client,
            Err(e) => {
                eprintln!("Command failed; {}", e);
                process::exit(1);
            }
        };

        let request = tonic::Request::new(gofer_proto::GetPipelineRequest {
            namespace_id: self
                .config
                .namespace
                .clone()
                .unwrap_or_else(|| DEFAULT_NAMESPACE.to_string()),
            id: id.to_string(),
        });
        let response = match client.get_pipeline(request).await {
            Ok(response) => response.into_inner(),
            Err(e) => {
                eprintln!("Command failed; {}", e.message());
                process::exit(1);
            }
        };

        let pipeline = response.pipeline.unwrap();

        println!(
            "[{}] {} :: Created {}

      {}",
            pipeline.id.green(),
            pipeline.name,
            humanize_duration(pipeline.created as i64),
            pipeline.description
        );
    }
}

fn get_pipeline_template() {
    const TEMPLATE: &str = r#"[{id}] {name} :: {state}

{description}
{{- if recent_runs}}
📦 Recent Runs
  {{- for run in recent_runs}}
  • {run.id} :: {run.started} by trigger {run.trigger_name} ({run.trigger_kind}) :: {run.state_prefix} {run.lasted} :: {run.state}
  {{- endfor}}
{{- endif}}
{{- if tasks }}
🗒 Tasks:
  {{- for task in tasks}}
  • {task.name}
  {{- if task.depends_on -}}
    {{- for dependant in task.depends_on }}
      - {dependant}
    {{- endfor -}}
  {{- endif -}}
  {{- endfor -}}
{{- endif}}
{{- if objects}}
☁︎ Objects: [{objects}]
{{- endfor}}
{{- if triggers }}
🗘 Attached Triggers:
  {{- for trigger in triggers}}
  ⟳ [{trigger.state}] {trigger.label} ({trigger.kind})
    {{- for event in trigger.events }}
    + {event.processed} | {event.details}
    {{- endfor}}
  {{- endfor}}
{{- endif}}
{{- if notifiers }}
🕪 Attached Notifiers:
  {{- for notifier in notifiers range}}
  🕩 {notifier.label} ({notifier.kind})
  {{- endfor}}
{{- endif}}

Created {created} | Last Run {last_run} | Health {health}"#;

    let template = tinytemplate::TinyTemplate::new();
}
