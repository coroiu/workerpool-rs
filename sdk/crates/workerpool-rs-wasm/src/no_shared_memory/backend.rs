// use super::worker;
use serde::{de::DeserializeOwned, Serialize};
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

        function(input)
        // todo!()
    }
}
