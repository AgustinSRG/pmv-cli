// Home page API

use crate::{
    models::{
        HomePageAddGroupBody, HomePageElement, HomePageGroup, HomePageGroupMoveBody,
        HomePageGroupRenameBody, HomePageGroupSetElementsBody,
    },
    tools::{do_delete_request, do_get_request, do_post_request, RequestError, VaultURI},
};

pub async fn api_call_get_home_groups(
    url: &VaultURI,
    debug: bool,
) -> Result<Vec<HomePageGroup>, RequestError> {
    let body_str = do_get_request(url, "/api/home".to_string(), debug).await?;

    let parsed_body: Result<Vec<HomePageGroup>, _> = serde_json::from_str(&body_str);

    if parsed_body.is_err() {
        return Err(RequestError::Json {
            message: parsed_body.err().unwrap().to_string(),
            body: body_str,
        });
    }

    Ok(parsed_body.unwrap())
}

pub async fn api_call_home_add_group(
    url: &VaultURI,
    req_body: HomePageAddGroupBody,
    debug: bool,
) -> Result<HomePageGroup, RequestError> {
    let body_str = do_post_request(
        url,
        "/api/home".to_string(),
        serde_json::to_string(&req_body).unwrap(),
        debug,
    )
    .await?;

    let parsed_body: Result<HomePageGroup, _> = serde_json::from_str(&body_str);

    if parsed_body.is_err() {
        return Err(RequestError::Json {
            message: parsed_body.err().unwrap().to_string(),
            body: body_str,
        });
    }

    Ok(parsed_body.unwrap())
}

pub async fn api_call_get_home_group_elements(
    url: &VaultURI,
    id: u64,
    debug: bool,
) -> Result<Vec<HomePageElement>, RequestError> {
    let body_str = do_get_request(url, format!("/api/home/{}/elements", id), debug).await?;

    let parsed_body: Result<Vec<HomePageElement>, _> = serde_json::from_str(&body_str);

    if parsed_body.is_err() {
        return Err(RequestError::Json {
            message: parsed_body.err().unwrap().to_string(),
            body: body_str,
        });
    }

    Ok(parsed_body.unwrap())
}

pub async fn api_call_set_home_group_elements(
    url: &VaultURI,
    id: u64,
    req_body: HomePageGroupSetElementsBody,
    debug: bool,
) -> Result<(), RequestError> {
    do_post_request(
        url,
        format!("/api/home/{}/elements", id),
        serde_json::to_string(&req_body).unwrap(),
        debug,
    )
    .await?;

    Ok(())
}

pub async fn api_call_rename_home_group(
    url: &VaultURI,
    id: u64,
    req_body: HomePageGroupRenameBody,
    debug: bool,
) -> Result<(), RequestError> {
    do_post_request(
        url,
        format!("/api/home/{}/name", id),
        serde_json::to_string(&req_body).unwrap(),
        debug,
    )
    .await?;

    Ok(())
}

pub async fn api_call_move_home_group(
    url: &VaultURI,
    id: u64,
    req_body: HomePageGroupMoveBody,
    debug: bool,
) -> Result<(), RequestError> {
    do_post_request(
        url,
        format!("/api/home/{}/move", id),
        serde_json::to_string(&req_body).unwrap(),
        debug,
    )
    .await?;

    Ok(())
}

pub async fn api_call_delete_home_group(
    url: &VaultURI,
    id: u64,
    debug: bool,
) -> Result<(), RequestError> {
    do_delete_request(url, format!("/api/home/{}", id), debug).await?;

    Ok(())
}
