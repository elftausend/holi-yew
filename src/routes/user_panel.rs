use yew::prelude::*;

#[function_component(UserPanel)]
pub fn user_panel() -> Html {
    html! {
        <>
            <div class="row highlight">
                <a href="{{url_for('upload')}}" class="col et_bg_color card square">
                    <h1 class="text-center push-down text-white" style="margin-top: 56px;">{"Upload"}</h1>
                </a>
            </div>
            <div class="row highlight">
                <a href="delete?page=0&tags" class="col it_bg_color card square">
                    <h1 class="text-center push-down text-white" style="margin-top: 56px;">{"Delete"}</h1>
                </a>
            </div>
        </>
    }
}
