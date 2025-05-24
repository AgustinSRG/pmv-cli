// Configuration command

use std::process;

use clap::Subcommand;
use tokio::fs;

use crate::{
    api::{api_call_get_config, api_call_set_config},
    commands::logout::do_logout,
    models::{ConfigImageResolution, ConfigVideoResolution, VaultConfig},
    tools::{ask_user, ensure_login, parse_vault_uri},
};

use super::{get_vault_url, print_request_error, CommandGlobalOptions};

#[derive(Subcommand)]
pub enum ConfigCommand {
    /// Gets vault configuration
    Get,

    /// Gets custom CSS code configured for the vault
    GetCSS,

    /// Sets vault title
    SetTitle {
        /// Vault title
        title: String,
    },

    /// Sets max tasks in parallel
    SetMaxTasks {
        /// Max tasks in parallel
        max_tasks: i32,
    },

    /// Sets number of encoding threads to use
    SetEncodingThreads {
        /// Number of encoding threads to use
        encoding_threads: i32,
    },

    /// Sets the video previews interval in seconds
    SetVideoPreviewsInterval {
        /// Interval in seconds
        interval_seconds: i32,
    },

    /// Sets the max number of invited sessions by user
    SetMaxInvites {
        /// Max number of invited sessions by user
        invite_limit: i32,
    },

    /// Sets the option to preserve original files, before encoding, as an attachment
    SetPreserveOriginals {
        /// Preserve original media, before encoding, as an attachment?
        preserve_originals: bool,
    },

    /// Sets custom CSS for the vault
    SetCSS {
        /// Path to the css file to use
        file_path: String,
    },

    /// Clears custom CSS for the vault
    ClearCSS,

    /// Adds video resolution
    AddVideoResolution {
        /// Video resolution. Example: 1280x720:30
        resolution: String,
    },

    /// Removes video resolution
    RemoveVideoResolution {
        /// Video resolution. Example: 1280x720:30
        resolution: String,
    },

    /// Adds image resolution
    AddImageResolution {
        /// Image resolution. Example: 1280x720
        resolution: String,
    },

    /// Removes image resolution
    RemoveImageResolution {
        /// Image resolution. Example: 1280x720
        resolution: String,
    },
}

pub async fn run_config_cmd(global_opts: CommandGlobalOptions, cmd: ConfigCommand) {
    match cmd {
        ConfigCommand::Get => {
            run_cmd_config_get(global_opts).await;
        }
        ConfigCommand::GetCSS => {
            run_cmd_config_get_css(global_opts).await;
        }
        ConfigCommand::SetTitle { title } => {
            run_cmd_config_set_title(global_opts, title).await;
        }
        ConfigCommand::SetMaxTasks { max_tasks } => {
            run_cmd_config_set_max_tasks(global_opts, max_tasks).await;
        }
        ConfigCommand::SetEncodingThreads { encoding_threads } => {
            run_cmd_config_set_encoding_threads(global_opts, encoding_threads).await;
        }
        ConfigCommand::SetVideoPreviewsInterval { interval_seconds } => {
            run_cmd_config_set_video_previews_interval(global_opts, interval_seconds).await;
        }
        ConfigCommand::SetMaxInvites { invite_limit } => {
            run_cmd_config_set_invite_limit(global_opts, invite_limit).await;
        }
        ConfigCommand::SetPreserveOriginals { preserve_originals } => {
            run_cmd_config_set_preserve_originals(global_opts, preserve_originals).await;
        }
        ConfigCommand::SetCSS { file_path } => {
            run_cmd_config_set_css(global_opts, file_path).await;
        }
        ConfigCommand::ClearCSS => {
            run_cmd_config_clear_css(global_opts).await;
        }
        ConfigCommand::AddVideoResolution { resolution } => {
            run_cmd_config_add_video_resolution(global_opts, resolution).await;
        }
        ConfigCommand::RemoveVideoResolution { resolution } => {
            run_cmd_config_remove_video_resolution(global_opts, resolution).await;
        }
        ConfigCommand::AddImageResolution { resolution } => {
            run_cmd_config_add_image_resolution(global_opts, resolution).await;
        }
        ConfigCommand::RemoveImageResolution { resolution } => {
            run_cmd_config_remove_image_resolution(global_opts, resolution).await;
        }
    }
}

