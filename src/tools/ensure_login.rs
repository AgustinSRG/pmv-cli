// Tool to ensure login, or ask for credentials from stdin

use crate::{
    api::api_call_login,
    commands::print_request_error,
    models::Credentials,
    tools::{ask_user, ask_user_password},
};

use super::VaultURI;

pub async fn ensure_login_ext(
    url: &VaultURI,
    given_username: &Option<String>,
    given_password: &Option<String>,
    given_tfa_code: &Option<String>,
    duration: &Option<String>,
    debug: bool,
    required_tfa: bool,
) -> Result<VaultURI, ()> {
    match url.clone() {
        VaultURI::LoginURI {
            base_url,
            username,
            password,
        } => {
            let username_m = match given_username {
                Some(given_username_string) => given_username_string.clone(),
                None => {
                    if username.is_empty() {
                        // Ask username
                        eprintln!("Input username for vault: {base_url}");
                        ask_user("Username: ").await.unwrap_or("".to_string())
                    } else {
                        username.clone()
                    }
                }
            };

            let tfa_code_m = match given_tfa_code {
                Some(code) => Some(code.clone()),
                None => {
                    if required_tfa {
                        let code = ask_user("Two factor authentication code: ")
                            .await
                            .unwrap_or("".to_string());
                        Some(code)
                    } else {
                        None
                    }
                }
            };

            let mut password_m = password.clone();

            if password_m.is_empty() {
                match given_password {
                    Some(p) => {
                        password_m = p.clone();
                    }
                    None => {
                        // Ask password
                        eprintln!("Input password for vault: {base_url}");
                        password_m = ask_user_password("Password: ")
                            .await
                            .unwrap_or("".to_string());
                    }
                }
            }

            // Login

            let login_res = api_call_login(
                url,
                Credentials {
                    username: username_m.clone(),
                    password: password_m.clone(),
                    duration: duration.clone(),
                    tfa_code: tfa_code_m,
                },
                debug,
            )
            .await;

            if login_res.is_err() {
                let error = login_res.err().unwrap();

                match error.clone() {
                    super::RequestError::Api {
                        status,
                        code,
                        message: _,
                    } => {
                        if status == 403 && code == "TFA_REQUIRED" {
                            return Box::pin(ensure_login_ext(
                                url,
                                &Some(username_m),
                                &Some(password_m),
                                &None,
                                duration,
                                debug,
                                true,
                            ))
                            .await;
                        } else {
                            print_request_error(error);
                            return Err(());
                        }
                    }
                    _ => {
                        print_request_error(error);
                        return Err(());
                    }
                }
            }

            let session_id = login_res.unwrap().session_id;

            Ok(VaultURI::SessionURI {
                base_url: base_url.clone(),
                session: session_id,
            })
        }
        VaultURI::SessionURI {
            base_url,
            session: _,
        } => {
            // Session URI is already logged in
            if debug {
                eprintln!("DEBUG: Provided session URL for vault: {base_url}");
            }
            Ok(url.clone())
        }
    }
}

pub async fn ensure_login(
    url: &VaultURI,
    given_username: &Option<String>,
    debug: bool,
) -> Result<VaultURI, ()> {
    ensure_login_ext(url, given_username, &None, &None, &None, debug, false).await
}
