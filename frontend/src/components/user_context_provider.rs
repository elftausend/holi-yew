use reqwest::Method;
use yew::prelude::*;
use yew_hooks::prelude::*;

use crate::{api::request, routes::htl_auth::UserInfo, app::set_jwt};

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
                    user_ctx.set(user_info)
                } else {
                    set_jwt(None);
                    user_ctx.set(UserInfo::default())
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