pub async fn run_cmd_config_get(global_opts: CommandGlobalOptions) {
    let url_parse_res = parse_vault_uri(get_vault_url(&global_opts.vault_url));

    if url_parse_res.is_err() {
        match url_parse_res.err().unwrap() {
            crate::tools::VaultURIParseError::InvalidProtocol => {
                eprintln!("Invalid vault URL provided. Must be an HTTP or HTTPS URL.");
            }
            crate::tools::VaultURIParseError::URLError(e) => {
                let err_msg = e.to_string();
                eprintln!("Invalid vault URL provided: {err_msg}");
            }
        }

        process::exit(1);
    }

    let mut vault_url = url_parse_res.unwrap();

    let logout_after_operation = vault_url.is_login();
    let login_result = ensure_login(&vault_url, &None, global_opts.debug).await;

    if login_result.is_err() {
        process::exit(1);
    }

    vault_url = login_result.unwrap();

    // Call API

    let api_res = api_call_get_config(&vault_url, global_opts.debug).await;

    match api_res {
        Ok(config) => {
            if logout_after_operation {
                let logout_res = do_logout(&global_opts, &vault_url).await;

                match logout_res {
                    Ok(_) => {}
                    Err(_) => {
                        process::exit(1);
                    }
                }
            }

            println!("---------------------------");

            if let Some(title) = config.title {
                println!("Vault title: {title}");
            }

            if let Some(css) = config.css {
                let css_len = css.len();
                println!("Custom CSS: {css_len} Bytes | Use the 'get-css' command to retrieve it.");
            }

            let res_max_tasks = config.max_tasks;
            let res_encoding_threads = config.encoding_threads;
            let mut res_video_previews_interval = config.video_previews_interval.unwrap_or(0);

            if res_video_previews_interval <= 0 {
                res_video_previews_interval = 3;
            }

            let mut invite_limit = config.invite_limit;

            if invite_limit == 0 {
                invite_limit = 10;
            }

            println!("Max tasks in parallel: {res_max_tasks}");
            println!("Number of encoding threads: {res_encoding_threads}");
            println!("Video previews interval: {res_video_previews_interval} seconds");
            println!("Max number of invited sessions by user: {invite_limit}");
            println!(
                "Preserve original files, before encoding, as an attachment?: {}",
                if config.preserve_originals {
                    "Yes"
                } else {
                    "No"
                }
            );

            if !config.resolutions.is_empty() {
                let list: Vec<String> = config
                    .resolutions
                    .iter()
                    .map(|r| r.to_resolution_string())
                    .collect();
                let list_str = list.join(", ");

                println!("Video resolutions: {list_str}");
            }

            if !config.image_resolutions.is_empty() {
                let list: Vec<String> = config
                    .image_resolutions
                    .iter()
                    .map(|r| r.to_resolution_string())
                    .collect();
                let list_str = list.join(", ");

                println!("Image resolutions: {list_str}");
            }

            println!("---------------------------");
        }
        Err(e) => {
            print_request_error(e);
            if logout_after_operation {
                let logout_res = do_logout(&global_opts, &vault_url).await;

                match logout_res {
                    Ok(_) => {}
                    Err(_) => {
                        process::exit(1);
                    }
                }
            }
            process::exit(1);
        }
    }
}

pub async fn run_cmd_config_get_css(global_opts: CommandGlobalOptions) {
    let url_parse_res = parse_vault_uri(get_vault_url(&global_opts.vault_url));

    if url_parse_res.is_err() {
        match url_parse_res.err().unwrap() {
            crate::tools::VaultURIParseError::InvalidProtocol => {
                eprintln!("Invalid vault URL provided. Must be an HTTP or HTTPS URL.");
            }
            crate::tools::VaultURIParseError::URLError(e) => {
                let err_msg = e.to_string();
                eprintln!("Invalid vault URL provided: {err_msg}");
            }
        }

        process::exit(1);
    }

    let mut vault_url = url_parse_res.unwrap();

    let logout_after_operation = vault_url.is_login();
    let login_result = ensure_login(&vault_url, &None, global_opts.debug).await;

    if login_result.is_err() {
        process::exit(1);
    }

    vault_url = login_result.unwrap();

    // Call API

    let api_res = api_call_get_config(&vault_url, global_opts.debug).await;

    match api_res {
        Ok(config) => {
            if logout_after_operation {
                let logout_res = do_logout(&global_opts, &vault_url).await;

                match logout_res {
                    Ok(_) => {}
                    Err(_) => {
                        process::exit(1);
                    }
                }
            }

            if let Some(css) = config.css {
                println!("{css}");
            }
        }
        Err(e) => {
            print_request_error(e);
            if logout_after_operation {
                let logout_res = do_logout(&global_opts, &vault_url).await;

                match logout_res {
                    Ok(_) => {}
                    Err(_) => {
                        process::exit(1);
                    }
                }
            }
            process::exit(1);
        }
    }
}

