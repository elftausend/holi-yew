use yew::prelude::*;
use yew_router::prelude::use_history;

use crate::hooks::use_user_context;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub children: Children,
}

#[function_component(ResendToken)]
pub fn auth(props: &Props) -> Html {
    let user_ctx = use_user_context();
    let history = use_history().unwrap();

    /*use_effect_with_deps(
        move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                if is_logged_in().await {
                    user_ctx.login(UserInfo {
                        user_id: user_ctx.inner.user_id.clone(),
                        token: get_jwt().unwrap()
                    });
                    log::info!("logged in !");
                } else {
                    log::info!("not logged in !");
                    user_ctx.inner.set(UserInfo {
                        user_id: user_ctx.inner.user_id.clone(),
                        token: String::new()
                    });
                }
            });
            || ()
        },
        (), //user_ctx,
    );*/

    /*use_mount(move || {
        wasm_bindgen_futures::spawn_local(async move {
            if is_logged_in().await {
                user_ctx.login(UserInfo {
                    user_id: user_ctx.inner.user_id.clone(),
                    token: get_jwt().unwrap()
                });
                log::info!("logged in !");
            } else {
                log::info!("not logged in !");
                user_ctx.inner.set(UserInfo {
                    user_id: user_ctx.inner.user_id.clone(),
                    token: String::new()
                });
                set_jwt(None);
            }
        });
    });*/

    html! {
        { for props.children.iter() }
    }
}
