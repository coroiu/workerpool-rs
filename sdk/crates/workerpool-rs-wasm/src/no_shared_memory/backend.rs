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
        I: Send + Serialize + DeserializeOwned,
        O: Send + Serialize + DeserializeOwned,
    {
        use web_sys::{WorkerOptions, WorkerType};

        let options = WorkerOptions::new();
        options.set_type(WorkerType::Module);
        let worker = web_sys::Worker::new_with_options(self.worker_url.as_str(), &options).unwrap();

        let listener = Closure::<dyn FnMut(_)>::new(move |event: MessageEvent| {
            web_sys::console::log_1(&JsValue::from_str(&format!(
                "Backend received message: {:?}",
                event.data()
            )));
        });
        worker
            .add_event_listener_with_callback("message", listener.as_ref().unchecked_ref())
            .unwrap();
        // TODO: FIX MEMORY LEAK
        listener.forget();

        // worker
        //     .post_message(&JsValue::from_serde(&input).unwrap())
        //     .unwrap();
        worker
            .post_message(&JsValue::from_str("Hello from backend"))
            .unwrap();

        function(input)
        // todo!()
    }
}
