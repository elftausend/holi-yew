use yew::prelude::*;
use yew_hooks::{use_async, use_async_with_options, UseAsyncOptions, use_mount};
use yew_router::prelude::*;

use crate::{api::request, hooks::use_user_context};

use super::{login::UserInfo, Route, current_user};

pub struct EntryInfo {
    uploader: String,
}

#[function_component(Entries)]
pub fn entries() -> Html {

    let user_ctx = use_user_context();

    let history = use_history().unwrap();
    
    let history1 = history.clone();
    let user = user_ctx.clone();
    use_effect_with_deps(
        move |_| {
            if !user.is_auth() {
                history1.push(Route::Login)
            }
            //}        
            || ()
        },
        user_ctx.clone(),
    );

    let onclick = Callback::once(move |_| history.push(Route::Login));
    html! {
        <div>
            <h1>{ "Entries" }</h1>
            {"UserID:"} {user_ctx.user_id.clone()}
            <button {onclick}>{ "Go Home" }</button>
        </div>
    }
}
