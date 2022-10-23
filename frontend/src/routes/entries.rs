use reqwest::Method;
use serde::{Deserialize, Serialize};
use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::{CardGroup, Footer, Pagination, SearchBar, SearchQuery, Auth};
use crate::{api::request, error::HoliError, hooks::use_user_context};
use crate::{image_path, pdf_path, ENTRIES_ON_PAGE};

use super::show_upload::HashQuery;
use super::Route;

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct EntryInfo {
    pub uid: i32,
    pub title: String,
    pub date: String,
    pub tags: Vec<String>,
    pub view: String,
    pub img_exts: Vec<String>,
    // mind 'anonymous' upload etc
    pub usid: String,
    pub ut: String,
    pub ext: String,
    pub hash: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct EntryCount {
    entry_count: u64,
}

pub async fn get_entry_count() -> Result<EntryCount, HoliError> {
    request(Method::GET, "entry_count", (), false).await
}

pub async fn get_entry(uid: i32) -> Result<EntryInfo, HoliError> {
    request(Method::GET, &format!("entry/{uid}"), (), false).await
}

pub async fn get_entries(page: u64, tags: &str) -> Result<Vec<EntryInfo>, HoliError> {
    request(
        Method::GET,
        &format!("entries?page={page}&tags={tags}"),
        (),
        false,
    )
    .await
}

#[function_component(Entries)]
pub fn entries() -> Html {
    //let page = use_state(|| 1);
    let search_info = use_state(SearchQuery::default);

    let total_pages = use_state(|| 0);

    let user_ctx = use_user_context();
    let location = use_location().unwrap();

    let entries = use_state(Vec::<EntryInfo>::new);

    {
        let entries = entries.clone();
        let search_info1 = search_info.clone();

        let location_inner = location.clone();
        let total_pages = total_pages.clone();
        use_effect_with_deps(
            move |_| {
                let search_query = location_inner.query::<SearchQuery>().unwrap_or_default();
                search_info1.set(search_query.clone());

                log::info!("page: {search_query:?}");

                wasm_bindgen_futures::spawn_local(async move {
                    if let Ok(api_entries) =
                        get_entries(search_query.page, &search_query.tags).await
                    {
                        entries.set(api_entries);

                        if let Ok(entry_count) = get_entry_count().await {
                            total_pages.set(entry_count.entry_count / *ENTRIES_ON_PAGE);
                        }
                    } else {
                        entries.set(Vec::new());
                        total_pages.set(0);
                    }
                });

                || ()
            },
            location.query::<SearchQuery>().unwrap_or_default(),
        );
    }
    let card = move |title: String| -> Html {
        html! {}
    };

    html! {
        <div>
            <Auth>
                <div class="container-fluid">
                    <div class="row highlight">
                        <a href="/et" class="col et_bg_color card square">
                            <div class="">
                                <h1 class="text-center push-down text-white" style="margin-top: 56px;">{"ET"}</h1>
                            </div>
                        </a>

                        <a href="/?page=1&tags=IT#search_field" class="col it_bg_color card square">
                        <div class="">
                            <h1 class="text-center push-down text-white" style="margin-top: 56px;">{"IT"}</h1>
                        </div>
                    </a>

                    <a href="/el" class="col el_bg_color card square">
                        <div class="">
                        <h1 class="text-center push-down text-white" style="margin-top: 56px;">{"EL"}</h1>
                        </div>
                    </a>

                    <a href="/me" class="col me_bg_color card square">
                        <div class="">
                            <h1 class="text-center text-white" style="margin-top: 56px;">{"ME"}</h1>
                        </div>
                    </a>

                    <a href="/mb" class="col mb_bg_color card square">
                        <div class="">
                        <h1 class="text-center push-down text-white" style="margin-top: 56px;">{"MB"}</h1>
                        </div>

                    </a>

                    <a href="/wi_log" class="col wi_bg_color card square">
                        <div class="">
                        <h1 class="text-center push-down text-white" style="margin-top: 56px;">{"WIL"}</h1>
                        </div>

                    </a>

                    <a href="/wi_inf" class="col wi_bg_color square card">
                        <div class="">
                        <h1 class="text-center push-down text-white" style="margin-top: 56px;">{"WII"}</h1>
                        </div>

                    </a>
                    </div>
                </div>

                <SearchBar search_info={SearchQuery {
                    page: search_info.page,
                    tags: search_info.tags.clone()
                }} />

                {
                    html!{
                        {
                        entries.chunks(4).map(|chunk|
                            html! {
                                <CardGroup>
                                {
                                chunk.iter().map(|entry| {
                                    if entry.img_exts.len() > 0 {
                                        html! {
                                            <div class="card">
                                                <Link<Route, HashQuery>
                                                    to={Route::ShowUpload}
                                                    query={Some(HashQuery{uid: entry.uid})}
                                                >
                                                    <img style="max-width: 50%; max-width: 10rem;" class="card-img-top " src={image_path(&format!("{}_0.{}", entry.hash.clone(), entry.img_exts.first().unwrap_or(&"".into())))} alt="picture" />
                                                    <div class="card-body">
                                                        <h5 class="card-title">
                                                            {entry.title.clone()}
                                                        </h5>
                                                        <p class="card-text">
                                                            {
                                                                entry.tags.iter().map(|tag| {
                                                                    html! {
                                                                        <span class="badge me-1 bg-secondary tag">{tag}</span>
                                                                    }
                                                                }).collect::<Html>()
                                                            }
                                                        </p>
                                                    </div>
                                                </Link<Route, HashQuery>>
                                            </div>
                                        }
                                    } else {
                                        html! {
                                            <div class="card">
                                                <a href={pdf_path(&format!("{}.{}", &entry.hash, &entry.ext))} download={"true"}>
                                                    <img style="max-width: 50%; max-width: 10rem;" class="card-img-top " src={image_path(&entry.view)} alt="picture" />
                                                    <div class="card-body">
                                                        <h5 class="card-title">
                                                            {entry.title.clone()}
                                                        </h5>
                                                        <p class="card-text">
                                                            {
                                                                entry.tags.iter().map(|tag| {
                                                                    html! {
                                                                        <span class="badge me-1 bg-secondary tag">{tag}</span>
                                                                    }
                                                                }).collect::<Html>()
                                                            }
                                                        </p>
                                                    </div>
                                                </a>
                                            </div>
                                        }
                                    }

                                }).collect::<Html>()
                            }
                            </CardGroup>
                            }
                            ).collect::<Html>()

                    }

                    }
                }

                <Pagination
                    search_info={SearchQuery {
                        page: search_info.page,
                        tags: search_info.tags.clone()
                    }}
                    total_pages={*total_pages}
                    route_to_page={Route::Entries}
                />

                <Footer />
            </Auth>
        </div>

    }
}
