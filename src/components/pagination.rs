use serde::{Deserialize, Serialize};
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
    let Props {
        search_info,
        total_pages,
        route_to_page,
    } = props.clone();

    html! {
        <div class="d-flex justify-content-center flex-nowrap mt-3">
            <ul class="pagination">
                <li class="page-item">
                    {
                        if search_info.page == 1 {
                            html!{
                                <button style="width: 37.5px; height: 38px;" class="btn btn-danger"></button>
                            }
                        } else {
                            html!{
                                <Link<Route, SearchQuery>
                                    classes={classes!("page-link")}
                                    disabled={search_info.page==1}
                                    to={route_to_page.clone()}
                                    query={Some(SearchQuery{page: search_info.page-1, tags: "".to_string()})}
                                >
                                    { "<" }
                                </Link<Route, SearchQuery>>
                            }
                        }
                    }


                </li>
                <li class="page-item">
                    {
                        if search_info.page > total_pages {
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
                                    query={Some(SearchQuery{page: search_info.page+1, tags: search_info.tags})}
                                >
                                    { ">" }
                                </Link<Route, SearchQuery>>
                                </a>
                            }
                        }
                    }


                </li>
            </ul>
        </div>
    }
}
