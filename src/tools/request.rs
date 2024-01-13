// HTTP requests

use super::super::models::*;

use super::vault_uri::VaultURI;
use hyper::{http::Request, Body, Client, Method};
use hyper_tls::HttpsConnector;

pub const SESSION_HEADER_NAME: &str = "x-session-token";

#[derive(Debug)]
pub enum RequestError {
    StatusCode(hyper::StatusCode),
    Api {
        status: hyper::StatusCode,
        code: String,
        message: String,
    },
    Hyper(hyper::Error),
    Json {
        message: String,
        body: String,
    },
    FileSystem(String),
}

impl From<hyper::Error> for RequestError {
    fn from(value: hyper::Error) -> Self {
        RequestError::Hyper(value)
    }
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

pub async fn do_get_request(
    uri: &VaultURI,
    path: String,
    debug: bool,
) -> Result<String, RequestError> {
    let final_uri = resolve_vault_api_uri(uri.clone(), path);

    if debug {
        eprintln!("\rDEBUG: GET {final_uri}");
    }

    let mut request_builder = Request::builder().method(Method::GET).uri(final_uri);

    let session = get_session_from_uri(uri.clone());

    if let Some(s) = session {
        request_builder = request_builder.header(SESSION_HEADER_NAME, s);
    }

    let request = request_builder.body(Body::empty()).unwrap();

    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    // Response received

    let response = client.request(request).await?;

    let res_status = response.status();

    // Read body

    let res_body_bytes = hyper::body::to_bytes(response).await;

    if res_body_bytes.is_err() {
        // Connection error receiving the body
        return Err(RequestError::Hyper(res_body_bytes.err().unwrap()));
    }

    let res_body = String::from_utf8(res_body_bytes.unwrap().to_vec()).unwrap_or("".to_string());

    if res_status != 200 {
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

    Ok(res_body)
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

    let mut request_builder = Request::builder()
        .method(Method::POST)
        .uri(final_uri)
        .header("Content-Type", "application/json");

    let session = get_session_from_uri(uri.clone());

    if let Some(s) = session {
        request_builder = request_builder.header(SESSION_HEADER_NAME, s);
    }

    let request = request_builder.body(Body::from(body)).unwrap();

    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    let result = client.request(request).await;

    if result.is_err() {
        // Network error
        return Err(RequestError::Hyper(result.err().unwrap()));
    }

    // Response received

    let response = result.unwrap();

    let res_status = response.status();

    // Read body

    let res_body_bytes = hyper::body::to_bytes(response).await;

    if res_body_bytes.is_err() {
        // Connection error receiving the body
        return Err(RequestError::Hyper(res_body_bytes.err().unwrap()));
    }

    let res_body = String::from_utf8(res_body_bytes.unwrap().to_vec()).unwrap_or("".to_string());

    if res_status != 200 {
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

    Ok(res_body)
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

    let mut request_builder = Request::builder().method(Method::DELETE).uri(final_uri);

    let session = get_session_from_uri(uri.clone());

    if let Some(s) = session {
        request_builder = request_builder.header(SESSION_HEADER_NAME, s);
    }

    let request = request_builder.body(Body::empty()).unwrap();

    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    // Response received

    let response = client.request(request).await?;

    let res_status = response.status();

    // Read body

    let res_body_bytes = hyper::body::to_bytes(response).await;

    if res_body_bytes.is_err() {
        // Connection error receiving the body
        return Err(RequestError::Hyper(res_body_bytes.err().unwrap()));
    }

    let res_body = String::from_utf8(res_body_bytes.unwrap().to_vec()).unwrap_or("".to_string());

    if res_status != 200 {
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

    Ok(res_body)
}
