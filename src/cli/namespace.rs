use super::CliHarness;
use crate::cli::connect;
use crate::cli::epoch;
use crate::proto::{
    CreateNamespaceRequest, DeleteNamespaceRequest, GetNamespaceRequest, ListNamespacesRequest,
    UpdateNamespaceRequest,
};
use clap::{Args, Subcommand};
use comfy_table::{presets::ASCII_MARKDOWN, Cell, CellAlignment, Color, ContentArrangement};
use std::process;

#[derive(Debug, Args)]
pub struct NamespaceSubcommands {
    #[clap(subcommand)]
    pub command: NamespaceCommands,
}

#[derive(Debug, Subcommand)]
pub enum NamespaceCommands {
    /// List namespaces.
    List,

    /// Create a new namespace.
    Create {
        /// Identifier for namespace; Must be alphanumeric, lowercase,
        /// with only hyphens/dashes as alternate characters.
        id: String,
        /// Humanized name for namespace.
        #[clap(short, long)]
        name: Option<String>,
        /// Helpful description of namespace.
        #[clap(short, long)]
        description: Option<String>,
    },

    /// Detail namespace by id.
    Get { id: String },

    /// Update a namespace.
    Update {
        /// Identifier for namespace
        id: String,
        /// Humanized name for namespace.
        #[clap(short, long)]
        name: Option<String>,
        /// Helpful description of namespace.
        #[clap(short, long)]
        description: Option<String>,
    },

    /// Delete a namespace.
    Delete { id: String },
}

impl CliHarness {
    pub async fn namespace_list(&self) {
        let mut client = match connect(&self.config.server).await {
            Ok(client) => client,
            Err(e) => {
                eprintln!("Could not list namespaces; {}", e);
                process::exit(1);
            }
        };

        let request = tonic::Request::new(ListNamespacesRequest {
            offset: 0,
            limit: 0,
        });
        let response = match client.list_namespaces(request).await {
            Ok(response) => response.into_inner(),
            Err(e) => {
                eprintln!("Could not list namespaces; {}", e.message());
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
                Cell::new("created")
                    .set_alignment(CellAlignment::Center)
                    .fg(Color::Blue),
            ]);

        for namespace in response.namespaces {
            let time_diff = namespace.created as i64 - epoch() as i64;
            let time_diff_duration = chrono::Duration::milliseconds(time_diff);
            let humanized_create_time = chrono_humanize::HumanTime::from(time_diff_duration);
            table.add_row(vec![
                Cell::new(namespace.id).fg(Color::Yellow),
                Cell::new(namespace.name),
                Cell::new(namespace.description),
                Cell::new(humanized_create_time),
            ]);
        }

        println!("{table}",);
    }
    pub async fn namespace_create(
        &self,
        id: &str,
        name: Option<String>,
        description: Option<String>,
    ) {
        let mut client = match connect(&self.config.server).await {
            Ok(client) => client,
            Err(e) => {
                eprintln!("Could not create namespace; {}", e);
                process::exit(1);
            }
        };

        let request = tonic::Request::new(CreateNamespaceRequest {
            id: id.to_string(),
            name: name.unwrap_or_default(),
            description: description.unwrap_or_default(),
        });
        let response = match client.create_namespace(request).await {
            Ok(response) => response.into_inner(),
            Err(e) => {
                eprintln!("Could not create namespace; {}", e.message());
                process::exit(1);
            }
        };

        let namespace = response.namespace.unwrap();

        println!("Created namespace: [{}] {}", namespace.id, namespace.name);
    }
    pub async fn namespace_get(&self) {}
    pub async fn namespace_update(&self) {}
    pub async fn namespace_delete(&self) {}
}
