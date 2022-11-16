use reqwest::Method;
use serde::{Deserialize, Serialize};
use yew::prelude::*;

use crate::request;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserListInfo {
    usid: String,
    username: String,
    flag_count: usize
}

#[function_component(Users)]
pub fn user_panel() -> Html {
    let user_infos = use_state(Vec::new);

    use_effect_with_deps(|_| {
        wasm_bindgen_futures::spawn_local(async move {
            let Ok(users) = request::<(), Vec<UserListInfo>>(Method::GET, "users", ()).await else {
                return;
            };
            log::info!("users: {users:?}");
            user_infos.set(users);
        }); 
        || ()
    }, ());

    html! {

    }
}