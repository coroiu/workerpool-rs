extern crate console_error_panic_hook;

mod main_application_client;
mod routines;

use main_application_client::MainApplicationClient;
use wasm_bindgen::prelude::*;

// This function will be callable from JavaScript
#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[wasm_bindgen]
pub fn init_as_main(worker_count: usize) -> MainApplicationClient {
    console_error_panic_hook::set_once();
    let client = MainApplicationClient::new(4);
    client
}
