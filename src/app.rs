use gloo::storage::{LocalStorage, Storage};
use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::{Route, switch};

const TOKEN_KEY: &str = "yew.token";

pub fn set_jwt(token: Option<String>) {
    if let Some(token) = token.clone() {
        LocalStorage::set(TOKEN_KEY, token).expect("failed to set");
    } else {
        LocalStorage::delete(TOKEN_KEY);
    }

}

pub fn get_jwt() -> Option<String> {
    LocalStorage::get(TOKEN_KEY).ok()
}


#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}
