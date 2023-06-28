// Media API

use crate::{tools::{VaultURI, RequestError, do_get_request}, models::MediaMetadata};

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

