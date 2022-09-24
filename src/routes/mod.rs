pub mod login;
pub mod entries;
pub mod upload;

pub use login::Login;
pub use entries::Entries;
//pub use upload::Upload;

use yew_router::prelude::*;
use yew::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Login,
    #[at("/entries")]
    Entries,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: &Route) -> Html {
    match routes {
        Route::Login => html! { <Login /> },
        Route::Entries => html! {
            <Entries />
        },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