pub async fn run_cmd_config_set_title(global_opts: CommandGlobalOptions, title: String) {
    let url_parse_res = parse_vault_uri(get_vault_url(&global_opts.vault_url));

    if url_parse_res.is_err() {
        match url_parse_res.err().unwrap() {
            crate::tools::VaultURIParseError::InvalidProtocol => {
                eprintln!("Invalid vault URL provided. Must be an HTTP or HTTPS URL.");
            }
            crate::tools::VaultURIParseError::URLError(e) => {
                let err_msg = e.to_string();
                eprintln!("Invalid vault URL provided: {err_msg}");
            }
        }

        process::exit(1);
    }

    let mut vault_url = url_parse_res.unwrap();

    let logout_after_operation = vault_url.is_login();
    let login_result = ensure_login(&vault_url, &None, global_opts.debug).await;

    if login_result.is_err() {
        process::exit(1);
    }

    vault_url = login_result.unwrap();

    // Get config

    let api_res_get_conf = api_call_get_config(&vault_url, global_opts.debug).await;

    let current_config: VaultConfig = match api_res_get_conf {
        Ok(config) => config,
        Err(e) => {
            print_request_error(e);
            if logout_after_operation {
                let logout_res = do_logout(&global_opts, &vault_url).await;

                match logout_res {
                    Ok(_) => {}
                    Err(_) => {
                        process::exit(1);
                    }
                }
            }
            process::exit(1);
        }
    };

    // Changes

    let mut new_config = current_config.clone();

    new_config.title = Some(title.clone());

    // Set config

    let api_res_set_conf = api_call_set_config(&vault_url, new_config, global_opts.debug).await;

    match api_res_set_conf {
        Ok(_) => {
            eprintln!("Successfully changed vault title: {title}");
        }
        Err(e) => {
            print_request_error(e);
            if logout_after_operation {
                let logout_res = do_logout(&global_opts, &vault_url).await;

                match logout_res {
                    Ok(_) => {}
                    Err(_) => {
                        process::exit(1);
                    }
                }
            }
            process::exit(1);
        }
    }
}

pub async fn run_cmd_config_set_max_tasks(global_opts: CommandGlobalOptions, max_tasks: i32) {
    let url_parse_res = parse_vault_uri(get_vault_url(&global_opts.vault_url));

    if url_parse_res.is_err() {
        match url_parse_res.err().unwrap() {
            crate::tools::VaultURIParseError::InvalidProtocol => {
                eprintln!("Invalid vault URL provided. Must be an HTTP or HTTPS URL.");
            }
            crate::tools::VaultURIParseError::URLError(e) => {
                let err_msg = e.to_string();
                eprintln!("Invalid vault URL provided: {err_msg}");
            }
        }

        process::exit(1);
    }

    let mut vault_url = url_parse_res.unwrap();

    let logout_after_operation = vault_url.is_login();
    let login_result = ensure_login(&vault_url, &None, global_opts.debug).await;

    if login_result.is_err() {
        process::exit(1);
    }

    vault_url = login_result.unwrap();

    // Get config

    let api_res_get_conf = api_call_get_config(&vault_url, global_opts.debug).await;

    let current_config: VaultConfig = match api_res_get_conf {
        Ok(config) => config,
        Err(e) => {
            print_request_error(e);
            if logout_after_operation {
                let logout_res = do_logout(&global_opts, &vault_url).await;

                match logout_res {
                    Ok(_) => {}
                    Err(_) => {
                        process::exit(1);
                    }
                }
            }
            process::exit(1);
        }
    };

    // Changes

    let mut new_config = current_config.clone();

    new_config.max_tasks = max_tasks;

    // Set config

    let api_res_set_conf = api_call_set_config(&vault_url, new_config, global_opts.debug).await;

    match api_res_set_conf {
        Ok(_) => {
            eprintln!("Successfully changed max number of parallel tasks: {max_tasks}");
        }
        Err(e) => {
            print_request_error(e);
            if logout_after_operation {
                let logout_res = do_logout(&global_opts, &vault_url).await;

                match logout_res {
                    Ok(_) => {}
                    Err(_) => {
                        process::exit(1);
                    }
                }
            }
            process::exit(1);
        }
    }
}

pub async fn run_cmd_config_set_encoding_threads(
    global_opts: CommandGlobalOptions,
    encoding_threads: i32,
) {
    let url_parse_res = parse_vault_uri(get_vault_url(&global_opts.vault_url));

    if url_parse_res.is_err() {
        match url_parse_res.err().unwrap() {
            crate::tools::VaultURIParseError::InvalidProtocol => {
                eprintln!("Invalid vault URL provided. Must be an HTTP or HTTPS URL.");
            }
            crate::tools::VaultURIParseError::URLError(e) => {
                let err_msg = e.to_string();
                eprintln!("Invalid vault URL provided: {err_msg}");
            }
        }

        process::exit(1);
    }

    let mut vault_url = url_parse_res.unwrap();

    let logout_after_operation = vault_url.is_login();
    let login_result = ensure_login(&vault_url, &None, global_opts.debug).await;

    if login_result.is_err() {
        process::exit(1);
    }

    vault_url = login_result.unwrap();

    // Get config

    let api_res_get_conf = api_call_get_config(&vault_url, global_opts.debug).await;

    let current_config: VaultConfig = match api_res_get_conf {
        Ok(config) => config,
        Err(e) => {
            print_request_error(e);
            if logout_after_operation {
                let logout_res = do_logout(&global_opts, &vault_url).await;

                match logout_res {
                    Ok(_) => {}
                    Err(_) => {
                        process::exit(1);
                    }
                }
            }
            process::exit(1);
        }
    };

    // Changes

    let mut new_config = current_config.clone();

    new_config.encoding_threads = encoding_threads;

    // Set config

    let api_res_set_conf = api_call_set_config(&vault_url, new_config, global_opts.debug).await;

    match api_res_set_conf {
        Ok(_) => {
            eprintln!("Successfully changed number of encoding threads: {encoding_threads}");
        }
        Err(e) => {
            print_request_error(e);
            if logout_after_operation {
                let logout_res = do_logout(&global_opts, &vault_url).await;

                match logout_res {
                    Ok(_) => {}
                    Err(_) => {
                        process::exit(1);
                    }
                }
            }
            process::exit(1);
        }
    }
}

