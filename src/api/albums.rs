// Albums API

use std::sync::{Arc, Mutex};

use crate::{
    models::{
        Album, AlbumIdResponse, AlbumListItem, AlbumMediaBody, AlbumMoveMediaBody, AlbumNameBody,
        MediaUpdateThumbnailResponse,
    },
    tools::{
        do_get_request, do_multipart_upload_request, do_multipart_upload_request_memory,
        do_post_request, ProgressReceiver, RequestError, VaultURI,
    },
};

pub async fn api_call_get_albums(
    url: &VaultURI,
    debug: bool,
) -> Result<Vec<AlbumListItem>, RequestError> {
    let body_str = do_get_request(url, "/api/albums".to_string(), debug).await?;

    let parsed_body: Result<Vec<AlbumListItem>, _> = serde_json::from_str(&body_str);

    if parsed_body.is_err() {
        return Err(RequestError::Json {
            message: parsed_body.err().unwrap().to_string(),
            body: body_str,
        });
    }

    Ok(parsed_body.unwrap())
}

pub async fn api_call_get_album(
    url: &VaultURI,
    album: u64,
    debug: bool,
) -> Result<Album, RequestError> {
    let body_str = do_get_request(url, format!("/api/albums/{album}"), debug).await?;

    let parsed_body: Result<Album, _> = serde_json::from_str(&body_str);

    if parsed_body.is_err() {
        return Err(RequestError::Json {
            message: parsed_body.err().unwrap().to_string(),
            body: body_str,
        });
    }

    Ok(parsed_body.unwrap())
}

pub async fn api_call_create_album(
    url: &VaultURI,
    req_body: AlbumNameBody,
    debug: bool,
) -> Result<AlbumIdResponse, RequestError> {
    let body_str = do_post_request(
        url,
        "/api/albums".to_string(),
        serde_json::to_string(&req_body).unwrap(),
        debug,
    )
    .await?;

    let parsed_body: Result<AlbumIdResponse, _> = serde_json::from_str(&body_str);

    if parsed_body.is_err() {
        return Err(RequestError::Json {
            message: parsed_body.err().unwrap().to_string(),
            body: body_str,
        });
    }

    Ok(parsed_body.unwrap())
}

pub async fn api_call_rename_album(
    url: &VaultURI,
    album: u64,
    req_body: AlbumNameBody,
    debug: bool,
) -> Result<(), RequestError> {
    do_post_request(
        url,
        format!("/api/albums/{album}/rename"),
        serde_json::to_string(&req_body).unwrap(),
        debug,
    )
    .await?;

    Ok(())
}

pub async fn api_call_delete_album(
    url: &VaultURI,
    album: u64,
    debug: bool,
) -> Result<(), RequestError> {
    do_post_request(
        url,
        format!("/api/albums/{album}/delete"),
        "".to_string(),
        debug,
    )
    .await?;

    Ok(())
}

pub async fn api_call_album_add_media(
    url: &VaultURI,
    album: u64,
    req_body: AlbumMediaBody,
    debug: bool,
) -> Result<(), RequestError> {
    do_post_request(
        url,
        format!("/api/albums/{album}/add"),
        serde_json::to_string(&req_body).unwrap(),
        debug,
    )
    .await?;

    Ok(())
}

pub async fn api_call_album_remove_media(
    url: &VaultURI,
    album: u64,
    req_body: AlbumMediaBody,
    debug: bool,
) -> Result<(), RequestError> {
    do_post_request(
        url,
        format!("/api/albums/{album}/remove"),
        serde_json::to_string(&req_body).unwrap(),
        debug,
    )
    .await?;

    Ok(())
}

pub async fn api_call_album_move_media(
    url: &VaultURI,
    album: u64,
    req_body: AlbumMoveMediaBody,
    debug: bool,
) -> Result<(), RequestError> {
    do_post_request(
        url,
        format!("/api/albums/{album}/move"),
        serde_json::to_string(&req_body).unwrap(),
        debug,
    )
    .await?;

    Ok(())
}

pub async fn api_call_album_change_thumbnail(
    url: &VaultURI,
    album: u64,
    file_path: String,
    debug: bool,
    progress_receiver: Arc<Mutex<dyn ProgressReceiver + Send>>,
) -> Result<MediaUpdateThumbnailResponse, RequestError> {
    let body_str = do_multipart_upload_request(
        url,
        format!("/api/albums/{album}/thumbnail"),
        "file".to_string(),
        file_path,
        debug,
        progress_receiver,
    )
    .await?;

    let parsed_body: Result<MediaUpdateThumbnailResponse, _> = serde_json::from_str(&body_str);

    if parsed_body.is_err() {
        return Err(RequestError::Json {
            message: parsed_body.err().unwrap().to_string(),
            body: body_str,
        });
    }

    Ok(parsed_body.unwrap())
}

pub async fn api_call_album_change_thumbnail_memory(
    url: &VaultURI,
    album: u64,
    thumb_data: Vec<u8>,
    debug: bool,
) -> Result<MediaUpdateThumbnailResponse, RequestError> {
    let body_str = do_multipart_upload_request_memory(
        url,
        format!("/api/albums/{album}/thumbnail"),
        "file".to_string(),
        thumb_data,
        "thumbnail.jpg".to_string(),
        debug,
    )
    .await?;

    let parsed_body: Result<MediaUpdateThumbnailResponse, _> = serde_json::from_str(&body_str);

    if parsed_body.is_err() {
        return Err(RequestError::Json {
            message: parsed_body.err().unwrap().to_string(),
            body: body_str,
        });
    }

    Ok(parsed_body.unwrap())
}
