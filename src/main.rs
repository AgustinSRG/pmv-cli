// Main

use clap::{Parser, Subcommand, command};

mod api;
mod models;
mod tools;

/// Command line interface client for PersonalMediaVault
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Turn debugging information on
    #[arg(short, long)]
    debug: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Logins into an active vault, printing a session URL into the standard output 
    Login {
        /// HTTP connection URL to the active vault
        #[arg(short, long)]
        url: String,

        /// Vault username. You can also specify the credentials in the URL
        #[arg(short = 'U', long)]
        username: Option<String>,
    },

    /// Closes the active session, given a session URL
    Logout {
        /// HTTP connection URL to the active vault, with the session included as the password
        #[arg(short, long)]
        url: String,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let cli = Cli::parse();

    if cli.debug {
        println!("Debug mode is on");
    }

    println!("Hello world");

    Ok(())
}
