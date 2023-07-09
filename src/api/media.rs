// Media API

use std::sync::{Arc, Mutex};

use crate::{tools::{VaultURI, RequestError, do_get_request, do_multipart_upload_request, ProgressReceiver}, models::{MediaMetadata, MediaUploadResponse}};

pub async fn api_call_get_media(url: VaultURI, media: u64, debug: bool) -> Result<MediaMetadata, RequestError> {
    let res = do_get_request(url, format!("/api/media/{media}"), debug).await;

    match res {
        Ok(body_str) => {
            let parsed_body: Result<MediaMetadata, _> = serde_json::from_str(&body_str);

            if parsed_body.is_err() {
                return Err(RequestError::JSONError{
                    message: parsed_body.err().unwrap().to_string(),
                    body: body_str.clone(),
                });
            }

            return Ok(parsed_body.unwrap());
        },
        Err(err) => {
            return Err(err);
        },
    }
}

pub async fn api_call_get_media_albums(url: VaultURI, media: u64, debug: bool) -> Result<Vec<u64>, RequestError> {
    let res = do_get_request(url, format!("/api/media/{media}/albums"), debug).await;

    match res {
        Ok(body_str) => {
            let parsed_body: Result<Vec<u64>, _> = serde_json::from_str(&body_str);

            if parsed_body.is_err() {
                return Err(RequestError::JSONError{
                    message: parsed_body.err().unwrap().to_string(),
                    body: body_str.clone(),
                });
            }

            return Ok(parsed_body.unwrap());
        },
        Err(err) => {
            return Err(err);
        },
    }
}

pub async fn api_call_upload_media(url: VaultURI, file_path: String, title: Option<String>, album: Option<u64>, debug: bool, progress_receiver: Arc<Mutex<dyn ProgressReceiver + Send>>) -> Result<MediaUploadResponse, RequestError> {
    let mut url_path = "/api/upload".to_string();
    let mut any_arg = false;

    match title {
        Some(t) => {
            any_arg = true;
            url_path.push_str(&("?title=".to_owned() + &urlencoding::encode(&t)));
        },
        None => {},
    } 

    match album {
        Some(a) => {
            let album_str = a.to_string();
            if any_arg {
                url_path.push_str(&("&album=".to_owned() + &urlencoding::encode(&album_str)));
            } else {
                url_path.push_str(&("?album=".to_owned() + &urlencoding::encode(&album_str)));
            }
            
        },
        None => {},
    } 
    
    let res = do_multipart_upload_request(url, url_path, "".to_string(), file_path, debug, progress_receiver).await;

    match res {
        Ok(body_str) => {
            let parsed_body: Result<MediaUploadResponse, _> = serde_json::from_str(&body_str);

            if parsed_body.is_err() {
                return Err(RequestError::JSONError{
                    message: parsed_body.err().unwrap().to_string(),
                    body: body_str.clone(),
                });
            }

            return Ok(parsed_body.unwrap());
        },
        Err(err) => {
            return Err(err);
        },
    }
}

