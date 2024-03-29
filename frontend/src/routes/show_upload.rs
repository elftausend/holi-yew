use reqwest::Method;
use serde::{Deserialize, Serialize};
use yew::prelude::*;
use yew_router::prelude::{use_history, use_location, History, Location, Link};

use crate::{
    components::{Auth, Tag},
    error::HoliError,
    hooks::{use_user_context, UseUserContextHandle},
    image_path, pdf_path, request,
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
    request(Method::POST, &format!("favo?uid={uid}"), ()).await
}

pub async fn unfavo_request(uid: u32) -> Result<(), HoliError> {
    request(Method::POST, &format!("unfavo?uid={uid}"), ()).await
}

fn favo(uid: u32, user_ctx: UseUserContextHandle) -> Callback<MouseEvent> {
    Callback::from(move |_e: MouseEvent| {
        let user_ctx = user_ctx.clone();
        wasm_bindgen_futures::spawn_local(async move {
            favo_request(uid).await.unwrap();
            let mut user_info = (*(user_ctx.inner)).clone();
            user_info.favs.push(uid);
            user_ctx.inner.set(user_info);

            //if let Ok(_) = favo_request(uid).await {
            //    let mut user_info = (*(user_ctx.inner)).clone();
            //    user_info.favs.push(uid);
            //    user_ctx.inner.set(user_info);
            //    //fav.set(true)
            //}
        });
    })
}

fn unfavo(uid: u32, user_ctx: UseUserContextHandle) -> Callback<MouseEvent> {
    Callback::from(move |_e: MouseEvent| {
        let user_ctx = user_ctx.clone();
        wasm_bindgen_futures::spawn_local(async move {
            unfavo_request(uid).await.unwrap();
            let mut user_info = (*(user_ctx.inner)).clone();

            user_info.favs.sort();
            let idx = user_info.favs.binary_search(&uid).unwrap();
            user_info.favs.remove(idx);
            user_ctx.inner.set(user_info);
        });
    })
}

fn favo_button(entry_info: &EntryInfo, user_ctx: UseUserContextHandle) -> Html {
    if user_ctx.inner.favs.contains(&entry_info.uid) {
        html! {
            <button onclick={unfavo(entry_info.uid, user_ctx.clone())} class="btn btn-secondary mt-1 me-2">
                <svg style="fill: rgb(227, 179, 65);" aria-hidden="true" height="16" viewBox="0 0 16 16" version="1.1" width="16" data-view-component="true" class="me-1">
                    <path fill-rule="evenodd" d="M8 .25a.75.75 0 01.673.418l1.882 3.815 4.21.612a.75.75 0 01.416 1.279l-3.046 2.97.719 4.192a.75.75 0 01-1.088.791L8 12.347l-3.766 1.98a.75.75 0 01-1.088-.79l.72-4.194L.818 6.374a.75.75 0 01.416-1.28l4.21-.611L7.327.668A.75.75 0 018 .25z"></path>
                </svg>
                {"Gemerkt"}
            </button>
        }
    } else {
        html! {
            <button onclick={favo(entry_info.uid, user_ctx.clone())} class="btn btn-secondary mt-1 me-2">
                <svg aria-hidden="true" height="16" viewBox="0 0 16 16" version="1.1" width="16" data-view-component="true" class="me-1">
                    <path fill-rule="evenodd" d="M8 .25a.75.75 0 01.673.418l1.882 3.815 4.21.612a.75.75 0 01.416 1.279l-3.046 2.97.719 4.192a.75.75 0 01-1.088.791L8 12.347l-3.766 1.98a.75.75 0 01-1.088-.79l.72-4.194L.818 6.374a.75.75 0 01.416-1.28l4.21-.611L7.327.668A.75.75 0 018 .25z"></path>
                </svg>
                {"Merken"}
            </button>
        }
    }
}

#[function_component(ShowUpload)]
pub fn show_upload() -> Html {
    let user_ctx = use_user_context();
    let entry_info = use_state(|| None);
    let history = use_history().unwrap();
    let fav = use_state(|| false);

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
        let user_ctx1 = user_ctx.clone();
        let user_ctx = user_ctx.clone();
        let fav = fav.clone();
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    let hash = location.query::<HashQuery>().unwrap_or_default();
                    if let Ok(entry) = get_entry(hash.uid as i32).await {
                        fav.set(user_ctx.inner.favs.contains(&entry.uid));
                        entry_info.set(Some(entry))
                    } else {
                        entry_info.set(None);
                        history.back();
                    }
                });
                || {}
            },
            user_ctx1.inner.favs.clone(),
        );
    }

    let edit_button = |uid: u32| if user_ctx.inner.is_editable(uid) {
        html! {
            <Link<Route, HashQuery>
                to={Route::EditUpload}
                query={Some(HashQuery{uid})}
            >
                <button class="btn btn-primary me-2 mt-1">
                    {"editieren"}
                </button>
            </Link<Route, HashQuery>>
        }
    } else {
        html!()
    };

    match (*entry_info).clone() {
        Some(entry_info) => {
            let favo_button = favo_button(&entry_info, user_ctx.clone());

            html! {
                <div>
                    <Auth>
                        <div>
                            <div class="container-fluid mt-1">

                                <div style="font-weight: bold; font-size: x-large;">
                                    <div style="float: left;" class="mb-3">
                                        <button onclick={onback} class="btn btn-primary mt-1">
                                            {"Zurück"}
                                        </button>
                                        <span class="ms-2 me-2 mt-3">{entry_info.title.clone()}</span>
                                        {favo_button}
                                        {edit_button(entry_info.uid)}
                                        <br/>
                                        <div class="mt-1">
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
                                        <p class="mt-1">
                                            // download does not work because the link to the download is not the same origin
                                            <a class="me-2" href={pdf_path(&format!("{}.{}", &entry_info.hash, &entry_info.ext))} download={format!("{}.{}", &entry_info.title, &entry_info.ext)}>
                                                <button class="btn btn-primary">{"download"}</button>
                                            </a>

                                            <a href={pdf_path(&format!("{}.{}", &entry_info.hash, &entry_info.ext))}>
                                                <button class="btn btn-danger me-2">
                                                {"PDF anzeigen"}
                                                </button>
                                            </a>
                                        </p>
                                        </div>

                                        <br />

                                        <p style="margin-top: 130px; font-size:x-large;">

                                            {
                                                if !entry_info.img_exts.is_empty() {
                                                    html! {
                                                        <>
                                                            <h4>{"Extrahierte Bilder"}</h4>
                                                        {
                                                        (0..entry_info.img_exts.len()).into_iter().map(|idx| {
                                                            html!{
                                                                <>
                                                                    <img
                                                                        class="mb-3"
                                                                        style="width: 55%;"
                                                                        src={image_path(&format!("{}_{idx}.{}", &entry_info.hash, &entry_info.img_exts[idx]))}
                                                                        alt="Some holi image"
                                                                    />
                                                                </>
                                                            }
                                                        }).collect::<Html>()
            }
                                                    </> }
                                                } else {
                                                    html! {
                                                        <div style="margin-top: 120px" class="container-fluid">
                                                            {"PDF Preview"}<br /><br />
                                                            <iframe class="pdf-preview" src={pdf_path(&format!("{}.{}", &entry_info.hash, &entry_info.ext))} />
                                                        </div>
                                                    }
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
        None => {
            html! {
                {"Beitrag wird geladen..."}
            }
        }
    }
}
