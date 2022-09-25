use yew::prelude::*;
use yew_router::prelude::*;

use crate::hooks::use_user_context;
use super::{Route, is_logged_in};
use crate::components::NavBar;

pub struct EntryInfo {
    uploader: String,
}

#[function_component(Entries)]
pub fn entries() -> Html {
    let user_ctx = use_user_context();
    let history = use_history().unwrap();
    
    {
        let history = history.clone();
        use_effect_with_deps(move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                if !is_logged_in().await {           
                    history.push(Route::Login);
                }
            });
            || ()
        }, ());
    
    }
    
    html! {
        
        <div>
            <NavBar />  
            
            <div class="container-fluid">
                <div class="row highlight">
                    <a href="/et" class="col et_bg_color card square">
                        <div class="">
                            <h1 class="text-center push-down text-white" style="margin-top: 56px;">{"ET"}</h1>
                        </div>
                    </a>

                    <a href="/?page=0&tags=IT#search_field" class="col it_bg_color card square">
                      <div class="">
                        <h1 class="text-center push-down text-white" style="margin-top: 56px;">{"IT"}</h1>
                      </div>
                  </a>

                  <a href="/el" class="col el_bg_color card square">
                    <div class="">
                      <h1 class="text-center push-down text-white" style="margin-top: 56px;">{"EL"}</h1>
                    </div>
                  </a>

                  <a href="/me" class="col me_bg_color card square">
                      <div class="">
                        <h1 class="text-center text-white" style="margin-top: 56px;">{"ME"}</h1>
                      </div>
                  </a>

                  <a href="/mb" class="col mb_bg_color card square">
                    <div class="">
                      <h1 class="text-center push-down text-white" style="margin-top: 56px;">{"MB"}</h1>
                    </div>
                    
                  </a>

                  <a href="/wi_log" class="col wi_bg_color card square">
                    <div class="">
                      <h1 class="text-center push-down text-white" style="margin-top: 56px;">{"WIL"}</h1>
                    </div>
                    
                  </a>

                  <a href="/wi_inf" class="col wi_bg_color square card">
                    <div class="">
                      <h1 class="text-center push-down text-white" style="margin-top: 56px;">{"WII"}</h1>
                    </div>
                    
                  </a>
                </div>
            </div>
            
        </div>

    }
}
