// Invites API

use crate::{
    models::{
        InviteCodeGenerateBody, InviteCodeLoginBody, InviteCodeStatus, InvitedSession, LoginResult,
    },
    tools::{do_delete_request, do_get_request, do_post_request, RequestError, VaultURI},
};

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

pub async fn api_call_check_invite(
    url: &VaultURI,
    debug: bool,
) -> Result<InviteCodeStatus, RequestError> {
    let body_str = do_get_request(url, "/api/invites".to_string(), debug).await?;

    let parsed_body: Result<InviteCodeStatus, _> = serde_json::from_str(&body_str);

    if parsed_body.is_err() {
        return Err(RequestError::Json {
            message: parsed_body.err().unwrap().to_string(),
            body: body_str,
        });
    }

    Ok(parsed_body.unwrap())
}

pub async fn api_call_list_invited_sessions(
    url: &VaultURI,
    debug: bool,
) -> Result<Vec<InvitedSession>, RequestError> {
    let body_str = do_get_request(url, "/api/invites/sessions".to_string(), debug).await?;

    let parsed_body: Result<Vec<InvitedSession>, _> = serde_json::from_str(&body_str);

    if parsed_body.is_err() {
        return Err(RequestError::Json {
            message: parsed_body.err().unwrap().to_string(),
            body: body_str,
        });
    }

    Ok(parsed_body.unwrap())
}

pub async fn api_call_delete_invited_session(
    url: &VaultURI,
    index: u64,
    debug: bool,
) -> Result<(), RequestError> {
    do_delete_request(url, format!("/api/invites/sessions/{index}"), debug).await?;

    Ok(())
}

pub async fn api_call_generate_invite(
    url: &VaultURI,
    req_body: InviteCodeGenerateBody,
    debug: bool,
) -> Result<InviteCodeStatus, RequestError> {
    let body_str = do_post_request(
        url,
        "/api/invites/generate".to_string(),
        serde_json::to_string(&req_body).unwrap(),
        debug,
    )
    .await?;

    let parsed_body: Result<InviteCodeStatus, _> = serde_json::from_str(&body_str);

    if parsed_body.is_err() {
        return Err(RequestError::Json {
            message: parsed_body.err().unwrap().to_string(),
            body: body_str,
        });
    }

    Ok(parsed_body.unwrap())
}

pub async fn api_call_clear_invite(url: &VaultURI, debug: bool) -> Result<(), RequestError> {
    do_post_request(url, "/api/invites/clear".to_string(), "".to_string(), debug).await?;

    Ok(())
}