pub async fn run_cmd_config_set_video_previews_interval(
    global_opts: CommandGlobalOptions,
    interval_seconds: i32,
) {
    let url_parse_res = parse_vault_uri(get_vault_url(&global_opts.vault_url));

    if url_parse_res.is_err() {
        match url_parse_res.err().unwrap() {
            crate::tools::VaultURIParseError::InvalidProtocol => {
                eprintln!("Invalid vault URL provided. Must be an HTTP or HTTPS URL.");
            }
            crate::tools::VaultURIParseError::URLError(e) => {
                let err_msg = e.to_string();
                eprintln!("Invalid vault URL provided: {err_msg}");
            }
        }

        process::exit(1);
    }

    let mut vault_url = url_parse_res.unwrap();

    let logout_after_operation = vault_url.is_login();
    let login_result = ensure_login(&vault_url, &None, global_opts.debug).await;

    if login_result.is_err() {
        process::exit(1);
    }

    vault_url = login_result.unwrap();

    // Get config

    let api_res_get_conf = api_call_get_config(&vault_url, global_opts.debug).await;

    let current_config: VaultConfig = match api_res_get_conf {
        Ok(config) => config,
        Err(e) => {
            print_request_error(e);
            if logout_after_operation {
                let logout_res = do_logout(&global_opts, &vault_url).await;

                match logout_res {
                    Ok(_) => {}
                    Err(_) => {
                        process::exit(1);
                    }
                }
            }
            process::exit(1);
        }
    };

    // Changes

    let mut new_config = current_config.clone();

    new_config.video_previews_interval = Some(interval_seconds);

    // Set config

    let api_res_set_conf = api_call_set_config(&vault_url, new_config, global_opts.debug).await;

    match api_res_set_conf {
        Ok(_) => {
            let interval_seconds_fixed: i32 = if interval_seconds > 0 {
                interval_seconds
            } else {
                3
            };
            eprintln!(
                "Successfully changed video previews interval: {interval_seconds_fixed} seconds"
            );
        }
        Err(e) => {
            print_request_error(e);
            if logout_after_operation {
                let logout_res = do_logout(&global_opts, &vault_url).await;

                match logout_res {
                    Ok(_) => {}
                    Err(_) => {
                        process::exit(1);
                    }
                }
            }
            process::exit(1);
        }
    }
}

pub async fn run_cmd_config_set_invite_limit(global_opts: CommandGlobalOptions, invite_limit: i32) {
    let url_parse_res = parse_vault_uri(get_vault_url(&global_opts.vault_url));

    if url_parse_res.is_err() {
        match url_parse_res.err().unwrap() {
            crate::tools::VaultURIParseError::InvalidProtocol => {
                eprintln!("Invalid vault URL provided. Must be an HTTP or HTTPS URL.");
            }
            crate::tools::VaultURIParseError::URLError(e) => {
                let err_msg = e.to_string();
                eprintln!("Invalid vault URL provided: {err_msg}");
            }
        }

        process::exit(1);
    }

    let mut vault_url = url_parse_res.unwrap();

    let logout_after_operation = vault_url.is_login();
    let login_result = ensure_login(&vault_url, &None, global_opts.debug).await;

    if login_result.is_err() {
        process::exit(1);
    }

    vault_url = login_result.unwrap();

    // Get config

    let api_res_get_conf = api_call_get_config(&vault_url, global_opts.debug).await;

    let current_config: VaultConfig = match api_res_get_conf {
        Ok(config) => config,
        Err(e) => {
            print_request_error(e);
            if logout_after_operation {
                let logout_res = do_logout(&global_opts, &vault_url).await;

                match logout_res {
                    Ok(_) => {}
                    Err(_) => {
                        process::exit(1);
                    }
                }
            }
            process::exit(1);
        }
    };

    // Changes

    let mut new_config = current_config.clone();

    new_config.invite_limit = invite_limit;

    // Set config

    let api_res_set_conf = api_call_set_config(&vault_url, new_config, global_opts.debug).await;

    match api_res_set_conf {
        Ok(_) => {
            eprintln!(
                "Successfully changed max number of invited sessions by user: {invite_limit}"
            );
        }
        Err(e) => {
            print_request_error(e);
            if logout_after_operation {
                let logout_res = do_logout(&global_opts, &vault_url).await;

                match logout_res {
                    Ok(_) => {}
                    Err(_) => {
                        process::exit(1);
                    }
                }
            }
            process::exit(1);
        }
    }
}

