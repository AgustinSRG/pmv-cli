// Configuration API

use crate::{
    models::VaultConfig,
    tools::{do_get_request, do_post_request, RequestError, VaultURI},
};

pub async fn api_call_get_config(url: &VaultURI, debug: bool) -> Result<VaultConfig, RequestError> {
    let body_str = do_get_request(url, "/api/config".to_string(), debug).await?;

    let parsed_body: Result<VaultConfig, _> = serde_json::from_str(&body_str);

    if parsed_body.is_err() {
        return Err(RequestError::Json {
            message: parsed_body.err().unwrap().to_string(),
            body: body_str,
        });
    }

    Ok(parsed_body.unwrap())
}

pub async fn api_call_set_config(
    url: &VaultURI,
    req_body: VaultConfig,
    debug: bool,
) -> Result<(), RequestError> {
    do_post_request(
        url,
        "/api/config".to_string(),
        serde_json::to_string(&req_body).unwrap(),
        debug,
    )
    .await?;

    Ok(())
}
