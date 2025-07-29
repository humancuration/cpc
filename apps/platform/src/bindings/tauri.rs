use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/src/bindings/tauri.js")]
extern "C" {
    #[wasm_bindgen(catch)]
    pub async fn invoke(cmd: &str, args: &JsValue) -> Result<JsValue, JsValue>;
}