pub async fn run_cmd_config_set_preserve_originals(
    global_opts: CommandGlobalOptions,
    preserve_originals: bool,
) {
    let url_parse_res = parse_vault_uri(get_vault_url(&global_opts.vault_url));

    if url_parse_res.is_err() {
        match url_parse_res.err().unwrap() {
            crate::tools::VaultURIParseError::InvalidProtocol => {
                eprintln!("Invalid vault URL provided. Must be an HTTP or HTTPS URL.");
            }
            crate::tools::VaultURIParseError::URLError(e) => {
                let err_msg = e.to_string();
                eprintln!("Invalid vault URL provided: {err_msg}");
            }
        }

        process::exit(1);
    }

    let mut vault_url = url_parse_res.unwrap();

    let logout_after_operation = vault_url.is_login();
    let login_result = ensure_login(&vault_url, &None, global_opts.debug).await;

    if login_result.is_err() {
        process::exit(1);
    }

    vault_url = login_result.unwrap();

    // Get config

    let api_res_get_conf = api_call_get_config(&vault_url, global_opts.debug).await;

    let current_config: VaultConfig = match api_res_get_conf {
        Ok(config) => config,
        Err(e) => {
            print_request_error(e);
            if logout_after_operation {
                let logout_res = do_logout(&global_opts, &vault_url).await;

                match logout_res {
                    Ok(_) => {}
                    Err(_) => {
                        process::exit(1);
                    }
                }
            }
            process::exit(1);
        }
    };

    // Changes

    let mut new_config = current_config.clone();

    new_config.preserve_originals = preserve_originals;

    // Set config

    let api_res_set_conf = api_call_set_config(&vault_url, new_config, global_opts.debug).await;

    match api_res_set_conf {
        Ok(_) => {
            eprintln!(
                "Successfully changed the option to preserve original files: {preserve_originals}"
            );
        }
        Err(e) => {
            print_request_error(e);
            if logout_after_operation {
                let logout_res = do_logout(&global_opts, &vault_url).await;

                match logout_res {
                    Ok(_) => {}
                    Err(_) => {
                        process::exit(1);
                    }
                }
            }
            process::exit(1);
        }
    }
}

pub async fn run_cmd_config_set_css(global_opts: CommandGlobalOptions, file_path: String) {
    let url_parse_res = parse_vault_uri(get_vault_url(&global_opts.vault_url));

    if url_parse_res.is_err() {
        match url_parse_res.err().unwrap() {
            crate::tools::VaultURIParseError::InvalidProtocol => {
                eprintln!("Invalid vault URL provided. Must be an HTTP or HTTPS URL.");
            }
            crate::tools::VaultURIParseError::URLError(e) => {
                let err_msg = e.to_string();
                eprintln!("Invalid vault URL provided: {err_msg}");
            }
        }

        process::exit(1);
    }

    let mut vault_url = url_parse_res.unwrap();

    let logout_after_operation = vault_url.is_login();
    let login_result = ensure_login(&vault_url, &None, global_opts.debug).await;

    if login_result.is_err() {
        process::exit(1);
    }

    vault_url = login_result.unwrap();

    // Get config

    let api_res_get_conf = api_call_get_config(&vault_url, global_opts.debug).await;

    let current_config: VaultConfig = match api_res_get_conf {
        Ok(config) => config,
        Err(e) => {
            print_request_error(e);
            if logout_after_operation {
                let logout_res = do_logout(&global_opts, &vault_url).await;

                match logout_res {
                    Ok(_) => {}
                    Err(_) => {
                        process::exit(1);
                    }
                }
            }
            process::exit(1);
        }
    };

    // Read file

    let new_css_res = fs::read_to_string(&file_path).await;

    // Changes

    let mut new_config = current_config.clone();

    match new_css_res {
        Ok(new_css) => {
            new_config.css = Some(new_css);
        }
        Err(e) => {
            let e_str = e.to_string();

            eprintln!("Error reading the file {file_path} | Error: {e_str}");

            if logout_after_operation {
                let logout_res = do_logout(&global_opts, &vault_url).await;

                match logout_res {
                    Ok(_) => {}
                    Err(_) => {
                        process::exit(1);
                    }
                }
            }
            process::exit(1);
        }
    }

    // Set config

    let api_res_set_conf = api_call_set_config(&vault_url, new_config, global_opts.debug).await;

    match api_res_set_conf {
        Ok(_) => {
            eprintln!("Successfully changed custom vault CSS");
        }
        Err(e) => {
            print_request_error(e);
            if logout_after_operation {
                let logout_res = do_logout(&global_opts, &vault_url).await;

                match logout_res {
                    Ok(_) => {}
                    Err(_) => {
                        process::exit(1);
                    }
                }
            }
            process::exit(1);
        }
    }
}

