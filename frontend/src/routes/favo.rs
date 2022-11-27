use reqwest::Method;
use yew::prelude::*;
use yew_router::prelude::*;

use super::entries::EntriesWithPages;
use crate::{
    components::{Auth, Pagination, SearchBar, SearchQuery, EntryList},
    error::HoliError,
    request,
    routes::Route,
    utils::entries_from_fn,
};

pub async fn get_favos_with_total(page: u64, tags: String) -> Result<EntriesWithPages, HoliError> {
    request(Method::GET, &format!("favos?page={page}&tags={tags}"), ()).await
}

#[function_component(Favo)]
pub fn favo() -> Html {
    let search_info = use_state(SearchQuery::default);
    let history = use_history().unwrap();
    let total_pages = use_state(|| 0);

    let entries = use_state(|| None);

    entries_from_fn(
        search_info.clone(),
        entries.clone(),
        history.clone(),
        total_pages.clone(),
        Route::Favo,
        get_favos_with_total,
    );

    html! {
        <>
        <Auth>
            <div class="container" style="margin-top: 30px;">
                <div class="d-flex mt-4 mb-4">
                    <SearchBar route={Route::Entries} search_info={SearchQuery {
                        page: search_info.page,
                        tags: search_info.tags.clone(),
                        scroll_to_bar: true
                    }} />
                </div>
            </div>

            <div >
                <h4>{"Deine favorisierten Beiträge:"}</h4>
            </div>

            <EntryList entries={(*entries).clone()} />

            <Pagination
                search_info={SearchQuery {
                    page: search_info.page,
                    tags: search_info.tags.clone(),
                    scroll_to_bar: true
                }}
                total_pages={*total_pages}
                route_to_page={Route::Favo}
            />
        </Auth>
        </>
    }
}
