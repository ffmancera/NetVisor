mod graphics;
mod network;

use std::io::Write;

use clap::{Parser, Subcommand};
use env_logger::Builder;
use log::{error, LevelFilter};

use crate::graphics::draw_picture;
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
        builder.filter(Some("netvisor"), LevelFilter::Debug);
    } else {
        builder.filter(Some("netvisor"), LevelFilter::Info);
    }
    builder.init();

    let network_state = query_network();
    match network_state {
        Ok(v) => println!("{:?}", v),
        Err(_) => {
            error!("Error when fetching the host network configuration.");
            return;
        },
    };

    draw_picture();
}
