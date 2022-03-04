use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/src/share.js")]
extern "C" {
    #[wasm_bindgen(js_name = "share")]
    pub fn share(title: &str, text: &str);
}
