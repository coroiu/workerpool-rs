use std::vec;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct MainApplicationClient {}

#[wasm_bindgen]
impl MainApplicationClient {
    pub fn new(worker_count: usize) -> Self {
        MainApplicationClient {}
    }

    /// Directly call the test routine without any workerpool-rs involvement
    pub fn run_direct(&self) -> u8 {
        let result = crate::routines::sleep_then_add(vec![5, 2, 3]);
        result.unwrap()[0]
    }
}
