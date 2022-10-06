use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub children: Children,
}

#[function_component(CardGroup)]
pub fn card_group(props: &Props) -> Html {
    html! {
        <div class="card-group highlight-in-use">
        { for props.children.iter() }
        </div>
    }
}