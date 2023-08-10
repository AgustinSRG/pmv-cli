// Tags API

use crate::{
    models::{AddTagBody, MediaTag, RemoveTagBody},
    tools::{do_get_request, do_post_request, RequestError, VaultURI},
};

pub async fn api_call_get_tags(url: VaultURI, debug: bool) -> Result<Vec<MediaTag>, RequestError> {
    let body_str = do_get_request(url, "/api/tags".to_string(), debug).await?;

    let parsed_body: Result<Vec<MediaTag>, _> = serde_json::from_str(&body_str);

    if parsed_body.is_err() {
        return Err(RequestError::Json {
            message: parsed_body.err().unwrap().to_string(),
            body: body_str,
        });
    }

    Ok(parsed_body.unwrap())
}

pub async fn api_call_tag_add(
    url: VaultURI,
    req_body: AddTagBody,
    debug: bool,
) -> Result<MediaTag, RequestError> {
    let body_str = do_post_request(
        url,
        "/api/tags/add".to_string(),
        serde_json::to_string(&req_body).unwrap(),
        debug,
    )
    .await?;

    let parsed_body: Result<MediaTag, _> = serde_json::from_str(&body_str);

    if parsed_body.is_err() {
        return Err(RequestError::Json {
            message: parsed_body.err().unwrap().to_string(),
            body: body_str,
        });
    }

    Ok(parsed_body.unwrap())
}

pub async fn api_call_tag_remove(
    url: VaultURI,
    req_body: RemoveTagBody,
    debug: bool,
) -> Result<(), RequestError> {
    do_post_request(
        url,
        "/api/tags/remove".to_string(),
        serde_json::to_string(&req_body).unwrap(),
        debug,
    )
    .await?;

    Ok(())
}
