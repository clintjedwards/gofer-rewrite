use super::CliHarness;
use crate::cli::{humanize_duration, init_spinner, DEFAULT_NAMESPACE};
use crate::models;
use anyhow::{anyhow, Context, Result};
use clap::{Args, Subcommand};
use colored::Colorize;
use comfy_table::{presets::ASCII_MARKDOWN, Cell, CellAlignment, Color, ContentArrangement};
use indicatif::{ProgressBar, ProgressStyle};
use std::io::{BufRead, BufReader, Read};
use std::{path::PathBuf, process};

#[derive(Debug, Args)]
pub struct PipelineSubcommands {
    /// Set namespace for command to act upon.
    #[clap(long)]
    pub namespace: Option<String>,

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
        path: String,
    },

    /// Detail pipeline by id.
    Get { id: String },

    /// Start executing a pipeline.
    Run { id: String },

    /// Update a new pipeline.
    ///
    /// Updating a pipeline requires a pipeline configuration file. You can find documentation on
    /// how to manage your pipeline configuration file
    /// [here](https://clintjedwards.com/gofer/docs/getting-started/first-steps/generate-pipeline-config).
    Update {
        /// Path to a pipeline configuration file.
        path: String,
    },

    /// Delete pipeline by id.
    Delete { id: String },
}

impl CliHarness {
    pub async fn pipeline_list(&self) {
        let mut client = match self.connect().await {
            Ok(client) => client,
            Err(e) => {
                eprintln!("Command failed; {}", e.source().unwrap());
                process::exit(1);
            }
        };

        let request = tonic::Request::new(gofer_proto::ListPipelinesRequest {
            namespace_id: self
                .config
                .namespace
                .clone()
                .unwrap_or_else(|| DEFAULT_NAMESPACE.to_string()),
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
                    let state =
                        gofer_proto::pipeline::PipelineState::from_i32(pipeline.state).unwrap();
                    models::PipelineState::from(state).to_string()
                }),
                Cell::new(humanize_duration(pipeline.created as i64)),
            ]);
        }

        println!("{table}",);
    }

    /// The ability to create and manage pipelines is a huge selling point for Gofer.
    /// In the pursuit of making this as easy as possible we allow the user to use rust
    /// as a way to generate and manage their pipeline configurations. For that to work
    /// though we have to be able to compile and run programs which implement the sdk and
    /// then collect the output.
    pub async fn pipeline_create(&self, path: &str) {
        let spinner = init_spinner();
        spinner.set_message("Creating pipeline");

        // Figure out absolute path for any given path string.
        let path = PathBuf::from(path);
        let full_path = match path.canonicalize() {
            Ok(path) => path,
            Err(e) => {
                spinner.finish_and_clear();
                println!(
                    "{} Could not determine full path for '{}'; {}",
                    "x".red(),
                    path.to_string_lossy(),
                    e
                );
                process::exit(1);
            }
        };
        let full_path = full_path.to_string_lossy();

        // Spawn the relevant binary to build the configuration and collect
        // the output.
        // The stderr we use as status markers since they mostly stem from
        // the build tool's debug output.
        // The stdout we use as the final output and attempt to parse that.
        let mut cmd = match process::Command::new("cargo")
            .args(["run", &format!("--manifest-path={full_path}/Cargo.toml")])
            .stderr(process::Stdio::piped())
            .stdout(process::Stdio::piped())
            .spawn()
        {
            Ok(cmd) => cmd,
            Err(e) => {
                spinner.finish_and_clear();
                println!(
                    "{} Could not run build command for target config '{}'; {}",
                    "x".red(),
                    full_path,
                    e
                );
                process::exit(1);
            }
        };

        // Print out the stderr as status markers
        let stderr = cmd.stderr.take().unwrap();
        let stderr_reader = BufReader::new(stderr).lines();

        for line in stderr_reader {
            let line = line.unwrap();
            spinner.set_message({
                let mut status_line = format!("Building pipeline config: {}", line.trim());
                status_line.truncate(80);
                status_line
            });
        }

        let exit_status = match cmd.wait() {
            Ok(status) => status,
            Err(e) => {
                spinner.finish_and_clear();
                println!(
                    "{} Could not run build command for target config; {}",
                    "x".red(),
                    e
                );
                process::exit(1);
            }
        };

        if !exit_status.success() {
            let mut output = String::from("");
            cmd.stderr.unwrap().read_to_string(&mut output).unwrap();

            spinner.finish_and_clear();
            println!(
                "{} Could not run build command for target config; {}",
                "x".red(),
                output
            );
            process::exit(1);
        }

        spinner.set_message("Parsing pipeline config");

        let mut output = "".to_string();
        cmd.stdout.unwrap().read_to_string(&mut output).unwrap();

        let config: gofer_sdk::config::Pipeline = match serde_json::from_str(&output) {
            Ok(config) => config,
            Err(e) => {
                spinner.finish_and_clear();
                println!("{} Could not parse pipeline config; {}", "x".red(), e);
                process::exit(1);
            }
        };

        spinner.set_message("Creating pipeline config");

        let mut client = match self.connect().await {
            Ok(client) => client,
            Err(e) => {
                spinner.finish_and_clear();
                println!("{} Could not create pipeline; {}", "x".red(), e);
                process::exit(1);
            }
        };

        let request = tonic::Request::new(gofer_proto::CreatePipelineRequest {
            namespace_id: self
                .config
                .namespace
                .clone()
                .unwrap_or_else(|| DEFAULT_NAMESPACE.to_string()),
            pipeline_config: Some(config.into()),
        });
        let response = match client.create_pipeline(request).await {
            Ok(response) => response.into_inner(),
            Err(e) => {
                spinner.finish_and_clear();
                println!("{} Could not create pipeline; {}", "x".red(), e);
                process::exit(1);
            }
        };

        let created_pipeline = response.pipeline.unwrap();

        spinner.finish_and_clear();

        println!(
            "{} Created pipeline: [{}] {}",
            "✓".green(),
            created_pipeline.id.green(),
            created_pipeline.name
        );
        println!(
            "  View details of your new pipeline: {}",
            format!("gofer pipeline get {}", created_pipeline.id)
                .dimmed()
                .yellow()
        );
        println!(
            "  Start a new run: {}",
            format!("gofer pipeline run {}", created_pipeline.id)
                .dimmed()
                .yellow()
        );
    }

    //     pub async fn pipeline_get(&self, namespace: Option<String>, id: &str) {
    //         let mut client = match self.connect().await {
    //             Ok(client) => client,
    //             Err(e) => {
    //                 eprintln!("Command failed; {}", e);
    //                 process::exit(1);
    //             }
    //         };

    //         let request = tonic::Request::new(gofer_proto::GetPipelineRequest {
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

    //         let request = tonic::Request::new(gofer_proto::GetPipelineRequest {
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

    //         let request = tonic::Request::new(gofer_proto::UpdatePipelineRequest {
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
