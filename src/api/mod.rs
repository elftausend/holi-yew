use gloo_net::http::Request;
use serde::{de::DeserializeOwned, Serialize};

use crate::{app::get_jwt, error::HoliError, API_ROOT};

pub async fn get<R: DeserializeOwned>(url: &str) -> R {
    Request::get(url)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap()
}

pub async fn request<B, T>(
    method: reqwest::Method,
    url: &str,
    body: B,
    cors: bool,
) -> Result<T, HoliError>
where
    T: DeserializeOwned + 'static + std::fmt::Debug,
    B: Serialize + std::fmt::Debug,
{
    let allow_body = method == reqwest::Method::POST || method == reqwest::Method::PUT;

    let url = format!("{API_ROOT}{url}");

    log::info!("url: {url}");

    let mut builder = reqwest::Client::new()
        .request(method, url)
        .header("Content-Type", "application/json");
    //.fetch_mode_no_cors();

    if let Some(token) = get_jwt() {
        builder = builder.header("Authorization", format!("JWT {token}"));
    }

    if allow_body {
        //builder = builder.fetch_mode_no_cors();
        builder = builder.json(&body);
    }

    let response = builder.send().await;
    log::info!("status: {response:?}",);

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
