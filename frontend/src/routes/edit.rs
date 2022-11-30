use super::{
    entries::{EntriesWithPages, EntryInfo},
    show_upload::HashQuery,
    Route,
};
use crate::{
    components::{Auth, CardGroup, Pagination, SearchBar, SearchQuery, Tag},
    error::HoliError,
    image_path, request,
    utils::entries_from_fn,
};
use reqwest::Method;
use yew::prelude::*;

use yew_router::prelude::*;

pub async fn get_editable_entries(page: u64, tags: String) -> Result<EntriesWithPages, HoliError> {
    request(
        Method::GET,
        &format!("editable_entries?page={page}&tags={tags}"),
        (),
    )
    .await
}

#[function_component(Edit)]
pub fn edit() -> Html {
    let search_info = use_state(SearchQuery::default);
    let history = use_history().unwrap();
    let total_pages = use_state(|| 0);

    let entries = use_state(|| None);

    entries_from_fn(
        search_info.clone(),
        entries.clone(),
        history,
        total_pages.clone(),
        Route::Edit,
        get_editable_entries,
    );

    let has_view_or_not = |entry: &EntryInfo| {
        if entry.img_exts.is_empty() {
            html! {
                <img style="max-width: 50%; max-width: 10rem;" class="card-img-top " src={image_path(&entry.view)} alt="picture" />
            }
        } else {
            html! {
                <img style="max-width: 50%; max-width: 10rem;" class="card-img-top " src={image_path(&format!("{}_0.{}", entry.hash.clone(), entry.img_exts.first().unwrap_or(&"".into())))} alt="picture" />
            }
        }
    };

    html! {
        <div>
        <Auth>
            <div class="container-fluid mt-3">
                <div class="d-flex mt-4 mb-4">
                    <SearchBar route={Route::Edit} search_info={SearchQuery {
                        page: search_info.page,
                        tags: search_info.tags.clone(),
                        scroll_to_bar: false
                    }} />
                </div>
                <div >
                    <h4>{"Deine Uploads"}</h4>
                </div>
            </div>


            {
                match (*entries).clone() {
                    Some(entries) => {
                        entries.chunks(4).map(|chunk| {
                            html! {
                                <CardGroup>
                                {
                                    chunk.iter().map(|entry| {

                                        html! {
                                            <div class="card">
                                                {has_view_or_not(entry)}
                                                <div class="card-body">
                                                    <h5 class="card-title">
                                                        {entry.title.clone()}
                                                    </h5>
                                                    <p class="card-text">
                                                        {
                                                            entry.tags.iter().map(|tag| {
                                                                html! {
                                                                    <Tag name={tag.clone()} route={Route::Edit} />
                                                                    //<span class="badge me-1 bg-secondary tag">{tag}</span>
                                                                }
                                                            }).collect::<Html>()
                                                        }
                                                    </p>
                                                    <p>
                                                        <Link<Route, HashQuery>
                                                            to={Route::EditUpload}
                                                            query={Some(HashQuery{uid: entry.uid})}
                                                        >
                                                            <button class="btn btn-primary">
                                                                {"editieren"}
                                                            </button>

                                                        </Link<Route, HashQuery>>
                                                    </p>
                                                </div>
                                            </div>
                                        }

                                    }).collect::<Html>()
                                }
                                </CardGroup>
                        }}).collect::<Html>()
                    }
                    None => html! {
                        {"Einträge werden geladen..."}
                    }
                }

            }
            <Pagination
                search_info={SearchQuery {
                    page: search_info.page,
                    tags: search_info.tags.clone(),
                    scroll_to_bar: true
                }}
                total_pages={*total_pages}
                route_to_page={Route::Edit}
            />

            </Auth>
        </div>
    }
}
