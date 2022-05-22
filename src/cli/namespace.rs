use super::CliHarness;
use crate::models;
use crate::proto::{gofer_client::GoferClient, *};
use clap::{Args, Subcommand};
use humantime;
use std::process;
use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};
use tabled::{Style, Table, Tabled};

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

#[derive(Debug, Tabled)]
struct TabledNamespace {
    /// Unique user defined identifier.
    pub id: String,
    /// Humanized name; great for reading from UIs.
    pub name: String,
    /// Short description of what namespace is used for.
    pub description: String,
    /// The creation time in epoch milli.
    pub created: String,
}

impl From<models::Namespace> for TabledNamespace {
    fn from(ns: models::Namespace) -> Self {
        let current_epoch = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();

        let time_diff = current_epoch - ns.created;
        let humanized_create_time =
            humantime::format_duration(Duration::from_millis(time_diff as u64));

        TabledNamespace {
            id: ns.id,
            name: ns.name,
            description: ns.description,
            created: humanized_create_time.to_string(),
        }
    }
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
                eprintln!("Could not get info; {}", e);
                process::exit(1);
            }
        };

        let namespaces: Vec<models::Namespace> = response
            .namespaces
            .into_iter()
            .map(|namespace| namespace.into())
            .collect();

        let tabled_namespaces: Vec<TabledNamespace> = namespaces
            .into_iter()
            .map(|namespace| namespace.into())
            .collect();

        println!("{}", Table::new(tabled_namespaces).with(Style::psql()));
    }
    pub async fn namespace_create(&self) {}
    pub async fn namespace_get(&self) {}
    pub async fn namespace_update(&self) {}
    pub async fn namespace_delete(&self) {}
}
