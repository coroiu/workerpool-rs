use wasm_bindgen::prelude::*;
use workerpool::{
    task::{TaskRequest, TaskResponse},
    WorkerBackend,
};

#[wasm_bindgen(module = "/src/wasm_workerpool/web_worker_backend.js")]
extern "C" {
    // types created in JS are not Send
    // type JsWorker;

    fn spawn_worker() -> usize;

    // #[wasm_bindgen(constructor)]
    // fn new() -> JsWo;

    // #[wasm_bindgen(method, getter)]
    // fn number(this: &MyClass) -> u32;
    // #[wasm_bindgen(method, setter)]
    // fn set_number(this: &MyClass, number: u32) -> MyClass;
    // #[wasm_bindgen(method)]
    // fn render(this: &MyClass) -> String;
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
        todo!()
    }
}
