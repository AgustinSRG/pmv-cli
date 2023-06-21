// Search API

use hyper::StatusCode;

use crate::{tools::{VaultURI, do_get_request, RequestError, RequestAPIError}, models::{SearchMediaResult, RandomMediaResult}};

pub async fn api_call_search(url: VaultURI, tag: Option<String>, reverse_order: bool, page: u32, page_size: u32) -> Result<SearchMediaResult, RequestError> {
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

    let res = do_get_request(url, url_path).await;

    match res {
        Ok(body_str) => {
            let parsed_body: Result<SearchMediaResult, _> = serde_json::from_str(&body_str);

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


pub async fn api_call_random(url: VaultURI, tag: Option<String>, seed: i64, page_size: u32) -> Result<RandomMediaResult, RequestError> {
    let mut url_path = "/api/random?".to_string();

    url_path.push_str(&("page_size=".to_owned() + &page_size.to_string()));
    url_path.push_str(&("&seed=".to_owned() + &seed.to_string()));

    match tag {
        Some(t) => {
            url_path.push_str(&("&tag=".to_owned() + &urlencoding::encode(&t)));
        },
        None => {},
    }

    let res = do_get_request(url, url_path).await;

    match res {
        Ok(body_str) => {
            let parsed_body: Result<RandomMediaResult, _> = serde_json::from_str(&body_str);

            eprintln!("{body_str}");

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