mod diag_ctxt;
mod graphics;
mod network;

use std::io::Write;

use clap::{Parser, Subcommand};
use env_logger::Builder;
use log::LevelFilter;

use crate::diag_ctxt::DiagramCtxt;
use crate::graphics::draw_picture;

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
    /// Generate the image representing the network topology
    Show {
        /// specify the output path
        #[arg(short, long)]
        file: Option<String>,
    },
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

    let diag_ctxt =
        DiagramCtxt::new().expect("Error when fetching the host network configuration.");

    match &cli.command {
        Some(Commands::Show { file }) => match file {
            Some(path) => draw_picture(diag_ctxt, path),
            None => draw_picture(diag_ctxt, &"output".to_string()),
        },
        None => {}
    }
}
