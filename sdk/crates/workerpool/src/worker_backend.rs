pub trait WorkerBackend {
    type Task; // The type of task to be executed
    type Output; // The type of result returned after execution

    // Spawns a new worker instance
    fn spawn_worker() -> Self;

    // Sends a task to the worker and waits for the result asynchronously
    fn execute_task(
        &self,
        task: Self::Task,
    ) -> impl std::future::Future<Output = Result<Self::Output, String>> + Send;
}

pub struct SingleThreadedWorkerBackend;

impl WorkerBackend for SingleThreadedWorkerBackend {
    type Task = Box<dyn FnOnce() -> i32 + Send>; // A task that returns an integer
    type Output = i32; // The result of the task

    fn spawn_worker() -> Self {
        SingleThreadedWorkerBackend
    }

    fn execute_task(
        &self,
        task: Self::Task,
    ) -> impl std::future::Future<Output = Result<Self::Output, String>> + Send {
        async move {
            let result = task();
            Ok(result)
        }
    }
}
