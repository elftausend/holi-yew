use yew::prelude::*;
use yew_router::prelude::*;

use crate::{
    hooks::use_user_context,
    routes::{is_logged_in, Route},
};

#[function_component(NavBar)]
pub fn navbar() -> Html {
    let user_ctx = use_user_context();

    let logged_in = use_state(|| false);

    {
        let logged_in = logged_in.clone();
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(
                    async move { logged_in.set(is_logged_in().await) },
                );
                || ()
            },
            user_ctx.clone(),
        );
    }

    html! {
        {if *logged_in {
            html! {
                <nav class="navbar navbar-expand-sm holi-green navbar-dark">
        <div class="container-fluid">
            <div class="navbar-brand">
                <img id="navbar-holi" src="./assets/images/holi.svg" alt="Holi Logo" loading="lazy"/>
            </div>

            <div class="navbar-collapse ">
                <ul class="navbar-nav">
                    <Link<Route> classes={classes!("nav-link")} to={Route::Entries}>
                        { "Lernmaterial" }
                    </Link<Route>>
                </ul>

                <ul class="navbar-nav">
                    <Link<Route> classes={classes!("nav-link")} to={Route::UserPanel}>
                        { "User Panel" }<span class="ms-1 badge bg-secondary">{"User"}</span>
                    </Link<Route>>
                </ul>




                </div>
                <span class="grey">{format!("logged in as {}", &user_ctx.inner.user_id)}</span><br/>
                <Link<Route> classes={classes!("badge", "bg-danger", "ms-1")} to={Route::Logout}>
                    { "logout" }
                </Link<Route>>

        </div>
    </nav>
            }
        } else {
            html!{}
        }}

    }
}
