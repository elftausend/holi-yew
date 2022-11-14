use yew::prelude::*;
use yew_router::prelude::*;

use crate::{hooks::use_user_context, routes::Route};

#[function_component(NavBar)]
pub fn navbar() -> Html {
    let user_ctx = use_user_context();

    if user_ctx.inner.is_auth() {
        html! {
            <nav class="navbar navbar-expand-sm holi-green navbar-dark">
                <div class="container-fluid">
                    <div class="navbar-brand">
                        <Link<Route> to={Route::Entries}>
                            <img id="navbar-holi" src="./assets/images/holi.svg" alt="Holi Logo" />
                        </Link<Route>>
                    </div>

                    <div class="navbar-collapse ">
                        <ul class="navbar-nav">
                            <li>
                                <Link<Route> classes={classes!("nav-link")} to={Route::Entries}>
                                    { "Lernmaterial" }
                                </Link<Route>>
                            </li>
                            <li>
                                <Link<Route> classes={classes!("nav-link")} to={Route::UserPanel}>
                                    { "User Panel" }<span class="ms-1 badge bg-secondary">{"User"}</span>
                                </Link<Route>>
                            </li>
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
    }

    
}
