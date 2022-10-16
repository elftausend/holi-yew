use serde::{Deserialize, Serialize};
use yew::prelude::*;
use yew_hooks::use_mount;
use yew_router::prelude::{use_history, use_location, History, Location};

use crate::{image_path, pdf_path};

use super::entries::{get_entry, EntryInfo};

#[derive(Debug, Default, PartialEq, Deserialize, Clone, Serialize)]
pub struct HashQuery {
    pub hash: String,
}

#[function_component(ShowUpload)]
pub fn show_upload() -> Html {
    let entry_info = use_state(EntryInfo::default);
    let hash_query = use_state(HashQuery::default);
    let history = use_history().unwrap();

    let onback = {
        let history = history.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            history.back();
        })
    };

    let location = use_location().unwrap();
    /*{
        let location = location.clone();
        let entry_info = entry_info.clone();
        use_effect_with_deps(move |_| {
            let hash = location.query::<HashQuery>().unwrap_or_default();
            //hash_query.set(hash.clone());

            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(entry) = get_entry(&hash.hash).await {
                    log::info!("ENTRY {entry:?}");
                    entry_info.set(entry)
                }
            });

            || ()
        }, history.location().query::<HashQuery>().unwrap_or_default());
    }*/

    {
        let location = location.clone();
        let entry_info = entry_info.clone();
        use_mount(move || {
            wasm_bindgen_futures::spawn_local(async move {
                let hash = location.query::<HashQuery>().unwrap_or_default();
                if let Ok(entry) = get_entry(&hash.hash).await {
                    log::info!("ENTRY {entry:?}");
                    entry_info.set(entry)
                } else {
                    entry_info.set(EntryInfo::default())
                }
            });
        });
    }
    html! {
        <div>
            <div>
                <div class="container-fluid mt-3">

                    <div style="font-weight: bold; font-size: x-large;" class="mt-3">
                        <div style="float: left;" class="mb-3">
                            <button onclick={onback} class="btn btn-primary">
                                {"Zurück"}
                            </button>
                            <span class="ms-2">{entry_info.title.clone()}</span>
                            // download does not work because the link to the download is not the same origin
                            <a class="ms-2 me-2" href={pdf_path(&format!("{}.{}", &entry_info.hash, &entry_info.ext))} download={"true"}>
                                <button class="btn btn-primary">{"download"}</button>
                            </a>

                            <button class="btn btn-danger">
                            <a href={pdf_path(&format!("{}.{}", &entry_info.hash, &entry_info.ext))}>
                                {"PDF anzeigen"}
                            </a>
                            </button>

                            <br/>

                            {
                                entry_info.tags.iter().map(|tag| {
                                    html! {
                                        <span class="badge me-1 bg-secondary tag">{tag}</span>
                                        //<a href="it?page=0&tags={{ tag }}" style="font-size: 14px;" class="badge tag">{{ tag }}</a>
                                    }
                                }).collect::<Html>()
                            }

                        </div>
                    </div>
                </div>
            </div>
            <br />
            <div class="container">
            <div id="carouselExampleControls" class="carousel slide" data-bs-ride="carousel" data-interval="false">
                <div class="carousel-inner">
                    <div class="carousel-item active">
                        <img class="d-block carousel-image" src={image_path(&format!("{}_0.{}", &entry_info.hash, entry_info.img_exts.first().unwrap_or(&"".into())))} alt="Slide picture" />
                    </div>
                    {
                        if entry_info.img_exts.len() >= 1 {
                            (1..entry_info.img_exts.len()).into_iter().map(|idx| {
                                html!{
                                    <div class="carousel-item">
                                        <img class="d-block carousel-image" src={image_path(&format!("{}_{idx}.{}", &entry_info.hash, &entry_info.img_exts[idx]))} alt="Slide picture" />
                                    </div>
                                }
                            }).collect::<Html>()
                        } else {
                            html!()
                        }

                    }

                </div>
                <button class="carousel-control-prev carousel-control-size" type="button" data-bs-target="#carouselExampleControls" data-bs-slide="prev">
                    <span class="carousel-control-prev-icon it_bg_color" aria-hidden="true"></span>
                    <span class="visually-hidden">{"Previous"}</span>
                </button>
                <button class="carousel-control-next carousel-control-size" type="button" data-bs-target="#carouselExampleControls" data-bs-slide="next">
                    <span class="carousel-control-next-icon it_bg_color" aria-hidden="true"></span>
                    <span class="visually-hidden">{"Next"}</span>
                </button>
            </div>
            </div>
        </div>
    }
}
