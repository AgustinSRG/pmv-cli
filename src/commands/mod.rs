// Commands index

mod account;
use account::*;

mod login;
use login::*;

mod logout;
use logout::*;

mod random;
use random::*;

use clap::{Subcommand};

use crate::tools::RequestError;

#[derive(Clone)]
pub struct CommandGlobalOptions {
    pub verbose: bool,
    pub auto_confirm: bool,
    pub vault_url: Option<String>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Logins into an active vault, printing a session URL into the standard output 
    Login {
        /// Vault username. You can also specify the credentials in the URL
        #[arg(short = 'U', long)]
        username: Option<String>,
    },

    /// Closes the active session, given a session URL
    Logout,

    /// Manages accounts
    Account {
        #[command(subcommand)]
        account_cmd: AccountCommand,
    },

    /// Retrieves random media assets from the vault
    Random {
        /// PRNG seed
        #[arg(short, long)]
        seed: Option<i64>,

        /// Page size, 10 by default
        #[arg(short, long)]
        page_size: Option<u32>,

        /// Filter by a tag
        #[arg(short, long)]
        tag: Option<String>,

        /// Extended version of the results table
        #[arg(short, long)]
        extended: bool,
    },
}

pub async fn run_cmd(global_opts: CommandGlobalOptions, cmd: Commands) -> () {
    match cmd {
        Commands::Login { username } => {
            run_cmd_login(global_opts, username).await;
        },
        Commands::Logout => {
            run_cmd_logout(global_opts).await;
        },
        Commands::Account { account_cmd } => {
            run_account_cmd(global_opts, account_cmd).await;
        },
        Commands::Random { seed, page_size, tag, extended } => {
            run_cmd_random(global_opts, seed, page_size, tag, extended).await;
        },
    }
}

pub fn get_vault_url(global_opts_url: Option<String>) -> String {
    match global_opts_url {
        Some(u) => {
            return u;
        },
        None => {
            let env_val = std::env::var("PMV_URL");

            match env_val {
                Ok(u) => {
                    return u;
                }
                Err(_) => {
                    return "http://localhost".to_string();
                },
            }
        },
    }
}

pub fn print_request_error(e: RequestError) -> () {
    match e {
        crate::tools::RequestError::StatusCodeError(s) => {
            if s == 401 {
                eprintln!(
                    "Error: The session URL you provided was invalid or expired."
                );
            } else {
                eprintln!("Error: API ended with unexpected status code: {s}");
            }
        }
        crate::tools::RequestError::ApiError(e) => {
            let s = e.status;
            let code = e.code;
            let msg = e.message;
            eprintln!("API Error | Status: {s} | Code: {code} | Message: {msg}");
        }
        crate::tools::RequestError::HyperError(e) => {
            let e_str = e.to_string();
            eprintln!("Error: {e_str}");
        }
    }
}
