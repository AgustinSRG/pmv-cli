// Main

use clap::{Parser, command};
use commands::{run_cmd, Commands, CommandGlobalOptions};

mod api;
mod commands;
mod models;
mod tools;

/// Command line interface client for PersonalMediaVault
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Turn verbose messages on
    #[arg(short, long)]
    pub verbose: bool,

    /// Auto confirm actions
    #[arg(short, long)]
    pub yes: bool,

    #[command(subcommand)]
    pub command: Commands,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let cli = Cli::parse();

    let global_opts = CommandGlobalOptions{
        verbose: cli.verbose,
        auto_confirm: cli.yes,
    };

    run_cmd(global_opts, cli.command).await;

    Ok(())
}
