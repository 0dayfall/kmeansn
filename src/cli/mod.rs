pub mod args;

use crate::commands;
use clap::Parser;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let cli = args::Cli::parse();
    match cli.command {
        args::Command::Fit(cmd) => commands::fit::run(cmd),
        args::Command::Assign(cmd) => commands::assign::run(cmd),
        args::Command::ClusterNeighbors(cmd) => commands::cluster_neighbors::run(cmd),
    }
}
