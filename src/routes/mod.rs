pub mod entries;
pub mod login;
pub mod upload;

pub use entries::Entries;
pub use login::Login;
//pub use upload::Upload;

use reqwest::Method;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::{api::request, error::HoliError};

use self::login::UserInfo;

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

pub async fn current_user() -> Result<UserInfo, HoliError>{
    request::<(), UserInfo>(Method::GET, "user", ()).await
}
