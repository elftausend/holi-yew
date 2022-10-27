use yew::prelude::*;
use yew_router::prelude::{use_history, History};

use crate::routes::Route;

use super::SearchQuery;

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct Props {
    pub name: String,
    #[prop_or_default]
    pub route: Route,
}

#[function_component(Tag)]
pub fn tag(props: &Props) -> Html {
    let history = use_history().unwrap();

    let onclick = {
        let props = props.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            history
                .push_with_query(
                    props.route.clone(),
                    SearchQuery {
                        page: 0,
                        tags: props.name.clone(),
                        scroll_to_bar: true,
                    },
                )
                .unwrap();
        })
    };
    html! {
        <>
            <button onclick={onclick} class="badge me-1 bg-secondary tag">{props.name.clone()}</button>
        </>
    }
}
