// Tool to ensure login, or ask for credentials from stdin

use crate::{tools::{ask_user, ask_user_password}, models::Credentials, api::api_call_login, commands::print_request_error};

use super::VaultURI;

pub async fn ensure_login(url: VaultURI, given_username: Option<String>, debug: bool) -> Result<VaultURI, ()> {
    match url.clone() {
        VaultURI::LoginURI{base_url, username, password} => {
            let mut username_m = username.clone();

            if given_username.is_some() {
                username_m = given_username.unwrap();
            } else if username.is_empty() {
                // Ask username
                eprintln!("Input username for vault: {base_url}");
                username_m = ask_user("Username: ").await.unwrap_or("".to_string());
            }

            let mut password_m = password.clone();

            if password_m.is_empty() {
                // Ask password
                eprintln!("Input password for vault: {base_url}");
                password_m = ask_user_password("Password: ").await.unwrap_or("".to_string());
            }

            // Login

            let login_res = api_call_login(url.clone(), Credentials{
                username: username_m,
                password: password_m,
            }, debug).await;

            if login_res.is_err() {
                print_request_error(login_res.err().unwrap());
                return Err(());
            }

            let session_id = login_res.unwrap().session_id;
            
            return Ok(VaultURI::SessionURI{
                base_url: base_url.clone(),
                session: session_id,
            });
        },
        VaultURI::SessionURI{base_url, session: _} => {
            // Session URI is already logged in
            if debug {
                eprintln!("DEBUG: Provided session URL for vault: {base_url}");
            }
            return Ok(url);
        }
    }
}
