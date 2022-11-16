use reqwest::Method;
use serde::{Deserialize, Serialize};
use yew::prelude::*;

use crate::components::{CardGroup, UserCard};
use crate::request;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct UserListInfo {
    pub usid: String,
    pub username: String,
    pub flag_count: usize,
    pub class: String,
}

#[function_component(Users)]
pub fn user_panel() -> Html {
    let user_infos = use_state(|| None);

    {
        let user_infos = user_infos.clone();
        use_effect_with_deps(|_| {
            wasm_bindgen_futures::spawn_local(async move {
                let Ok(users) = request::<(), Vec<UserListInfo>>(Method::GET, "users", ()).await else {
                    return;
                };
                log::info!("users: {users:?}");
                user_infos.set(Some(users));
            }); 
            || ()
        }, ());
    }

    match &*user_infos {
        Some(user_infos) => {
            user_infos.chunks(5).into_iter().map(|chunk| {
                html! {
                    <CardGroup>
                        {
                            chunk.iter().map(|user| {
                                html! {
                                    <UserCard user_info={user.clone()} />
                                }
                            }).collect::<Html>()
                        }
                    </CardGroup>
                }
            }).collect::<Html>()

        }
        None => {
            html!()
        }
    }
}