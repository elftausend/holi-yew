use reqwest::Method;
use yew::prelude::*;
use yew_hooks::prelude::*;

use crate::{api::request, routes::login::UserInfo};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub children: Children,
}

/// User context provider.
#[function_component(UserContextProvider)]
pub fn user_context_provider(props: &Props) -> Html {
    let user_ctx = use_state(UserInfo::default);

    {
        let user_ctx = user_ctx.clone();
        use_mount(move || {
            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(user_info) = request::<_, UserInfo>(Method::GET, "user", (), true).await {
                    log::info!("Logged in");
                    user_ctx.set(UserInfo {
                        user_id: user_info.user_id,
                        token: user_info.token,
                    });
                }
            });
        });
    }

    html! {
        <ContextProvider<UseStateHandle<UserInfo>> context={user_ctx}>
            { for props.children.iter() }
        </ContextProvider<UseStateHandle<UserInfo>>>
    }
}