use reqwest::Method;
use serde::{Deserialize, Serialize};
use web_sys::console;
use yew::prelude::*;
use yew_router::prelude::*;

<<<<<<< HEAD
use crate::components::{CardGroup, SearchBar, PageQuery, Pagination, Footer, SearchQuery};
use crate::{image_path, ENTRIES_ON_PAGE};
=======
use crate::components::{NavBar, CardGroup, Auth};
use crate::image_path;
>>>>>>> f9c2480 (Add auth comp, searchbar,)
use crate::{api::request, error::HoliError, hooks::use_user_context};

use super::Route;
use super::show_upload::HashQuery;

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct EntryInfo {
    pub title: String,
    pub date: String,
    pub tags: Vec<String>,
    pub path: Vec<String>,
    pub pdf: String,
    // mind rename from type to upload_type
    pub r#type: String,
    pub file_type: String,
	// mind 'anonymous' upload etc
    pub uploader: String,
    pub hash: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct EntryCount {
	entry_count: u64,
}

pub async fn get_entry_count() -> Result<EntryCount, HoliError> {
    request(Method::GET, "entry_count", (), false).await
}

pub async fn get_entry(hash: &str) -> Result<EntryInfo, HoliError> {
    request(Method::GET, &format!("entry/{hash}"), (), false).await
}

pub async fn get_entries(page: u64, tags: &str) -> Result<Vec<EntryInfo>, HoliError> {
    request(Method::GET, &format!("entries?page={page}&tags={tags}"), (), false).await
}

#[function_component(Entries)]
pub fn entries() -> Html {
<<<<<<< HEAD
	//let page = use_state(|| 1);
	let search_info = use_state(SearchQuery::default);
	
	let total_pages = use_state(|| 1);

	let user_ctx = use_user_context();
    let location = use_location().unwrap();
=======
	let user_ctx = use_user_context();
    let history = use_history().unwrap();
>>>>>>> f9c2480 (Add auth comp, searchbar,)

    let entries = use_state(Vec::<EntryInfo>::new);

	{
		let entries = entries.clone();
<<<<<<< HEAD
		let search_info1 = search_info.clone();		
		
		let location_inner = location.clone();
		let total_pages = total_pages.clone();
		use_effect_with_deps(move |_| {
			let search_query = location_inner.query::<SearchQuery>().unwrap_or_default();
			search_info1.set(search_query.clone());		

			
			log::info!("page: {search_query:?}");

			wasm_bindgen_futures::spawn_local(async move {
				if let Ok(api_entries) = get_entries(search_query.page -1, &search_query.tags).await {
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
		}, location.query::<SearchQuery>().unwrap_or_default());

		
		
=======
		use_effect_with_deps(
			move |_| {
				wasm_bindgen_futures::spawn_local(async move {
					if let Ok(api_entries) = get_entries().await {
						//log::info!("{api_entries:?}");
						entries.set(api_entries);
					}
				});
				|| ()
			},
			(),
		);
>>>>>>> f9c2480 (Add auth comp, searchbar,)
	}
	let card = move |title: String| -> Html {
		html! {

		}
	};
	
    html! {
        <div>
<<<<<<< HEAD
=======
			<Auth>
>>>>>>> f9c2480 (Add auth comp, searchbar,)
            <div class="container-fluid">
                <div class="row highlight">
                    <a href="/et" class="col et_bg_color card square">
                        <div class="">
                            <h1 class="text-center push-down text-white" style="margin-top: 56px;">{"ET"}</h1>
                        </div>
                    </a>

                    <a href="/?page=0&tags=IT#search_field" class="col it_bg_color card square">
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
							chunk.iter().map(|name| {
								html! {
									<div class="card">
										<Link<Route, HashQuery>
	                                    	to={Route::ShowUpload}
                                    		query={Some(HashQuery{hash: name.hash.clone()})}
                                		>
											<img style="max-width: 50%; max-width: 10rem;" class="card-img-top " src={image_path(&name.path[0])} alt="picture" />
											<div class="card-body">
												<h5 class="card-title">
													{name.title.clone()}
												</h5>
												<p class="card-text">
													{
														name.tags.iter().map(|tag| {
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
							}).collect::<Html>()
						}
						</CardGroup>
						}
						).collect::<Html>()
					
				}
					
				}
			}
<<<<<<< HEAD

			<Pagination
				search_info={SearchQuery {
					page: search_info.page,
					tags: search_info.tags.clone()
				}}
				total_pages={*total_pages}
				route_to_page={Route::Entries}
			/>

			<Footer />
=======
			</Auth>
>>>>>>> f9c2480 (Add auth comp, searchbar,)
        </div>
			
    }
}
