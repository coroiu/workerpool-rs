use workerpool::{
    task::{TaskRequest, TaskResponse},
    WorkerBackend,
};

struct WebWorkerBackend;

impl WebWorkerBackend {
    pub fn new() -> Self {
        WebWorkerBackend
    }
}

impl WorkerBackend for WebWorkerBackend {
    type Worker = ();
    type Input = Vec<u8>;
    type Output = Vec<u8>;
    type BackendError = ();
    type Error = ();

    fn spawn_worker() -> Self::Worker {
        todo!()
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