pub async fn run_cmd_config_clear_css(global_opts: CommandGlobalOptions) {
    let url_parse_res = parse_vault_uri(get_vault_url(&global_opts.vault_url));

    if url_parse_res.is_err() {
        match url_parse_res.err().unwrap() {
            crate::tools::VaultURIParseError::InvalidProtocol => {
                eprintln!("Invalid vault URL provided. Must be an HTTP or HTTPS URL.");
            }
            crate::tools::VaultURIParseError::URLError(e) => {
                let err_msg = e.to_string();
                eprintln!("Invalid vault URL provided: {err_msg}");
            }
        }

        process::exit(1);
    }

    let mut vault_url = url_parse_res.unwrap();

    let logout_after_operation = vault_url.is_login();
    let login_result = ensure_login(&vault_url, &None, global_opts.debug).await;

    if login_result.is_err() {
        process::exit(1);
    }

    vault_url = login_result.unwrap();

    // Get config

    let api_res_get_conf = api_call_get_config(&vault_url, global_opts.debug).await;

    let current_config: VaultConfig = match api_res_get_conf {
        Ok(config) => config,
        Err(e) => {
            print_request_error(e);
            if logout_after_operation {
                let logout_res = do_logout(&global_opts, &vault_url).await;

                match logout_res {
                    Ok(_) => {}
                    Err(_) => {
                        process::exit(1);
                    }
                }
            }
            process::exit(1);
        }
    };

    // Ask confirmation

    if !global_opts.auto_confirm {
        eprintln!("Are you sure you want to clear the custom css?");
        let confirmation = ask_user("Continue? y/n: ").await.unwrap_or("".to_string());

        if confirmation.to_lowercase() != "y" {
            if logout_after_operation {
                let logout_res = do_logout(&global_opts, &vault_url).await;

                match logout_res {
                    Ok(_) => {}
                    Err(_) => {
                        process::exit(1);
                    }
                }
            }
            process::exit(1);
        }
    }

    // Changes

    let mut new_config = current_config.clone();

    new_config.css = Some("".to_string());

    // Set config

    let api_res_set_conf = api_call_set_config(&vault_url, new_config, global_opts.debug).await;

    match api_res_set_conf {
        Ok(_) => {
            eprintln!("Successfully cleared custom vault CSS");
        }
        Err(e) => {
            print_request_error(e);
            if logout_after_operation {
                let logout_res = do_logout(&global_opts, &vault_url).await;

                match logout_res {
                    Ok(_) => {}
                    Err(_) => {
                        process::exit(1);
                    }
                }
            }
            process::exit(1);
        }
    }
}

pub async fn run_cmd_config_add_video_resolution(
    global_opts: CommandGlobalOptions,
    resolution: String,
) {
    let url_parse_res = parse_vault_uri(get_vault_url(&global_opts.vault_url));

    if url_parse_res.is_err() {
        match url_parse_res.err().unwrap() {
            crate::tools::VaultURIParseError::InvalidProtocol => {
                eprintln!("Invalid vault URL provided. Must be an HTTP or HTTPS URL.");
            }
            crate::tools::VaultURIParseError::URLError(e) => {
                let err_msg = e.to_string();
                eprintln!("Invalid vault URL provided: {err_msg}");
            }
        }

        process::exit(1);
    }

    let mut vault_url = url_parse_res.unwrap();

    let logout_after_operation = vault_url.is_login();
    let login_result = ensure_login(&vault_url, &None, global_opts.debug).await;

    if login_result.is_err() {
        process::exit(1);
    }

    vault_url = login_result.unwrap();

    // Get config

    let api_res_get_conf = api_call_get_config(&vault_url, global_opts.debug).await;

    let current_config: VaultConfig = match api_res_get_conf {
        Ok(config) => config,
        Err(e) => {
            print_request_error(e);
            if logout_after_operation {
                let logout_res = do_logout(&global_opts, &vault_url).await;

                match logout_res {
                    Ok(_) => {}
                    Err(_) => {
                        process::exit(1);
                    }
                }
            }
            process::exit(1);
        }
    };

    // Param

    let parse_res = ConfigVideoResolution::from_str(&resolution);
    let parsed_resolution: ConfigVideoResolution = match parse_res {
        Ok(r) => r,
        Err(_) => {
            eprintln!("Invalid video resolution specified: {resolution}");
            if logout_after_operation {
                let logout_res = do_logout(&global_opts, &vault_url).await;

                match logout_res {
                    Ok(_) => {}
                    Err(_) => {
                        process::exit(1);
                    }
                }
            }
            process::exit(1);
        }
    };

    let mut already_exists = false;

    for r in current_config.resolutions.iter() {
        if *r == parsed_resolution {
            already_exists = true;
            break;
        }
    }

    if already_exists {
        eprintln!("The video resolution already exists in the configuration: {resolution}");
        if logout_after_operation {
            let logout_res = do_logout(&global_opts, &vault_url).await;

            match logout_res {
                Ok(_) => {}
                Err(_) => {
                    process::exit(1);
                }
            }
        }
        process::exit(1);
    }

    // Changes

    let mut new_config = current_config.clone();

    new_config.resolutions.push(parsed_resolution);

    // Set config

    let api_res_set_conf = api_call_set_config(&vault_url, new_config, global_opts.debug).await;

    match api_res_set_conf {
        Ok(_) => {
            eprintln!("Successfully added video resolution: {resolution}");
        }
        Err(e) => {
            print_request_error(e);
            if logout_after_operation {
                let logout_res = do_logout(&global_opts, &vault_url).await;

                match logout_res {
                    Ok(_) => {}
                    Err(_) => {
                        process::exit(1);
                    }
                }
            }
            process::exit(1);
        }
    }
}

