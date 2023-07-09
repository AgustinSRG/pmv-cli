// Download request

use std::time::Instant;

use crate::tools::{resolve_vault_api_uri, get_session_from_uri, SESSION_HEADER_NAME};

use super::{VaultURI, RequestError};
use hyper::{http::Request, Body, Client, Method, body::HttpBody};
use hyper_tls::HttpsConnector;
use tokio::{fs::File, io::AsyncWriteExt};

pub trait ProgressReceiver {
    fn progress_start(self: &mut Self) -> ();
    fn progress_finish(self: &mut Self) -> ();

    fn progress_update(self: &mut Self, loaded: u64, total: u64) -> ();
}

pub async fn do_get_download_request(
    uri: VaultURI,
    path: String,
    file_path: String,
    debug: bool,
    progress_receiver: &mut dyn ProgressReceiver
) -> Result<(), RequestError> {
    let final_uri = resolve_vault_api_uri(uri.clone(), path);

    if debug {
        eprintln!("DEBUG: DOWNLOAD {final_uri} -> {file_path}");
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

    if res_status != 200 {
        return Err(RequestError::StatusCodeError(res_status));
    }

    // Write body into a file

    let file_open_res = File::create(file_path).await;

    if file_open_res.is_err() {
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

    progress_receiver.progress_start();

    let mut start = Instant::now();

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

                        let elapsed = start.elapsed().as_millis();

                        if elapsed > 100 {
                            // Report progress
                            progress_receiver.progress_update(downloaded_bytes, body_length);

                            // Restart counter
                            start = Instant::now();
                        }
                    }
                    Err(e) => {
                        progress_receiver.progress_finish();
                        return Err(RequestError::FileSystemError(e.to_string()));
                    }
                }
            }
            Err(e) => {
                progress_receiver.progress_finish();
                return Err(RequestError::HyperError(e));
            }
        }
    }

    progress_receiver.progress_update(downloaded_bytes, body_length);
    progress_receiver.progress_finish();
    return Ok(());
}
