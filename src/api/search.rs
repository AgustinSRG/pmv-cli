// Search API

use crate::{
    models::{RandomMediaResult, SearchMediaResult},
    tools::{do_get_request, RequestError, VaultURI},
};

pub async fn api_call_search(
    url: VaultURI,
    tag: Option<String>,
    reverse_order: bool,
    page: u32,
    page_size: u32,
    debug: bool,
) -> Result<SearchMediaResult, RequestError> {
    let mut url_path = "/api/search?".to_string();

    url_path.push_str(&("page_index=".to_owned() + &page.to_string()));
    url_path.push_str(&("&page_size=".to_owned() + &page_size.to_string()));

    if reverse_order {
        url_path.push_str("&order=asc");
    }

    if let Some(t) = tag {
        url_path.push_str(&("&tag=".to_owned() + &urlencoding::encode(&t)));
    }

    let body_str = do_get_request(url, url_path, debug).await?;

    let parsed_body: Result<SearchMediaResult, _> = serde_json::from_str(&body_str);

    if parsed_body.is_err() {
        return Err(RequestError::Json {
            message: parsed_body.err().unwrap().to_string(),
            body: body_str,
        });
    }

    Ok(parsed_body.unwrap())
}

pub async fn api_call_random(
    url: VaultURI,
    tag: Option<String>,
    seed: i64,
    page_size: u32,
    debug: bool,
) -> Result<RandomMediaResult, RequestError> {
    let mut url_path = "/api/random?".to_string();

    url_path.push_str(&("page_size=".to_owned() + &page_size.to_string()));
    url_path.push_str(&("&seed=".to_owned() + &seed.to_string()));

    if let Some(t) = tag {
        url_path.push_str(&("&tag=".to_owned() + &urlencoding::encode(&t)));
    }

    let body_str = do_get_request(url, url_path, debug).await?;

    let parsed_body: Result<RandomMediaResult, _> = serde_json::from_str(&body_str);

    if parsed_body.is_err() {
        return Err(RequestError::Json {
            message: parsed_body.err().unwrap().to_string(),
            body: body_str,
        });
    }

    Ok(parsed_body.unwrap())
}
