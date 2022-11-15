use super::Route;
use crate::{components::Auth, hooks::use_user_context};
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(UserPanel)]
pub fn user_panel() -> Html {
    let user_ctx = use_user_context();


    let upload_banned_modal = if user_ctx.inner.upload_banned {
        html! {
            <>
                <div class="row highlight" data-bs-toggle="modal" data-bs-target="#noupload">
                    <div class="col banned_bg_color card square">
                        <h1 class="text-center push-down text-white" style="margin-top: 56px;">{"Upload"}</h1>
                    </div>
                </div>
                <div class="modal fade" id="noupload" tabindex="-1" aria-labelledby="nouploadLabel" aria-hidden="true">
                    <div class="modal-dialog">
                        <div class="modal-content">
                            <div class="modal-header">
                                <h1 class="modal-title fs-5" id="nouploadLabel">{"Gesperrt"}</h1>
                                <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
                            </div>
                            <div class="modal-body">
                                {"Diesem Nutzer wurde die Berechtigung, Beiträge hochzuladen, entzogen."}
                            </div>
                            <div class="modal-footer">
                                <button type="button" class="btn btn-primary" data-bs-dismiss="modal">{"Ok"}</button>
                            </div>
                        </div>
                    </div>
                </div>
            </>
        }
    } else {
        html! {
            <div class="row highlight">
                <Link<Route> classes={classes!("col", "et_bg_color", "card", "square")} to={Route::Upload}>
                    <h1 class="text-center push-down text-white" style="margin-top: 56px;">{"Upload"}</h1>
                </Link<Route>>
            </div>
        }
    };

    html! {
        <>
            <Auth>
                {upload_banned_modal}
                <div class="row highlight">
                    <Link<Route> classes={classes!("col", "it_bg_color", "card", "square")} to={Route::Edit}>
                        <h1 class="text-center push-down text-white" style="margin-top: 56px;">{"Edit"}</h1>
                    </Link<Route>>
                </div>
            </Auth>
        </>
    }
}
