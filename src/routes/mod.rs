pub mod entries;
pub mod login;
pub mod upload;

pub use entries::Entries;
use crate::components::NavBar;
pub use login::Login;
//pub use upload::Upload;

use reqwest::Method;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::api::request;

use self::login::UserInfo;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Login,
    #[at("/entries")]
    Entries,
    #[at("/user_panel")]
    UserPanel,
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
        Route::UserPanel => html! {
            <div>
                <NavBar />
                {"HI"}
            </div>
            
            
        },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

pub async fn is_logged_in() -> bool {
    request::<_, UserInfo>(Method::GET, "user", ()).await.is_ok()
}

//pub async fn current_user() -> Result<UserInfo, HoliError>{
//    request::<(), UserInfo>(Method::GET, "user", ()).await
//}
