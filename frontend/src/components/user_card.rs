use reqwest::Method;
use serde::{Serialize, Deserialize};
use yew::prelude::*;

use crate::{routes::users::UserListInfo, request};

#[derive(Debug, Properties, Clone, Eq, PartialEq)]
pub struct Props {
    pub user_info: UserListInfo
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FlagInfo {
    flag_incr: i32,
    usid: String
}

pub fn flag_incr(flag_info: FlagInfo) {
    request::<_, ()>(Method::POST, "incr_flag", flag_info);
}

#[function_component(UserCard)]
pub fn user_card(props: &Props) -> Html {

    let Props {
        user_info,
    } = props.clone();


    let on_incr_flag = {
        let user_info = user_info.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            flag_incr(FlagInfo {
                flag_incr: 1,
                usid: user_info.usid.clone()
            })
        })
    };

    html! {
        <div class = "card">
            <div class="card-body">
                <h5 class="card-title">
                    {&user_info.username}
                </h5>
                <p class="card-text">
                    {&user_info.usid}<br />
                    {&user_info.class}<br/>
                    {"Flags: "} {user_info.flag_count}
                </p>
                <button style="width: 37px;" class="btn btn-primary">{"-"}</button>
                <button onclick={on_incr_flag} style="width: 37px;" class="btn btn-primary ms-1">{"+"}</button>
            </div>
        </div>
    }
}