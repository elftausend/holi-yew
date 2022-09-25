use app::App;
use dotenv_codegen::dotenv;

mod routes;
mod app;
mod api;
mod error;
mod components;

const API_ROOT: &str = dotenv!("API_ROOT");

fn main() {
    yew::start_app::<App>();
}