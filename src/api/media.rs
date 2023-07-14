// Media API

use std::sync::{Arc, Mutex};

use crate::{tools::{VaultURI, RequestError, do_get_request, do_multipart_upload_request, ProgressReceiver, do_post_request}, models::{MediaMetadata, MediaUploadResponse, MediaUpdateDescriptionBody, MediaUpdateExtraBody, MediaTimeSlice, ImageNote, MediaUpdateThumbnailResponse, MediaUpdateTitleBody, MediaAssetSizeStats, TaskEncodeResolution, MediaResolution, MediaSubtitle, MediaAudioTrack, MediaUpdateExtendedDescriptionBody}};

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
    
    let res = do_multipart_upload_request(url, url_path, "file".to_string(), file_path, debug, progress_receiver).await;

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

pub async fn api_call_get_media_stats(url: VaultURI, media: u64, debug: bool) -> Result<MediaAssetSizeStats, RequestError> {
    let res = do_get_request(url, format!("/api/media/{media}/size_stats"), debug).await;

    match res {
        Ok(body_str) => {
            let parsed_body: Result<MediaAssetSizeStats, _> = serde_json::from_str(&body_str);

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

pub async fn api_call_media_change_title(url: VaultURI, media: u64, req_body: MediaUpdateTitleBody, debug: bool) -> Result<(), RequestError> {
    let res = do_post_request(url, format!("/api/media/{media}/edit/title"), serde_json::to_string(&req_body).unwrap(), debug).await;

    match res {
        Ok(_) => {
            return Ok(());
        },
        Err(err) => {
            return Err(err);
        },
    }
}

pub async fn api_call_media_change_description(url: VaultURI, media: u64, req_body: MediaUpdateDescriptionBody, debug: bool) -> Result<(), RequestError> {
    let res = do_post_request(url, format!("/api/media/{media}/edit/description"), serde_json::to_string(&req_body).unwrap(), debug).await;

    match res {
        Ok(_) => {
            return Ok(());
        },
        Err(err) => {
            return Err(err);
        },
    }
}

pub async fn api_call_media_change_extended_description(url: VaultURI, media: u64, req_body: MediaUpdateExtendedDescriptionBody, debug: bool) -> Result<(), RequestError> {
    let res = do_post_request(url, format!("/api/media/{media}/edit/ext_desc"), serde_json::to_string(&req_body).unwrap(), debug).await;

    match res {
        Ok(_) => {
            return Ok(());
        },
        Err(err) => {
            return Err(err);
        },
    }
}

pub async fn api_call_media_change_extra(url: VaultURI, media: u64, req_body: MediaUpdateExtraBody, debug: bool) -> Result<(), RequestError> {
    let res = do_post_request(url, format!("/api/media/{media}/edit/extra"), serde_json::to_string(&req_body).unwrap(), debug).await;

    match res {
        Ok(_) => {
            return Ok(());
        },
        Err(err) => {
            return Err(err);
        },
    }
}

pub async fn api_call_media_change_time_slices(url: VaultURI, media: u64, req_body: Vec<MediaTimeSlice>, debug: bool) -> Result<(), RequestError> {
    let res = do_post_request(url, format!("/api/media/{media}/edit/time_slices"), serde_json::to_string(&req_body).unwrap(), debug).await;

    match res {
        Ok(_) => {
            return Ok(());
        },
        Err(err) => {
            return Err(err);
        },
    }
}

pub async fn api_call_media_change_notes(url: VaultURI, media: u64, req_body: Vec<ImageNote>, debug: bool) -> Result<(), RequestError> {
    let res = do_post_request(url, format!("/api/media/{media}/edit/notes"), serde_json::to_string(&req_body).unwrap(), debug).await;

    match res {
        Ok(_) => {
            return Ok(());
        },
        Err(err) => {
            return Err(err);
        },
    }
}

pub async fn api_call_media_change_thumbnail(url: VaultURI, media: u64, file_path: String, debug: bool, progress_receiver: Arc<Mutex<dyn ProgressReceiver + Send>>) -> Result<MediaUpdateThumbnailResponse, RequestError> {  
    let res = do_multipart_upload_request(url, format!("/api/media/{media}/edit/thumbnail"), "file".to_string(), file_path, debug, progress_receiver).await;

    match res {
        Ok(body_str) => {
            let parsed_body: Result<MediaUpdateThumbnailResponse, _> = serde_json::from_str(&body_str);

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

pub async fn api_call_media_re_encode(url: VaultURI, media: u64, debug: bool) -> Result<(), RequestError> {
    let res = do_post_request(url, format!("/api/media/{media}/encode"), "".to_string(), debug).await;

    match res {
        Ok(_) => {
            return Ok(());
        },
        Err(err) => {
            return Err(err);
        },
    }
}

pub async fn api_call_media_delete(url: VaultURI, media: u64, debug: bool) -> Result<(), RequestError> {
    let res = do_post_request(url, format!("/api/media/{media}/delete"), "".to_string(), debug).await;

    match res {
        Ok(_) => {
            return Ok(());
        },
        Err(err) => {
            return Err(err);
        },
    }
}

pub async fn api_call_media_add_resolution(url: VaultURI, media: u64, req_body: TaskEncodeResolution, debug: bool) -> Result<MediaResolution, RequestError> {
    let res = do_post_request(url, format!("/api/media/{media}/resolution/add"), serde_json::to_string(&req_body).unwrap(), debug).await;

    match res {
        Ok(body_str) => {
            let parsed_body: Result<MediaResolution, _> = serde_json::from_str(&body_str);

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

pub async fn api_call_media_remove_resolution(url: VaultURI, media: u64, req_body: TaskEncodeResolution, debug: bool) -> Result<(), RequestError> {
    let res = do_post_request(url, format!("/api/media/{media}/resolution/remove"), serde_json::to_string(&req_body).unwrap(), debug).await;

    match res {
        Ok(_) => {
            return Ok(());
        },
        Err(err) => {
            return Err(err);
        },
    }
}

pub async fn api_call_media_set_subtitle(url: VaultURI, media: u64, sub_id: String, sub_name: String, file_path: String, debug: bool, progress_receiver: Arc<Mutex<dyn ProgressReceiver + Send>>) -> Result<MediaSubtitle, RequestError> {  
    let mut url_path = format!("/api/media/{media}/subtitles/set");

    url_path.push_str(&("?id=".to_owned() + &urlencoding::encode(&sub_id)));
    url_path.push_str(&("&name=".to_owned() + &urlencoding::encode(&sub_name)));
    
    let res = do_multipart_upload_request(url, url_path, "file".to_string(), file_path, debug, progress_receiver).await;

    match res {
        Ok(body_str) => {
            let parsed_body: Result<MediaSubtitle, _> = serde_json::from_str(&body_str);

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

pub async fn api_call_media_remove_subtitle(url: VaultURI, media: u64, sub_id: String, debug: bool) -> Result<(), RequestError> {
    let mut url_path = format!("/api/media/{media}/subtitles/remove");

    url_path.push_str(&("?id=".to_owned() + &urlencoding::encode(&sub_id)));

    let res = do_post_request(url, url_path, "".to_string(), debug).await;

    match res {
        Ok(_) => {
            return Ok(());
        },
        Err(err) => {
            return Err(err);
        },
    }
}

pub async fn api_call_media_set_audio(url: VaultURI, media: u64, audio_id: String, audio_name: String, file_path: String, debug: bool, progress_receiver: Arc<Mutex<dyn ProgressReceiver + Send>>) -> Result<MediaAudioTrack, RequestError> {  
    let mut url_path = format!("/api/media/{media}/audios/set");

    url_path.push_str(&("?id=".to_owned() + &urlencoding::encode(&audio_id)));
    url_path.push_str(&("&name=".to_owned() + &urlencoding::encode(&audio_name)));
    
    let res = do_multipart_upload_request(url, url_path, "file".to_string(), file_path, debug, progress_receiver).await;

    match res {
        Ok(body_str) => {
            let parsed_body: Result<MediaAudioTrack, _> = serde_json::from_str(&body_str);

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

pub async fn api_call_media_remove_audio(url: VaultURI, media: u64, audio_id: String, debug: bool) -> Result<(), RequestError> {
    let mut url_path = format!("/api/media/{media}/audios/remove");

    url_path.push_str(&("?id=".to_owned() + &urlencoding::encode(&audio_id)));

    let res = do_post_request(url, url_path, "".to_string(), debug).await;

    match res {
        Ok(_) => {
            return Ok(());
        },
        Err(err) => {
            return Err(err);
        },
    }
}
