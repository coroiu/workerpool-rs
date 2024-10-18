use wasm_bindgen::prelude::*;
use workerpool_rs::WorkerBackend;

use crate::context::Context;

#[wasm_bindgen]
pub struct MainApplicationClient {
    context: crate::context::Context,
}

impl MainApplicationClient {
    pub fn new(worker_url: String, worker_count: usize) -> Self {
        MainApplicationClient {
            context: Context {
                backend: workerpool_rs_wasm::WasmWorkerBackend::new(worker_url),
            },
        }
    }
}

pub fn increment(input: u8) -> u8 {
    input + 1
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

    pub async fn run_in_worker(&self, input: u8) -> u8 {
        self.context.backend.exec(input, increment).await;
        println!("Running in worker");
        0
    }
}
