use yew::prelude::*;
use yew_router::prelude::Link;

use crate::{hooks::use_user_context, routes::Route};

#[function_component(Footer)]
pub fn footer() -> Html {
    let user_ctx = use_user_context();

    if user_ctx.inner.is_auth() {
        html! {
            <footer>
                <div class="text-center">
                    <Link<Route> to={Route::Entries}>
                        <img src="./assets/images/holi.svg" alt="holi logo" style="width: 20rem;" />
                    </Link<Route>>
                </div>
                <div class="dev-info">
                    <div class="row">

                        <div class="col">
                        {"Entwickler: Florian Nagy, 4B"}
                        <span class="it_color">{"HITS"}</span>{", "}
                        <a style="color: yellow;" href="https://github.com/elftausend" target="_blank" rel="noopener noreferrer">
                            {"GitHub"}
                        </a>
                        </div>
                        <div class="col">
                            <div style="float: right;">
                                <Link<Route> to={Route::Tos}>
                                    {"Über "}
                                    <span class="et_color">{"h"}</span>
                                    <span class="it_color">{"o"}</span>
                                    <span class="el_color">{"l"}</span>
                                    <span class="me_color">{"i"}</span>
                                    <span class="mb_color">{"."}</span>
                                    <br />

                                    {"Nutzungsbedingungen"}
                                </Link<Route>>
                            </div>
                        </div>
                    </div>
                </div>
            </footer>
        }
    } else {
        html!()
    }
}
