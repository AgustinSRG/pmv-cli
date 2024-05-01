// HTTP requests

use super::super::models::*;

use super::vault_uri::VaultURI;

pub const SESSION_HEADER_NAME: &str = "x-session-token";

#[derive(Debug)]
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

pub async fn send_request(request_builder: reqwest::RequestBuilder) -> Result<String, RequestError> {
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
                            let parsed_body: Result<APIErrorResponse, _> = serde_json::from_str(&res_body);
                
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
                
                    return Ok(res_body)
                },
                Err(err) => {
                    return Err(RequestError::NetworkError(err.to_string()));
                },
            }
        },
        Err(err) => {
            return Err(RequestError::NetworkError(err.to_string()));
        },
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

    return send_request(request_builder).await;
}

pub async fn do_post_request(
    uri: &VaultURI,
    path: String,
    body: String,
    debug: bool,
) -> Result<String, RequestError> {
    let final_uri = resolve_vault_api_uri(uri.clone(), path);

    if debug {
        eprintln!("\rDEBUG: POST {final_uri}");
    }

    let client = reqwest::Client::new();

    let mut request_builder = client.post(final_uri)
        .header("Content-Type", "application/json")
        .body(body);

    let session = get_session_from_uri(uri.clone());

    if let Some(s) = session {
        request_builder = request_builder.header(SESSION_HEADER_NAME, s);
    }

    return send_request(request_builder).await;
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

    return send_request(request_builder).await;
}
