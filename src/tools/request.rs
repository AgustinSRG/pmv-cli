// HTTP requests

use super::super::models::*;

use super::vault_uri::VaultURI;
use hyper::{
    http::Request,
    Body, Client, Method,
};
use hyper_multipart_rfc7578::client::{multipart};

const SESSION_HEADER_NAME: &str = "x-session-token";

pub struct RequestAPIError {
    pub status: hyper::StatusCode,
    pub code: String,
    pub message: String,
}

pub enum RequestError {
    StatusCodeError(hyper::StatusCode),
    ApiError(RequestAPIError),
    HyperError(hyper::Error),
}

fn resolve_vault_api_uri(uri: VaultURI, path: String) -> String {
    match uri {
        VaultURI::LoginURI(u) => {
            return u.base_url.join(&path).unwrap().to_string();
        }
        VaultURI::SessionURI(u) => {
            return u.base_url.join(&path).unwrap().to_string();
        }
    }
}

fn get_session_from_uri(uri: VaultURI) -> Option<String> {
    match uri {
        VaultURI::LoginURI(_) => {
            return None;
        }
        VaultURI::SessionURI(u) => {
            return Some(u.session.clone());
        }
    }
}

pub async fn do_get_request(uri: VaultURI, path: String) -> Result<String, RequestError> {
    let mut request_builder = Request::builder()
        .method(Method::GET)
        .uri(resolve_vault_api_uri(uri.clone(), path));

    let session = get_session_from_uri(uri.clone());

    if session.is_some() {
        request_builder = request_builder.header(SESSION_HEADER_NAME, session.unwrap());
    }

    let request = request_builder.body(Body::empty()).unwrap();

    let client = Client::new();

    let result = client.request(request).await;

    if result.is_err() {
        // Network error
        return Err(RequestError::HyperError(result.err().unwrap()));
    }

    // Response received

    let response = result.unwrap();

    let res_status = response.status();

    // Read body

    let res_body_bytes = hyper::body::to_bytes(response).await;

    if res_body_bytes.is_err() {
        // Connection error receiving the body
        return Err(RequestError::HyperError(res_body_bytes.err().unwrap()));
    }

    let res_body = String::from_utf8(res_body_bytes.unwrap().to_vec()).unwrap_or("".to_string());

    if res_status != 200 {
        if !res_body.is_empty() {
            let parsed_body: Result<APIErrorResponse, _> = serde_json::from_str(&res_body);

            match parsed_body {
                Ok(r) => {
                    return Err(RequestError::ApiError(RequestAPIError {
                        status: res_status,
                        code: r.code,
                        message: r.message,
                    }));
                }
                Err(_) => {
                    return Err(RequestError::StatusCodeError(res_status));
                }
            }
        }

        return Err(RequestError::StatusCodeError(res_status));
    }

    return Ok(res_body);
}

pub async fn do_post_request(
    uri: VaultURI,
    path: String,
    body: String,
) -> Result<String, RequestError> {
    let mut request_builder = Request::builder()
        .method(Method::POST)
        .uri(resolve_vault_api_uri(uri.clone(), path))
        .header("Content-Type", "application/json");

    let session = get_session_from_uri(uri.clone());

    if session.is_some() {
        request_builder = request_builder.header(SESSION_HEADER_NAME, session.unwrap());
    }

    let request = request_builder.body(Body::from(body)).unwrap();

    let client = Client::new();

    let result = client.request(request).await;

    if result.is_err() {
        // Network error
        return Err(RequestError::HyperError(result.err().unwrap()));
    }

    // Response received

    let response = result.unwrap();

    let res_status = response.status();

    // Read body

    let res_body_bytes = hyper::body::to_bytes(response).await;

    if res_body_bytes.is_err() {
        // Connection error receiving the body
        return Err(RequestError::HyperError(res_body_bytes.err().unwrap()));
    }

    let res_body = String::from_utf8(res_body_bytes.unwrap().to_vec()).unwrap_or("".to_string());

    if res_status != 200 {
        if !res_body.is_empty() {
            let parsed_body: Result<APIErrorResponse, _> = serde_json::from_str(&res_body);

            match parsed_body {
                Ok(r) => {
                    return Err(RequestError::ApiError(RequestAPIError {
                        status: res_status,
                        code: r.code,
                        message: r.message,
                    }));
                }
                Err(_) => {
                    return Err(RequestError::StatusCodeError(res_status));
                }
            }
        }

        return Err(RequestError::StatusCodeError(res_status));
    }

    return Ok(res_body);
}

pub enum MultipartRequestError {
    FileOpenError(String),
    StatusCodeError(hyper::StatusCode),
    ApiError(RequestAPIError),
    HyperError(hyper::Error),
}

pub async fn do_multipart_upload_request(
    uri: VaultURI,
    path: String,
    field: String,
    file_path: String,
) -> Result<String, MultipartRequestError> {
    let mut request_builder = Request::builder()
        .method(Method::POST)
        .uri(resolve_vault_api_uri(uri.clone(), path))
        .header("Content-Type", "application/json");

    let session = get_session_from_uri(uri.clone());

    if session.is_some() {
        request_builder = request_builder.header(SESSION_HEADER_NAME, session.unwrap());
    }

    let mut form = multipart::Form::default();

    let add_file_res = form.add_file(field, file_path);

    if add_file_res.is_err() {
        return Err(MultipartRequestError::FileOpenError(
            add_file_res.err().unwrap().to_string(),
        ));
    }

    let request_build_result =
        form.set_body_convert::<hyper::Body, multipart::Body>(request_builder);

    if request_build_result.is_err() {
        return Err(MultipartRequestError::FileOpenError(
            request_build_result.err().unwrap().to_string(),
        ));
    }

    let request = request_build_result.unwrap();

    let client = Client::new();

    let result = client.request(request).await;

    if result.is_err() {
        // Network error
        return Err(MultipartRequestError::HyperError(result.err().unwrap()));
    }

    // Response received

    let response = result.unwrap();

    let res_status = response.status();

    // Read body

    let res_body_bytes = hyper::body::to_bytes(response).await;

    if res_body_bytes.is_err() {
        // Connection error receiving the body
        return Err(MultipartRequestError::HyperError(
            res_body_bytes.err().unwrap(),
        ));
    }

    let res_body = String::from_utf8(res_body_bytes.unwrap().to_vec()).unwrap_or("".to_string());

    if res_status != 200 {
        if !res_body.is_empty() {
            let parsed_body: Result<APIErrorResponse, _> = serde_json::from_str(&res_body);

            match parsed_body {
                Ok(r) => {
                    return Err(MultipartRequestError::ApiError(RequestAPIError {
                        status: res_status,
                        code: r.code,
                        message: r.message,
                    }));
                }
                Err(_) => {
                    return Err(MultipartRequestError::StatusCodeError(res_status));
                }
            }
        }

        return Err(MultipartRequestError::StatusCodeError(res_status));
    }

    return Ok(res_body);
}
