use gloo::file::File;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::{use_history, History};

#[derive(Debug, Default, Clone)]
struct FileDetails {
    name: String,
    file_type: String,
    data: Vec<u8>,
}

#[derive(Debug, Default, Clone)]
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

                    let upload_info = upload_info.clone();
                    let mut info = (*upload_info).clone();

                    let task = gloo::file::callbacks::read_as_bytes(&file, move |res| {
                        let data = res.expect("Failed to read the file");
                        
                        info.file = FileDetails {
                            name,
                            file_type,
                            data
                        };
                        
                        upload_info.set(info);
                    });
                    handle.set(Some(task));
                                        
                }
            }

            
        })
    };

    let on_click_upload = {
        let upload_info = upload_info.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            handle.set(None);
            log::info!("{:?}", upload_info.file);
            
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
            <div class="container-fluid mt-3">
                <button onclick={onback} class="btn btn-primary">
                    {"Zurück"}
                </button>
                
                <form>
                
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
                        <h3>{"Füge einen kurzen und knackigen Titel hinzu"}</h3>
                        <textarea 
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
                        <button onclick={on_click_upload} class="btn btn-primary">
                            {"Upload"}
                        </button>         
                    </div>               

                </form>
            
            </div>

        </div>
    }
}