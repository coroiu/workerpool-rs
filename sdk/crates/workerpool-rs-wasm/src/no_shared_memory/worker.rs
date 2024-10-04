use wasm_bindgen::JsValue;

pub struct WasmWorker;

impl WasmWorker {
    pub fn setup() {
        web_sys::console::log_1(&JsValue::from_str("WasmWorker running"));
    }
}
