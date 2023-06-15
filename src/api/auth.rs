// Authentication API

use hyper::StatusCode;

use crate::{tools::{VaultURI, RequestError, do_post_request, RequestAPIError}, models::{Credentials, LoginResult}};

pub async fn api_call_login(url: VaultURI, credentials: Credentials) -> Result<LoginResult, RequestError> {
    let res = do_post_request(url, "/api/auth/login".to_string(), serde_json::to_string(&credentials).unwrap()).await;

    match res {
        Ok(body_str) => {
            let parsed_body: Result<LoginResult, _> = serde_json::from_str(&body_str);

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

pub async fn api_call_logout(url: VaultURI) -> Result<(), RequestError> {
    let res = do_post_request(url, "/api/auth/logout".to_string(), "".to_string()).await;

    match res {
        Ok(_) => {
            return Ok(());
        },
        Err(err) => {
            return Err(err);
        },
    }
}
