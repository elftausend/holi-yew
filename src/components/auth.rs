use web_sys::window;
use yew::prelude::*;

use crate::hooks::use_user_context;

#[function_component(Auth)]
pub fn auth() -> Html {
    let user_ctx = use_user_context();

    if !user_ctx.inner.is_auth() {
        window().unwrap().location().set_href("https://auth.htl-hl.ac.at/authorize.php?response_type=code&client_id=holi.htl-hl.ac.at&redirect_uri=https://holi.htl-hl.ac.at/authenticated&state=new").unwrap();
    }

    html!()
}