// Server info command

use crate::{api::api_call_about, tools::{ensure_login, parse_vault_uri}, commands::logout::do_logout};

use std::process;

use super::{get_vault_url, CommandGlobalOptions, print_request_error};

pub async fn run_cmd_server_info(global_opts: CommandGlobalOptions) {
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

    let api_res = api_call_about(&vault_url, global_opts.debug).await;

    match api_res {
        Ok(server_info) => {
            if logout_after_operation {
                let logout_res = do_logout(&global_opts, &vault_url).await;

                match logout_res {
                    Ok(_) => {}
                    Err(_) => {
                        process::exit(1);
                    }
                }
            }

            let res_server_version = server_info.version;
            let res_last_release = server_info.last_release;
            let res_ffmpeg_version = server_info.ffmpeg_version;

            println!("---------------------------");

            println!("Server version: {res_server_version}");
            println!("Last release: {res_last_release}");
            println!("FFmpeg version: {res_ffmpeg_version}");

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
