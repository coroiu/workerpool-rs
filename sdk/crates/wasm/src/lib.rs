extern crate console_error_panic_hook;

mod context;
mod main_application_client;
mod routines;
mod wasm_workerpool;

use main_application_client::MainApplicationClient;
use wasm_bindgen::prelude::*;
use workerpool::{global::register_routine, Routine};

// This function will be callable from JavaScript
#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[wasm_bindgen]
pub fn init_as_main(worker_count: usize) -> MainApplicationClient {
    console_error_panic_hook::set_once();

    // Manually register the routine because ctor does not support WASM
    register_routine(Routine::new(routines::sleep_then_add));

    let client = MainApplicationClient::new(4);
    client
}
