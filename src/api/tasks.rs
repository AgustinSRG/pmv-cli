// Tasks API

use crate::{
    models::Task,
    tools::{do_get_request, RequestError, VaultURI},
};

pub async fn api_call_get_tasks(url: VaultURI, debug: bool) -> Result<Vec<Task>, RequestError> {
    let res = do_get_request(url, "/api/tasks".to_string(), debug).await;

    match res {
        Ok(body_str) => {
            let parsed_body: Result<Vec<Task>, _> = serde_json::from_str(&body_str);

            if parsed_body.is_err() {
                return Err(RequestError::JSONError {
                    message: parsed_body.err().unwrap().to_string(),
                    body: body_str.clone(),
                });
            }

            return Ok(parsed_body.unwrap());
        }
        Err(err) => {
            return Err(err);
        }
    }
}

pub async fn api_call_get_task(url: VaultURI, task: u64, debug: bool) -> Result<Task, RequestError> {
    let res = do_get_request(url, format!("/api/tasks/{task}"), debug).await;

    match res {
        Ok(body_str) => {
            let parsed_body: Result<Task, _> = serde_json::from_str(&body_str);

            if parsed_body.is_err() {
                return Err(RequestError::JSONError {
                    message: parsed_body.err().unwrap().to_string(),
                    body: body_str.clone(),
                });
            }

            return Ok(parsed_body.unwrap());
        }
        Err(err) => {
            return Err(err);
        }
    }
}
