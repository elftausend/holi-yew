use reqwest::Method;
use serde::{Deserialize, Serialize};
use web_sys::window;
use yew::prelude::*;
use yew_router::prelude::{use_location, Location, use_history, History};

use crate::{hooks::use_user_context, request, REDIRECT};

use super::{Route};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
struct CodeQuery {
    code: String,
}


#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserInfo {
    pub user_id: String,
    pub division: String,
    pub token: String,
    pub uploaded: Vec<u32>,
    pub favs: Vec<u32>
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
                        user_ctx.login(UserInfo {
                            token: jwt.access_token,
                            ..Default::default()
                        });

                        if let Ok(user_info) = request::<_, UserInfo>(Method::GET, "user", (), true).await {
                            log::info!("userinfo: {:?}", user_info);
                            user_ctx.inner.set(user_info);
                        }

                        history.push(Route::Entries);
                    } else {
                        let href = format!("https://auth.htl-hl.ac.at/authorize.php?response_type=code&client_id=holi.htl-hl.ac.at&redirect_uri={REDIRECT}&state=new");
                        window().unwrap().location().set_href(&href).unwrap();
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
