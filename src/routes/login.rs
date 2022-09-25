use reqwest::Method;
use serde::{Serialize, Deserialize};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_hooks::prelude::*;
use yew_router::prelude::*;

use crate::api::request;

use super::Route;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct LoginInfo {
    pub user_id: String,
    pub password: String
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    pub user_id: String,
    pub token: String,
}

#[function_component(Login)]
pub fn login() -> Html {
    let login_info = use_state(LoginInfo::default);

    let user_login = {
        let login_info = (*login_info).clone();
        use_async(async move {
            request::<_, UserInfo>(Method::POST, "login".into(), login_info).await
        })
    };
    
    let onlogin = {
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            user_login.run()
        })
    };


    let on_user_change = {
        let login_info = login_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*login_info).clone();
            info.user_id = input.value();
            login_info.set(info);
        }
    )};

    let on_pw_change = {
        let login_info = login_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*login_info).clone();
            info.password = input.value();
            login_info.set(info);})
    };

    html! {                
        <div class="hero">
            <div class="hero-body container pb-0">
                <div class="login">
                <img src="./assets/images/holi.svg" alt="Holi Logo" loading="lazy"/>
                    <input class="input" type="text" oninput={on_user_change} value={login_info.user_id.clone()}
                        maxlength="128" placeholder="HTLHL UserID" 
                    />

                    <input class="input" type="password" oninput={on_pw_change} value={login_info.password.clone()}
                        maxlength="128" placeholder="Password" 
                    />
                    <button onclick={onlogin} class="button is-primary">
                        {"Login"}
                    </button>
                </div>
            </div>
        </div>            
    }
}