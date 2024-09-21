use crate::task::{TaskRequest, TaskResponse};

pub trait WorkerBackend {
    type Worker; // The type of worker created by this backend
    type Input; // The type of inputs used by the routines executed by the worker
    type Output; // The type of result returned after execution
    type Error; // The type of error returned after execution

    // Spawns a new worker instance
    fn spawn_worker() -> Self::Worker;

    // Sends a task to the worker and waits for the result asynchronously
    fn execute_task(
        &self,
        worker: &Self::Worker,
        request: TaskRequest<Self::Input>,
    ) -> impl std::future::Future<Output = TaskResponse<Self::Output, Self::Error>> + Send;
}
