use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::Route;

#[function_component(NavBar)]
pub fn navbar() -> Html {
    html! {
        <nav class="navbar navbar-expand-sm holi-green navbar-dark">
        <div class="navbar-brand">
            <img id="navbar-holi" src="./assets/images/holi.svg" alt="Holi Logo" loading="lazy"/>
        </div>
        
        <div class="navbar-collapse ">
            <ul class="navbar-nav">

                //<li class="nav-item active">
                    <Link<Route> classes={classes!("nav-link", "active")} to={Route::Entries}>
                        { "Lernmaterial" }
                    </Link<Route>>
                //</li>
                <li class="nav-item">
                    <Link<Route> classes={classes!("nav-link")} to={Route::UserPanel}>
                        { "User Panel" }
                    </Link<Route>>
                </li>
            </ul>
        </div>
    </nav>
    }
}