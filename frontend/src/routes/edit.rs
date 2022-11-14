use super::{entries::EntryInfo, show_upload::HashQuery, Route};
use crate::{
    components::{Auth, CardGroup, SearchBar, SearchQuery, Tag},
    error::HoliError,
    image_path, request,
};
use reqwest::Method;
use yew::prelude::*;
use yew_hooks::use_mount;
use yew_router::prelude::*;

pub async fn get_editable_entries(page: u64, tags: &str) -> Result<Vec<EntryInfo>, HoliError> {
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
    let location = use_location().unwrap();
    let entries = use_state(Vec::<EntryInfo>::new);

    {
        use_mount(|| {});
    }

    {
        let location_inner = location.clone();
        let search_info = search_info.clone();
        let entries = entries.clone();
        use_effect_with_deps(
            move |_| {
                let search_query = location_inner.query::<SearchQuery>().unwrap_or_default();
                search_info.set(search_query.clone());

                log::info!("page: {search_query:?}");

                wasm_bindgen_futures::spawn_local(async move {
                    if let Ok(mut api_entries) =
                        get_editable_entries(search_query.page, &search_query.tags).await
                    {
                        api_entries.sort_by(|a, b| b.uid.cmp(&a.uid));
                        log::info!("editable: {api_entries:?}");
                        entries.set(api_entries);

                        //if let Ok(entry_count) = get_entry_count().await {
                        //    total_pages.set(entry_count.entry_count / *ENTRIES_ON_PAGE);
                        //}
                    } /* else {
                          entries.set(Vec::new());
                          total_pages.set(0);
                      }*/
                });

                || ()
            },
            location.query::<SearchQuery>().unwrap_or_default(),
        );
    }

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

        </Auth>
        </div>
    }
}
