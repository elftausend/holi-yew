use serde::{Deserialize, Serialize};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::{use_history, History};

use crate::routes::Route;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct SearchQuery {
    pub page: u64,
    pub tags: String,
}

impl Default for SearchQuery {
    fn default() -> Self {
        Self {
            page: 1,
            tags: "".into(),
        }
    }
}

#[derive(Default, Clone)]
pub struct SearchBarInput {
    pub tags: String,
}

#[derive(Debug, Properties, PartialEq, Clone)]
pub struct Props {
    pub search_info: SearchQuery,
}

#[function_component(SearchBar)]
pub fn search_bar(props: &Props) -> Html {
    let tag_input = use_state(SearchBarInput::default);
    let page = props.search_info.page;
    let history = use_history().unwrap();

    let on_search = {
        let history = history.clone();
        let tag_input = tag_input.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            history
                .push_with_query(
                    Route::Entries,
                    SearchQuery {
                        page,
                        tags: tag_input.tags.clone(),
                    },
                )
                .unwrap();
        })
    };

    let on_input_change = {
        let history = history.clone();
        let tag_input = tag_input.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*tag_input).clone();
            info.tags = input.value();

            history
                .push_with_query(
                    Route::Entries,
                    SearchQuery {
                        page,
                        tags: info.tags.clone(),
                    },
                )
                .unwrap();

            tag_input.set(info);
        })
    };

    let onkeypress = {
        Callback::from(move |e: KeyboardEvent| {
            // check for enter key
            if e.key_code() != 13 {
                return;
            }

            history
                .push_with_query(
                    Route::Entries,
                    SearchQuery {
                        page,
                        tags: tag_input.tags.clone(),
                    },
                )
                .unwrap();
        })
    };

    html! {
        <div class="container" style="margin-top: 30px;">
          <div class="d-flex mt-4 mb-4">

         //   <div class="autocomplete">
              <input autocomplete="off" 
                    value={props.search_info.tags.clone()} 
                    onkeypress={onkeypress} 
                    oninput={on_input_change} 
                    id="search_field" 
                    class="form-control input-field" 
                    type="search" 
                    placeholder="Tags oder Titel eingeben" 
                    name="tags" 
                />
        //    </div>
              <button style="width: 80px;" onclick={on_search} id="search_button" class="btn btn-secondary ms-2">{"Suchen"}</button>

          </div>

      </div>
    }
}
