mod app;
mod bindings;
mod components;
mod date;
mod traning;

use app::App;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
