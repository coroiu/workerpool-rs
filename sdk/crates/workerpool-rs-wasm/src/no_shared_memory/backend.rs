use serde::{de::DeserializeOwned, Serialize};
use wasm_bindgen::prelude::*;
use web_sys::MessageEvent;
use workerpool_rs::WorkerBackend;

pub struct WasmWorkerBackend {
    worker_url: String,
}

impl WasmWorkerBackend {
    pub fn new(worker_url: String) -> Self {
        Self { worker_url }
    }
}

impl WorkerBackend for WasmWorkerBackend {
    type Worker = ();

    #[cfg(not(feature = "shared-memory"))]
    async fn exec<I, O>(&self, input: I, function: fn(input: I) -> O) -> O
    where
        I: Send + Serialize + DeserializeOwned + 'static,
        O: Send + Serialize + DeserializeOwned + 'static,
    {
        use web_sys::{WorkerOptions, WorkerType};

        use crate::js_channel::JsChannel;

        let options = WorkerOptions::new();
        options.set_type(WorkerType::Module);
        let worker = web_sys::Worker::new_with_options(self.worker_url.as_str(), &options).unwrap();

        let channel = JsChannel::<I, O>::connect_to(&worker);

        // pointer to function
        let function_ptr = function as u32;
        web_sys::console::log_1(&JsValue::from_str(
            format!("Backend function pointer: {:?}", function_ptr).as_str(),
        ));

        // function(input)
        todo!()
    }
}
