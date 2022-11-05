use serde::{Deserialize, Serialize};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::Route;

use super::SearchQuery;

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct PageQuery {
    // TODO: pub tags: String
    pub page: u64,
}

#[derive(Debug, Properties, PartialEq, Clone)]
pub struct Props {
    pub search_info: SearchQuery,
    pub total_pages: u64,
    pub route_to_page: Route,
}

#[function_component(Pagination)]
pub fn pagination(props: &Props) -> Html {
    let invalid_input = use_state(|| false);
    let history = use_history().unwrap();

    let Props {
        search_info,
        total_pages,
        route_to_page,
    } = props.clone();

    let wrong_page = if *invalid_input {
        "Ungültige Seite!"
    } else {
        ""
    };

    html! {
        <div class="d-flex justify-content-center flex-nowrap mt-3">
            <ul class="pagination">
                <li class="page-item">
                    {
                        if search_info.page == 0 {
                            html!{
                                <button style="width: 37.5px; height: 38px;" class="btn btn-danger"></button>
                            }
                        } else {
                            html!{
                                <Link<Route, SearchQuery>
                                    classes={classes!("page-link")}
                                    disabled={search_info.page==1}
                                    to={route_to_page.clone()}
                                    query={Some(SearchQuery{page: search_info.page-1, tags: "".to_string(), scroll_to_bar: true})}
                                >
                                    { "<" }
                                </Link<Route, SearchQuery>>
                            }
                        }
                    }


                </li>
                <li class="page-item">
                    {
                        if search_info.page >= total_pages {
                            html! {
                                <button style="width: 37.5px; height: 38px;" class="btn btn-danger"></button>
                            }
                        } else {
                            html! {
                                <a href="#">
                                <Link<Route, SearchQuery>
                                    classes={classes!("page-link")}
                                    disabled={true}
                                    to={route_to_page}
                                    query={Some(SearchQuery{page: search_info.page+1, tags: search_info.tags, scroll_to_bar: true})}
                                >
                                    { ">" }
                                </Link<Route, SearchQuery>>
                                </a>
                            }
                        }
                    }


                </li>
            </ul>
            <div class="text-center ">
                <input 
                    class="ms-2"
                    autocomplete="off" 
                    /*onkeypress="return onlyDigits(event)"*/ 
                    onkeypress={only_digits()}
                    oninput={enter_page(props.clone(), history.clone(), invalid_input)}
                    style="width: 48px; height: 38px;" 
                    id="page-input" 
                    name="page-input" 
                    placeholder = {(props.search_info.page +1).to_string()}
                />
                <span class="ms-1">
                    {format!("[{}/{}]", props.search_info.page+1, props.total_pages+1)}
                </span>
                <div class="mt-1">
                    {wrong_page}
                </div>
            </div>
        </div>
    }
}

fn enter_page(props: Props, history: AnyHistory, invalid_input: UseStateHandle<bool>) -> Callback<InputEvent> {
    Callback::from(move |e: InputEvent| {
        invalid_input.set(false);
        let props = props.clone();
        let input_field: HtmlInputElement = e.target_unchecked_into();
    
        let page = input_field.value().parse::<u64>().unwrap_or(1) -1;
        if page > props.total_pages {
            invalid_input.set(true);
            return;
        }
                
        history.push_with_query(
            props.route_to_page,
            SearchQuery {
                page,
                tags: props.search_info.tags,
                scroll_to_bar: props.search_info.scroll_to_bar,
            }
        ).unwrap();       
    })
}

fn only_digits() -> Callback<KeyboardEvent> {
    Callback::from(move |e: KeyboardEvent| {
    
        let key_code = e.key_code();
        if !(key_code == 13 || key_code >= 48 && key_code <= 57) {
            e.prevent_default();
            return;
        }    
    })
}