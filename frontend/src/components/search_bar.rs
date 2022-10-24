use gloo::utils::document;
use serde::{Deserialize, Serialize};
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_hooks::use_mount;
use yew_router::prelude::{use_history, History};

use crate::routes::Route;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct SearchQuery {
    pub page: u64,
    pub tags: String,
}

impl Default for SearchQuery {
    fn default() -> Self {
        Self {
            page: 0,
            tags: "".into(),
        }
    }
}

#[derive(Default, Clone)]
pub struct SearchBarInput {
    pub tags: String,
}

#[derive(Debug, Properties, PartialEq, Clone)]
pub struct Props {
    pub search_info: SearchQuery,
    pub route: Route,
}

#[function_component(SearchBar)]
pub fn search_bar(props: &Props) -> Html {
    let tag_input = use_state(SearchBarInput::default);
    let page = props.search_info.page;
    let history = use_history().unwrap();

    let on_search = {
        let props = props.clone();
        let history = history.clone();
        let tag_input = tag_input.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            history
                .push_with_query(
                    props.route.clone(),
                    SearchQuery {
                        page,
                        tags: tag_input.tags.clone(),
                    },
                )
                .unwrap();
        })
    };

    let current_focus = use_state(|| 0);

    use_mount(|| {
        let testing_ipts = ["Test", "Hallo", "Spannend", "Alphabet", "Custos"];

        let search_field: HtmlInputElement = document()
            .get_element_by_id("search_field")
            .unwrap()
            .unchecked_into();

        let input_callback = {
            let search_field = search_field.clone();
            Closure::wrap(Box::new(move |input: InputEvent| {
                let value = search_field.value();
                if &value == "" {
                    return;
                }
                let list_div = document().create_element("div").unwrap();
                list_div
                    .set_attribute("id", &format!("{} autocomplete-list", search_field.id()))
                    .unwrap();
                list_div
                    .set_attribute(
                        "class",
                        &format!("{} autocomplete-items", search_field.id()),
                    )
                    .unwrap();
                search_field
                    .parent_node()
                    .unwrap()
                    .append_child(&list_div)
                    .unwrap();

                for inpt in testing_ipts {
                    if (&inpt[..value.len()]).to_uppercase() == value.to_uppercase() {
                        let div = document().create_element("div").unwrap();
                        div.set_inner_html(&format!(
                            "
                                <strong>{}</strong>
                                <input type='hidden' value='{}'/>
                            ",
                            &inpt[..value.len()],
                            inpt
                        ));
                        let search_field = search_field.clone();
                        div.add_event_listener_with_callback(
                            "click",
                            Closure::wrap(Box::new(move |e: MouseEvent| {
                                let list_input: HtmlInputElement = e.target_unchecked_into();
                                search_field.set_value(&list_input.value());
                            }) as Box<dyn FnMut(_)>)
                            .as_ref()
                            .unchecked_ref(),
                        )
                        .unwrap();

                        list_div.append_child(&list_div).unwrap();
                    }
                }
            }) as Box<dyn FnMut(_)>)
        };

        search_field
            .add_event_listener_with_callback("input", input_callback.as_ref().unchecked_ref())
            .unwrap();
    });

    let on_input_change = {
        let props = props.clone();
        let history = history.clone();
        let tag_input = tag_input.clone();
        let current_focus = current_focus.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();

            let mut info = (*tag_input).clone();
            info.tags = input.value();

            history
                .push_with_query(
                    props.route.clone(),
                    SearchQuery {
                        page,
                        tags: info.tags.clone(),
                    },
                )
                .unwrap();

            tag_input.set(info);
        })
    };

    let onkeypress = {
        Callback::from(move |e: KeyboardEvent| {
            // check for enter key
            if e.key_code() != 13 {
                return;
            }

            history
                .push_with_query(
                    Route::Entries,
                    SearchQuery {
                        page,
                        tags: tag_input.tags.clone(),
                    },
                )
                .unwrap();
        })
    };

    html! {
        <>
        //<div class="d-flex mt-4 mb-4">
        //   <div class="autocomplete">
            <input autocomplete="off"
                value={props.search_info.tags.clone()}
                onkeypress={onkeypress}
                oninput={on_input_change}
                id="search_field"
                class="form-control input-field"
                type="search"
                placeholder="Tags oder Titel eingeben"
                name="tags"
            />
    //    </div>
            <button style="width: 80px;" onclick={on_search} id="search_button" class="btn btn-secondary ms-2">{"Suchen"}</button>
            </>
       // </div>
    }
}
