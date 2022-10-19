use reqwest::Method;
use serde::{Deserialize, Serialize};
use yew::prelude::*;
use yew_router::prelude::{use_location, Location};

use crate::{hooks::use_user_context, request};

use super::login::{UserInfo, JWT};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
struct CodeQuery {
    code: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
struct CodeInfo {
    // irrelevant
    username: String,
    code: String,
}

impl CodeInfo {
    pub fn new(token: String) -> Self {
        CodeInfo {
            username: "11111".into(),
            code: token
        }
    }
}

#[function_component(OAuth2)]
pub fn auth() -> Html {
    let location = use_location().unwrap();
    let user_ctx = use_user_context();
    {
        let location_inner = location.clone();
        use_effect_with_deps(
            move |_| {
                let code_query = location_inner.query::<CodeQuery>().unwrap_or_default();

                let code_info = CodeInfo::new(code_query.code);

                wasm_bindgen_futures::spawn_local(async move {
                    if let Ok(jwt) = request::<_, JWT>(Method::POST, "auth", code_info, true).await
                    {
                        user_ctx.login(UserInfo {
                            user_id: "must_get_from_htlhl".into(),
                            token: jwt.access_token,
                        });
                    }
                });

                || ()
            },
            location.query::<CodeQuery>().unwrap_or_default(),
        );
    }

    html! {}
}
