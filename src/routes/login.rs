use reqwest::Method;
use serde::{Deserialize, Serialize};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::{use_history, History};

use crate::api::request;
use crate::hooks::use_user_context;

use super::{is_logged_in, Route};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct LoginInfo {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserInfo {
    pub user_id: String,
    pub token: String,
}

impl UserInfo {
    pub fn is_auth(&self) -> bool {
        log::info!("auth {}", !self.token.is_empty());
        !self.token.is_empty()
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct JWT {
    pub access_token: String,
}

#[function_component(Login)]
pub fn login() -> Html {
    let user_ctx = use_user_context();
    let login_info = use_state(LoginInfo::default);

    let history = use_history().unwrap();

    use_effect_with_deps(
        move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                if is_logged_in().await {
                    history.push(Route::Entries);
                }
            });
            || ()
        },
        (),
    );

    let onlogin = {
        let login_info = login_info.clone();
        Callback::from(move |e: MouseEvent| {
            let user_ctx = user_ctx.clone();
            let login_info = (*login_info).clone();
            e.prevent_default();

            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(jwt) =
                    request::<_, JWT>(Method::POST, "auth", login_info.clone(), true).await
                {
                    user_ctx.login(UserInfo {
                        user_id: login_info.username,
                        token: jwt.access_token,
                    });
                }
            });
        })
    };

    let on_user_change = {
        let login_info = login_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*login_info).clone();
            info.username = input.value();
            login_info.set(info);
        })
    };

    let on_pw_change = {
        let login_info = login_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*login_info).clone();
            info.password = input.value();
            login_info.set(info);
        })
    };

    html! {
        <div class="container-fluid">
            <div class="login-form">
                <div class="row">
                    <img src="./assets/images/holi.svg" alt="Holi Logo" loading="lazy"/>

                </div>
                <input class="form-control input-field" type="text" oninput={on_user_change} value={login_info.username.clone()}
                    maxlength="128" placeholder="HTLHL UserID"
                />
                <input class="form-control input-field" type="password" oninput={on_pw_change} value={login_info.password.clone()}
                    maxlength="128" placeholder="Password"
                />
                <button onclick={onlogin} class="btn btn-danger">
                    {"Login"}
                </button>
            </div>
        </div>
    }
}
