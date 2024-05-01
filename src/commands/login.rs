// Login command

use std::process;

use reqwest::StatusCode;

use crate::{
    api::{api_call_context, api_call_login_invite_code},
    commands::{get_vault_url, print_request_error},
    models::InviteCodeLoginBody,
    tools::{ensure_login_ext, parse_vault_uri, VaultURI},
};

use super::CommandGlobalOptions;

pub async fn run_cmd_login(
    global_opts: CommandGlobalOptions,
    username: Option<String>,
    duration: Option<String>,
    invite_code: Option<String>,
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
                    crate::tools::RequestError::NetworkError(e) => {
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

    match invite_code {
        Some(code) => {
            vault_url = VaultURI::LoginURI {
                base_url: vault_url.get_base_url(),
                username: "".to_string(),
                password: "".to_string(),
            };

            if let VaultURI::LoginURI {
                base_url,
                username: _,
                password: _,
            } = vault_url.clone()
            {
                let login_res = api_call_login_invite_code(
                    &vault_url,
                    InviteCodeLoginBody { code: code.clone() },
                    global_opts.debug,
                )
                .await;

                if login_res.is_err() {
                    print_request_error(login_res.err().unwrap());
                    process::exit(1);
                }

                let session_id = login_res.unwrap().session_id;

                vault_url = VaultURI::SessionURI {
                    base_url: base_url.clone(),
                    session: session_id,
                }
            }
        }
        None => {
            let login_result =
                ensure_login_ext(&vault_url, &username, &duration, global_opts.debug).await;

            if login_result.is_err() {
                process::exit(1);
            }

            vault_url = login_result.unwrap();
        }
    }

    let vault_url_str = vault_url.to_url_string();

    eprintln!("Vault session opened.");

    println!("{vault_url_str}");

    eprintln!(
        "You can set PMV_URL={vault_url_str} for the next commands to be already authenticated."
    );
    eprintln!("You can also pass the session URL via the --vault-url option.");
}
