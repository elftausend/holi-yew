
use crate::text_input::TextInput;
use yew::prelude::*;

pub enum Msg {
    InputUserID(String),
    InputPassword(String),
    TryLogin,
}

#[derive(Debug, Default)]
pub struct App {
    user: String,
    password: String,
    logged_in: bool
}

impl Component for App {
    type Message = Msg;

    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self::default()
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::InputUserID(next_input) => self.user = next_input,
            Msg::InputPassword(next_pw) => self.password = next_pw,
            Msg::TryLogin => {return true;},
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_user_change = ctx.link().callback(Msg::InputUserID);
        let on_pw_change = ctx.link().callback(Msg::InputPassword);

        let onlogin = ctx.link().callback(|_| Msg::TryLogin);

        html! {                
            <div class="hero">
                <div class="hero-body container pb-0">
                    <div class="login">
                        <img alt="Holi Logo" src="./holi-logo.png" />
                        <TextInput input_type="text" on_change={on_user_change} value={self.user.clone()} maxlength="128" placeholder="HTLHL UserID" />
                        <TextInput input_type="password" on_change={on_pw_change} value={self.password.clone()} maxlength="1024" placeholder="Password" />
                        <button onclick={onlogin} class="button is-primary">
                            {"Login"}
                        </button>
                    </div>
                </div>
            </div>

                /*
                <div class="login-form">
                    <div class="">
                        <div class="row">
                            <TextInput input_type="text" on_change={on_user_change} value={self.user.clone()} maxlength="128" placeholder="HTLHL UserID" />
                        </div>
                        <div class="row">
                            <TextInput input_type="password" on_change={on_pw_change} value={self.password.clone()} maxlength="1024" placeholder="Password" />
                            <button onclick={onlogin} class="btn btn-secondary">
                                {"Login"}
                            </button>
                        </div>
                    </div>
                </div>    
                */
                /*<div class="login">
                    <div>
                        <TextInput on_change={on_user_change} value={self.user.clone()} />
                    </div>
                    <div>
                        <TextInput on_change={on_pw_change} value={self.user.clone()} />
                    </div>
                </div>*/
            
        }
    }
}