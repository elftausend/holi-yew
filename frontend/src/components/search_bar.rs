use std::{cell::RefCell, rc::Rc};

use gloo::utils::document;
use serde::{Deserialize, Serialize};
use wasm_bindgen::{closure::WasmClosure, convert::FromWasmAbi, prelude::Closure, JsCast};
use web_sys::{Element, HtmlCollection, HtmlInputElement, HtmlElement};
use yew::prelude::*;
use yew_hooks::use_mount;
use yew_router::prelude::{use_history, AnyHistory, History};

use crate::{
    api::{get_unique_tags, UniqueTag},
    routes::Route,
};

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

fn update_search(
    history: AnyHistory,
    value: String,
    tag_input: UseStateHandle<SearchBarInput>,
    props: Props,
) {
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

fn mount_tags(unique_tags: UseStateHandle<Vec<UniqueTag>>) {
    use_mount(|| {
        wasm_bindgen_futures::spawn_local(async move {
            let tags = get_unique_tags().await.unwrap();
            log::info!("tags: {tags:?}");
            let Ok(mut tags) = get_unique_tags().await else {
                return;
            };
            tags.sort_by(|a, b| b.count.cmp(&a.count));
            unique_tags.set(tags);
        });
    });
}

fn on_search_callback(
    history: AnyHistory,
    props: Props,
    tag_input: UseStateHandle<SearchBarInput>,
) -> Callback<MouseEvent> {
    Callback::from(move |e: MouseEvent| {
        e.prevent_default();
        history
            .push_with_query(
                props.route.clone(),
                SearchQuery {
                    page: props.search_info.page,
                    tags: tag_input.tags.clone(),
                    scroll_to_bar: false,
                },
            )
            .unwrap();
    })
}
#[function_component(SearchBar)]
pub fn search_bar(props: &Props) -> Html {
    let unique_tags = use_state(Vec::new);
    let tag_input = use_state(SearchBarInput::default);
    let page = props.search_info.page;
    let history = use_history().unwrap();

    let on_search = on_search_callback(history.clone(), props.clone(), tag_input.clone());

    mount_tags(unique_tags.clone());

    {
        let unique_tags1 = unique_tags.clone();
        let history = history.clone();
        let tag_input = tag_input.clone();
        let props = props.clone();

        use_effect_with_deps(
            move |_| {
                let current_focus = Rc::new(RefCell::new(-1));

                let tags = (*unique_tags1).clone();

                let search_field: HtmlInputElement = document()
                    .get_element_by_id("search_field")
                    .unwrap()
                    .unchecked_into();

                let input_callback = {
                    let search_field = search_field.clone();
                    let current_focus = current_focus.clone();
                    Closure::wrap(Box::new(move |_input: InputEvent| {

                        *current_focus.borrow_mut() = -1;
                        //let mut unsorted_tag_divs = vec![];

                        let tags = tags.clone();
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
                        for tag in tags {
                            let splitted = value.split(' ');

                            let Some(value) = splitted.last() else {
                                continue;
                            };

                            let Some(div) = create_tag_div_if_match(&value, &tag) else {
                                continue;
                            };

                            // do not show more than 6 tags at a time
                            if idx == 6 {
                                break;
                            }

                            idx += 1;

                            let click_tag = click_tag(
                                props.clone(),
                                history.clone(),
                                search_field.clone(),
                                tag_input.clone(),
                                idx,
                                value.to_string(),
                            );

                            div.add_event_listener_with_callback(
                                "click",
                                click_tag.as_ref().unchecked_ref(),
                            )
                            .unwrap();

                            list_div.append_child(&div).unwrap();
                            // unsorted_tag_divs.push((div, tag.count));
                            click_tag.forget();
                        }

                        //unsorted_tag_divs.sort_by(|a, b| b.1.cmp(&a.1));
                        //for (div, _) in unsorted_tag_divs {
                        //    list_div.append_child(&div).unwrap();
                        //}
                    }) as Box<dyn FnMut(_)>)
                };
                //ffclick_tag.forget();

                search_field
                    .add_event_listener_with_callback(
                        "input",
                        input_callback.as_ref().unchecked_ref(),
                    )
                    //.add_event_listener_with_callback("input", callback.)
                    .unwrap();
                input_callback.forget();

                let keydown_callback = searchbar_keydown(search_field.clone(), current_focus);

                search_field
                    .add_event_listener_with_callback(
                        "keydown",
                        keydown_callback.as_ref().unchecked_ref(),
                    )
                    .unwrap();

                keydown_callback.forget();
                || {}
            },
            unique_tags.clone(),
        );
    }

    let on_input_change = {
        let props = props.clone();
        let history = history.clone();
        let tag_input = tag_input.clone();
        //let current_focus = current_focus.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            update_search(
                history.clone(),
                input.value(),
                tag_input.clone(),
                props.clone(),
            );
        })
    };

    // TODO: conflicts with tag ??
    /*let onkeypress = {
        let props = props.clone();
        Callback::from(move |e: KeyboardEvent| {
            // check for enter key
            if e.key_code() != 13 {
                return;
            }

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
    };*/

    html! {
        <>
        //<div class="d-flex mt-4 mb-4">
           <div class="autocomplete">
            <input autocomplete="off"
                value={props.search_info.tags.clone()}
                //onkeypress={onkeypress}
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

pub fn searchbar_keydown(
    search_field: HtmlInputElement,
    current_focus: Rc<RefCell<i32>>,
) -> Closure<dyn FnMut(KeyboardEvent)> {
    Closure::wrap(Box::new(move |e: KeyboardEvent| {
        let Some(list) = document().get_element_by_id(&format!("{}autocomplete-list", search_field.id())) else {
            return;
        };

        let tags = list.get_elements_by_tag_name("div");
        if tags.length() == 0 {
            return;
        }

        let mut current_focus = current_focus.borrow_mut();
        log::info!("focus: {}", current_focus);

        // down key
        if e.key_code() == 40 {
            *current_focus += 1;
            add_active(&tags, &mut *current_focus);
        } else if e.key_code() == 38 {
            // up key

            // prevent moving cursor to the start of the input field.
            e.prevent_default();
            *current_focus -= 1;

            add_active(&tags, &mut *current_focus);
        } else if e.key_code() == 13 {
            // enter
            e.prevent_default();

            // update route
            if *current_focus == -1 {

            } else {
                let tag: HtmlElement = tags.get_with_index(*current_focus as u32).unwrap().unchecked_into();
                tag.click();
                *current_focus = -1;
                
            }
        }
    }) as Box<dyn FnMut(KeyboardEvent)>)
}

pub fn add_active(tags: &HtmlCollection, current_focus: &mut i32) {
    remove_active(tags);

    if *current_focus >= tags.length() as i32 {
        *current_focus = 0;
    }
    if *current_focus < 0 {
        *current_focus = tags.length() as i32 - 1
    }

    let Some(tag) = tags.get_with_index(*current_focus as u32) else {
        return
    };

    tag.class_list().add_1("autocomplete-active").unwrap_or_default();
}

pub fn remove_active(tags: &HtmlCollection) {
    for i in 0..tags.length() {
        tags.get_with_index(i)
            .unwrap()
            .class_list()
            .remove_1("autocomplete-active")
            .unwrap_or_default();
    }
}

pub fn click_tag(
    props: Props,
    history: AnyHistory,
    search_field: HtmlInputElement,
    tag_input: UseStateHandle<SearchBarInput>,
    idx: usize,
    value: String,
) -> Closure<dyn FnMut(MouseEvent)> {
    Closure::wrap(Box::new(move |_e: MouseEvent| {
        let list_input: HtmlInputElement = document()
            .get_elements_by_tag_name("input")
            .get_with_index(idx as u32)
            .unwrap()
            .unchecked_into();
        //let list_input: HtmlInputElement = e.target_unchecked_into();

        let tags = search_field.value();

        // removes the pre-autocompletion input
        let mut tags = tags[..tags.len() - value.len()].to_string();

        tags.push_str(&format!("{} ", list_input.value()));

        update_search(history.clone(), tags, tag_input.clone(), props.clone());

        close_list();
    }) as Box<dyn FnMut(MouseEvent)>)
}

pub fn create_tag_div_if_match(value: &str, tag: &UniqueTag) -> Option<Element> {
    let input = &tag.name;
    
    if value == "" {
        return None;
    }

    if value.len() > input.len() {
        return None;
    }
    if (&input[..value.len()]).to_uppercase() != value.to_uppercase() {
        return None;
    }
    let div = document().create_element("div").unwrap();
    div.set_inner_html(&format!(
        "
            <strong>{}</strong>{}
            {tag_count}
            <input type='hidden' value='{}'/>
        ",
        &input[..value.len()],
        &input[value.len()..],
        input,
        tag_count = format!(
            r#"<span style="float=right; color:violet"> ({}) </span>"#,
            tag.count
        )
    ));

    Some(div)
}
