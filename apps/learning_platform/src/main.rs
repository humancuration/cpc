use learning_platform::App;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn run() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}

// Tauri entry point
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(target_arch = "wasm32")]
fn main() {
    run();
}