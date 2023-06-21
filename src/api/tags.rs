// Tags API

use hyper::StatusCode;

use crate::{tools::{VaultURI, do_get_request, RequestError, RequestAPIError, do_post_request}, models::{MediaTag, AddTagBody, RemoveTagBody}};

pub async fn api_call_get_tags(url: VaultURI) -> Result<Vec<MediaTag>, RequestError> {
    let res = do_get_request(url, "/api/tags".to_string()).await;

    match res {
        Ok(body_str) => {
            let parsed_body: Result<Vec<MediaTag>, _> = serde_json::from_str(&body_str);

            if parsed_body.is_err() {
                return Err(RequestError::ApiError(RequestAPIError{
                    status: StatusCode::OK,
                    code: "INVALID_JSON".to_string(),
                    message: "Invalid JSON body received: ".to_string() + &parsed_body.err().unwrap().to_string(),
                }));
            }

            return Ok(parsed_body.unwrap());
        },
        Err(err) => {
            return Err(err);
        },
    }
}

pub async fn api_call_tag_add(url: VaultURI, req_body: AddTagBody) -> Result<MediaTag, RequestError> {
    let res = do_post_request(url, "/api/tags/add".to_string(), serde_json::to_string(&req_body).unwrap()).await;

    match res {
        Ok(body_str) => {
            let parsed_body: Result<MediaTag, _> = serde_json::from_str(&body_str);

            if parsed_body.is_err() {
                return Err(RequestError::ApiError(RequestAPIError{
                    status: StatusCode::OK,
                    code: "INVALID_JSON".to_string(),
                    message: "Invalid JSON body received: ".to_string() + &parsed_body.err().unwrap().to_string(),
                }));
            }

            return Ok(parsed_body.unwrap());
        },
        Err(err) => {
            return Err(err);
        },
    }
}

pub async fn api_call_tag_remove(url: VaultURI, req_body: RemoveTagBody) -> Result<(), RequestError> {
    let res = do_post_request(url, "/api/tags/remove".to_string(), serde_json::to_string(&req_body).unwrap()).await;

    match res {
        Ok(_) => {
            return Ok(());
        },
        Err(err) => {
            return Err(err);
        },
    }
}
