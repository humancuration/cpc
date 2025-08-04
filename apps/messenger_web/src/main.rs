use messenger_web::App;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn run() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}