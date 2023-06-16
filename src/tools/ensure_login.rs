// Tool to ensure login, or ask for credentials from stdin

use crate::{tools::{ask_user, ask_user_password}, models::Credentials, api::api_call_login};

use super::VaultURI;

pub async fn ensure_login(url: VaultURI, given_username: Option<String>, verbose: bool) -> Result<VaultURI, ()> {
    match url.clone() {
        VaultURI::LoginURI(u) => {
            let base_url = u.base_url.to_string();
            let mut username = u.username;

            if given_username.is_some() {
                username = given_username.unwrap();
            } else if username.is_empty() {
                // Ask username
                if verbose {
                    eprintln!("Input username for vault: {base_url}");
                }
                username = ask_user("Username: ".to_string()).await.unwrap_or("".to_string());
            }

            let mut password = u.password;

            if password.is_empty() {
                // Ask password
                if verbose {
                    eprintln!("Input password for vault: {base_url}");
                }
                password = ask_user_password("Password: ".to_string()).await.unwrap_or("".to_string());
            }

            // Login

            if verbose {
                eprintln!("Logging into vault: {base_url}");
            }

            let login_res = api_call_login(url.clone(), Credentials{
                username: username,
                password: password,
            }).await;

            if login_res.is_err() {
                match login_res.err().unwrap() {
                    super::RequestError::StatusCodeError(s) => {
                        if s == 400 {
                            eprintln!("Error: Username and password cannot be left blank");
                        } else if s == 401 {
                            eprintln!("Error: Invalid credentials");
                        } else {
                            eprintln!("Error: Login API ended with unexpected status code: {s}");
                        }
                    },
                    super::RequestError::ApiError(e) => {
                        let s = e.status;
                        let code = e.code;
                        let msg = e.message;
                        eprintln!("Error: Could not login with the given credentials | Status: {s} | Code: {code} | Message: {msg}");
                    },
                    super::RequestError::HyperError(e) => {
                        let e_str = e.to_string();
                        eprintln!("Error: {e_str}");
                    },
                }

                return Err(());
            }

            let session_id = login_res.unwrap().session_id;
            
            if verbose {
                eprintln!("Login successful!");
            }
            
            return Ok(VaultURI::SessionURI(super::VaultSessionURI{
                base_url: u.base_url.clone(),
                session: session_id,
            }));
        },
        VaultURI::SessionURI(u) => {
            // Session URI is already logged in
            let base_url = u.base_url.to_string();
            if verbose {
                eprintln!("Provided session URL for vault: {base_url}");
            }
            return Ok(url);
        }
    }
}
