use app::App;
use dotenv_codegen::dotenv;
//use gloo_net::http::Request;

mod api;
mod app;
mod components;
mod error;
mod hooks;
mod routes;

const API_ROOT: &str = dotenv!("API_ROOT");

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
