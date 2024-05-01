// Download request

use std::time::Instant;

use crate::tools::{get_session_from_uri, resolve_vault_api_uri, SESSION_HEADER_NAME};

use super::{RequestError, VaultURI};
use tokio::{fs::File, io::AsyncWriteExt};

pub trait ProgressReceiver {
    fn progress_start(&mut self);
    fn progress_finish(&mut self);

    fn progress_update(&mut self, loaded: u64, total: u64);
}

pub async fn do_get_download_request(
    uri: &VaultURI,
    path: String,
    file_path: String,
    debug: bool,
    progress_receiver: &mut dyn ProgressReceiver,
) -> Result<(), RequestError> {
    let final_uri = resolve_vault_api_uri(uri.clone(), path);

    if debug {
        eprintln!("\rDEBUG: DOWNLOAD {final_uri} -> {file_path}");
    }

    let client = reqwest::Client::new();

    // Build request

    let mut request_builder = client.get(final_uri);

    let session = get_session_from_uri(uri.clone());

    if let Some(s) = session {
        request_builder = request_builder.header(SESSION_HEADER_NAME, s);
    }

    // Send request

    let response_result = request_builder.send().await;

    if let Err(err) = response_result {
        return Err(RequestError::NetworkError(err.to_string()));
    }

    let mut response = response_result.unwrap();

    let res_status = response.status();

    if res_status != 200 {
        return Err(RequestError::StatusCode(res_status));
    }

    // Write body into a file

    let file_open_res = File::create(file_path).await;

    if file_open_res.is_err() {
        return Err(RequestError::FileSystem(
            file_open_res.err().unwrap().to_string(),
        ));
    }

    let mut file = file_open_res.unwrap();

    let mut body_length = 0;

    if let Some(content_length) = response.content_length() {
        body_length = content_length;
    }

    let mut downloaded_bytes: u64 = 0;

    let mut start = Instant::now();
    progress_receiver.progress_start();

    let mut finished = false;

    while !finished {
        // Grab chunk

        let chunk_res = response.chunk().await;

        match chunk_res {
            Ok(chunk_opt) => match chunk_opt {
                Some(mut chunk) => {
                    downloaded_bytes += chunk.len() as u64;

                    // Write chunk to file

                    let write_res = file.write_all_buf(&mut chunk).await;

                    if let Err(err) = write_res {
                        return Err(RequestError::FileSystem(err.to_string()));
                    }

                    // Report progress

                    let elapsed = start.elapsed().as_millis();

                    if elapsed > 100 {
                        // Report progress
                        progress_receiver.progress_update(downloaded_bytes, body_length);

                        // Restart counter
                        start = Instant::now();
                    }
                }
                None => {
                    finished = true
                },
            },
            Err(err) => {
                return Err(RequestError::NetworkError(err.to_string()));
            }
        }
    }

    progress_receiver.progress_update(downloaded_bytes, body_length);
    progress_receiver.progress_finish();
    return Ok(());
}
