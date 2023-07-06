// HTTP requests

use std::sync::Arc;

use super::super::models::*;

use super::vault_uri::VaultURI;
use hyper::body::HttpBody;
use hyper::{http::Request, Body, Client, Method};
use hyper_multipart_rfc7578::client::multipart;
use hyper_tls::HttpsConnector;
use std::sync::Mutex;
use tokio::{fs::File, io::AsyncWriteExt};

const SESSION_HEADER_NAME: &str = "x-session-token";

#[derive(Debug)]
pub enum RequestError {
    StatusCodeError(hyper::StatusCode),
    ApiError {
        status: hyper::StatusCode,
        code: String,
        message: String,
    },
    HyperError(hyper::Error),
    JSONError {
        message: String,
        body: String,
    },
    FileSystemError(String),
}

fn resolve_vault_api_uri(uri: VaultURI, path: String) -> String {
    match uri {
        VaultURI::LoginURI {
            base_url,
            username: _,
            password: _,
        } => {
            return base_url.join(&path).unwrap().to_string();
        }
        VaultURI::SessionURI {
            base_url,
            session: _,
        } => {
            return base_url.join(&path).unwrap().to_string();
        }
    }
}

fn get_session_from_uri(uri: VaultURI) -> Option<String> {
    match uri {
        VaultURI::LoginURI {
            base_url: _,
            username: _,
            password: _,
        } => {
            return None;
        }
        VaultURI::SessionURI {
            base_url: _,
            session,
        } => {
            return Some(session.clone());
        }
    }
}

