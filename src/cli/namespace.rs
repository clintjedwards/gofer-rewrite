use super::CliHarness;
use crate::proto::{gofer_client::GoferClient, *};
use clap::{Args, Subcommand};
use comfy_table::{presets::ASCII_MARKDOWN, Cell, CellAlignment, Color, ContentArrangement};
use std::process;
use std::time::{SystemTime, UNIX_EPOCH};

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
        name: Option<String>,
        /// Helpful description of namespace.
        description: Option<String>,
    },

    /// Detail namespace by id.
    Get { id: String },

    /// Update a namespace.
    Update {
        /// Identifier for namespace
        id: String,
        /// Humanized name for namespace.
        name: Option<String>,
        /// Helpful description of namespace.
        description: Option<String>,
    },

    /// Delete a namespace.
    Delete { id: String },
}

impl CliHarness {
    pub async fn namespace_list(&self) {
        let channel = match tonic::transport::Channel::from_shared(self.config.server.to_string()) {
            Ok(channel) => channel,
            Err(e) => {
                eprintln!("Could not open transport channel; {}", e);
                process::exit(1);
            }
        };

        let conn = match channel.connect().await {
            Ok(conn) => conn,
            Err(e) => {
                eprintln!("Could not connect to server; {}", e);
                process::exit(1);
            }
        };
        let mut client = GoferClient::new(conn);
        let request = tonic::Request::new(ListNamespacesRequest {
            offset: 0,
            limit: 0,
        });
        let response = match client.list_namespaces(request).await {
            Ok(response) => response.into_inner(),
            Err(e) => {
                eprintln!("Could not get namespaces; {}", e.message());
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
    pub async fn namespace_create(&self) {}
    pub async fn namespace_get(&self) {}
    pub async fn namespace_update(&self) {}
    pub async fn namespace_delete(&self) {}
}

fn epoch() -> u64 {
    let current_epoch = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();

    u64::try_from(current_epoch).unwrap()
}