pub async fn run_cmd_config_remove_video_resolution(
    global_opts: CommandGlobalOptions,
    resolution: String,
) {
    let url_parse_res = parse_vault_uri(get_vault_url(&global_opts.vault_url));

    if url_parse_res.is_err() {
        match url_parse_res.err().unwrap() {
            crate::tools::VaultURIParseError::InvalidProtocol => {
                eprintln!("Invalid vault URL provided. Must be an HTTP or HTTPS URL.");
            }
            crate::tools::VaultURIParseError::URLError(e) => {
                let err_msg = e.to_string();
                eprintln!("Invalid vault URL provided: {err_msg}");
            }
        }

        process::exit(1);
    }

    let mut vault_url = url_parse_res.unwrap();

    let logout_after_operation = vault_url.is_login();
    let login_result = ensure_login(&vault_url, &None, global_opts.debug).await;

    if login_result.is_err() {
        process::exit(1);
    }

    vault_url = login_result.unwrap();

    // Get config

    let api_res_get_conf = api_call_get_config(&vault_url, global_opts.debug).await;

    let current_config: VaultConfig = match api_res_get_conf {
        Ok(config) => config,
        Err(e) => {
            print_request_error(e);
            if logout_after_operation {
                let logout_res = do_logout(&global_opts, &vault_url).await;

                match logout_res {
                    Ok(_) => {}
                    Err(_) => {
                        process::exit(1);
                    }
                }
            }
            process::exit(1);
        }
    };

    // Param

    let parse_res = ConfigVideoResolution::from_str(&resolution);

    let parsed_resolution: ConfigVideoResolution = match parse_res {
        Ok(r) => r,
        Err(_) => {
            eprintln!("Invalid video resolution specified: {resolution}");
            if logout_after_operation {
                let logout_res = do_logout(&global_opts, &vault_url).await;

                match logout_res {
                    Ok(_) => {}
                    Err(_) => {
                        process::exit(1);
                    }
                }
            }
            process::exit(1);
        }
    };

    let mut already_exists = false;

    for r in current_config.resolutions.iter() {
        if *r == parsed_resolution {
            already_exists = true;
            break;
        }
    }

    if !already_exists {
        eprintln!("The video resolution was not found in the configuration: {resolution}");
        if logout_after_operation {
            let logout_res = do_logout(&global_opts, &vault_url).await;

            match logout_res {
                Ok(_) => {}
                Err(_) => {
                    process::exit(1);
                }
            }
        }
        process::exit(1);
    }

    // Changes

    let mut new_config = current_config.clone();

    new_config.resolutions.retain(|r| *r != parsed_resolution);

    // Set config

    let api_res_set_conf = api_call_set_config(&vault_url, new_config, global_opts.debug).await;

    match api_res_set_conf {
        Ok(_) => {
            eprintln!("Successfully removed video resolution: {resolution}");
        }
        Err(e) => {
            print_request_error(e);
            if logout_after_operation {
                let logout_res = do_logout(&global_opts, &vault_url).await;

                match logout_res {
                    Ok(_) => {}
                    Err(_) => {
                        process::exit(1);
                    }
                }
            }
            process::exit(1);
        }
    }
}

