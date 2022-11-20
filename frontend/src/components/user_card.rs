use reqwest::Method;
use serde::{Serialize, Deserialize};
use yew::prelude::*;

use crate::{routes::users::UserListInfo, request, error::HoliError};

#[derive(Debug, Properties, Clone, Eq, PartialEq)]
pub struct Props {
    pub user_info: UserListInfo
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FlagInfo {
    flag_incr: i32,
    usid: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FlagCount {
    flag_count: i32,
}

pub async fn flag_incr_req(flag_info: FlagInfo) -> Result<FlagCount, HoliError> {
    request::<_, FlagCount>(Method::POST, "incr_flag", flag_info).await
}

fn incr_flag_button(flag_incr: i32, user_info: &UserListInfo, flag_count: UseStateHandle<i32>) -> Callback<MouseEvent> {
    let user_info = user_info.clone();

    Callback::from(move |e: MouseEvent| {
        let flag_count = flag_count.clone();

        let user_info = user_info.clone();
        e.prevent_default();
        wasm_bindgen_futures::spawn_local(async move {
            flag_count.set(flag_incr_req(FlagInfo {
                flag_incr,
                usid: user_info.usid.clone()
            }).await.unwrap().flag_count);
        });      
    })
}

#[function_component(UserCard)]
pub fn user_card(props: &Props) -> Html {

    let Props {
        user_info,
    } = props.clone();

    let flag_count = use_state(|| user_info.flag_count);

    let on_incr_flag = incr_flag_button(1, &user_info, flag_count.clone());
    let on_decr_flag = incr_flag_button(-1, &user_info, flag_count.clone());

    html! {
        <div class = "card">
            <div class="card-body">
                <h5 class="card-title">
                    {&user_info.username}
                </h5>
                <p class="card-text">
                    {&user_info.usid}<br />
                    {&user_info.class}<br/>
                    {"Flags: "} {*flag_count}
                </p>
                <button onclick={on_decr_flag} style="width: 37px;" class="btn btn-primary">{"-"}</button>
                <button onclick={on_incr_flag} style="width: 37px;" class="btn btn-primary ms-1">{"+"}</button>
            </div>
        </div>
    }
}