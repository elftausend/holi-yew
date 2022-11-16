use std::{cell::RefCell, rc::Rc};

use gloo::utils::document;
use serde::{Deserialize, Serialize};
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{Element, HtmlCollection, HtmlElement, HtmlInputElement};
use yew::prelude::*;
use yew_hooks::use_mount;
use yew_router::prelude::{use_history, AnyHistory, History};

use crate::{
    api::{get_unique_tags, UniqueTag},
    routes::Route, utils::is_mobile,
};

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
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

#[derive(Default, Clone, PartialEq, Eq)]
pub struct SearchBarInput {
    pub tags: String,
}

#[derive(Debug, Properties, Eq, PartialEq, Clone)]
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

fn update_division_tags(props: Props) {
    let props1 = props.clone();
    use_effect_with_deps(
        move |_| {
            let search_field: HtmlInputElement = document()
                .get_element_by_id("search_field")
                .unwrap()
                .unchecked_into();

            search_field.set_value(&props1.search_info.tags);
            || {}
        },
        props.search_info.tags,
    );
}

#[function_component(SearchBar)]
pub fn search_bar(props: &Props) -> Html {
    let current_focus = Rc::new(RefCell::new(-1));
    let unique_tags = use_state(Vec::new);
    let tag_input = use_state(|| SearchBarInput {
        tags: props.search_info.tags.clone(),
    });

    let history = use_history().unwrap();

    let on_search = on_search_callback(history.clone(), props.clone(), tag_input.clone());

    update_division_tags(props.clone());
    mount_tags(unique_tags.clone());

    {
        let unique_tags1 = unique_tags.clone();
        let history = history.clone();
        let tag_input = tag_input.clone();
        let props = props.clone();
        let current_focus = current_focus.clone();

        use_effect_with_deps(
            move |_| {
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

                        let tags = tags.clone();
                        let value = search_field.value();

                        close_list();
                        if value.is_empty() {
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

                        let selection_start = search_field.selection_start().unwrap().unwrap();

                        let mut idx = 0;
                        for tag in tags {
                            log::info!("{selection_start:?}");
                            let split_inputs = value.split(' ').collect::<Vec<&str>>();

                            let tag_pos = tag_idx_at_cursor(selection_start, &split_inputs);

                            // if a tag was already written in the search field, then
                            // do not add this tag to the tag autocompletion again.
                            // if the tag that is currently edited/written, do not skip it -> (tag_pos)
                            if is_tag_in_search_bar(&split_inputs, &tag, tag_pos) {
                                continue;
                            }

                            let Some(value) = split_inputs.get(tag_pos) else {
                                continue;
                            };

                            let Some(div) = create_tag_div_if_match(value, &tag) else {
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
                                tag_pos,
                            );

                            div.add_event_listener_with_callback(
                                "click",
                                click_tag.as_ref().unchecked_ref(),
                            )
                            .unwrap();

                            list_div.append_child(&div).unwrap();
                            click_tag.forget();
                        }
                    }) as Box<dyn FnMut(_)>)
                };

                search_field
                    .add_event_listener_with_callback(
                        "input",
                        input_callback.as_ref().unchecked_ref(),
                    )
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
            unique_tags,
        );
    }

    let on_input_change = {
        let props = props.clone();
        let tag_input = tag_input.clone();
        let history = history.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut tags = (*tag_input).clone();
            tags.tags = input.value();

            tag_input.set(tags);

            if !is_mobile() {
                update_search(
                    history.clone(),
                    input.value(),
                    tag_input.clone(),
                    props.clone(),
                );
            }
        })
    };

    // TODO: conflicts with tag ??
    let onkeypress = {
        let props = props.clone();
        let tag_input = tag_input.clone();
        Callback::from(move |e: KeyboardEvent| {
            // check for enter key
            if e.key_code() != 13 {
                return;
            }

            if *current_focus.borrow() != -1 {
                return;
            }

            log::info!("key press!!");

            close_list();

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
    };

    html! {
        <>
        //<div class="d-flex mt-4 mb-4">
           <div class="autocomplete">
            <input autocomplete="off"
                value={tag_input.tags.clone()}
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
            e.prevent_default();
            *current_focus += 1;
            add_active(&tags, &mut current_focus);
        } else if e.key_code() == 38 {
            // up key

            // prevent moving cursor to the start of the input field.
            e.prevent_default();
            *current_focus -= 1;

            add_active(&tags, &mut current_focus);
        } else if e.key_code() == 13 {
            // enter
            //e.prevent_default();

            // could update route
            if *current_focus == -1 {
                return
            }
            
            let tag: HtmlElement = tags
                .get_with_index(*current_focus as u32)
                .unwrap()
                .unchecked_into();
            tag.click();
            e.prevent_default();
            *current_focus = -1;
        
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

    tag.class_list()
        .add_1("autocomplete-active")
        .unwrap_or_default();
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
    tag_idx: usize,
) -> Closure<dyn FnMut(MouseEvent)> {
    Closure::wrap(Box::new(move |_e: MouseEvent| {
        let list_input: HtmlInputElement = document()
            .get_elements_by_tag_name("input")
            .get_with_index(idx as u32)
            .unwrap()
            .unchecked_into();
        //let list_input: HtmlInputElement = e.target_unchecked_into();

        let list_input = list_input.value();
        let tags = search_field.value();

        let mut tags = tags.split(' ').collect::<Vec<&str>>();
        tags[tag_idx] = &list_input;

        let tags = tags
            .into_iter()
            .map(|tag| format!("{tag} "))
            .collect::<String>();

        update_search(history.clone(), tags, tag_input.clone(), props.clone());

        close_list();
    }) as Box<dyn FnMut(MouseEvent)>)
}

pub fn tag_idx_at_cursor(selection_start: u32, split_inputs: &[&str]) -> usize {
    let mut tag_pos = split_inputs.len() - 1;
    let mut len = 0;
    for (idx, input) in split_inputs.iter().enumerate() {
        if selection_start >= len {
            tag_pos = idx;
        }
        // +1 for the stripped away whitespace
        len += input.len() as u32 + 1;
    }
    tag_pos
}

pub fn create_tag_div_if_match(value: &str, tag: &UniqueTag) -> Option<Element> {
    let input = &tag.name;

    // would show a list with tags when whitespace was entered
    if value.is_empty() {
        return None;
    }

    if value.len() > input.len() {
        return None;
    }
    if (input[..value.len()]).to_uppercase() != value.to_uppercase() {
        return None;
    }
    let div = document().create_element("div").unwrap();
    div.set_inner_html(&format!(
        r#"
            <strong>{}</strong>{}
            <span style="float=right; color:violet"> ({tag_count}) </span>
            <input type='hidden' value='{}'/>
        "#,
        &input[..value.len()],
        &input[value.len()..],
        input,
        tag_count = tag.count        
    ));

    Some(div)
}

fn is_tag_in_search_bar(split_inputs: &[&str], tag: &UniqueTag, tag_idx: usize) -> bool {
    for (idx, input) in split_inputs.iter().enumerate() {
        if idx == tag_idx {
            continue;
        }
        if input == &tag.name {
            return true;
        }
    }
    false
}
