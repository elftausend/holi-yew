use super::Route;
use crate::components::Auth;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(UserPanel)]
pub fn user_panel() -> Html {
    html! {
        <>
            <Auth>
                <div class="row highlight">
                <Link<Route> classes={classes!("col", "et_bg_color", "card", "square")} to={Route::Upload}>
                    <h1 class="text-center push-down text-white" style="margin-top: 56px;">{"Upload"}</h1>
                </Link<Route>>
                </div>
                <div class="row highlight">
                    <Link<Route> classes={classes!("col", "it_bg_color", "card", "square")} to={Route::Edit}>
                        <h1 class="text-center push-down text-white" style="margin-top: 56px;">{"Edit"}</h1>
                    </Link<Route>>
                </div>
            </Auth>
        </>
    }
}
