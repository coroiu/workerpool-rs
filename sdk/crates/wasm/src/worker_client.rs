use crate::wasm_workerpool::{JsTaskRequest, JsTaskResponse};
use wasm_bindgen::prelude::*;
use workerpool::{
    global::get_registry,
    routines::routine_registry::RoutineRegistryTrait,
    task::{TaskRequest, TaskResponse},
};

#[wasm_bindgen]
pub struct WorkerClient;

impl WorkerClient {
    pub fn new() -> Self {
        WorkerClient
    }
}

#[wasm_bindgen]
impl WorkerClient {
    pub fn execute_routine(&self, js_request: JsTaskRequest) -> JsTaskResponse {
        let request: TaskRequest<Vec<u8>> = js_request.into();
        let routine_name = request.routine_name;
        let args = request.args;

        let registry = get_registry();
        let routine = registry
            .lock()
            .unwrap()
            .get_routine(routine_name.as_str())
            // TODO: Fix serialization of error
            .unwrap();
        let result = routine.execute(args);

        let response = TaskResponse::new(request.request_id, result);
        let js_response: JsTaskResponse = response.into();
        js_response
    }
}
