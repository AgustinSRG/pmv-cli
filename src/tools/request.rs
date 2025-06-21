// HTTP requests

use crate::tools::{ask_user, ask_user_password};

use super::super::models::*;

use super::vault_uri::VaultURI;

pub const SESSION_HEADER_NAME: &str = "x-session-token";
pub const AUTH_CONFIRMATION_PASSWORD_HEADER_NAME: &str = "x-auth-confirmation-pw";
pub const AUTH_CONFIRMATION_TFA_HEADER_NAME: &str = "x-auth-confirmation-tfa";

#[derive(Debug, Clone)]
pub enum RequestError {
    StatusCode(reqwest::StatusCode),
    Api {
        status: reqwest::StatusCode,
        code: String,
        message: String,
    },
    NetworkError(String),
    Json {
        message: String,
        body: String,
    },
    FileSystem(String),
}

pub fn resolve_vault_api_uri(uri: VaultURI, path: String) -> String {
    match uri {
        VaultURI::LoginURI {
            base_url,
            username: _,
            password: _,
        } => base_url.join(&path).unwrap().to_string(),
        VaultURI::SessionURI {
            base_url,
            session: _,
        } => base_url.join(&path).unwrap().to_string(),
    }
}

pub fn get_session_from_uri(uri: VaultURI) -> Option<String> {
    match uri {
        VaultURI::LoginURI {
            base_url: _,
            username: _,
            password: _,
        } => None,
        VaultURI::SessionURI {
            base_url: _,
            session,
        } => Some(session),
    }
}

pub async fn send_request(
    request_builder: reqwest::RequestBuilder,
) -> Result<String, RequestError> {
    let response_result = request_builder.send().await;

    match response_result {
        Ok(response) => {
            let res_status = response.status();

            // Grab body

            let body_result = response.text().await;

            match body_result {
                Ok(res_body) => {
                    if res_status != reqwest::StatusCode::OK {
                        if !res_body.is_empty() {
                            let parsed_body: Result<APIErrorResponse, _> =
                                serde_json::from_str(&res_body);

                            match parsed_body {
                                Ok(r) => {
                                    return Err(RequestError::Api {
                                        status: res_status,
                                        code: r.code,
                                        message: r.message,
                                    });
                                }
                                Err(_) => {
                                    return Err(RequestError::StatusCode(res_status));
                                }
                            }
                        }

                        return Err(RequestError::StatusCode(res_status));
                    }

                    Ok(res_body)
                }
                Err(err) => Err(RequestError::NetworkError(err.to_string())),
            }
        }
        Err(err) => Err(RequestError::NetworkError(err.to_string())),
    }
}

pub async fn do_get_request(
    uri: &VaultURI,
    path: String,
    debug: bool,
) -> Result<String, RequestError> {
    let final_uri = resolve_vault_api_uri(uri.clone(), path);

    if debug {
        eprintln!("\rDEBUG: GET {final_uri}");
    }

    let client = reqwest::Client::new();

    let session = get_session_from_uri(uri.clone());

    // Build request

    let mut request_builder = client.get(final_uri);

    if let Some(s) = session {
        request_builder = request_builder.header(SESSION_HEADER_NAME, s);
    }

    // Send request

    send_request(request_builder).await
}

pub async fn request_auth_confirmation_password() -> String {
    eprintln!("Input your account password to confirm the operation");
    ask_user_password("Password: ")
        .await
        .unwrap_or("".to_string())
}

pub async fn request_auth_confirmation_tfa() -> String {
    eprintln!("Input your one-time two factor authentication code to confirm the operation");
    ask_user("Two factor authentication code: ")
        .await
        .unwrap_or("".to_string())
}

pub async fn do_post_request(
    uri: &VaultURI,
    path: String,
    body: String,
    debug: bool,
) -> Result<String, RequestError> {
    let final_uri = resolve_vault_api_uri(uri.clone(), path.clone());

    if debug {
        eprintln!("\rDEBUG: POST {final_uri}");
    }

    let client = reqwest::Client::new();

    let mut request_builder = client
        .post(final_uri)
        .header("Content-Type", "application/json")
        .body(body.clone());

    let session = get_session_from_uri(uri.clone());

    if let Some(s) = session {
        request_builder = request_builder.header(SESSION_HEADER_NAME, s);
    }

    match send_request(request_builder).await {
        Ok(r) => Ok(r),
        Err(err) => match err.clone() {
            RequestError::Api {
                status,
                code,
                message: _,
            } => {
                if status == 403 {
                    if code == "AUTH_CONFIRMATION_REQUIRED_TFA" {
                        let confirmation_tfa = request_auth_confirmation_tfa().await;
                        do_post_request_with_confirmation(uri, path, body, debug, None, Some(confirmation_tfa)).await
                    } else if code == "AUTH_CONFIRMATION_REQUIRED_PW" {
                        let confirmation_pw = request_auth_confirmation_password().await;
                        do_post_request_with_confirmation(uri, path, body, debug, Some(confirmation_pw), None).await
                    } else {
                        Err(err)
                    }
                } else {
                    Err(err)
                }
            }
            _ => Err(err),
        },
    }
}

pub async fn do_post_request_with_confirmation(
    uri: &VaultURI,
    path: String,
    body: String,
    debug: bool,
    confirmation_password: Option<String>,
    confirmation_tfa: Option<String>,
) -> Result<String, RequestError> {
    let final_uri = resolve_vault_api_uri(uri.clone(), path);

    if debug {
        eprintln!("\rDEBUG: POST {final_uri}");
    }

    let client = reqwest::Client::new();

    let mut request_builder = client
        .post(final_uri)
        .header("Content-Type", "application/json")
        .body(body);

    let session = get_session_from_uri(uri.clone());

    if let Some(s) = session {
        request_builder = request_builder.header(SESSION_HEADER_NAME, s);
    }

    if let Some(cp) = confirmation_password {
        request_builder = request_builder.header(AUTH_CONFIRMATION_PASSWORD_HEADER_NAME, cp);
    }

    if let Some(ct) = confirmation_tfa {
        request_builder = request_builder.header(AUTH_CONFIRMATION_TFA_HEADER_NAME, ct);
    }

    send_request(request_builder).await
}

pub async fn do_delete_request(
    uri: &VaultURI,
    path: String,
    debug: bool,
) -> Result<String, RequestError> {
    let final_uri = resolve_vault_api_uri(uri.clone(), path);

    if debug {
        eprintln!("\rDEBUG: DELETE {final_uri}");
    }

    let client = reqwest::Client::new();

    let mut request_builder = client.delete(final_uri);

    let session = get_session_from_uri(uri.clone());

    if let Some(s) = session {
        request_builder = request_builder.header(SESSION_HEADER_NAME, s);
    }

    send_request(request_builder).await
}
