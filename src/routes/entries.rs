use yew::prelude::*;
use yew_router::prelude::*;

use super::Route;

pub struct EntryInfo {
    uploader: String,

}

#[function_component(Entries)]
pub fn entries() -> Html {
    let history = use_history().unwrap();

    let onclick = Callback::once(move |_| history.push(Route::Login));
    html! {
        <div>
            <h1>{ "Entries" }</h1>
            <button {onclick}>{ "Go Home" }</button>
        </div>
    }
}