use std::str::FromStr;

use crate::conf;
use clap::{Parser, Subcommand};
use slog::o;
use sloggers::terminal::{Destination, TerminalLoggerBuilder};
use sloggers::types::Severity;
use sloggers::Build;

#[derive(Debug, Parser)]
#[clap(name = "gofer")]
#[clap(about = "Gofer is a distributed, continuous thing do-er.")]
#[clap(
    long_about = "Gofer is a distributed, continous thing do-er.\n\n It uses a similar model to concourse
    (https://concourse-ci.org/), leveraging the docker container as a key mechanism to run short-lived workloads.
    This results in simplicity; No foreign agents, no cluster setup, just run containers.\n\n
    Read more at https://clintjedwards.com/gofer"
)]
#[clap(version)]
struct Cli {
    /// Set configuration path; if empty default paths are used
    #[clap(long, value_name = "PATH")]
    config: Option<String>,

    #[clap(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {}

fn init_logging(severity: Severity) -> slog_scope::GlobalLoggerGuard {
    let mut builder = TerminalLoggerBuilder::new();
    builder.level(severity);
    builder.destination(Destination::Stderr);

    let root_logger = builder.build().unwrap();
    let log = slog::Logger::root(root_logger, o!());

    slog_scope::set_global_logger(log)
}

/// init the CLI and appropriately run the correct command.
pub async fn init() {
    let args = Cli::parse();

    let config;
    match conf::Kind::new_cli_config().parse(&args.config).unwrap() {
        conf::Kind::Cli(parsed_config) => config = parsed_config,
        _ => {
            panic!("Incorrect configuration file received")
        }
    }

    match args.command {
        _ => {}
    }
}
