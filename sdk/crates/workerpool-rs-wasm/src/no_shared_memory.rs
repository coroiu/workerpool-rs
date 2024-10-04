use serde::{de::DeserializeOwned, Serialize};
use workerpool_rs::WorkerBackend;

pub struct WasmWorkerBackend;

impl WorkerBackend for WasmWorkerBackend {
    type Worker = ();

    #[cfg(not(feature = "shared-memory"))]
    async fn exec<I, O>(&self, input: I, function: fn(input: I) -> O) -> O
    where
        I: Send + Serialize + DeserializeOwned,
        O: Send + Serialize + DeserializeOwned,
    {
        let worker = web_sys::Worker::new("./worker.js");

        todo!()
    }
}
