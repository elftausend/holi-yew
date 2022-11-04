use gloo::utils::document;
use js_sys::Function;
use serde::{Deserialize, Serialize};
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{HtmlInputElement, HtmlElement};
use yew::prelude::*;
use yew_hooks::use_mount;
use yew_router::prelude::{use_history, History, AnyHistory};

use crate::routes::Route;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct SearchQuery {
    pub page: u64,
    pub tags: String,
    pub scroll_to_bar: bool,
}

impl Default for SearchQuery {
    fn default() -> Self {
        Self {
            page: 0,
            tags: "".into(),
            scroll_to_bar: false,
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

fn close_list() {
    let list_items = document().get_elements_by_class_name("autocomplete-items");
    for i in 0..list_items.length() {
        let list_item = list_items.item(i).unwrap();
        list_item
            .parent_node()
            .unwrap()
            .remove_child(&list_item)
            .unwrap();
    }
}

fn update_search(history: AnyHistory, value: String, tag_input: UseStateHandle<SearchBarInput>, props: Props) {
    let mut info = (*tag_input).clone();
    info.tags = value;

    history
        .push_with_query(
            props.route.clone(),
            SearchQuery {
                page: props.search_info.page,
                tags: info.tags.clone(),
                scroll_to_bar: false,
            },
        )
        .unwrap();

    tag_input.set(info);
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
                        scroll_to_bar: false,
                    },
                )
                .unwrap();
        })
    };

    let current_focus = use_state(|| 0);

    {
        let history = history.clone();
        let tag_input = tag_input.clone();
        let props = props.clone();
        use_mount(move || {
            let testing_ipts = [
                "Test", "Hallo", "Spannend", "Alphabet", "Custos", "Test2", "Testok",
            ];

            let search_field: HtmlInputElement = document()
                .get_element_by_id("search_field")
                .unwrap()
                .unchecked_into();

            let input_callback = {
                let search_field = search_field.clone();
                Closure::wrap(Box::new(move |input: InputEvent| {
                    let value = search_field.value();

                    close_list();
                    if &value == "" {
                        return;
                    }
                    let list_div = document().create_element("div").unwrap();
                    list_div
                        .set_attribute("id", &format!("{}autocomplete-list", search_field.id()))
                        .unwrap();

                    list_div
                        .set_attribute("class", "autocomplete-items")
                        .unwrap();

                    search_field
                        .parent_node()
                        .unwrap()
                        .append_child(&list_div)
                        .unwrap();

                    
                    let mut idx = 0;
                    for input in testing_ipts {
                        let tag_input = tag_input.clone();
                        if value.len() > input.len() {
                            continue;
                        }
                        if (&input[..value.len()]).to_uppercase() == value.to_uppercase() {
                            idx += 1;

                            let div = document().create_element("div").unwrap();
                            div.set_inner_html(&format!(
                                "
                                    <strong>{}</strong>{}
                                    <input type='hidden' value='{}'/>
                                ",
                                &input[..value.len()],
                                &input[value.len()..],
                                input
                            ));
                            let search_field = search_field.clone();

                            let click_tag = {
                                let tag_input = tag_input.clone();
                                let search_field = search_field.clone();
                                let props = props.clone();
                                let history = history.clone();
                                Closure::wrap(Box::new(move |_e: MouseEvent| {
                                    let props = props.clone();
                                    let list_input: HtmlInputElement = document().get_elements_by_tag_name("input").get_with_index(idx as u32).unwrap().unchecked_into();
                                    //let list_input: HtmlInputElement = e.target_unchecked_into();
                                    update_search(history.clone(), list_input.value(), tag_input.clone(), props.clone());

                                }) as Box<dyn FnMut(_)>)
                            };


                            div.add_event_listener_with_callback(
                                "click",
                                click_tag.as_ref().unchecked_ref(),
                            )
                            .unwrap();

                            list_div.append_child(&div).unwrap();
                            click_tag.forget();
                        }
                    }
                }) as Box<dyn FnMut(_)>)
            };
            //ffclick_tag.forget();

            search_field
                .add_event_listener_with_callback("input", input_callback.as_ref().unchecked_ref())
                //.add_event_listener_with_callback("input", callback.)
                .unwrap();
            input_callback.forget();
        });
    }

    let on_input_change = {
        let props = props.clone();
        let history = history.clone();
        let tag_input = tag_input.clone();
        let current_focus = current_focus.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            update_search(history.clone(), input.value(), tag_input.clone(), props.clone());
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
                        scroll_to_bar: false,
                    },
                )
                .unwrap();
        })
    };

    html! {
        <>
        //<div class="d-flex mt-4 mb-4">
           <div class="autocomplete">
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
        </div>
            <button style="width: 80px;" onclick={on_search} id="search_button" class="btn btn-secondary ms-2">{"Suchen"}</button>
            </>
       // </div>
    }
}
