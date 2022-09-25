use reqwest::Method;
use serde::{Deserialize, Serialize};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_hooks::prelude::*;
use yew_router::prelude::{use_history, History};

use crate::{api::request, hooks::use_user_context};

use super::Route;

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

    if user_ctx.is_auth() {
    //    history.push(Route::Entries);
    }

    let token = {
        let login_info = (*login_info).clone();
        use_async(async move { request::<_, JWT>(Method::POST, "auth", login_info).await })
    };

    {
        let login_info = login_info.clone();
        use_effect_with_deps(
            move |token| {
                if let Some(token) = &token.data {
                    user_ctx.login(UserInfo {
                        user_id: login_info.username.clone(),
                        token: token.access_token.clone(),
                    });
                }
                || ()
            },
            token.clone(),
        );
    }

    let onlogin = {
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            token.run()
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
        <div class="hero">
            <div class="hero-body container pb-0">
                <div class="login">
                <img src="./assets/images/holi.svg" alt="Holi Logo" loading="lazy"/>
                    <input class="input" type="text" oninput={on_user_change} value={login_info.username.clone()}
                        maxlength="128" placeholder="HTLHL UserID"
                    />

                    <input class="input" type="password" oninput={on_pw_change} value={login_info.password.clone()}
                        maxlength="128" placeholder="Password"
                    />
                    <button onclick={onlogin} class="button is-danger">
                        {"Login"}
                    </button>
                </div>
            </div>
        </div>
    }
}
