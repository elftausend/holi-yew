use yew::prelude::*;
use yew_router::prelude::*;

use crate::{routes::{entries::EntryInfo, Route, show_upload::HashQuery}, image_path, pdf_path};


#[derive(Debug, Properties, PartialEq, Eq)]
pub struct Props {
    pub entry: EntryInfo
}

#[function_component(EntryCard)]
pub fn entry_card(props: &Props) -> Html {
    
    let Props {
        entry,
    } = props.clone();

    if !entry.img_exts.is_empty() {
        html! {
            <div class="card">
                <Link<Route, HashQuery>
                    to={Route::ShowUpload}
                    query={Some(HashQuery{uid: entry.uid})}
                >
                    <img style="max-width: 50%; max-width: 10rem;" class="card-img-top " src={image_path(&format!("{}_0.{}", entry.hash.clone(), entry.img_exts.first().unwrap_or(&"".into())))} alt="picture" />
                    <div class="card-body">
                        <h5 class="card-title">
                            {entry.title.clone()}
                        </h5>
                        <p class="card-text">
                            {
                                entry.tags.iter().map(|tag| {
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
    } else {
        html! {
            <div class="card">
                <a href={pdf_path(&format!("{}.{}", &entry.hash, &entry.ext))} download={format!("{}.{}", &entry.title, &entry.ext)}>
                    <img style="max-width: 50%; max-width: 10rem;" class="card-img-top " src={image_path(&entry.view)} alt="picture" />
                    <div class="card-body">
                        <h5 class="card-title">
                            {entry.title.clone()}
                        </h5>
                        <p class="card-text">
                            {
                                entry.tags.iter().map(|tag| {
                                    html! {
                                        <span class="badge me-1 bg-secondary tag">{tag}</span>
                                    }
                                }).collect::<Html>()
                            }
                        </p>
                    </div>
                </a>
            </div>
        }
    }


}       
