use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct MainApplicationClient {}

impl MainApplicationClient {
    pub fn new(worker_count: usize) -> Self {
        MainApplicationClient {}
    }
}
