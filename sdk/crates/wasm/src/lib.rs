extern crate console_error_panic_hook;

mod context;
mod main_application_client;

use main_application_client::MainApplicationClient;
use wasm_bindgen::prelude::*;

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
