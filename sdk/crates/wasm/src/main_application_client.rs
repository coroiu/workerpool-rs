use std::vec;

use wasm_bindgen::prelude::*;

use crate::context::Context;

#[wasm_bindgen]
pub struct MainApplicationClient {
    context: crate::context::Context,
}

impl MainApplicationClient {
    pub fn new(worker_count: usize) -> Self {
        MainApplicationClient {
            context: Context {},
        }
    }
}

#[wasm_bindgen]
impl MainApplicationClient {
    /// Directly call the test routine without any workerpool_rs:: involvement
    pub fn run_direct(&self) -> u8 {
        todo!()
    }

    pub async fn run_same_thread(&self) -> u8 {
        todo!()
    }

    pub async fn run_in_worker(&self) -> u8 {
        todo!()
    }
}
