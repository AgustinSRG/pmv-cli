// Logout command

use std::{process};

use crate::{
    api::api_call_logout,
    tools::{parse_vault_uri, VaultURI},
};

use super::{CommandGlobalOptions, get_vault_url, print_request_error};

pub async fn run_cmd_logout(global_opts: CommandGlobalOptions) -> () {
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

    let vault_url = url_parse_res.unwrap();

    let logout_res = do_logout(global_opts, vault_url).await;

    match logout_res {
        Ok(_) => {
            eprintln!("Vault session closed.");
        }
        Err(_) => {
            process::exit(1);
        }
    }
}

pub async fn do_logout(global_opts: CommandGlobalOptions, vault_url: VaultURI) -> Result<(), ()> {
    match vault_url {
        crate::tools::VaultURI::LoginURI{base_url: _, username: _, password: _} => {
            eprintln!("You must provide a session URL in order to log out.");
            return Err(());
        }
        crate::tools::VaultURI::SessionURI{base_url, session} => {
            let logout_res = api_call_logout(VaultURI::SessionURI{base_url, session}, global_opts.debug).await;

            match logout_res {
                Ok(_) => {
                    return Ok(());
                }
                Err(e) => {
                    print_request_error(e);
                    return Err(());
                },
            }
        }
    }
}
