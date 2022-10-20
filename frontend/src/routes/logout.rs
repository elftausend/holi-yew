use crate::hooks::use_user_context;
use yew::prelude::*;

#[function_component(Logout)]
pub fn logout() -> Html {
    let user_ctx = use_user_context();
    user_ctx.logout();

    html!()
}
