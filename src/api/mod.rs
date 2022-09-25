use serde::{de::DeserializeOwned, Serialize};

use crate::{app::get_jwt, error::HoliError, API_ROOT};

pub async fn request<B, T>(method: reqwest::Method, url: &str, body: B) -> Result<T, HoliError>
where
    T: DeserializeOwned + 'static + std::fmt::Debug,
    B: Serialize + std::fmt::Debug,
{
    let allow_body = method == reqwest::Method::POST || method == reqwest::Method::PUT;

    let url = format!("{API_ROOT}{url}");

    let mut builder = reqwest::Client::new()
        .request(method, url)
        .header("Content-Type", "application/json");

    if let Some(token) = get_jwt() {
        builder = builder.header("Authorization", format!("jwt {token}"));
    }

    if allow_body {
        builder = builder.json(&body);
    }

    let response = builder.send().await;

    if let Ok(data) = response {
        if !data.status().is_success() {
            return match data.status().as_u16() {
                401 => Err(HoliError::Unauthorized),
                403 => Err(HoliError::Forbidden),
                404 => Err(HoliError::NotFound),
                500 => Err(HoliError::InternalServerError),
                _ => Err(HoliError::RequestError),
            };
        }
        let data: Result<T, _> = data.json::<T>().await;

        if let Ok(data) = data {
            return Ok(data);
        }

        return Err(HoliError::DeserializeError);
    }
    Err(HoliError::RequestError)
}
