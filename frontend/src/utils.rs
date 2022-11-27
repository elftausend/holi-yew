use std::future::Future;

use web_sys::window;
use yew::{use_effect_with_deps, UseStateHandle};
use yew_router::prelude::{AnyHistory, History, Location};

use crate::{
    components::SearchQuery,
    error::HoliError,
    routes::{
        entries::{EntriesWithPages, EntryInfo},
        Route,
    },
};

pub fn is_mobile() -> bool {
    let width = window().unwrap().inner_width().unwrap().as_f64().unwrap();
    let height = window().unwrap().inner_height().unwrap().as_f64().unwrap();

    width <= 600. && height <= 960.
}

pub fn entries_from_fn<'a, T, F>(
    search_info: UseStateHandle<SearchQuery>,
    entries: UseStateHandle<Option<Vec<EntryInfo>>>,
    history: AnyHistory,
    total_pages: UseStateHandle<u64>,
    route: Route,
    f: F,
) where
    T: Future<Output = Result<EntriesWithPages, HoliError>>,
    F: FnOnce(u64, String) -> T + 'static,
{
    let location = history.location().clone();
    let location_inner = history.location().clone();

    use_effect_with_deps(
        move |_| {
            let search_query = location_inner.query::<SearchQuery>().unwrap_or_default();

            // scroll to search bar
            if search_query.scroll_to_bar {
                let doc = window().unwrap().document().unwrap();

                if let Some(search) = doc.get_element_by_id("search_field") {
                    search.scroll_into_view();
                }
            }

            search_info.set(search_query.clone());

            log::info!("page: {search_query:?}");

            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(mut api_entries) = f(search_query.page, search_query.tags).await {
                    api_entries.entries.sort_by(|a, b| b.uid.cmp(&a.uid));
                    total_pages.set(api_entries.page_count);

                    entries.set(Some(api_entries.entries));
                } else {
                    // else: probably an invalid page

                    entries.set(Some(Vec::new()));
                    total_pages.set(0);
                    history
                        .push_with_query(route, SearchQuery::default())
                        .unwrap();
                }
            });
            || ()
        },
        location.query::<SearchQuery>().unwrap_or_default(),
    );
}
