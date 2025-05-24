// Server info command

use crate::{
    api::api_call_disk_usage,
    commands::logout::do_logout,
    tools::{ensure_login, parse_vault_uri, render_size_bytes},
};

use std::process;

use super::{get_vault_url, print_request_error, CommandGlobalOptions};

pub async fn run_cmd_disk_usage(global_opts: CommandGlobalOptions) {
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

    let api_res = api_call_disk_usage(&vault_url, global_opts.debug).await;

    match api_res {
        Ok(disk_usage) => {
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

            println!("Disk usage: {}", disk_usage.usage.round());
            println!("Available: {}", render_size_bytes(disk_usage.available));
            println!("Free: {}", render_size_bytes(disk_usage.free));
            println!("Total size: {}", render_size_bytes(disk_usage.total));

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
