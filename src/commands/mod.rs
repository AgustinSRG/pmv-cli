// Commands index

mod login;
pub use login::*;

mod logout;
pub use logout::*;

use clap::{Subcommand};

#[derive(Clone)]
pub struct CommandGlobalOptions {
    pub verbose: bool,
    pub auto_confirm: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Logins into an active vault, printing a session URL into the standard output 
    Login {
        /// HTTP connection URL to the active vault
        url: String,

        /// Vault username. You can also specify the credentials in the URL
        #[arg(short = 'U', long)]
        username: Option<String>,
    },

    /// Closes the active session, given a session URL
    Logout {
        /// HTTP connection URL to the active vault, with the session included as the password
        url: String,
    },
}

pub async fn run_cmd(global_opts: CommandGlobalOptions, cmd: Commands) -> () {
    match cmd {
        Commands::Login { url, username } => {
            run_cmd_login(global_opts, url, username).await;
        },
        Commands::Logout { url } => {
            run_cmd_logout(global_opts, url).await;
        },
    }
}
