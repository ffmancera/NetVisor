mod network;

use clap::{Parser, Subcommand};

use crate::network::query_network;

/// Host network topology visualization tool.
#[derive(Parser)]
#[command(version, about)]
struct Cli {
    /// Turn on debug logs
    #[arg(short, long)]
    debug: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Shows the diagram of the network topology
    Show,
}

fn main() {
    let cli = Cli::parse();
    println!("{:?}", query_network().unwrap());
}
