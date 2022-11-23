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
                    <div>
                        
                        {"Entwickler: Florian Nagy, 3B"}
                        <span class="it_color">{"HITS"}</span>{", "}
                        <a style="color: yellow;" href="https://github.com/elftausend" target="_blank" rel="noopener noreferrer">
                            {"GitHub"}
                        </a>
                        <div style="float: right;">
                            <Link<Route> to={Route::Tos}>
                                {"Nutzungsbedingungen"}
                            </Link<Route>>
                        </div>
                    </div>

                </div>
            </footer>
        }
    } else {
        html!()
    }
}