pub async fn run_cmd_config_add_image_resolution(
    global_opts: CommandGlobalOptions,
    resolution: String,
) {
    let url_parse_res = parse_vault_uri(get_vault_url(&global_opts.vault_url));

    if url_parse_res.is_err() {
        match url_parse_res.err().unwrap() {
            crate::tools::VaultURIParseError::InvalidProtocol => {
                eprintln!("Invalid vault URL provided. Must be an HTTP or HTTPS URL.");
            }
            crate::tools::VaultURIParseError::URLError(e) => {
                let err_msg = e.to_string();
                eprintln!("Invalid vault URL provided: {err_msg}");
            }
        }

        process::exit(1);
    }

    let mut vault_url = url_parse_res.unwrap();

    let logout_after_operation = vault_url.is_login();
    let login_result = ensure_login(&vault_url, &None, global_opts.debug).await;

    if login_result.is_err() {
        process::exit(1);
    }

    vault_url = login_result.unwrap();

    // Get config

    let api_res_get_conf = api_call_get_config(&vault_url, global_opts.debug).await;

    let current_config: VaultConfig = match api_res_get_conf {
        Ok(config) => config,
        Err(e) => {
            print_request_error(e);
            if logout_after_operation {
                let logout_res = do_logout(&global_opts, &vault_url).await;

                match logout_res {
                    Ok(_) => {}
                    Err(_) => {
                        process::exit(1);
                    }
                }
            }
            process::exit(1);
        }
    };

    // Param

    let parse_res = ConfigImageResolution::from_str(&resolution);

    let parsed_resolution: ConfigImageResolution = match parse_res {
        Ok(r) => r,
        Err(_) => {
            eprintln!("Invalid image resolution specified: {resolution}");
            if logout_after_operation {
                let logout_res = do_logout(&global_opts, &vault_url).await;

                match logout_res {
                    Ok(_) => {}
                    Err(_) => {
                        process::exit(1);
                    }
                }
            }
            process::exit(1);
        }
    };

    let mut already_exists = false;

    for r in current_config.image_resolutions.iter() {
        if *r == parsed_resolution {
            already_exists = true;
            break;
        }
    }

    if already_exists {
        eprintln!("The image resolution already exists in the configuration: {resolution}");
        if logout_after_operation {
            let logout_res = do_logout(&global_opts, &vault_url).await;

            match logout_res {
                Ok(_) => {}
                Err(_) => {
                    process::exit(1);
                }
            }
        }
        process::exit(1);
    }

    // Changes

    let mut new_config = current_config.clone();

    new_config.image_resolutions.push(parsed_resolution);

    // Set config

    let api_res_set_conf = api_call_set_config(&vault_url, new_config, global_opts.debug).await;

    match api_res_set_conf {
        Ok(_) => {
            eprintln!("Successfully added image resolution: {resolution}");
        }
        Err(e) => {
            print_request_error(e);
            if logout_after_operation {
                let logout_res = do_logout(&global_opts, &vault_url).await;

                match logout_res {
                    Ok(_) => {}
                    Err(_) => {
                        process::exit(1);
                    }
                }
            }
            process::exit(1);
        }
    }
}

pub async fn run_cmd_config_remove_image_resolution(
    global_opts: CommandGlobalOptions,
    resolution: String,
) {
    let url_parse_res = parse_vault_uri(get_vault_url(&global_opts.vault_url));

    if url_parse_res.is_err() {
        match url_parse_res.err().unwrap() {
            crate::tools::VaultURIParseError::InvalidProtocol => {
                eprintln!("Invalid vault URL provided. Must be an HTTP or HTTPS URL.");
            }
            crate::tools::VaultURIParseError::URLError(e) => {
                let err_msg = e.to_string();
                eprintln!("Invalid vault URL provided: {err_msg}");
            }
        }

        process::exit(1);
    }

    let mut vault_url = url_parse_res.unwrap();

    let logout_after_operation = vault_url.is_login();
    let login_result = ensure_login(&vault_url, &None, global_opts.debug).await;

    if login_result.is_err() {
        process::exit(1);
    }

    vault_url = login_result.unwrap();

    // Get config

    let api_res_get_conf = api_call_get_config(&vault_url, global_opts.debug).await;

    let current_config: VaultConfig = match api_res_get_conf {
        Ok(config) => config,
        Err(e) => {
            print_request_error(e);
            if logout_after_operation {
                let logout_res = do_logout(&global_opts, &vault_url).await;

                match logout_res {
                    Ok(_) => {}
                    Err(_) => {
                        process::exit(1);
                    }
                }
            }
            process::exit(1);
        }
    };

    // Param

    let parse_res = ConfigImageResolution::from_str(&resolution);

    let parsed_resolution: ConfigImageResolution = match parse_res {
        Ok(r) => r,
        Err(_) => {
            eprintln!("Invalid image resolution specified: {resolution}");
            if logout_after_operation {
                let logout_res = do_logout(&global_opts, &vault_url).await;

                match logout_res {
                    Ok(_) => {}
                    Err(_) => {
                        process::exit(1);
                    }
                }
            }
            process::exit(1);
        }
    };

    let mut already_exists = false;

    for r in current_config.image_resolutions.iter() {
        if *r == parsed_resolution {
            already_exists = true;
            break;
        }
    }

    if !already_exists {
        eprintln!("The image resolution was not found in the configuration: {resolution}");
        if logout_after_operation {
            let logout_res = do_logout(&global_opts, &vault_url).await;

            match logout_res {
                Ok(_) => {}
                Err(_) => {
                    process::exit(1);
                }
            }
        }
        process::exit(1);
    }

    // Changes

    let mut new_config = current_config.clone();

    new_config
        .image_resolutions
        .retain(|r| *r != parsed_resolution);

    // Set config

    let api_res_set_conf = api_call_set_config(&vault_url, new_config, global_opts.debug).await;

    match api_res_set_conf {
        Ok(_) => {
            eprintln!("Successfully removed image resolution: {resolution}");
        }
        Err(e) => {
            print_request_error(e);
            if logout_after_operation {
                let logout_res = do_logout(&global_opts, &vault_url).await;

                match logout_res {
                    Ok(_) => {}
                    Err(_) => {
                        process::exit(1);
                    }
                }
            }
            process::exit(1);
        }
    }
}
