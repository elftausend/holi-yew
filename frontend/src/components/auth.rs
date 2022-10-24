use web_sys::window;
use yew::prelude::*;
use yew_hooks::use_mount;

use crate::{hooks::use_user_context, routes::is_logged_in, REDIRECT};

#[derive(Properties, Debug, PartialEq)]
pub struct Props {
    pub children: Children,
}

#[function_component(Auth)]
pub fn auth(props: &Props) -> Html {
    let user_ctx = use_user_context();
    let logged_in = use_state(|| false);
    let user_ctx_check = user_ctx.clone();

    {
        let logged_in = logged_in.clone();
        use_mount(move || {
            wasm_bindgen_futures::spawn_local(async move {
                let is_logged_in = is_logged_in().await;
                logged_in.set(is_logged_in);
                if !is_logged_in {
                    let href = format!("https://auth.htl-hl.ac.at/authorize.php?response_type=code&client_id=holi.htl-hl.ac.at&redirect_uri={REDIRECT}&state=new");
                    window().unwrap().location().set_href(&href).unwrap();
                    //log::info!("-  - - - - - - - would redirect !!!");    
                }
            });
            //if !user_ctx.inner.is_auth() {
            //    log::info!("-  - - - - - - - would redirect !!!");
            //    //window().unwrap().location().set_href("https://auth.htl-hl.ac.at/authorize.php?response_type=code&client_id=holi.htl-hl.ac.at&redirect_uri=https://holi.htl-hl.ac.at/authenticated&state=new").unwrap();
            //}
        });
    }
    /*use_effect_with_deps(move |_| {
        if !user_ctx.inner.is_auth() {
            log::info!("-  - - - - - - - would redirect !!!");
            //window().unwrap().location().set_href("https://auth.htl-hl.ac.at/authorize.php?response_type=code&client_id=holi.htl-hl.ac.at&redirect_uri=https://holi.htl-hl.ac.at/authenticated&state=new").unwrap();
        }
        || ()
    }, user_ctx_check);*/
    if *logged_in {
        html! {
            { for props.children.iter() }
        }
    } else {
        html! {
            
        }
    }
}