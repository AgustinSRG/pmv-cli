// Account API

use crate::{
    models::{
        AccountContext, AccountCreateBody, AccountDeleteBody, AccountListItem, ChangePasswordBody,
        Credentials,
    },
    tools::{do_get_request, do_post_request, RequestError, VaultURI},
};

pub async fn api_call_context(url: VaultURI, debug: bool) -> Result<AccountContext, RequestError> {
    let body_str = do_get_request(url, "/api/account".to_string(), debug).await?;

    let parsed_body: Result<AccountContext, _> = serde_json::from_str(&body_str);

    if parsed_body.is_err() {
        return Err(RequestError::Json {
            message: parsed_body.err().unwrap().to_string(),
            body: body_str,
        });
    }

    Ok(parsed_body.unwrap())
}

pub async fn api_call_change_username(
    url: VaultURI,
    credentials: Credentials,
    debug: bool,
) -> Result<(), RequestError> {
    do_post_request(
        url,
        "/api/account/username".to_string(),
        serde_json::to_string(&credentials).unwrap(),
        debug,
    )
    .await?;

    Ok(())
}

pub async fn api_call_change_password(
    url: VaultURI,
    req_body: ChangePasswordBody,
    debug: bool,
) -> Result<(), RequestError> {
    do_post_request(
        url,
        "/api/account/password".to_string(),
        serde_json::to_string(&req_body).unwrap(),
        debug,
    )
    .await?;

    Ok(())
}

pub async fn api_call_list_accounts(
    url: VaultURI,
    debug: bool,
) -> Result<Vec<AccountListItem>, RequestError> {
    let body_str = do_get_request(url, "/api/admin/accounts".to_string(), debug).await?;

    let parsed_body: Result<Vec<AccountListItem>, _> = serde_json::from_str(&body_str);

    if parsed_body.is_err() {
        return Err(RequestError::Json {
            message: parsed_body.err().unwrap().to_string(),
            body: body_str,
        });
    }

    Ok(parsed_body.unwrap())
}

pub async fn api_call_create_account(
    url: VaultURI,
    req_body: AccountCreateBody,
    debug: bool,
) -> Result<(), RequestError> {
    do_post_request(
        url,
        "/api/admin/accounts".to_string(),
        serde_json::to_string(&req_body).unwrap(),
        debug,
    )
    .await?;

    Ok(())
}

pub async fn api_call_delete_account(
    url: VaultURI,
    req_body: AccountDeleteBody,
    debug: bool,
) -> Result<(), RequestError> {
    do_post_request(
        url,
        "/api/admin/accounts/delete".to_string(),
        serde_json::to_string(&req_body).unwrap(),
        debug,
    )
    .await?;

    Ok(())
}
