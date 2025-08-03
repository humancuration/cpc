//! Main entry point for the web application

use yew::Renderer;
use cpc_live_streaming::ui::app::App;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    Renderer::<App>::new().render();
}