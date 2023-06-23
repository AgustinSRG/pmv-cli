// Search API

use crate::{tools::{VaultURI, do_get_request, RequestError}, models::{SearchMediaResult, RandomMediaResult}};

pub async fn api_call_search(url: VaultURI, tag: Option<String>, reverse_order: bool, page: u32, page_size: u32, debug: bool) -> Result<SearchMediaResult, RequestError> {
    let mut url_path = "/api/search?".to_string();

    url_path.push_str(&("page_index=".to_owned() + &page.to_string()));
    url_path.push_str(&("&page_size=".to_owned() + &page_size.to_string()));

    if reverse_order {
        url_path.push_str("&order=asc");
    }

    match tag {
        Some(t) => {
            url_path.push_str(&("&tag=".to_owned() + &urlencoding::encode(&t)));
        },
        None => {},
    }

    let res = do_get_request(url, url_path, debug).await;

    match res {
        Ok(body_str) => {
            let parsed_body: Result<SearchMediaResult, _> = serde_json::from_str(&body_str);

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


pub async fn api_call_random(url: VaultURI, tag: Option<String>, seed: i64, page_size: u32, debug: bool) -> Result<RandomMediaResult, RequestError> {
    let mut url_path = "/api/random?".to_string();

    url_path.push_str(&("page_size=".to_owned() + &page_size.to_string()));
    url_path.push_str(&("&seed=".to_owned() + &seed.to_string()));

    match tag {
        Some(t) => {
            url_path.push_str(&("&tag=".to_owned() + &urlencoding::encode(&t)));
        },
        None => {},
    }

    let res = do_get_request(url, url_path, debug).await;

    match res {
        Ok(body_str) => {
            let parsed_body: Result<RandomMediaResult, _> = serde_json::from_str(&body_str);

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