use web_sys::window;
use yew::prelude::*;
use yew_router::prelude::{use_history, History};

use crate::{
    hooks::use_user_context,
    routes::{is_logged_in, login::UserInfo, Route},
};

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
                        token: "".into(),
                    });
                    window().unwrap().location().set_href("https://auth.htl-hl.ac.at/authorize.php?response_type=code&client_id=holi.htl-hl.ac.at&redirect_uri=https://holi.htl-hl.ac.at/authenticated&state=new").unwrap();
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
