use gloo::storage::{LocalStorage, Storage};
use web_sys::window;
use yew::{function_component, html};
use yew_router::hooks::use_history;

use crate::REDIRECT;

#[function_component(RedirectLocal)]
pub fn redirect() -> Html {
    LocalStorage::set("req_local_redirect", "true").expect("failed to set");
    let history = use_history().unwrap();
    let href = format!("https://auth.htl-hl.ac.at/authorize.php?response_type=code&client_id=holi.htl-hl.ac.at&redirect_uri={REDIRECT}&state=new");
    window().unwrap().location().set_href(&href).unwrap();

    html! {}
}
