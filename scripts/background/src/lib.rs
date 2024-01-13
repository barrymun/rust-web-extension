use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["console"])]
    fn log(message: &str);

    #[wasm_bindgen(js_namespace = ["chrome", "runtime"])]
    fn getManifest() -> JsValue;
}

#[wasm_bindgen(start)]
pub fn main() {
    log("This should appear in the background console");
    let x = getManifest();
    log(&format!("manifest: {:?}", x));
}
