use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer>
            <div class="text-center">
                <img src="./assets/images/holi.svg" alt="holi logo" style="width: 20rem;" />
            </div>
        </footer>
    }
}