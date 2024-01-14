// About API

use crate::{
    models::ServerInformation,
    tools::{do_get_request, RequestError, VaultURI},
};

pub async fn api_call_about(
    url: &VaultURI,
    debug: bool,
) -> Result<ServerInformation, RequestError> {
    let body_str = do_get_request(url, "/api/about".to_string(), debug).await?;

    let parsed_body: Result<ServerInformation, _> = serde_json::from_str(&body_str);

    if parsed_body.is_err() {
        return Err(RequestError::Json {
            message: parsed_body.err().unwrap().to_string(),
            body: body_str,
        });
    }

    Ok(parsed_body.unwrap())
}
