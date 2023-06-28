// Albums API

use crate::{tools::{VaultURI, do_get_request, RequestError, do_post_request}, models::{AlbumListItem, Album, AlbumNameBody, AlbumIdResponse, AlbumSetOrderBody, AlbumMediaBody}};

pub async fn api_call_get_albums(url: VaultURI, debug: bool) -> Result<Vec<AlbumListItem>, RequestError> {
    let res = do_get_request(url, "/api/albums".to_string(), debug).await;

    match res {
        Ok(body_str) => {
            let parsed_body: Result<Vec<AlbumListItem>, _> = serde_json::from_str(&body_str);

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

pub async fn api_call_get_album(url: VaultURI, album: u64, debug: bool) -> Result<Album, RequestError> {
    let res = do_get_request(url, format!("/api/albums/{album}"), debug).await;

    match res {
        Ok(body_str) => {
            let parsed_body: Result<Album, _> = serde_json::from_str(&body_str);

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

pub async fn api_call_create_album(url: VaultURI, req_body: AlbumNameBody, debug: bool) -> Result<AlbumIdResponse, RequestError> {
    let res = do_post_request(url, "/api/albums".to_string(), serde_json::to_string(&req_body).unwrap(), debug).await;

    match res {
        Ok(body_str) => {
            let parsed_body: Result<AlbumIdResponse, _> = serde_json::from_str(&body_str);

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

pub async fn api_call_rename_album(url: VaultURI, album: u64, req_body: AlbumNameBody, debug: bool) -> Result<(), RequestError> {
    let res = do_post_request(url, format!("/api/albums/{album}/rename"), serde_json::to_string(&req_body).unwrap(), debug).await;

    match res {
        Ok(_) => {
            return Ok(());
        },
        Err(err) => {
            return Err(err);
        },
    }
}

pub async fn api_call_delete_album(url: VaultURI, album: u64, debug: bool) -> Result<(), RequestError> {
    let res = do_post_request(url, format!("/api/albums/{album}/delete"), "".to_string(), debug).await;

    match res {
        Ok(_) => {
            return Ok(());
        },
        Err(err) => {
            return Err(err);
        },
    }
}


pub async fn api_call_album_set_order(url: VaultURI, album: u64, req_body: AlbumSetOrderBody, debug: bool) -> Result<(), RequestError> {
    let res = do_post_request(url, format!("/api/albums/{album}/set"), serde_json::to_string(&req_body).unwrap(), debug).await;

    match res {
        Ok(_) => {
            return Ok(());
        },
        Err(err) => {
            return Err(err);
        },
    }
}

pub async fn api_call_album_add_media(url: VaultURI, album: u64, req_body: AlbumMediaBody, debug: bool) -> Result<(), RequestError> {
    let res = do_post_request(url, format!("/api/albums/{album}/add"), serde_json::to_string(&req_body).unwrap(), debug).await;

    match res {
        Ok(_) => {
            return Ok(());
        },
        Err(err) => {
            return Err(err);
        },
    }
}

pub async fn api_call_album_remove_media(url: VaultURI, album: u64, req_body: AlbumMediaBody, debug: bool) -> Result<(), RequestError> {
    let res = do_post_request(url, format!("/api/albums/{album}/remove"), serde_json::to_string(&req_body).unwrap(), debug).await;

    match res {
        Ok(_) => {
            return Ok(());
        },
        Err(err) => {
            return Err(err);
        },
    }
}
