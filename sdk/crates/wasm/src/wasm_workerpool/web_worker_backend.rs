use wasm_bindgen::prelude::*;
use workerpool::{
    task::{TaskRequest, TaskResponse},
    WorkerBackend,
};

#[wasm_bindgen(module = "/src/wasm_workerpool/web_worker_backend.js")]
extern "C" {
    // types created in JS are not Send
    // type JsWorker;

    type JsTaskRequest;
    type JsTaskResponse;

    fn spawn_worker() -> usize;

    async fn execute_task(worker: usize, request: JsTaskRequest) -> JsTaskResponse;

    // JsTaskRequest
    #[wasm_bindgen(constructor)]
    fn new() -> JsTaskRequest;

    #[wasm_bindgen(method, getter)]
    fn request_id(this: &JsTaskRequest) -> usize;
    #[wasm_bindgen(method, setter)]
    fn set_request_id(this: &JsTaskRequest, request_id: usize) -> JsTaskRequest;

    #[wasm_bindgen(method, getter)]
    fn routine_name(this: &JsTaskRequest) -> String;
    #[wasm_bindgen(method, setter)]
    fn set_routine_name(this: &JsTaskRequest, routine_name: String) -> JsTaskRequest;

    #[wasm_bindgen(method, getter)]
    fn args(this: &JsTaskRequest) -> js_sys::Uint8Array;
    #[wasm_bindgen(method, setter)]
    fn set_args(this: &JsTaskRequest, args: js_sys::Uint8Array) -> JsTaskRequest;

    // JsTaskResponse
    #[wasm_bindgen(constructor)]
    fn new() -> JsTaskResponse;

    #[wasm_bindgen(method, getter)]
    fn request_id(this: &JsTaskResponse) -> usize;
    #[wasm_bindgen(method, setter)]
    fn set_request_id(this: &JsTaskResponse, request_id: usize) -> JsTaskResponse;

    #[wasm_bindgen(method, getter)]
    fn result(this: &JsTaskResponse) -> js_sys::Uint8Array;
    #[wasm_bindgen(method, setter)]
    fn set_result(this: &JsTaskResponse, result: js_sys::Uint8Array) -> JsTaskResponse;
}

impl From<TaskRequest<Vec<u8>>> for JsTaskRequest {
    fn from(task_request: TaskRequest<Vec<u8>>) -> Self {
        let js_task_request = JsTaskRequest::new();
        js_task_request.set_request_id(task_request.request_id);
        js_task_request.set_routine_name(task_request.routine_name);
        js_task_request.set_args(js_sys::Uint8Array::from(&task_request.args[..]));
        js_task_request
    }
}

impl From<JsTaskRequest> for TaskRequest<Vec<u8>> {
    fn from(js_task_request: JsTaskRequest) -> Self {
        TaskRequest {
            request_id: js_task_request.request_id(),
            routine_name: js_task_request.routine_name(),
            args: js_task_request.args().to_vec(),
        }
    }
}

impl From<TaskResponse<Vec<u8>, ()>> for JsTaskResponse {
    fn from(task_response: TaskResponse<Vec<u8>, ()>) -> Self {
        let js_task_response = JsTaskResponse::new();
        js_task_response.set_request_id(task_response.request_id);
        js_task_response.set_result(js_sys::Uint8Array::from(
            &task_response.result.unwrap_or(vec![])[..],
        ));
        js_task_response
    }
}

impl From<JsTaskResponse> for TaskResponse<Vec<u8>, ()> {
    fn from(js_task_response: JsTaskResponse) -> Self {
        TaskResponse {
            request_id: js_task_response.request_id(),
            result: Ok(js_task_response.result().to_vec()),
        }
    }
}

pub struct WebWorkerBackend;

impl WebWorkerBackend {
    pub fn new() -> Self {
        WebWorkerBackend
    }
}

impl WorkerBackend for WebWorkerBackend {
    // TODO: Debug if Worker can work without Send
    type Worker = usize;
    type Input = Vec<u8>;
    type Output = Vec<u8>;
    type BackendError = ();
    type Error = ();

    // TODO: This should be async
    fn spawn_worker() -> Self::Worker {
        spawn_worker()
    }

    async fn execute_task(
        &self,
        worker: &Self::Worker,
        request: TaskRequest<Self::Input>,
    ) -> Result<
        // TODO: We probably want to flatten the error types here a bit
        TaskResponse<
            Self::Output,
            workerpool::routines::routine_registry::ExecuteRoutineError<Self::Error>,
        >,
        Self::BackendError,
    > {
        let js_task_request: JsTaskRequest = request.into();
        execute_task(*worker, js_task_request);
        // let js_task_response = execute_task(*worker, js_task_request).await;

        // todo!()
        Ok(TaskResponse {
            request_id: 0,
            result: Ok(vec![0]),
        })
    }
}
