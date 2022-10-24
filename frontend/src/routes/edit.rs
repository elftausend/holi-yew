use reqwest::Method;
use yew::prelude::*;
use yew_hooks::use_mount;
use yew_router::prelude::{use_location, Location};
use crate::{components::{SearchBar, SearchQuery}, request, error::HoliError};
use super::{Route, entries::EntryInfo};

pub async fn get_editable_entries(page: u64, tags: &str) -> Result<Vec<EntryInfo>, HoliError> {
    request(
        Method::GET,
        &format!("editable_entries?page={page}&tags={tags}"),
        (),
        false,
    )
    .await
}

#[function_component(Edit)]
pub fn edit() -> Html {
    let search_info = use_state(SearchQuery::default);
    let location = use_location().unwrap();
    let entries = use_state(Vec::<EntryInfo>::new);

    {
        use_mount(|| {
            
        });
    }

    {
        let location_inner = location.clone();
        let search_info = search_info.clone();
        use_effect_with_deps(
            move |_| {
                let search_query = location_inner.query::<SearchQuery>().unwrap_or_default();
                search_info.set(search_query.clone());

                log::info!("page: {search_query:?}");

                wasm_bindgen_futures::spawn_local(async move {
                    if let Ok(api_entries) =
                        get_editable_entries(search_query.page, &search_query.tags).await
                    {
                        log::info!("editable: {api_entries:?}");
                        entries.set(api_entries);

                        //if let Ok(entry_count) = get_entry_count().await {
                        //    total_pages.set(entry_count.entry_count / *ENTRIES_ON_PAGE);
                        //}
                    }/* else {
                        entries.set(Vec::new());
                        total_pages.set(0);
                    }*/
                });

                || ()
            },
            location.query::<SearchQuery>().unwrap_or_default(),
        );
    }

    html! {
        <div>
            <SearchBar route={Route::Edit} search_info={SearchQuery {
                page: search_info.page,
                tags: search_info.tags.clone()
            }} />

            {
                html! {

                }

            }
        </div>
    }
}