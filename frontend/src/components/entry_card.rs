use yew::prelude::*;
use yew_router::prelude::*;

use crate::{
    image_path,
    routes::{entries::EntryInfo, show_upload::HashQuery, Route},
};

#[derive(Debug, Properties, PartialEq, Eq)]
pub struct Props {
    pub entry: EntryInfo,
}

#[function_component(EntryCard)]
pub fn entry_card(props: &Props) -> Html {
    let Props { entry } = props.clone();

    if !entry.img_exts.is_empty() {
        html! {

            <div class="card">
                <Link<Route, HashQuery>
                    to={Route::ShowUpload}
                    query={Some(HashQuery{uid: entry.uid})}
                >
                <div>
                    <img style="max-width: 50%; max-width: 10rem;" class="card-img-top " src={image_path(&format!("{}_0.{}", entry.hash.clone(), entry.img_exts.first().unwrap_or(&"".into())))} alt="picture" />
                    <div style="float: right;">
                        <svg style="fill: rgb(25,25,25);" aria-hidden="true" height="16" viewBox="0 0 16 16" version="1.1" width="16" data-view-component="true" class="me-1">
                            <path fill-rule="evenodd" d="M8 .25a.75.75 0 01.673.418l1.882 3.815 4.21.612a.75.75 0 01.416 1.279l-3.046 2.97.719 4.192a.75.75 0 01-1.088.791L8 12.347l-3.766 1.98a.75.75 0 01-1.088-.79l.72-4.194L.818 6.374a.75.75 0 01.416-1.28l4.21-.611L7.327.668A.75.75 0 018 .25zm0 2.445L6.615 5.5a.75.75 0 01-.564.41l-3.097.45 2.24 2.184a.75.75 0 01.216.664l-.528 3.084 2.769-1.456a.75.75 0 01.698 0l2.77 1.456-.53-3.084a.75.75 0 01.216-.664l2.24-2.183-3.096-.45a.75.75 0 01-.564-.41L8 2.694v.001z"></path>
                        </svg>
                        {entry.favs}
                    </div>
                </div>
                    <div class="card-body">
                        <h5 class="card-title">
                            {entry.title.clone()}
                        </h5>
                        <p class="card-text">
                            {
                                entry.tags.iter().map(|tag| {
                                    html! {
                                        <span class="badge me-1 bg-secondary tag mt-1">{tag}</span>
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
                        <Link<Route, HashQuery>
                        to={Route::ShowUpload}
                        query={Some(HashQuery{uid: entry.uid})}
                        >
                       // <a href={pdf_path(&format!("{}.{}", &entry.hash, &entry.ext))} download={format!("{}.{}", &entry.title, &entry.ext)}>
                            <img style="max-width: 50%; max-width: 10rem;" class="card-img-top " src={image_path(&entry.view)} alt="picture" />
                            <div style="float: right;">
                                <svg style="fill: rgb(25,25,25);" aria-hidden="true" height="16" viewBox="0 0 16 16" version="1.1" width="16" data-view-component="true" class="me-1">
                                    <path fill-rule="evenodd" d="M8 .25a.75.75 0 01.673.418l1.882 3.815 4.21.612a.75.75 0 01.416 1.279l-3.046 2.97.719 4.192a.75.75 0 01-1.088.791L8 12.347l-3.766 1.98a.75.75 0 01-1.088-.79l.72-4.194L.818 6.374a.75.75 0 01.416-1.28l4.21-.611L7.327.668A.75.75 0 018 .25zm0 2.445L6.615 5.5a.75.75 0 01-.564.41l-3.097.45 2.24 2.184a.75.75 0 01.216.664l-.528 3.084 2.769-1.456a.75.75 0 01.698 0l2.77 1.456-.53-3.084a.75.75 0 01.216-.664l2.24-2.183-3.096-.45a.75.75 0 01-.564-.41L8 2.694v.001z"></path>
                                </svg>
                                {entry.favs}
                            </div>
                            <div class="card-body">
                                <h5 class="card-title">
                                    {entry.title.clone()}
                                </h5>
                                <p class="card-text">
                                    {
                                        entry.tags.iter().map(|tag| {
                                            html! {
                                                <span class="badge me-1 bg-secondary tag mt-1">{tag}</span>
                                            }
                                        }).collect::<Html>()
                                    }
                                </p>
                            </div>
        //                </a>
                        </Link<Route, HashQuery>>
                    </div>
                }
    }
}
