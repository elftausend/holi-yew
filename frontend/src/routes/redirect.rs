use gloo::storage::{LocalStorage, Storage};
use web_sys::window;
use yew::{function_component, html, use_effect_with_deps};
use yew_router::{history::{History, Location}, hooks::{use_history, use_location}};

use crate::{app::set_jwt, routes::htl_auth::CodeQuery};

use super::Route;


#[function_component(RedirectLocal)]
pub fn redirect() -> Html {

    LocalStorage::set("req_local_redirect", "true").expect("failed to set");
    let history = use_history().unwrap();
    history.push(Route::Auth);

    html!{

    }
}