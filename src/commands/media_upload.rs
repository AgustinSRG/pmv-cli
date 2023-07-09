// Media upload command

use std::{
    process,
    sync::{Arc, Mutex},
};

use tokio::join;
use unicode_width::UnicodeWidthStr;

use crate::{
    api::api_call_get_media,
    commands::logout::do_logout,
    models::{ConfigImageResolution, ConfigVideoResolution, TaskEncodeResolution},
    tools::{
        ask_user, do_get_download_request, ensure_login, parse_identifier,
        parse_vault_uri, VaultURI,
    },
};

use super::{get_vault_url, print_request_error, CommandGlobalOptions};

pub async fn run_cmd_download_media(
    global_opts: CommandGlobalOptions,
    path: String,
    title: Option<String>,
    album: Option<String>,
    tags: Option<String>,
) {
    let url_parse_res = parse_vault_uri(get_vault_url(global_opts.vault_url.clone()));

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
    let login_result = ensure_login(vault_url, None, global_opts.debug).await;

    if login_result.is_err() {
        process::exit(1);
    }

    vault_url = login_result.unwrap();

    // Params

    
}
