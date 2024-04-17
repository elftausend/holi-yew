use web_sys::window;
use yew::{function_component, html, use_effect_with_deps};
use yew_router::{history::Location, hooks::use_location};

use crate::routes::htl_auth::CodeQuery;


#[function_component(RedirectLocal)]
pub fn redirect() -> Html {


    let location = use_location().unwrap();
    {
        let location_inner = location.clone();
        // try with use_mount
        use_effect_with_deps(
            move |_| {
                let code_query = location_inner.query::<CodeQuery>().unwrap_or_default();
                let href = format!("http://127.0.0.1:4932/authenticated?code={}", code_query.code);
                window().unwrap().location().set_href(&href).unwrap();

               || ()
            },
            location.query::<CodeQuery>().unwrap_or_default(),
        );
    }

    html!{

    }
}