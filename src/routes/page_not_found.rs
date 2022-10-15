use yew::prelude::*;
use yew_router::prelude::{use_history, History};

#[function_component(NotFound)]
pub fn not_found() -> Html {
    let history = use_history().unwrap();

    let onback = {
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            history.back();
        })
    };

    html! {
        <>
            <div class="container-fluid mt-3">
                <button onclick={onback} class="btn btn-primary">
                    {"Zurück"}
                </button>
            </div>
            <div class="container mt-5">
                <div class="notfound">
                    <img src="./assets/images/questionmark.png" alt="Holi Logo" loading="lazy"/>
                        <h1>
                            {"Seite"}<br />
                            {"nicht"}<br />
                            {"gefunden"}<br />
                        </h1>
                </div>
            </div>
        </>
    }
}
