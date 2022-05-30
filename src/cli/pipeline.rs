use super::CliHarness;
use crate::cli::humanize_duration;
use crate::models;
use clap::{Args, Subcommand};
use comfy_table::{presets::ASCII_MARKDOWN, Cell, CellAlignment, Color, ContentArrangement};
use std::process;

#[derive(Debug, Args)]
pub struct PipelineSubcommands {
    /// Set namespace for command to act upon.
    #[clap(long)]
    namespace: Option<String>,

    #[clap(subcommand)]
    pub command: PipelineCommands,
}

#[derive(Debug, Subcommand)]
pub enum PipelineCommands {
    /// List pipelines.
    List,

    /// Create a new pipeline.
    ///
    /// Creating a pipeline requires a pipeline configuration file. You can find documentation on
    /// how to create a pipeline configuration file
    /// [here](https://clintjedwards.com/gofer/docs/getting-started/first-steps/generate-pipeline-config).
    Create {
        /// Path to a pipeline configuration file.
        #[clap(short, long)]
        path: String,
    },

    /// Detail pipeline by id.
    Get { id: String },

    /// Update a new pipeline.
    ///
    /// Updating a pipeline requires a pipeline configuration file. You can find documentation on
    /// how to manage your pipeline configuration file
    /// [here](https://clintjedwards.com/gofer/docs/getting-started/first-steps/generate-pipeline-config).
    Update {
        /// Path to a pipeline configuration file.
        #[clap(short, long)]
        path: String,
    },

    /// Delete a pipeline.
    Delete { id: String },
}

impl CliHarness {
    pub async fn pipeline_list(&self, namespace: Option<String>) {
        let mut client = match self.connect().await {
            Ok(client) => client,
            Err(e) => {
                eprintln!("Command failed; {}", e.source().unwrap());
                process::exit(1);
            }
        };

        let request = tonic::Request::new(proto::ListPipelinesRequest {
            namespace_id: namespace.unwrap_or_else(|| self.config.namespace.clone()),
            offset: 0,
            limit: 0,
        });
        let response = match client.list_pipelines(request).await {
            Ok(response) => response.into_inner(),
            Err(e) => {
                eprintln!("Command failed; {}", e.message());
                process::exit(1);
            }
        };

        let mut table = comfy_table::Table::new();
        table
            .load_preset(ASCII_MARKDOWN)
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_header(vec![
                Cell::new("id")
                    .set_alignment(CellAlignment::Center)
                    .fg(Color::Blue),
                Cell::new("name")
                    .set_alignment(CellAlignment::Center)
                    .fg(Color::Blue),
                Cell::new("description")
                    .set_alignment(CellAlignment::Center)
                    .fg(Color::Blue),
                Cell::new("last run")
                    .set_alignment(CellAlignment::Center)
                    .fg(Color::Blue),
                Cell::new("state")
                    .set_alignment(CellAlignment::Center)
                    .fg(Color::Blue),
                Cell::new("created")
                    .set_alignment(CellAlignment::Center)
                    .fg(Color::Blue),
            ]);

        for pipeline in response.pipelines {
            table.add_row(vec![
                Cell::new(pipeline.id).fg(Color::Green),
                Cell::new(pipeline.name),
                Cell::new(pipeline.description),
                Cell::new(humanize_duration(pipeline.last_run_time as i64)),
                Cell::new({
                    let state = proto::pipeline::PipelineState::from_i32(pipeline.state).unwrap();
                    models::PipelineState::from(state).to_string()
                }),
                Cell::new(humanize_duration(pipeline.created as i64)),
            ]);
        }

        println!("{table}",);
    }

    pub async fn pipeline_create(&self, namespace: Option<String>, pipeline_config: &str) {
        let mut client = match self.connect().await {
            Ok(client) => client,
            Err(e) => {
                eprintln!("Command failed; {}", e);
                process::exit(1);
            }
        };

        let request = tonic::Request::new(proto::CreatePipelineRequest {
            namespace_id: namespace.unwrap_or_else(|| self.config.namespace.clone()),
            pipeline_config: None,
        });
        let response = match client.create_pipeline(request).await {
            Ok(response) => response.into_inner(),
            Err(e) => {
                eprintln!("Command failed; {}", e.message());
                process::exit(1);
            }
        };

        let pipeline = response.pipeline.unwrap();

        println!("Created pipeline: [{}] {}", pipeline.id, pipeline.name);
    }

    //     pub async fn pipeline_get(&self, namespace: Option<String>, id: &str) {
    //         let mut client = match self.connect().await {
    //             Ok(client) => client,
    //             Err(e) => {
    //                 eprintln!("Command failed; {}", e);
    //                 process::exit(1);
    //             }
    //         };

    //         let request = tonic::Request::new(proto::GetPipelineRequest {
    //             namespace_id: namespace.unwrap_or(self.config.namespace.clone()),
    //             id: id.to_string(),
    //         });
    //         let response = match client.get_pipeline(request).await {
    //             Ok(response) => response.into_inner(),
    //             Err(e) => {
    //                 eprintln!("Command failed; {}", e.message());
    //                 process::exit(1);
    //             }
    //         };

    //         let pipeline = response.pipeline.unwrap();

    //         println!(
    //             "[{}] {} :: Created {}

    //   {}",
    //             pipeline.id.green(),
    //             pipeline.name,
    //             humanize_duration(pipeline.created as i64),
    //             pipeline.description
    //         );
    //     }

    //     pub async fn pipeline_update(&self, namespace: Option<String>, pipeline_config: &str) {
    //         let mut client = match self.connect().await {
    //             Ok(client) => client,
    //             Err(e) => {
    //                 eprintln!("Command failed; {}", e);
    //                 process::exit(1);
    //             }
    //         };

    //         let request = tonic::Request::new(proto::GetPipelineRequest {
    //             namespace_id: namespace.unwrap_or(self.config.namespace.clone()),
    //             id: id.to_string(),
    //         });
    //         let response = match client.get_pipeline(request).await {
    //             Ok(response) => response.into_inner(),
    //             Err(e) => {
    //                 eprintln!("Command failed; {}", e.message());
    //                 process::exit(1);
    //             }
    //         };

    //         let current_pipeline = response.pipeline.unwrap();

    //         let request = tonic::Request::new(proto::UpdatePipelineRequest {
    //             namespace_id: namespace.unwrap_or(self.config.namespace.clone()),
    //             id: id.to_string(),
    //             name: name.unwrap_or(current_pipeline.name),
    //             description: description.unwrap_or(current_pipeline.description),
    //         });
    //         match client.update_pipeline(request).await {
    //             Ok(_) => (),
    //             Err(e) => {
    //                 eprintln!("Command failed; {}", e.message());
    //                 process::exit(1);
    //             }
    //         };

    //         println!("Updated pipeline '{}'", id);
    //     }
    //     pub async fn pipeline_delete(&self, id: &str) {
    //         let mut client = match self.connect().await {
    //             Ok(client) => client,
    //             Err(e) => {
    //                 eprintln!("Command failed; {}", e);
    //                 process::exit(1);
    //             }
    //         };

    //         let request = tonic::Request::new(DeletePipelineRequest { id: id.to_string() });
    //         match client.delete_pipeline(request).await {
    //             Ok(_) => (),
    //             Err(e) => {
    //                 eprintln!("Command failed; {}", e.message());
    //                 process::exit(1);
    //             }
    //         };

    //         println!("Deleted pipeline '{}'", id);
    //     }
}
