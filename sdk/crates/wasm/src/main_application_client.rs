use std::vec;

use wasm_bindgen::prelude::*;
use workerpool_rs::{backends::SameThreadBackend, global::get_registry, WorkerPool};

use crate::{context::Context, wasm_workerpool::WebWorkerBackend};

#[wasm_bindgen]
pub struct MainApplicationClient {
    context: crate::context::Context,
}

impl MainApplicationClient {
    pub fn new(worker_count: usize) -> Self {
        MainApplicationClient {
            context: Context {
                samethread_pool: WorkerPool::new(
                    SameThreadBackend::new(get_registry()),
                    worker_count,
                ),

                webworker_pool: WorkerPool::new(WebWorkerBackend, 4),
            },
        }
    }
}

#[wasm_bindgen]
impl MainApplicationClient {
    /// Directly call the test routine without any workerpool_rs:: involvement
    pub fn run_direct(&self) -> u8 {
        let result = crate::routines::sleep_then_add(vec![5, 2, 3]);
        result.unwrap()[0]
    }

    pub async fn run_same_thread(&self) -> u8 {
        let result = self
            .context
            .samethread_pool
            .execute_function(crate::routines::sleep_then_add, vec![5, 2, 3])
            .await;
        result.unwrap().result.unwrap()[0]
    }

    pub async fn run_in_worker(&self) -> u8 {
        let result = self
            .context
            .webworker_pool
            .execute_function(crate::routines::sleep_then_add, vec![5, 2, 3])
            .await;
        result.unwrap().result.unwrap()[0]
    }
}
