// Login command

use std::process;

use crate::tools::{parse_vault_uri, ensure_login};

use super::CommandGlobalOptions;

pub async fn run_cmd_login(global_opts: CommandGlobalOptions, url: String, username: Option<String>) -> () {
    let url_parse_res = parse_vault_uri(url);

    if url_parse_res.is_err() {
        match url_parse_res.err().unwrap() {
            crate::tools::VaultURIParseError::InvalidProtocol => {
                eprintln!("Invalid vault URL provided. Must be an HTTP or HTTPS URL.");
            },
            crate::tools::VaultURIParseError::URLError(e) => {
                let err_msg = e.to_string();
                eprintln!("Invalid vault URL provided: {err_msg}");
            },
        }

        process::exit(1);
    }

    let mut vault_url = url_parse_res.unwrap();

    let login_result = ensure_login(vault_url, username, global_opts.verbose).await;

    if login_result.is_err() {
        process::exit(1);
    }

    vault_url = login_result.unwrap();

    let vault_url_str = vault_url.to_string();

    eprintln!("{vault_url_str}");
}
