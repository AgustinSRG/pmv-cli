// Logout command

use std::process;

use crate::{
    api::api_call_logout,
    tools::{parse_vault_uri, VaultURI},
};

use super::CommandGlobalOptions;

pub async fn run_cmd_logout(global_opts: CommandGlobalOptions, url: String) -> () {
    let url_parse_res = parse_vault_uri(url);

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

    let vault_url = url_parse_res.unwrap();

    match vault_url {
        crate::tools::VaultURI::LoginURI(_) => {
            eprintln!("You must provide a session URL in order to log out.");
            process::exit(1);
        }
        crate::tools::VaultURI::SessionURI(u) => {
            if global_opts.verbose {
                eprintln!("Logging out...");
            }

            let logout_res = api_call_logout(VaultURI::SessionURI(u.clone())).await;

            match logout_res {
                Ok(_) => {
                    if global_opts.verbose {
                        eprintln!("Done");
                    }
                }
                Err(e) => {
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
                    process::exit(1);
                },
            }
        }
    }
}
