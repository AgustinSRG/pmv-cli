// Invites API

use crate::{models::{InviteCodeLoginBody, LoginResult}, tools::{VaultURI, RequestError, do_post_request}};

pub async fn api_call_login_invite_code(
    url: &VaultURI,
    req_body: InviteCodeLoginBody,
    debug: bool,
) -> Result<LoginResult, RequestError> {
    let body_str = do_post_request(
        url,
        "/api/invites/login".to_string(),
        serde_json::to_string(&req_body).unwrap(),
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
