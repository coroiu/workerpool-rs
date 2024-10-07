extern crate console_error_panic_hook;

mod context;
mod main_application_client;

use main_application_client::MainApplicationClient;
use wasm_bindgen::prelude::*;
use workerpool_rs_wasm::Error;

// This function will be callable from JavaScript
#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[wasm_bindgen]
pub fn init_as_main(worker_url: String) -> MainApplicationClient {
    console_error_panic_hook::set_once();

    let client = MainApplicationClient::new(worker_url, 0);
    client
}

#[wasm_bindgen]
pub fn init_as_worker() -> Result<(), JsError> {
    console_error_panic_hook::set_once();

    workerpool_rs_wasm::WasmWorker::setup().map_err(|e| wasm_bindgen::JsError::from(e))?;

    Ok(())
}
