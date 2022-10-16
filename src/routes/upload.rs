use gloo::file::File;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::{use_history, History};

use crate::request;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
struct UploadMsgs {
    missing_file: String,
    missing_title: String,
    erroneous_date: String,
    missing_tags: String,
    no_user_terms: String,
    successful_upload: String,
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
    divison: String,
    tags: String,
}

#[function_component(Upload)]
pub fn upload() -> Html {
    let history = use_history().unwrap();
    let upload_info = use_state(UploadInfo::default);
    let upload_msgs = use_state(UploadMsgs::default);
    let handle = use_state(|| None);

    let on_file_change = {
        let handle = handle.clone();
        let upload_info = upload_info.clone();

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
        })
    };

    let on_click_upload = {
        let upload_msgs = upload_msgs.clone();
        let upload_info = upload_info.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            handle.set(None);
            //log::info!("{:?}", upload_info.file);

            let upload_info = upload_info.clone();
            let upload_msgs = upload_msgs.clone();
            
            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(err_msgs) =
                    request::<UploadInfo, UploadMsgs>(Method::POST, "upload", (*upload_info).clone(), false).await
                {
                    log::info!("err_msgs!!!!!!!!!!!!!!: {err_msgs:?}");
                    upload_msgs.set(err_msgs);
                    upload_info.set(UploadInfo::default())
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
        let history = history.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            history.back();
        })
    };

    let ondatepress = {
        Callback::from(move |e: KeyboardEvent| {
            let k = e.key_code();
            if !(k >= 48 && k <= 57 || k == 46) {
                e.prevent_default();
            }
        })
    };

    html! {
        <div>
            <div class="container-fluid mt-3">
                <button onclick={onback} class="btn btn-primary">
                    {"Zurück"}
                </button>

                <form>

                    <span style="color: red;">{ upload_msgs.missing_file.clone() }</span>
                    <br />                    
                    <label for="file-upload">{"PDF oder Source File hochladen"}</label>
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
                        <h3>{"Füge einen passenden Titel hinzu"}</h3>
                        <span style="color: red;">{ upload_msgs.missing_title.clone() }</span>
                        <textarea
                            oninput={on_title_input}
                            class="form-control"
                            autocomplete="off"
                            style="width: 300px; height: 70px;"
                            type="text"
                            placeholder="z.B.: Lineare Funktion"
                            name="title">
                            {"Input"}
                        </textarea>
                    </div>

                    <div class="mb-3">
                        <h3>{"Füge Tags hinzu"}</h3>
                        <span style="color: red;">{ upload_msgs.missing_tags.clone() }</span>
                        <textarea
                            oninput={on_tag_input}
                            class="form-control"
                            autocomplete="off"
                            style="width: 300px; height: 70px;"
                            type="text"
                            placeholder="z.B.: 2BHITS Mathematik Funktionen Steigung-zwei-Punkte"
                            name="title">
                            {"Input"}
                        </textarea>
                    </div>

                    <div class="mb-3">
                        <input autocomplete="off" 
                                id="dateinput" 
                                onkeypress={ondatepress} 
                                class="form-control" 
                                style="width: 120px; height: 50px;" 
                                maxlength="10" 
                                type="text" 
                                placeholder="{{ date }}" 
                                name="date"
                            />
                    </div>

                    <div class="mb-3">
                        <p>
                            <span style="color: rgb(4, 167, 4);">{upload_msgs.successful_upload.clone()}</span>
                        </p>
                        <button onclick={on_click_upload} class="btn btn-primary">
                            {"Upload"}
                        </button>
                    </div>

                </form>

            </div>

        </div>
    }
}
