pub mod entries;
pub mod htl_auth;
pub mod login;
pub mod logout;
pub mod page_not_found;
pub mod show_upload;
pub mod upload;
pub mod user_panel;

pub use entries::Entries;
pub use login::Login;
pub use logout::Logout;
pub use page_not_found::NotFound;
pub use show_upload::ShowUpload;
pub use upload::Upload;
pub use user_panel::UserPanel;
pub use htl_auth::OAuth2;
//pub use upload::Upload;

use reqwest::Method;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::api::request;

use self::login::UserInfo;

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/authenticated")]
    Auth,
    #[at("/login")]
    Login,
    #[at("/logout")]
    Logout,
    #[at("/")]
    Entries,
    #[at("/user_panel")]
    UserPanel,
    #[at("/upload")]
    Upload,
    #[at("/show_upload")]
    ShowUpload,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: &Route) -> Html {
    match routes {
        Route::Login => html! { <Login /> },
        Route::Logout => html! { <Logout /> },
        Route::Entries => html! {
            <Entries />
        },
        Route::UserPanel => html! { <UserPanel /> },
        Route::Upload => html! { <Upload /> },
        Route::ShowUpload => html! { <ShowUpload />},
        Route::NotFound => html! { <NotFound /> },
        Route::Auth => html! { <OAuth2 /> },
    }
}

pub async fn is_logged_in() -> bool {
    request::<_, UserInfo>(Method::GET, "user", (), true)
        .await
        .is_ok()
}

//pub async fn current_user() -> Result<UserInfo, HoliError>{
//    request::<(), UserInfo>(Method::GET, "user", ()).await
//}
