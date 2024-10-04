use serde::{de::DeserializeOwned, Serialize};

pub trait WorkerBackend {
    type Worker;

    #[cfg(not(feature = "shared-memory"))]
    async fn exec<I, O>(&self, input: I, function: fn(input: I) -> O) -> O
    where
        I: Send + Serialize + DeserializeOwned,
        O: Send + Serialize + DeserializeOwned;
}