pub async fn do_get_request(
    uri: VaultURI,
    path: String,
    debug: bool,
) -> Result<String, RequestError> {
    let final_uri = resolve_vault_api_uri(uri.clone(), path);

    if debug {
        eprintln!("DEBUG: GET {final_uri}");
    }

    let mut request_builder = Request::builder().method(Method::GET).uri(final_uri);

    let session = get_session_from_uri(uri.clone());

    if session.is_some() {
        request_builder = request_builder.header(SESSION_HEADER_NAME, session.unwrap());
    }

    let request = request_builder.body(Body::empty()).unwrap();

    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

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
                    return Err(RequestError::ApiError {
                        status: res_status,
                        code: r.code,
                        message: r.message,
                    });
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
    debug: bool,
) -> Result<String, RequestError> {
    let final_uri = resolve_vault_api_uri(uri.clone(), path);

    if debug {
        eprintln!("DEBUG: POST {final_uri}");
    }

    let mut request_builder = Request::builder()
        .method(Method::POST)
        .uri(final_uri)
        .header("Content-Type", "application/json");

    let session = get_session_from_uri(uri.clone());

    if session.is_some() {
        request_builder = request_builder.header(SESSION_HEADER_NAME, session.unwrap());
    }

    let request = request_builder.body(Body::from(body)).unwrap();

    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

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
                    return Err(RequestError::ApiError {
                        status: res_status,
                        code: r.code,
                        message: r.message,
                    });
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

pub async fn do_multipart_upload_request(
    uri: VaultURI,
    path: String,
    field: String,
    file_path: String,
    debug: bool,
) -> Result<String, RequestError> {
    let final_uri = resolve_vault_api_uri(uri.clone(), path);

    if debug {
        eprintln!("DEBUG: POST {final_uri}");
    }

    let mut request_builder = Request::builder().method(Method::POST).uri(final_uri);

    let session = get_session_from_uri(uri.clone());

    if session.is_some() {
        request_builder = request_builder.header(SESSION_HEADER_NAME, session.unwrap());
    }

    let mut form = multipart::Form::default();

    let add_file_res = form.add_file(field, file_path);

    if add_file_res.is_err() {
        return Err(RequestError::FileSystemError(
            add_file_res.err().unwrap().to_string(),
        ));
    }

    let request_build_result =
        form.set_body_convert::<hyper::Body, multipart::Body>(request_builder);

    if request_build_result.is_err() {
        return Err(RequestError::FileSystemError(
            request_build_result.err().unwrap().to_string(),
        ));
    }

    let request = request_build_result.unwrap();

    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

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
                    return Err(RequestError::ApiError {
                        status: res_status,
                        code: r.code,
                        message: r.message,
                    });
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

#[derive(Debug, Clone)]
pub struct RequestProgress {
    pub started: bool,
    pub loaded: u64,
    pub total: u64,

    pub finished: bool,
}

pub fn get_request_progress(progress: &Arc<Mutex<RequestProgress>>) -> RequestProgress {
    let progress_m = progress.lock().unwrap();
    return progress_m.clone();
}

pub fn set_request_progress(
    progress: &Arc<Mutex<RequestProgress>>,
    started: bool,
    loaded: u64,
    total: u64,
) -> () {
    let mut progress_m = progress.lock().unwrap();

    progress_m.started = started;
    progress_m.loaded = loaded;
    progress_m.total = total;
}

pub fn set_request_progress_finished(
    progress: &Arc<Mutex<RequestProgress>>
) -> () {
    let mut progress_m = progress.lock().unwrap();

    progress_m.finished = true;
}

pub async fn do_get_download_request(
    uri: VaultURI,
    path: String,
    file_path: String,
    progress: &Arc<Mutex<RequestProgress>>,
) -> Result<(), RequestError> {
    let final_uri = resolve_vault_api_uri(uri.clone(), path);

    let mut request_builder = Request::builder().method(Method::GET).uri(final_uri);

    let session = get_session_from_uri(uri.clone());

    if session.is_some() {
        request_builder = request_builder.header(SESSION_HEADER_NAME, session.unwrap());
    }

    let request = request_builder.body(Body::empty()).unwrap();

    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    let result = client.request(request).await;

    if result.is_err() {
        // Network error
        set_request_progress_finished(progress);
        return Err(RequestError::HyperError(result.err().unwrap()));
    }

    // Response received

    let response = result.unwrap();

    let res_status = response.status();

    if res_status != 200 {
        set_request_progress_finished(progress);
        return Err(RequestError::StatusCodeError(res_status));
    }

    // Write body into a file

    let file_open_res = File::create(file_path).await;

    if file_open_res.is_err() {
        set_request_progress_finished(progress);
        return Err(RequestError::FileSystemError(
            file_open_res.err().unwrap().to_string(),
        ));
    }

    let mut file = file_open_res.unwrap();

    let mut body_length = 0;

    let content_length_opt = response.headers().get("Content-Length");

    match content_length_opt {
        Some(content_length_header) => {
            let content_length_str_res = content_length_header.to_str();

            match content_length_str_res {
                Ok(content_length_str) => {
                    let content_length_parsed = content_length_str.parse::<u64>();

                    match content_length_parsed {
                        Ok(content_length) => {
                            body_length = content_length;
                        }
                        Err(_) => {}
                    }
                }
                Err(_) => {}
            }
        }
        None => {}
    }

    set_request_progress(progress, true, 0, body_length);

    let mut body = response.into_body();
    let mut downloaded_bytes: u64 = 0;

    while let Some(buf) = body.data().await {
        match buf {
            Ok(mut buf_u) => {
                let bug_u_len = buf_u.len();
                let write_res = file.write_all_buf(&mut buf_u).await;

                match write_res {
                    Ok(_) => {
                        downloaded_bytes += bug_u_len as u64;
                        set_request_progress(progress, true, downloaded_bytes, body_length);
                    }
                    Err(e) => {
                        set_request_progress_finished(progress);
                        return Err(RequestError::FileSystemError(e.to_string()));
                    }
                }
            }
            Err(e) => {
                set_request_progress_finished(progress);
                return Err(RequestError::HyperError(e));
            }
        }
    }

    set_request_progress_finished(progress);
    return Ok(());
}
