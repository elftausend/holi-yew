pub mod edit;
pub mod edit_upload;
pub mod entries;
pub mod htl_auth;
pub mod logout;
pub mod page_not_found;
pub mod show_upload;
pub mod upload;
pub mod user_panel;

pub use edit::Edit;
pub use edit_upload::EditUpload;
pub use entries::Entries;
pub use htl_auth::OAuth2;
pub use logout::Logout;
pub use page_not_found::NotFound;
pub use show_upload::ShowUpload;
pub use upload::Upload;
pub use user_panel::UserPanel;
//pub use upload::Upload;

use reqwest::Method;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::api::request;

use self::htl_auth::UserInfo;

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Entries,
    #[at("/authenticated")]
    Auth,
    #[at("/logout")]
    Logout,
    #[at("/user_panel")]
    UserPanel,
    #[at("/upload")]
    Upload,
    #[at("/edit")]
    Edit,
    #[at("/edit_upload")]
    EditUpload,
    #[at("/show_upload")]
    ShowUpload,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: &Route) -> Html {
    match routes {
        Route::Logout => html! { <Logout /> },
        Route::Entries => html! {
            <Entries />
        },
        Route::UserPanel => html! { <UserPanel /> },
        Route::Upload => html! { <Upload /> },
        Route::ShowUpload => html! { <ShowUpload />},
        Route::NotFound => html! { <NotFound /> },
        Route::Auth => html! { <OAuth2 /> },
        Route::Edit => html! { <Edit /> },
        Route::EditUpload => html! { <EditUpload /> },
    }
}

pub async fn is_logged_in() -> bool {
    request::<_, UserInfo>(Method::GET, "user", ())
        .await
        .is_ok()
}

//pub async fn current_user() -> Result<UserInfo, HoliError>{
//    request::<(), UserInfo>(Method::GET, "user", ()).await
//}
