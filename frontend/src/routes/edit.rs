use yew::prelude::*;
use yew_hooks::use_mount;
use yew_router::prelude::{use_location, Location};
use crate::components::{SearchBar, SearchQuery};
use super::{Route, entries::EntryInfo};

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