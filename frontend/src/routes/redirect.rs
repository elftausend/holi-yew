use web_sys::window;
use yew::prelude::*;
use yew_router::prelude::{use_history, History};

use crate::hooks::use_user_context;

use super::Route;

#[function_component(RedirectHTL)]
pub fn redirect_htl() -> Html {
    let history = use_history().unwrap();
    let user_ctx = use_user_context();
    if !user_ctx.inner.is_auth() {
        return html! {
            {"would redirect"}
        };
    //    window().unwrap().location().set_href("https://auth.htl-hl.ac.at/authorize.php?response_type=code&client_id=holi.htl-hl.ac.at&redirect_uri=https://holi.htl-hl.ac.at/authenticated&state=new").unwrap();
    }
    history.push(Route::Entries);
    html!()
}