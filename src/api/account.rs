// Account API

use crate::{
    models::{
        AccountContext, AccountCreateBody, AccountDeleteBody, AccountListItem, AccountSecuritySettings, AccountSetSecuritySettingsBody, AccountUpdateBody, ChangePasswordBody, ChangeUsernameBody, TfaDisableBody, TimeOtpEnableBody, TimeOtpOptions, TimeOtpSettings
    },
    tools::{do_get_request, do_post_request, RequestError, VaultURI},
};

pub async fn api_call_context(url: &VaultURI, debug: bool) -> Result<AccountContext, RequestError> {
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
    url: &VaultURI,
    req_body: ChangeUsernameBody,
    debug: bool,
) -> Result<(), RequestError> {
    do_post_request(
        url,
        "/api/account/username".to_string(),
        serde_json::to_string(&req_body).unwrap(),
        debug,
    )
    .await?;

    Ok(())
}

pub async fn api_call_change_password(
    url: &VaultURI,
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
    url: &VaultURI,
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
    url: &VaultURI,
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

pub async fn api_call_update_account(
    url: &VaultURI,
    req_body: AccountUpdateBody,
    debug: bool,
) -> Result<(), RequestError> {
    do_post_request(
        url,
        "/api/admin/accounts/update".to_string(),
        serde_json::to_string(&req_body).unwrap(),
        debug,
    )
    .await?;

    Ok(())
}

pub async fn api_call_delete_account(
    url: &VaultURI,
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


pub async fn api_call_get_account_settings(
    url: &VaultURI,
    debug: bool,
) -> Result<AccountSecuritySettings, RequestError> {
    let body_str = do_get_request(url, "/api/account/security".to_string(), debug).await?;

    let parsed_body: Result<AccountSecuritySettings, _> = serde_json::from_str(&body_str);

    if parsed_body.is_err() {
        return Err(RequestError::Json {
            message: parsed_body.err().unwrap().to_string(),
            body: body_str,
        });
    }

    Ok(parsed_body.unwrap())
}

pub async fn api_call_set_account_settings(
    url: &VaultURI,
    req_body: AccountSetSecuritySettingsBody,
    debug: bool,
) -> Result<(), RequestError> {
    do_post_request(
        url,
        "/api/account/security".to_string(),
        serde_json::to_string(&req_body).unwrap(),
        debug,
    )
    .await?;

    Ok(())
}

pub async fn api_call_get_totp_settings(
    url: &VaultURI,
    options: TimeOtpOptions,
    debug: bool,
) -> Result<TimeOtpSettings, RequestError> {
    let mut url_path = format!("/api/account/security/tfa/totp");

    url_path.push_str(&("?algorithm=".to_owned() + &urlencoding::encode(&options.algorithm.to_string())));
    url_path.push_str(&("&period=".to_owned() + &urlencoding::encode(&options.period.to_string())));

    if let Some(issuer) = options.issuer {
        url_path.push_str(&("&issuer=".to_owned() + &urlencoding::encode(&issuer)));
    }

    if let Some(account) = options.account {
        url_path.push_str(&("&account=".to_owned() + &urlencoding::encode(&account)));
    }

    if options.skew {
        url_path.push_str(&"&skew=allow".to_owned());
    } else {
        url_path.push_str(&"&skew=disallow".to_owned());
    }

    let body_str = do_get_request(url, "/api/account/security".to_string(), debug).await?;

    let parsed_body: Result<TimeOtpSettings, _> = serde_json::from_str(&body_str);

    if parsed_body.is_err() {
        return Err(RequestError::Json {
            message: parsed_body.err().unwrap().to_string(),
            body: body_str,
        });
    }

    Ok(parsed_body.unwrap())
}


pub async fn api_call_enable_totp(
    url: &VaultURI,
    req_body: TimeOtpEnableBody,
    debug: bool,
) -> Result<(), RequestError> {
    do_post_request(
        url,
        "/api/account/security/tfa/totp".to_string(),
        serde_json::to_string(&req_body).unwrap(),
        debug,
    )
    .await?;

    Ok(())
}

pub async fn api_call_disable_tfa(
    url: &VaultURI,
    req_body: TfaDisableBody,
    debug: bool,
) -> Result<(), RequestError> {
    do_post_request(
        url,
        "/api/account/security/tfa/disable".to_string(),
        serde_json::to_string(&req_body).unwrap(),
        debug,
    )
    .await?;

    Ok(())
}