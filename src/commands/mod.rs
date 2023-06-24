// Commands index

mod account;
use account::*;

mod login;
use login::*;

mod logout;
use logout::*;

mod random;
use random::*;

mod search_basic;
use search_basic::*;

use clap::Subcommand;

use crate::tools::RequestError;

#[derive(Clone)]
pub struct CommandGlobalOptions {
    pub debug: bool,
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

        /// CSV format
        #[arg(short, long)]
        csv: bool,
    },

    /// Searches for media assets in the vault (Basic)
    Search {
        /// Selects the results page. The fist page is the page 1.
        #[arg(short, long)]
        page: Option<u32>,

        /// Page size, 10 by default
        #[arg(short = 's', long)]
        page_size: Option<u32>,

        /// Filter by a tag
        #[arg(short, long)]
        tag: Option<String>,

        /// Reverses results sorting. By default newest results are first. With this option, oldest results are first.
        #[arg(short, long)]
        reverse: bool,

        /// Extended version of the results table
        #[arg(short, long)]
        extended: bool,

        /// CSV format
        #[arg(short, long)]
        csv: bool,
    },

    /// Searches for media assets in the vault (Advanced)
    AdvancedSearch {
        /// Filter by media type. Can be: video, audio or image
        #[arg(short, long)]
        media_type: Option<String>,

        /// Filter by tags. Expected a list of tag names, separated by spaces.
        #[arg(short, long)]
        tags: Option<String>,

        // Tag filtering mode. Can be: all, any, none or untagged
        #[arg(short = 'm', long)]
        tags_mode: Option<String>,

        /// Filter by album. Expected an album ID, like: #1
        #[arg(short, long)]
        album: Option<String>,

        /// Limit on the number of results to get. 25 by default.
        #[arg(short, long)]
        limit: Option<u32>,

        /// Number of results to skip. 0 by default.
        #[arg(short, long)]
        skip: Option<u32>,

        /// Reverses results sorting. By default newest results are first. With this option, oldest results are first.
        #[arg(short, long)]
        reverse: bool,

        /// Extended version of the results table
        #[arg(short, long)]
        extended: bool,

        /// CSV format
        #[arg(short, long)]
        csv: bool,
    },
}

pub async fn run_cmd(global_opts: CommandGlobalOptions, cmd: Commands) -> () {
    match cmd {
        Commands::Login { username } => {
            run_cmd_login(global_opts, username).await;
        }
        Commands::Logout => {
            run_cmd_logout(global_opts).await;
        }
        Commands::Account { account_cmd } => {
            run_account_cmd(global_opts, account_cmd).await;
        }
        Commands::Random {
            seed,
            page_size,
            tag,
            extended,
            csv,
        } => {
            run_cmd_random(global_opts, seed, page_size, tag, extended, csv).await;
        }
        Commands::Search {
            page,
            page_size,
            tag,
            reverse,
            extended,
            csv,
        } => {
            run_cmd_search_basic(global_opts, page, page_size, tag, reverse, extended, csv).await;
        }
        Commands::AdvancedSearch {
            media_type,
            tags,
            tags_mode,
            album,
            limit,
            skip,
            reverse,
            extended,
            csv,
        } => {}
    }
}

pub fn get_vault_url(global_opts_url: Option<String>) -> String {
    match global_opts_url {
        Some(u) => {
            return u;
        }
        None => {
            let env_val = std::env::var("PMV_URL");

            match env_val {
                Ok(u) => {
                    return u;
                }
                Err(_) => {
                    return "http://localhost".to_string();
                }
            }
        }
    }
}

pub fn print_request_error(e: RequestError) -> () {
    match e {
        RequestError::StatusCodeError(s) => {
            if s == 401 {
                eprintln!("Error: The session URL you provided was invalid or expired.");
            } else {
                eprintln!("Error: API ended with unexpected status code: {s}");
            }
        }
        RequestError::ApiError {
            status,
            code,
            message,
        } => {
            eprintln!("API Error | Status: {status} | Code: {code} | Message: {message}");
        }
        RequestError::HyperError(e) => {
            let e_str = e.to_string();
            eprintln!("Error: {e_str}");
        }
        RequestError::JSONError { message, body } => {
            eprintln!("Body received: {body}");
            eprintln!("Error parsing the body: {message}");
            eprintln!("This may be caused due to incompatibilities between the PersonalMediaVault backend and this tool.");
            eprintln!("If you are using the latest version, you should report this a a bug:");
            eprintln!("https://github.com/AgustinSRG/pmv-cli/issues");
        }
    }
}
