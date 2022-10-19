
use yew::prelude::*;

use crate::{hooks::{use_user_context, UseUserContextHandle}, routes::{is_logged_in, login::UserInfo}, app::get_jwt};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub children: Children,
}

fn validate_token(user_ctx_inner: UseUserContextHandle) {
    wasm_bindgen_futures::spawn_local(async move {
        if is_logged_in().await {
            user_ctx_inner.inner.set(UserInfo {
                user_id: user_ctx_inner.inner.user_id.clone(),
                token: get_jwt().unwrap(),
            });
        }
    });
}

#[function_component(NoAuth)]
pub fn no_auth(props: &Props) -> Html {
    let user_ctx = use_user_context();
    
    {
        let user_ctx_inner = user_ctx.clone();
        use_effect_with_deps(
            move |_| {
                validate_token(user_ctx_inner);
                || ()
            },
            user_ctx.clone(), 
        );
    }

    if !user_ctx.inner.is_auth() {
        html! {
            { for props.children.iter() }
        }
    } else {
        html! {}
    }
}