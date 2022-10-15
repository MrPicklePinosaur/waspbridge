//! Cli application to interact with the waspbridge daemon

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "wbcli")]
#[command(about = "cli to interact with the waspbridge daemon")]
struct Cli {
    name: Option<String>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Ring the watch to locate it
    Find,
    /// Get information on watch status (battery level etc)
    Status,
    /// Directly run a command on the watch
    Cmd { command: String },
    /// Kill the waspbridge daemon
    Kill,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Find) => {},
        Some(Commands::Status) => {},
        Some(Commands::Cmd { command }) => {},
        Some(Commands::Kill) => {},
        None => {},
    }
}
