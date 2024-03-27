mod network;

use std::io::Write;

use env_logger::Builder;
use clap::{Parser, Subcommand};
use log::{LevelFilter};

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

    let mut builder = Builder::from_default_env();
    builder.format(|buf, record| writeln!(buf, "{} - {}", record.level(), record.args()));

    if cli.debug {
        builder.filter_level(LevelFilter::Debug);
    } else {
        builder.filter_level(LevelFilter::Info);
    }
    builder.init();

    println!("{:?}", query_network().unwrap());
}
