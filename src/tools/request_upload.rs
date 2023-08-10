// Upload multipart requests

use std::fs::File;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::Instant;

use super::{super::models::*, ProgressReceiver};
use super::{get_session_from_uri, resolve_vault_api_uri, RequestError, SESSION_HEADER_NAME};

use super::vault_uri::VaultURI;
use hyper::{http::Request, Client, Method};
use hyper_multipart_rfc7578::client::multipart;
use hyper_tls::HttpsConnector;

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
    uri: VaultURI,
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

    let mut request_builder = Request::builder().method(Method::POST).uri(final_uri);

    let session = get_session_from_uri(uri.clone());

    if session.is_some() {
        request_builder = request_builder.header(SESSION_HEADER_NAME, session.unwrap());
    }

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

    let mut form = multipart::Form::default();

    form.add_reader_file(field, reporter.clone(), &file_name);

    let request_build_result =
        form.set_body_convert::<hyper::Body, multipart::Body>(request_builder);

    if request_build_result.is_err() {
        reporter.finish();
        return Err(RequestError::FileSystem(
            request_build_result.err().unwrap().to_string(),
        ));
    }

    let request = request_build_result.unwrap();

    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    let result = client.request(request).await;

    if result.is_err() {
        // Network error
        reporter.finish();
        return Err(RequestError::Hyper(result.err().unwrap()));
    }

    reporter.update(file_len);
    reporter.finish();

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
