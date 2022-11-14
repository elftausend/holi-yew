use reqwest::Method;
use web_sys::window;
use yew::prelude::*;
use yew_hooks::prelude::*;

use crate::REDIRECT;
use crate::{api::request, app::{set_jwt, get_jwt}, routes::htl_auth::UserInfo, error::HoliError};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub children: Children,
}

/// User context provider.
#[function_component(UserContextProvider)]
pub fn user_context_provider(props: &Props) -> Html {
    let user_ctx = use_state(UserInfo::default);

    {
        let user_ctx = user_ctx.clone();
        use_mount(move || {
            log::info!("userinfo: {user_ctx:?}");
            if get_jwt().is_none() {
                user_ctx.set(UserInfo::default());
                return;
            }

            wasm_bindgen_futures::spawn_local(async move {
                match request::<_, UserInfo>(Method::GET, "user", ()).await {
                    Ok(user_info) => {
                        user_ctx.set(user_info)
                    }
                    Err(e) => {
                        set_jwt(None);
                        user_ctx.set(UserInfo::default());
                        
                        let href = format!("https://auth.htl-hl.ac.at/authorize.php?response_type=code&client_id=holi.htl-hl.ac.at&redirect_uri={REDIRECT}&state=new");
                        window().unwrap().location().set_href(&href).unwrap();
                        //match e {
                        //    HoliError::Unauthorized | HoliError::Forbidden => {
                        //        set_jwt(None);
                        //        user_ctx.set(UserInfo::default());
                        //    },
                        //    _ => ()
                        //}
                    }
                }

            });
        });
    }

    html! {
        <ContextProvider<UseStateHandle<UserInfo>> context={user_ctx}>
            { for props.children.iter() }
        </ContextProvider<UseStateHandle<UserInfo>>>
    }
}
