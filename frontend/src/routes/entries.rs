use reqwest::Method;
use serde::{Deserialize, Serialize};
use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::{Auth, EntryList, Pagination, SearchBar, SearchQuery};
use crate::utils::entries_from_fn;
use crate::{api::request, error::HoliError};

use super::Route;

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EntryInfo {
    pub uid: u32,
    pub title: String,
    pub date: String,
    pub tags: Vec<String>,
    pub view: String,
    pub img_exts: Vec<String>,
    // mind 'anonymous' upload etc
    pub usid: String,
    pub ut: String,
    pub ext: String,
    pub favs: usize,
    pub hash: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EntriesWithPages {
    pub entries: Vec<EntryInfo>,
    pub page_count: u64,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct EntryCount {
    entry_count: u64,
}

//pub async fn get_entry_count() -> Result<EntryCount, HoliError> {
//    request(Method::GET, "entry_count", ()).await
//}

pub async fn get_entry(uid: i32) -> Result<EntryInfo, HoliError> {
    request(Method::GET, &format!("entry/{uid}"), ()).await
}

pub async fn get_entries_with_total(
    page: u64,
    tags: String,
) -> Result<EntriesWithPages, HoliError> {
    request(Method::GET, &format!("entries?page={page}&tags={tags}"), ()).await
}

pub enum Sort {}

#[function_component(Entries)]
pub fn entries() -> Html {
    //let page = use_state(|| 1);

    let search_info = use_state(SearchQuery::default);
    let history = use_history().unwrap();
    let total_pages = use_state(|| 0);

    let entries = use_state(|| None);

    entries_from_fn(
        search_info.clone(),
        entries.clone(),
        history,
        total_pages.clone(),
        Route::Entries,
        get_entries_with_total,
    );

    html! {
        <div>
            <Auth>
                <div class="container-fluid">
                    <div class="row highlight">
                        <Link<Route, SearchQuery>
                            classes={classes!("col", "et_bg_color", "card", "square")}
                            to={Route::Entries}
                            query={Some(SearchQuery {
                                page: 0,
                                tags: "ET".into(),
                                scroll_to_bar: true
                            })}
                        >
                            <div class="">
                                <h1 class="text-center push-down text-white" style="margin-top: 56px;">{"ET"}</h1>
                            </div>
                        </Link<Route, SearchQuery>>


                        //<a href="/?page=1&tags=IT#search_field" class="col it_bg_color card square">
                        <Link<Route, SearchQuery>
                            classes={classes!("col", "it_bg_color", "card", "square")}
                            to={Route::Entries}
                            query={Some(SearchQuery {
                                page: 0,
                                tags: "IT".into(),
                                scroll_to_bar: true
                            })}
                        >
                            <div class="">
                                <h1 class="text-center push-down text-white" style="margin-top: 56px;">{"IT"}</h1>
                            </div>
                        </Link<Route, SearchQuery>>

                        <Link<Route, SearchQuery>
                            classes={classes!("col", "el_bg_color", "card", "square")}
                            to={Route::Entries}
                            query={Some(SearchQuery {
                                page: 0,
                                tags: "EL".into(),
                                scroll_to_bar: true
                            })}
                        >
                            <div class="">
                                <h1 class="text-center push-down text-white" style="margin-top: 56px;">{"EL"}</h1>
                            </div>

                        </Link<Route, SearchQuery>>

                        <Link<Route, SearchQuery>
                            classes={classes!("col", "me_bg_color", "card", "square")}
                            to={Route::Entries}
                            query={Some(SearchQuery {
                                page: 0,
                                tags: "ME".into(),
                                scroll_to_bar: true
                            })}
                        >
                            <div class="">
                                <h1 class="text-center text-white" style="margin-top: 56px;">{"ME"}</h1>
                            </div>
                        </Link<Route, SearchQuery>>

                        <Link<Route, SearchQuery>
                            classes={classes!("col", "mb_bg_color", "card", "square")}
                            to={Route::Entries}
                            query={Some(SearchQuery {
                                page: 0,
                                tags: "MB".into(),
                                scroll_to_bar: true
                            })}
                        >
                            <div class="">
                                <h1 class="text-center push-down text-white" style="margin-top: 56px;">{"MB"}</h1>
                            </div>
                        </Link<Route, SearchQuery>>

                        <Link<Route, SearchQuery>
                            classes={classes!("col", "wi_bg_color", "card", "square")}
                            to={Route::Entries}
                            query={Some(SearchQuery {
                                page: 0,
                                tags: "WIL".into(),
                                scroll_to_bar: true
                            })}
                        >

                            <div class="">
                                <h1 class="text-center push-down text-white" style="margin-top: 56px;">{"WIL"}</h1>
                            </div>
                        </Link<Route, SearchQuery>>

                        <Link<Route, SearchQuery>
                            classes={classes!("col", "wi_bg_color", "card", "square")}
                            to={Route::Entries}
                            query={Some(SearchQuery {
                                page: 0,
                                tags: "WII".into(),
                                scroll_to_bar: true
                            })}
                        >

                        <div class="">
                        <h1 class="text-center push-down text-white" style="margin-top: 56px;">{"WII"}</h1>
                        </div>

                        </Link<Route, SearchQuery>>
                    </div>
                </div>

                <div class="container" style="margin-top: 30px;">
                    <div class="d-flex mt-4 mb-4">
                        <SearchBar route={Route::Entries} search_info={SearchQuery {
                            page: search_info.page,
                            tags: search_info.tags.clone(),
                            scroll_to_bar: true
                        }} />
                    </div>
                </div>

                <EntryList entries={(*entries).clone()} />

                <Pagination
                    search_info={SearchQuery {
                        page: search_info.page,
                        tags: search_info.tags.clone(),
                        scroll_to_bar: true
                    }}
                    total_pages={*total_pages}
                    route_to_page={Route::Entries}
                />
            </Auth>
        </div>

    }
}
