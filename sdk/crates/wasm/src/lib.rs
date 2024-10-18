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

    // let function_ptr = main_application_client::increment as *const fn(u32) -> u32;
    let function_ptr = main_application_client::increment as *const ();
    let to_send = function_ptr as u32;
    let f: fn(u32) -> u32 = unsafe { std::mem::transmute(to_send) };
    web_sys::console::log_1(&JsValue::from_str(
        format!(
            "init_as_main function pointer: 0x{:04x}. 1 + 1 = {}",
            to_send,
            f(1)
        )
        .as_str(),
    ));

    let client = MainApplicationClient::new(worker_url, 0);
    client
}

#[wasm_bindgen]
pub fn init_as_worker() -> Result<(), JsError> {
    console_error_panic_hook::set_once();

    // let function_ptr = main_application_client::increment as *const fn(u32) -> u32;
    let function_ptr = main_application_client::increment as *const ();
    let to_send = function_ptr as u32;
    let f: fn(u32) -> u32 = unsafe { std::mem::transmute(to_send) };
    web_sys::console::log_1(&JsValue::from_str(
        format!(
            "init_as_worker function pointer: 0x{:04x}. 1 + 1 = {}",
            to_send,
            f(1)
        )
        .as_str(),
    ));

    workerpool_rs_wasm::WasmWorker::setup().map_err(|e| wasm_bindgen::JsError::from(e))?;

    Ok(())
}
