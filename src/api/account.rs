// Account API

use crate::{tools::{do_get_request, VaultURI, RequestError, do_post_request}, models::{AccountContext, Credentials, ChangePasswordBody, AccountListItem, AccountDeleteBody, AccountCreateBody}};

pub async fn api_call_context(url: VaultURI, debug: bool) -> Result<AccountContext, RequestError> {
    let res = do_get_request(url, "/api/account".to_string(), debug).await;

    match res {
        Ok(body_str) => {
            let parsed_body: Result<AccountContext, _> = serde_json::from_str(&body_str);

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

pub async fn api_call_change_username(url: VaultURI, credentials: Credentials, debug: bool) -> Result<(), RequestError> {
    let res = do_post_request(url, "/api/account/username".to_string(), serde_json::to_string(&credentials).unwrap(), debug).await;

    match res {
        Ok(_) => {
            return Ok(());
        },
        Err(err) => {
            return Err(err);
        },
    }
}

pub async fn api_call_change_password(url: VaultURI, req_body: ChangePasswordBody, debug: bool) -> Result<(), RequestError> {
    let res = do_post_request(url, "/api/account/password".to_string(), serde_json::to_string(&req_body).unwrap(), debug).await;

    match res {
        Ok(_) => {
            return Ok(());
        },
        Err(err) => {
            return Err(err);
        },
    }
}

pub async fn api_call_list_accounts(url: VaultURI, debug: bool) -> Result<Vec<AccountListItem>, RequestError> {
    let res = do_get_request(url, "/api/admin/accounts".to_string(), debug).await;

    match res {
        Ok(body_str) => {
            let parsed_body: Result<Vec<AccountListItem>, _> = serde_json::from_str(&body_str);

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

pub async fn api_call_create_account(url: VaultURI, req_body: AccountCreateBody, debug: bool) -> Result<(), RequestError> {
    let res = do_post_request(url, "/api/admin/accounts".to_string(), serde_json::to_string(&req_body).unwrap(), debug).await;

    match res {
        Ok(_) => {
            return Ok(());
        },
        Err(err) => {
            return Err(err);
        },
    }
}

pub async fn api_call_delete_account(url: VaultURI, req_body: AccountDeleteBody, debug: bool) -> Result<(), RequestError> {
    let res = do_post_request(url, "/api/admin/accounts/delete".to_string(), serde_json::to_string(&req_body).unwrap(), debug).await;

    match res {
        Ok(_) => {
            return Ok(());
        },
        Err(err) => {
            return Err(err);
        },
    }
}
