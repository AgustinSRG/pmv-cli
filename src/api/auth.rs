// Authentication API

use crate::{
    models::{Credentials, LoginResult},
    tools::{do_post_request, RequestError, VaultURI},
};

pub async fn api_call_login(
    url: VaultURI,
    credentials: Credentials,
    debug: bool,
) -> Result<LoginResult, RequestError> {
    let body_str = do_post_request(
        url,
        "/api/auth/login".to_string(),
        serde_json::to_string(&credentials).unwrap(),
        debug,
    )
    .await?;

    let parsed_body: Result<LoginResult, _> = serde_json::from_str(&body_str);

    if parsed_body.is_err() {
        return Err(RequestError::Json {
            message: parsed_body.err().unwrap().to_string(),
            body: body_str,
        });
    }

    Ok(parsed_body.unwrap())
}

pub async fn api_call_logout(url: VaultURI, debug: bool) -> Result<(), RequestError> {
    do_post_request(url, "/api/auth/logout".to_string(), "".to_string(), debug).await?;

    Ok(())
}
