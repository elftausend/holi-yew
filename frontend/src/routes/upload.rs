use crate::{components::Auth, hooks::use_user_context};
use gloo::file::File;
use js_sys::Date;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_hooks::use_mount;
use yew_router::prelude::{use_history, History};

use crate::request;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct UploadMsgs {
    pub missing_file: String,
    pub missing_title: String,
    pub erroneous_date: String,
    pub missing_tags: String,
    pub no_user_terms: String,
    pub successful_upload: String,
    pub erroneous_division: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
struct FileDetails {
    name: String,
    file_type: String,
    data: Vec<u8>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct UploadInfo {
    file: FileDetails,
    title: String,
    date: String,
    // get through api
    htl_division: String,
    tags: String,
}

pub fn add_zero_pad(num: u32) -> String {
    if num >= 10 {
        num.to_string()
    } else {
        format!("0{num}")
    }
}

#[function_component(Upload)]
pub fn upload() -> Html {
    let user_ctx = use_user_context();

    let history = use_history().unwrap();
    let upload_info = use_state(UploadInfo::default);
    let upload_msgs = use_state(UploadMsgs::default);
    let date_str = use_state(String::new);

    let file_select = use_state(|| None);

    let disable_upload = use_state(|| false);
    let handle = use_state(|| None);

    {
        let date_str = date_str.clone();
        use_mount(move || {
            let date = Date::new_0();
            let day = add_zero_pad(date.get_date());
            let month = add_zero_pad(date.get_month() + 1);
            let year = date.get_full_year();

            date_str.set(format!("{day}.{month}.{year}"))
        });
    }

    let on_file_change = {
        let handle = handle.clone();
        let upload_info = upload_info.clone();
        let file_select = file_select.clone();

        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();

            if let Some(files) = input.files() {
                let files = js_sys::try_iter(&files)
                    .unwrap()
                    .unwrap()
                    .map(|v| web_sys::File::from(v.unwrap()))
                    .map(File::from);

                if let Some(file) = files.last() {
                    let name = file.name();
                    let file_type = file.raw_mime_type();
                    log::info!("file_type: {file_type}");

                    let upload_info = upload_info.clone();
                    let mut info = (*upload_info).clone();

                    let task = gloo::file::callbacks::read_as_bytes(&file, move |res| {
                        let data = res.expect("Failed to read the file");

                        info.file = FileDetails {
                            name,
                            file_type,
                            data,
                        };

                        upload_info.set(info);
                    });
                    handle.set(Some(task));
                }
            }
            file_select.set(Some(input));
        })
    };

    let on_click_upload = {
        let upload_msgs = upload_msgs.clone();
        let upload_info = upload_info.clone();

        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            if *disable_upload {
                return;
            }
            handle.set(None);
            //log::info!("{:?}", upload_info.file);

            let upload_info = upload_info.clone();
            let upload_msgs = upload_msgs.clone();
            let file_select = file_select.clone();

            disable_upload.set(true);

            let disable_upload = disable_upload.clone();

            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(err_msgs) = request::<UploadInfo, UploadMsgs>(
                    Method::POST,
                    "upload",
                    (*upload_info).clone(),
                )
                .await
                {
                    log::info!("err_msgs!!!!!!!!!!!!!!: {err_msgs:?}");

                    if !&err_msgs.successful_upload.is_empty() {
                        // unselect file
                        (*file_select).as_ref().unwrap().set_value("");
                        upload_info.set(UploadInfo::default());
                    }
                    disable_upload.set(false);
                    upload_msgs.set(err_msgs);
                }
            });
        })
    };

    let on_title_input = {
        let upload_info = upload_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*upload_info).clone();
            info.title = input.value();
            upload_info.set(info);
        })
    };

    let on_tag_input = {
        let upload_info = upload_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*upload_info).clone();
            info.tags = input.value();
            upload_info.set(info);
        })
    };

    let onback = {
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            history.back();
        })
    };

    let ondatepress = {
        Callback::from(move |e: KeyboardEvent| {
            let k = e.key_code();

            if !((48..=57).contains(&k) || k == 46) {
                e.prevent_default();
            }
        })
    };

    let ondateinput = {
        let upload_info = upload_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*upload_info).clone();
            info.date = input.value();
            upload_info.set(info);
        })
    };

    let ondivisioninput = {
        let upload_info = upload_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let value = input.value();
            
            if value.len() > 3 {
                e.prevent_default();
            }
            
            let mut info = (*upload_info).clone();
            info.htl_division = value;
            upload_info.set(info);
        })
    };

    let onclassinput = {
        let upload_info = upload_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*upload_info).clone();
            //info.class = input.value();
            upload_info.set(info);
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

                        <span style="color: red;">{ upload_msgs.missing_file.clone() }</span>
                        <br />
                        <label for="file-upload">{"PDF oder Source File hochladen*"}</label>
                        <br />
                        <input
                            class="mb-3"
                            id="file-upload"
                            type="file"
                            accept=".pdf,.rs,.java,.py,.js,.cpp,.c"
                            onchange={on_file_change}
                            multiple={false}
                        />
                        <div class="mb-3">
                            <h3>{"Füge einen passenden Titel hinzu*"}</h3>
                            <span style="color: red;">{ upload_msgs.missing_title.clone() }</span>
                            <textarea
                                oninput={on_title_input}
                                class="form-control"
                                autocomplete="off"
                                style="width: 300px; height: 70px;"
                                value={upload_info.title.clone()}
                                type="text"
                                placeholder="z.B.: Lineare Funktion"
                                name="title">
                                {"Input"}
                            </textarea>
                        </div>

                        <div class="mb-3">
                            <h3>{"Füge Tags hinzu*"}</h3>
                            <span style="color: red;">{ upload_msgs.missing_tags.clone() }</span>
                            <textarea
                                oninput={on_tag_input}
                                class="form-control"
                                autocomplete="off"
                                style="width: 300px; height: 70px;"
                                type="text"
                                value={upload_info.tags.clone()}
                                placeholder="z.B.: 2BHITS Mathematik Funktionen Steigung-zwei-Punkte"
                                name="title">
                                {"Input"}
                            </textarea>
                        </div>

                        <div class="mb-3">
                            <h4>{"Abteilung"}</h4>
                            <span style="color: red;">{ upload_msgs.erroneous_date.clone() }</span>
                            <input autocomplete="off"
                                    id="dateinput"
                                    oninput={ondivisioninput}
                                    class="form-control"
                                    style="width: 120px; height: 50px;"
                                    maxlength="10"
                                    type="text"
                                    placeholder={user_ctx.inner.division.clone()}
                                    name="date"
                                />
                        </div>

                        <div class="mb-3">
                            <h4>{"Datum"}</h4>
                            <span style="color: red;">{ upload_msgs.erroneous_date.clone() }</span>
                            <input autocomplete="off"
                                    id="dateinput"
                                    onkeypress={ondatepress}
                                    oninput={ondateinput}
                                    class="form-control"
                                    style="width: 120px; height: 50px;"
                                    maxlength="10"
                                    type="text"
                                    placeholder={(*date_str).clone()}
                                    name="date"
                                />
                        </div>

                        /*<div class="mb-3">
                            <h4>{"Abteilung"}</h4>
                            <input autocomplete="off"
                                    id="dateinput"
                                    onkeypress={ondivisionpress}
                                    class="form-control"
                                    style="width: 120px; height: 50px;"
                                    maxlength="10"
                                    type="text"
                                    placeholder={user_ctx.inner.division.clone()}
                                    name="date"
                            />
                        </div>*/

                        <div class="mb-3">
                            <p>
                                <span style="color: rgb(4, 167, 4);">{upload_msgs.successful_upload.clone()}</span>
                            </p>
                            <button onclick={on_click_upload} class="btn btn-primary">
                                {"Upload"}
                            </button>
                            <br />
                            <span style="color: red; font-style: italic;">{ "Felder markiert mit '*' müssen ausgefüllt werden."}
                                <br />
                                {"Die Abteilung des Uploaders wird automatisch als Tag hinzugefügt."}
                            </span>
                        </div>

                    </form>

                </div>
            </Auth>
        </div>
    }
}
