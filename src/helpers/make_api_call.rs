use anyhow::anyhow;
use serde::__private::Ok;
use serde::de::DeserializeOwned;
use serde_json::Value;
pub enum FunctionResult<T> {
    Success(T),
    Error(anyhow::Error),
}

pub async fn api_call<T: DeserializeOwned>(url: String) -> FunctionResult<T> {
    let resp = match reqwest::get(&url).await {
        Ok(response) => response,
        Err(err) => return FunctionResult::Error(err.into()),
    };

    match resp.status() {
        // 200 for successful attempts
        reqwest::StatusCode::OK => {
            let json_response: Value = match resp.json().await {
                Ok(json) => json,
                Err(err) => return FunctionResult::Error(err.into()),
            };
            println!("{:?}", json_response);

            match serde_json::from_value(json_response.to_owned()) {
                Ok(response) => FunctionResult::Success(response),
                Err(err) => FunctionResult::Error(anyhow!(
                    "Failed to parse response into {}: {}",
                    std::any::type_name::<T>(),
                    err
                )),
            }
        }
        // 403 for the cases where the user's profile is not public
        reqwest::StatusCode::FORBIDDEN => {
            let json_response: Value = match resp.json().await {
                Ok(json) => json,
                Err(err) => return FunctionResult::Error(err.into()),
            };
            println!("{:?}", json_response);

            match serde_json::from_value(json_response.to_owned()) {
                Ok(response) => FunctionResult::Success(response),
                Err(err) => FunctionResult::Error(anyhow!(
                    "Failed to parse response into {}: {}",
                    std::any::type_name::<T>(),
                    err
                )),
            }
        }
        // any other status code
        status_code => FunctionResult::Error(anyhow!("An error occurred: {}", status_code)),
    }
}

