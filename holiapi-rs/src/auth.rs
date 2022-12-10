use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    user::UserInfo, CLIENT_ID, CLIENT_SECRET, GRANT_TYPE, REDIRECT_URI, TOKEN_URL, USER_INFO_URL,
};

#[derive(Serialize)]
pub struct OAuthPayload {
    client_id: &'static str,
    client_secret: &'static str,
    grant_type: &'static str,
    code: String,
    redirect_uri: &'static str,
}

#[derive(Deserialize, Debug)]
pub struct OAuthToken {
    access_token: String,
}

pub async fn auth(code: String) -> Result<UserInfo, Box<dyn std::error::Error>> {
    let oauth_payload = OAuthPayload {
        client_id: CLIENT_ID,
        client_secret: CLIENT_SECRET.as_str(),
        grant_type: GRANT_TYPE,
        code,
        redirect_uri: REDIRECT_URI,
    };

    let token = reqwest::Client::new()
        .request(Method::POST, TOKEN_URL)
        .json(&oauth_payload)
        .send()
        .await?
        .json::<OAuthToken>()
        //.text()
        .await?;

    let user_info_raw = reqwest::get(format!("{USER_INFO_URL}{}", token.access_token))
        .await?
        .json::<Value>()
        .await?;

    let mut user_info = UserInfo::from(user_info_raw);
    user_info.htl_access_token = token.access_token;

    Ok(user_info)
}
