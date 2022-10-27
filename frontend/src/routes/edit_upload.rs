use crate::{components::Auth, error::HoliError, request};
use reqwest::Method;
use serde::{Deserialize, Serialize};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_hooks::use_mount;
use yew_router::prelude::*;

use super::{entries::EntryInfo, show_upload::HashQuery, upload::UploadMsgs};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct EditInfo {
    title: String,
    tags: String,
}

pub async fn get_edit_entry(uid: i32) -> Result<EntryInfo, HoliError> {
    request(Method::GET, &format!("edit_entry?uid={uid}"), (), false).await
}

#[function_component(EditUpload)]
pub fn edit_upload() -> Html {
    let history = use_history().unwrap();
    let edit_info = use_state(EditInfo::default);
    let disable_edit = use_state(|| false);
    let upload_msgs = use_state(UploadMsgs::default);

    let location = use_location().unwrap();
    {
        let location = location.clone();
        let edit_info = edit_info.clone();
        let history = history.clone();
        use_mount(move || {
            wasm_bindgen_futures::spawn_local(async move {
                let hash = location.query::<HashQuery>().unwrap_or_default();
                if let Ok(entry) = get_edit_entry(hash.uid).await {
                    log::info!("EDIT ENTRY {entry:?}");
                    edit_info.set(EditInfo {
                        title: entry.title,
                        tags: entry
                            .tags
                            .iter()
                            .map(|tag| format!("{tag} "))
                            .collect::<String>(),
                    })
                } else {
                    edit_info.set(EditInfo::default());
                    history.back();
                }
            });
        });
    }

    let on_title_input = {
        let edit_info = edit_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*edit_info).clone();
            info.title = input.value();
            edit_info.set(info);
        })
    };

    let on_tag_input = {
        let edit_info = edit_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*edit_info).clone();
            info.tags = input.value();
            edit_info.set(info);
        })
    };

    let on_click_save = {
        let upload_msgs = upload_msgs.clone();
        let edit_info = edit_info.clone();
        let location = location.clone();

        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            if *disable_edit {
                return;
            }

            //log::info!("{:?}", edit_info.file);

            let edit_info = edit_info.clone();
            let upload_msgs = upload_msgs.clone();
            let location = location.clone();

            disable_edit.set(true);

            let disable_edit = disable_edit.clone();

            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(err_msgs) = request::<EditInfo, UploadMsgs>(
                    Method::POST,
                    &format!(
                        "edit_entry?uid={}",
                        location.query::<HashQuery>().unwrap_or_default().uid
                    ),
                    (*edit_info).clone(),
                    false,
                )
                .await
                {
                    log::info!("err_msgs!!!!!!!!!!!!!!: {err_msgs:?}");

                    if &err_msgs.successful_upload != "" {
                        //   upload_info.set(UploadInfo::default())
                    }
                    disable_edit.set(false);
                    upload_msgs.set(err_msgs);
                }
            });
        })
    };

    let onback = {
        let history = history.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            history.back();
        })
    };

    html! {
        <div>
            <Auth>
            <div class="container-fluid mt-3">
                <button onclick={onback} class="btn btn-primary">
                    {"Zurück"}
                </button>

                <form>
                    <div class="mb-3">
                        <h3>{"Editiere den Titel*"}</h3>
                        <span style="color: red;">{ upload_msgs.missing_title.clone() }</span>
                        <textarea
                            oninput={on_title_input}
                            class="form-control"
                            autocomplete="off"
                            style="width: 300px; height: 70px;"
                            value={edit_info.title.clone()}
                            type="text"
                            placeholder="z.B.: Lineare Funktion"
                            name="title">
                            {"Input"}
                        </textarea>
                    </div>

                    <div class="mb-3">
                        <h3>{"Ändere die Tags*"}</h3>
                        <span style="color: red;">{ upload_msgs.missing_tags.clone() }</span>
                        <textarea
                            oninput={on_tag_input}
                            class="form-control"
                            autocomplete="off"
                            style="width: 300px; height: 70px;"
                            type="text"
                            value={edit_info.tags.clone()}
                            placeholder="z.B.: 2BHITS Mathematik Funktionen Steigung-zwei-Punkte"
                            name="title">
                            {"Input"}
                        </textarea>
                    </div>
                    <div class="mb-3">
                        <p>
                            <span style="color: rgb(4, 167, 4);">{upload_msgs.successful_upload.clone()}</span>
                        </p>
                        <button onclick={on_click_save} class="btn btn-primary">
                            {"Speichern"}
                        </button>

                        <button class="btn btn-danger ms-2">
                            {"Löschen"}
                        </button>
                        <br />
                        <span style="color: red; font-style: italic;">
                            { "Felder markiert mit '*' müssen ausgefüllt werden."}
                        </span>
                    </div>
                </form>
            </div>
            </Auth>
        </div>
    }
}