use app::App;
use dotenv_codegen::dotenv;

mod api;
mod app;
mod components;
mod error;
mod hooks;
mod routes;

const API_ROOT: &str = dotenv!("API_ROOT");

fn main() {
    yew::start_app::<App>();
}
