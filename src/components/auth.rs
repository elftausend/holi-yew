use yew::prelude::*;
use yew_router::prelude::{use_history, History};

use crate::{hooks::use_user_context, routes::{is_logged_in, Route, login::UserInfo}};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub children: Children,
}

#[function_component(Auth)]
pub fn auth(props: &Props) -> Html {
    let user_ctx = use_user_context();
    let history = use_history().unwrap();

    
    use_effect_with_deps(
        move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                if !is_logged_in().await {
                    user_ctx.inner.set(UserInfo {
                        user_id: user_ctx.inner.user_id.clone(),
                        token: "".into()
                    });
                    history.push(Route::Login);
                }
            });
            || ()
        },
        (), //user_ctx,
    );

    html! {
        { for props.children.iter() }
    }
}