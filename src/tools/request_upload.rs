// Upload multipart requests

use std::fs::File;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use tokio_sync_read_stream::SyncReadStream;

use super::{super::models::*, ProgressReceiver};
use super::{get_session_from_uri, resolve_vault_api_uri, RequestError, SESSION_HEADER_NAME};

use super::vault_uri::VaultURI;

pub struct UploadProgressReporter {
    file: std::fs::File,
    loaded: u64,
    file_size: u64,
    timer: Instant,
    progress_receiver: Arc<Mutex<dyn ProgressReceiver + Send>>,
}

impl UploadProgressReporter {
    fn new(
        file: std::fs::File,
        file_size: u64,
        progress_receiver: Arc<Mutex<dyn ProgressReceiver + Send>>,
    ) -> UploadProgressReporter {
        UploadProgressReporter {
            file,
            loaded: 0,
            file_size,
            timer: Instant::now(),
            progress_receiver,
        }
    }

    fn start(&mut self) {
        let mut pr = self.progress_receiver.lock().unwrap();
        pr.progress_start();
    }

    fn finish(&mut self) {
        let mut pr = self.progress_receiver.lock().unwrap();
        pr.progress_finish();
    }

    fn update(&mut self, loaded: u64) {
        self.loaded = loaded;
        let mut pr = self.progress_receiver.lock().unwrap();
        pr.progress_update(loaded, self.file_size);
    }
}

#[derive(Clone)]
pub struct UploadProgressReporterSync {
    progress_reporter: Arc<Mutex<UploadProgressReporter>>,
}

impl UploadProgressReporterSync {
    fn new(
        file: std::fs::File,
        file_size: u64,
        progress_receiver: Arc<Mutex<dyn ProgressReceiver + Send>>,
    ) -> UploadProgressReporterSync {
        UploadProgressReporterSync {
            progress_reporter: Arc::new(Mutex::new(UploadProgressReporter::new(
                file,
                file_size,
                progress_receiver,
            ))),
        }
    }

    fn start(&mut self) {
        let mut reporter = self.progress_reporter.lock().unwrap();
        reporter.start();
    }

    fn finish(&mut self) {
        let mut reporter = self.progress_reporter.lock().unwrap();
        reporter.finish();
    }

    fn update(&mut self, loaded: u64) {
        let mut reporter = self.progress_reporter.lock().unwrap();
        reporter.update(loaded);
    }
}

impl std::io::Read for UploadProgressReporterSync {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let mut reporter = self.progress_reporter.lock().unwrap();

        let r = reporter.file.read(buf);

        match r {
            Ok(s) => {
                reporter.loaded += s as u64;

                if reporter.timer.elapsed().as_millis() > 100 {
                    reporter.timer = Instant::now();
                    let mut pr = reporter.progress_receiver.lock().unwrap();
                    pr.progress_update(reporter.loaded, reporter.file_size);
                }

                Ok(s)
            }
            Err(e) => Err(e),
        }
    }
}

pub async fn do_multipart_upload_request(
    uri: &VaultURI,
    path: String,
    field: String,
    file_path: String,
    debug: bool,
    progress_receiver: Arc<Mutex<dyn ProgressReceiver + Send>>,
) -> Result<String, RequestError> {
    let final_uri = resolve_vault_api_uri(uri.clone(), path);

    if debug {
        eprintln!("\rDEBUG: UPLOAD {file_path} -> {final_uri}");
    }

    let client = reqwest::Client::new();

    // Load file

    let file_path_o = Path::new(&file_path);
    let file_name: String = match file_path_o.file_name() {
        Some(n) => n.to_str().unwrap_or("").to_string(),
        None => "".to_string(),
    };

    let file_res = File::open(&file_path);
    let file_len: u64;
    let mut reporter: UploadProgressReporterSync;

    match file_res {
        Ok(file_h) => {
            let file_meta = file_h.metadata();

            match file_meta {
                Ok(meta) => {
                    if meta.is_file() {
                        file_len = meta.len();
                        reporter =
                            UploadProgressReporterSync::new(file_h, file_len, progress_receiver);
                    } else {
                        return Err(RequestError::FileSystem("File not found".to_string()));
                    }
                }
                Err(e) => {
                    return Err(RequestError::FileSystem(e.to_string()));
                }
            }
        }
        Err(e) => {
            return Err(RequestError::FileSystem(e.to_string()));
        }
    }

    reporter.start();

    let stream: SyncReadStream<UploadProgressReporterSync> = reporter.clone().into();

    let file_part = reqwest::multipart::Part::stream(reqwest::Body::wrap_stream(stream)).file_name(file_name);

    let form = reqwest::multipart::Form::new().part(field, file_part);

    let mut request_builder = client.post(final_uri).multipart(form);

    let session = get_session_from_uri(uri.clone());

    if let Some(s) = session {
        request_builder = request_builder.header(SESSION_HEADER_NAME, s);
    }

    // Send request

    let response_result = request_builder.send().await;

    if let Err(err) = response_result {
        return Err(RequestError::NetworkError(err.to_string()));
    }

    // Finish the upload reporter

    reporter.update(file_len);
    reporter.finish();

    // Response received

    let response = response_result.unwrap();

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

            Ok(res_body)
        }
        Err(err) => {
            Err(RequestError::NetworkError(err.to_string()))
        }
    }
}

pub async fn do_multipart_upload_request_memory(
    uri: &VaultURI,
    path: String,
    field: String,
    data: Vec<u8>,
    file_name: String,
    debug: bool,
) -> Result<String, RequestError> {
    let final_uri = resolve_vault_api_uri(uri.clone(), path);

    if debug {
        eprintln!("\rDEBUG: POST {final_uri}");
    }

    let client = reqwest::Client::new();

    // Prepare request

    let file_part = reqwest::multipart::Part::bytes(data).file_name(file_name);

    let form = reqwest::multipart::Form::new().part(field, file_part);

    let mut request_builder = client.post(final_uri).multipart(form);

    let session = get_session_from_uri(uri.clone());

    if let Some(s) = session {
        request_builder = request_builder.header(SESSION_HEADER_NAME, s);
    }

    // Send request

    let response_result = request_builder.send().await;

    if let Err(err) = response_result {
        return Err(RequestError::NetworkError(err.to_string()));
    }

    // Response received

    let response = response_result.unwrap();

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

            Ok(res_body)
        }
        Err(err) => {
            Err(RequestError::NetworkError(err.to_string()))
        }
    }
}
