use reqwest::Method;
use serde::{Deserialize, Serialize};
use yew::prelude::*;
use yew_hooks::use_mount;
use yew_router::prelude::{use_history, use_location, History, Location};

use crate::{
    components::{Auth, Tag},
    image_path, pdf_path, hooks::use_user_context, request, error::HoliError,
};

use super::{
    entries::{get_entry, EntryInfo},
    Route,
};

#[derive(Debug, Default, PartialEq, Eq, Deserialize, Clone, Serialize)]
pub struct HashQuery {
    pub uid: u32,
}

#[derive(Debug, Default, PartialEq, Eq, Deserialize, Clone, Serialize)]
pub struct FavoInfo {
    pub uid: u32,
}

pub async fn favo_request(uid: u32) -> Result<(), HoliError> {
    request::<FavoInfo, ()>(Method::POST, "favo", FavoInfo { uid }).await
}

pub async fn unfavo_request(uid: u32) -> Result<(), HoliError> {
    request::<FavoInfo, ()>(Method::POST, "unfavo", FavoInfo { uid }).await
}

fn favo(uid: u32) -> Callback<MouseEvent> {
    Callback::from(move |_e: MouseEvent| {
        wasm_bindgen_futures::spawn_local(async move {
            if let Ok(_) = favo_request(uid).await {

            }
        });
    })
}

fn unfavo(uid: u32) -> Callback<MouseEvent> {
    Callback::from(move |_e: MouseEvent| {
        wasm_bindgen_futures::spawn_local(async move {
            if let Ok(_) = unfavo_request(uid).await {
                
            }
        });
    })
}

#[function_component(ShowUpload)]
pub fn show_upload() -> Html {
    let user_ctx = use_user_context();
    let entry_info = use_state(EntryInfo::default);
    let history = use_history().unwrap();

    let onback = {
        let history = history.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            history.back();
        })
    };

    let location = use_location().unwrap();
    {
        let entry_info = entry_info.clone();
        use_mount(move || {
            wasm_bindgen_futures::spawn_local(async move {
                let hash = location.query::<HashQuery>().unwrap_or_default();
                if let Ok(entry) = get_entry(hash.uid as i32).await {
                    log::info!("ENTRY {entry:?}");
                    entry_info.set(entry)
                } else {
                    entry_info.set(EntryInfo::default());
                    history.back();
                }
            });
        });
    }
    let favo_button = if user_ctx.inner.favs.contains(&entry_info.uid) {
        html! {
            <button onclick={unfavo(entry_info.uid)} class="btn btn-secondary"> 
                <svg style="fill: rgb(227, 179, 65);" aria-hidden="true" height="16" viewBox="0 0 16 16" version="1.1" width="16" data-view-component="true" class="me-1">
                    <path fill-rule="evenodd" d="M8 .25a.75.75 0 01.673.418l1.882 3.815 4.21.612a.75.75 0 01.416 1.279l-3.046 2.97.719 4.192a.75.75 0 01-1.088.791L8 12.347l-3.766 1.98a.75.75 0 01-1.088-.79l.72-4.194L.818 6.374a.75.75 0 01.416-1.28l4.21-.611L7.327.668A.75.75 0 018 .25z"></path>
                </svg>
                {"Gemerkt"}
            </button>
        }
    } else {
        html! {
            <button onclick={favo(entry_info.uid)} class="btn btn-secondary"> 
            <svg aria-hidden="true" height="16" viewBox="0 0 16 16" version="1.1" width="16" data-view-component="true" class="me-1">
                <path fill-rule="evenodd" d="M8 .25a.75.75 0 01.673.418l1.882 3.815 4.21.612a.75.75 0 01.416 1.279l-3.046 2.97.719 4.192a.75.75 0 01-1.088.791L8 12.347l-3.766 1.98a.75.75 0 01-1.088-.79l.72-4.194L.818 6.374a.75.75 0 01.416-1.28l4.21-.611L7.327.668A.75.75 0 018 .25z"></path>
            </svg>
            {"Merken"}
        </button>
        }
    };
    
    html! {
        <div>
            <Auth>
                <div>
                    <div class="container-fluid mt-3">

                        <div style="font-weight: bold; font-size: x-large;" class="mt-3">
                            <div style="float: left;" class="mb-3">
                                <button onclick={onback} class="btn btn-primary">
                                    {"Zurück"}
                                </button>
                                <span class="ms-2 me-2">{entry_info.title.clone()}</span>
                                // download does not work because the link to the download is not the same origin
                                <a class="me-2" href={pdf_path(&format!("{}.{}", &entry_info.hash, &entry_info.ext))} download={format!("{}.{}", &entry_info.title, &entry_info.ext)}>
                                    <button class="btn btn-primary">{"download"}</button>
                                </a>
                                <a href={pdf_path(&format!("{}.{}", &entry_info.hash, &entry_info.ext))}>
                                    <button class="btn btn-danger me-2">
                                    {"PDF anzeigen"}
                                    </button>
                                </a>
                                {favo_button}
                                <br/>
                                {
                                    entry_info.tags.iter().map(|tag| {
                                        html! {
                                            <Tag name={tag.clone()} route={Route::Entries} />
                                            //<span class="badge me-1 bg-secondary tag">{tag}</span>
                                            //<a href="it?page=0&tags={{ tag }}" style="font-size: 14px;" class="badge tag">{{ tag }}</a>
                                        }
                                    }).collect::<Html>()
                                }
                                </div>
                                
                                <br />
                                <br />

                                <p class="mt-5">
                                    <h4 style="float: left;">{"Extrahierte Bilder"}</h4><br />
                                    {
                                        if !entry_info.img_exts.is_empty() {
                                            (0..entry_info.img_exts.len()).into_iter().map(|idx| {
                                                html!{
                                                    <>
                                                        <img
                                                            class="mb-3"
                                                            style="width: 80%;"
                                                            src={image_path(&format!("{}_{idx}.{}", &entry_info.hash, &entry_info.img_exts[idx]))}
                                                            alt="Some holi image"
                                                        />
                                                    </>
                                                }
                                            }).collect::<Html>()
                                        } else {
                                            html!()
                                        }

                                    }
                                </p>
                            
                        </div>
                    </div>
                </div>
                <br />
            </Auth>
        </div>
    }
}
