// Login command

use std::process;

use hyper::StatusCode;

use crate::{
    api::api_call_context,
    commands::get_vault_url,
    tools::{ensure_login, parse_vault_uri, VaultURI},
};

use super::CommandGlobalOptions;

pub async fn run_cmd_login(global_opts: CommandGlobalOptions, username: Option<String>) {
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

    if vault_url.is_session() {
        // If the URL is a session URL, check if the session is valid
        let context_api_res = api_call_context(&vault_url, global_opts.debug).await;

        match context_api_res {
            Ok(_) => {}
            Err(e) => {
                match e {
                    crate::tools::RequestError::StatusCode(status) => {
                        if status == StatusCode::UNAUTHORIZED {
                            vault_url = VaultURI::LoginURI {
                                base_url: vault_url.get_base_url(),
                                username: "".to_string(),
                                password: "".to_string(),
                            };
                        } else {
                            eprintln!("Error: API ended with unexpected status code: {status}");
                            process::exit(1);
                        }
                    }
                    crate::tools::RequestError::Api {
                        status,
                        code,
                        message,
                    } => {
                        if status == StatusCode::UNAUTHORIZED {
                            vault_url = VaultURI::LoginURI {
                                base_url: vault_url.get_base_url(),
                                username: "".to_string(),
                                password: "".to_string(),
                            };
                        } else {
                            eprintln!(
                                "API Error | Status: {status} | Code: {code} | Message: {message}"
                            );
                            process::exit(1);
                        }
                    }
                    crate::tools::RequestError::Hyper(e) => {
                        eprintln!("Error: {e}");
                        process::exit(1);
                    }
                    crate::tools::RequestError::FileSystem(e) => {
                        eprintln!("Error: {e}");
                        process::exit(1);
                    }
                    crate::tools::RequestError::Json { message, body } => {
                        eprintln!("Body received: {body}");
                        eprintln!("Error parsing the body: {message}");
                        eprintln!("This may be caused due to incompatibilities between the PersonalMediaVault backend and this tool.");
                        eprintln!(
                            "If you are using the latest version, you should report this a a bug:"
                        );
                        eprintln!("https://github.com/AgustinSRG/pmv-cli/issues");
                        process::exit(1);
                    }
                };
            }
        }
    }

    let login_result = ensure_login(&vault_url, &username, global_opts.debug).await;

    if login_result.is_err() {
        process::exit(1);
    }

    vault_url = login_result.unwrap();

    let vault_url_str = vault_url.to_url_string();

    eprintln!("Vault session opened.");

    println!("{vault_url_str}");

    eprintln!(
        "You can set PMV_URL={vault_url_str} for the next commands to be already authenticated."
    );
    eprintln!("You can also pass the session URL via the --vault-url option.");
}
