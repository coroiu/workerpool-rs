use std::sync::{Arc, Mutex};

use workerpool::{
    backends::SameThreadBackend, routines::routine_registry::RoutineRegistry, WorkerPool,
};

use crate::wasm_workerpool::WebWorkerBackend;

pub struct Context {
    pub samethread_pool:
        WorkerPool<SameThreadBackend<Arc<Mutex<RoutineRegistry<Vec<u8>, Vec<u8>, ()>>>>>,

    pub webworker_pool: WorkerPool<WebWorkerBackend>,
}
