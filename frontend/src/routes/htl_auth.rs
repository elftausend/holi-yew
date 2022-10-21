use reqwest::Method;
use serde::{Deserialize, Serialize};
use yew::prelude::*;
use yew_router::prelude::{use_location, Location, use_history, History};

use crate::{hooks::use_user_context, request};

use super::{Route};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
struct CodeQuery {
    code: String,
}


#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserInfo {
    pub user_id: String,
    pub token: String,
}


impl UserInfo {
    pub fn is_auth(&self) -> bool {
        !self.token.is_empty()
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct JWT {
    pub access_token: String,
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
    let history = use_history().unwrap();
    let location = use_location().unwrap();
    let user_ctx = use_user_context();
    {
        let location_inner = location.clone();
        // try with use_mount
        use_effect_with_deps(
            move |_| {
                let code_query = location_inner.query::<CodeQuery>().unwrap_or_default();

                let code_info = CodeInfo::new(code_query.code);

                wasm_bindgen_futures::spawn_local(async move {
                    if let Ok(jwt) = request::<_, JWT>(Method::POST, "auth", code_info, true).await
                    {
                        //set_jwt(Some(jwt.access_token));
                        user_ctx.login(UserInfo {
                            user_id: "must_get_from_htlhl".into(),
                            token: jwt.access_token,
                        });

                        history.push(Route::Entries);
                    }
                });

                || ()
            },
            location.query::<CodeQuery>().unwrap_or_default(),
        );
    }

    /*let location_inner = location.clone();
    use_mount(move || {
        let code_query = location_inner.query::<CodeQuery>().unwrap_or_default();

        let code_info = CodeInfo::new(code_query.code);

        wasm_bindgen_futures::spawn_local(async move {
            if let Ok(jwt) = request::<_, JWT>(Method::POST, "auth", code_info, true).await
            {
                user_ctx.login(UserInfo {
                    user_id: "must_get_from_htlhl".into(),
                    token: jwt.access_token,
                });

                history.push(Route::Entries);
            }
        });        
    });*/

    html! {}
}
