// Commands index

mod account;
use account::*;

mod album;
use album::*;

mod album_export;
use album_export::*;

mod album_import;
use album_import::*;

mod album_thumbnail;
use album_thumbnail::*;

mod batch_operation;
use batch_operation::*;

mod config;
use config::*;

mod invites;
use invites::*;

mod login;
use login::*;

mod logout;
use logout::*;

mod media;
use media::*;

mod media_attachments;
mod media_audio_tracks;
mod media_download;
mod media_export;
mod media_extended_description;
mod media_image_notes;
mod media_import;
mod media_resolutions;
mod media_subtitles;
mod media_thumbnail;
mod media_time_slices;
mod media_upload;
mod media_replace;

mod random;
use random::*;

mod search_advanced;
use search_advanced::*;

mod search_basic;
use search_basic::*;

mod server_info;
use server_info::*;

mod tag;
use tag::*;

mod task;
use task::*;

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

        /// Session duration. Can be: day, week, month or year
        #[arg(short = 'D', long)]
        duration: Option<String>,

        /// Invite code. Setting this option will ignore the credentials and use the code.
        #[arg(short = 'I', long)]
        invite_code: Option<String>,
    },

    /// Closes the active session, given a session URL
    Logout,

    /// Manages accounts
    Account {
        #[command(subcommand)]
        account_cmd: AccountCommand,
    },

    /// Manages media assets
    Media {
        #[command(subcommand)]
        media_cmd: MediaCommand,
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
    #[clap(alias("adv-search"))]
    AdvancedSearch {
        /// Filter by title
        #[arg(short = 'q', long)]
        title: Option<String>,

        /// Filter by description.
        #[arg(short, long)]
        description: Option<String>,

        /// Filter by media type. Can be: video, audio or image
        #[arg(short = 'k', long)]
        media_type: Option<String>,

        /// Filter by tags. Expected a list of tag names, separated by spaces.
        #[arg(short, long)]
        tags: Option<String>,

        /// Tag filtering mode. Can be: all, any, none or untagged
        #[arg(short = 'm', long)]
        tags_mode: Option<String>,

        /// Filter by album. Expected an album ID, like: #1
        #[arg(short, long)]
        album: Option<String>,

        /// Limit on the number of results to get. 25 by default.
        #[arg(short, long)]
        limit: Option<u32>,

        /// Media id to use as a stating point for the scanning process
        #[arg(short, long)]
        start_from: Option<String>,

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

    /// Manages tags
    #[clap(alias("tags"))]
    Tag {
        #[command(subcommand)]
        tag_cmd: TagCommand,
    },

    /// Manages albums
    #[clap(alias("albums"))]
    Album {
        #[command(subcommand)]
        album_cmd: AlbumCommand,
    },

    /// Manages vault configuration
    #[clap(alias("configuration"))]
    Config {
        #[command(subcommand)]
        config_cmd: ConfigCommand,
    },

    /// Retrieves tasks information
    #[clap(alias("tasks"))]
    Task {
        #[command(subcommand)]
        task_cmd: TaskCommand,
    },

    /// Manages invites
    Invites {
        #[command(subcommand)]
        invites_cmd: InvitesCommand,
    },

    /// Applies a batch operation to a list of media assets
    Batch {
        /// Filter by title
        #[arg(short = 'q', long)]
        title: Option<String>,

        /// Filter by description.
        #[arg(short, long)]
        description: Option<String>,

        /// Filter by media type. Can be: video, audio or image
        #[arg(short = 'k', long)]
        media_type: Option<String>,

        /// Filter by tags. Expected a list of tag names, separated by spaces.
        #[arg(short, long)]
        tags: Option<String>,

        /// Tag filtering mode. Can be: all, any, none or untagged
        #[arg(short = 'm', long)]
        tags_mode: Option<String>,

        /// Filter by album. Expected an album ID, like: #1
        #[arg(short, long)]
        album: Option<String>,

        /// Do not filter. Apply to the entire vault instead.
        #[arg(short, long)]
        everything: bool,

        #[command(subcommand)]
        batch_command: BatchCommand,
    },

    /// Gets server information, like the version it is using.
    #[clap(alias("server-info"))]
    GetServerInformation,
}

pub async fn run_cmd(global_opts: CommandGlobalOptions, cmd: Commands) {
    match cmd {
        Commands::Login { username, duration, invite_code } => {
            run_cmd_login(global_opts, username, duration, invite_code).await;
        }
        Commands::Logout => {
            run_cmd_logout(global_opts).await;
        }
        Commands::Account { account_cmd } => {
            run_account_cmd(global_opts, account_cmd).await;
        }
        Commands::Media { media_cmd } => {
            run_media_cmd(global_opts, media_cmd).await;
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
            title,
            description,
            media_type,
            tags,
            tags_mode,
            album,
            limit,
            start_from,
            reverse,
            extended,
            csv,
        } => {
            run_cmd_search_advanced(
                global_opts,
                title,
                description,
                media_type,
                tags,
                tags_mode,
                album,
                limit,
                start_from,
                reverse,
                extended,
                csv,
            )
            .await;
        }
        Commands::Tag { tag_cmd } => {
            run_tag_cmd(global_opts, tag_cmd).await;
        }
        Commands::Album { album_cmd } => {
            run_album_cmd(global_opts, album_cmd).await;
        }
        Commands::Config { config_cmd } => {
            run_config_cmd(global_opts, config_cmd).await;
        }
        Commands::Task { task_cmd } => {
            run_task_cmd(global_opts, task_cmd).await;
        }
        Commands::Batch {
            title,
            description,
            media_type,
            tags,
            tags_mode,
            album,
            everything,
            batch_command,
        } => {
            run_cmd_batch_operation(
                global_opts,
                title,
                description,
                media_type,
                tags,
                tags_mode,
                album,
                everything,
                batch_command,
            )
            .await;
        }
        Commands::Invites { invites_cmd } => {
            run_invites_cmd(global_opts, invites_cmd).await;
        },
        Commands::GetServerInformation => {
            run_cmd_server_info(global_opts).await;
        },
    }
}

pub fn get_vault_url(global_opts_url: &Option<String>) -> String {
    match global_opts_url {
        Some(u) => u.clone(),
        None => {
            let env_val = std::env::var("PMV_URL");

            match env_val {
                Ok(u) => u,
                Err(_) => "http://localhost".to_string(),
            }
        }
    }
}

pub fn print_request_error(e: RequestError) {
    match e {
        RequestError::StatusCode(s) => {
            if s == 401 {
                eprintln!("Error: The session URL you provided was invalid or expired.");
            } else {
                eprintln!("Error: API ended with unexpected status code: {s}");
            }
        }
        RequestError::Api {
            status,
            code,
            message,
        } => {
            eprintln!("API Error | Status: {status} | Code: {code} | Message: {message}");
        }
        RequestError::NetworkError(e) => {
            eprintln!("Error: {e}");
        }
        RequestError::FileSystem(e) => {
            eprintln!("Error: {e}");
        }
        RequestError::Json { message, body } => {
            eprintln!("Body received: {body}");
            eprintln!("Error parsing the body: {message}");
            eprintln!("This may be caused due to incompatibilities between the PersonalMediaVault backend and this tool.");
            eprintln!("If you are using the latest version, you should report this a a bug:");
            eprintln!("https://github.com/AgustinSRG/pmv-cli/issues");
        }
    }
}
