mod unique_tag;
pub use unique_tag::*;

use crate::REDIRECT;
use crate::{app::get_jwt, error::HoliError, API_ROOT};
use serde::{de::DeserializeOwned, Serialize};

pub async fn request<B, T>(method: reqwest::Method, url: &str, body: B) -> Result<T, HoliError>
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

    if let Some(token) = get_jwt() {
        builder = builder.header("Authorization", format!("Bearer {token}"));
    }

    if allow_body {
        //builder = builder.fetch_mode_no_cors();
        builder = builder.json(&body);
    }

    let response = builder.send().await;
    //log::info!("status: {response:?}",);

    if let Ok(data) = response {
        if !data.status().is_success() {
            return match data.status().as_u16() {
                401 => Err(HoliError::Unauthorized),
                403 => Err(HoliError::Forbidden),
                404 => Err(HoliError::NotFound),
                500 => Err(HoliError::InternalServerError),
                422 => {
                    let href = format!("https://auth.htl-hl.ac.at/authorize.php?response_type=code&client_id=holi.htl-hl.ac.at&redirect_uri={REDIRECT}&state=new");
                    web_sys::window()
                        .unwrap()
                        .location()
                        .set_href(&href)
                        .unwrap();
                    Err(HoliError::Unauthorized)
                }
                _ => Err(HoliError::RequestError),
            };
        }

        let data: Result<T, _> = data.json::<T>().await;
        //log::info!(".... json {data:?}");

        if let Ok(data) = data {
            return Ok(data);
        }

        return Err(HoliError::DeserializeError);
    }
    Err(HoliError::RequestError)
}
