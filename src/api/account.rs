// Account API

use hyper::StatusCode;

use crate::{tools::{do_get_request, VaultURI, RequestError, RequestAPIError, do_post_request}, models::{AccountContext, Credentials, ChangePasswordBody, AccountListItem, AccountDeleteBody, AccountCreateBody}};

pub async fn api_call_context(url: VaultURI) -> Result<AccountContext, RequestError> {
    let res = do_get_request(url, "/api/account".to_string()).await;

    match res {
        Ok(body_str) => {
            let parsed_body: Result<AccountContext, _> = serde_json::from_str(&body_str);

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

pub async fn api_call_change_username(url: VaultURI, credentials: Credentials) -> Result<(), RequestError> {
    let res = do_post_request(url, "/api/account/username".to_string(), serde_json::to_string(&credentials).unwrap()).await;

    match res {
        Ok(_) => {
            return Ok(());
        },
        Err(err) => {
            return Err(err);
        },
    }
}

pub async fn api_call_change_password(url: VaultURI, req_body: ChangePasswordBody) -> Result<(), RequestError> {
    let res = do_post_request(url, "/api/account/password".to_string(), serde_json::to_string(&req_body).unwrap()).await;

    match res {
        Ok(_) => {
            return Ok(());
        },
        Err(err) => {
            return Err(err);
        },
    }
}

pub async fn api_call_list_accounts(url: VaultURI) -> Result<Vec<AccountListItem>, RequestError> {
    let res = do_get_request(url, "/api/admin/accounts".to_string()).await;

    match res {
        Ok(body_str) => {
            let parsed_body: Result<Vec<AccountListItem>, _> = serde_json::from_str(&body_str);

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

pub async fn api_call_create_account(url: VaultURI, req_body: AccountCreateBody) -> Result<(), RequestError> {
    let res = do_post_request(url, "/api/admin/accounts".to_string(), serde_json::to_string(&req_body).unwrap()).await;

    match res {
        Ok(_) => {
            return Ok(());
        },
        Err(err) => {
            return Err(err);
        },
    }
}

pub async fn api_call_delete_account(url: VaultURI, req_body: AccountDeleteBody) -> Result<(), RequestError> {
    let res = do_post_request(url, "/api/admin/accounts/delete".to_string(), serde_json::to_string(&req_body).unwrap()).await;

    match res {
        Ok(_) => {
            return Ok(());
        },
        Err(err) => {
            return Err(err);
        },
    }
}
