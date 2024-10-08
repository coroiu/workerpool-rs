use atomic_counter::{AtomicCounter, ConsistentCounter};

use crate::{
    routines::routine_registry::ExecuteRoutineError,
    task::{TaskRequest, TaskResponse},
    workers::worker_backend::WorkerBackend,
    Routine,
};

pub struct WorkerPool<B: WorkerBackend> {
    request_counter: ConsistentCounter,
    backend: B,              // The backend that the workers will use to execute tasks
    workers: Vec<B::Worker>, // A pool of workers created by the backend
}

impl<B: WorkerBackend> WorkerPool<B>
where
    B::Input: Send + 'static,
    B::Output: Send + 'static,
    B::Error: Send + 'static,
{
    pub fn new(backend: B, size: usize) -> Self {
        let mut workers = Vec::with_capacity(size);
        for _ in 0..size {
            workers.push(B::spawn_worker()); // Create and add workers to the pool
        }
        WorkerPool {
            request_counter: ConsistentCounter::new(0),
            backend,
            workers,
        }
    }

    pub async fn execute_task(
        &self,
        request: TaskRequest<B::Input>,
        // TODO: We probably want to flatten the error types here a bit
    ) -> Result<TaskResponse<B::Output, ExecuteRoutineError<B::Error>>, B::BackendError> {
        let worker = self.select_worker();
        self.backend.execute_task(&worker, request).await // Send the task and await the result
    }

    pub async fn execute_routine(
        &self,
        routine: &Routine<B::Input, B::Output, B::Error>,
        args: B::Input,
    ) -> Result<TaskResponse<B::Output, ExecuteRoutineError<B::Error>>, B::BackendError> {
        let request_id = self.request_counter.inc();
        let request = TaskRequest::new(request_id, routine, args);
        self.execute_task(request).await
    }

    pub async fn execute_function<
        F: Fn(B::Input) -> Result<B::Output, B::Error> + Send + Sync + 'static,
    >(
        &self,
        function: F,
        args: B::Input,
    ) -> Result<TaskResponse<B::Output, ExecuteRoutineError<B::Error>>, B::BackendError> {
        let routine = Routine::new(function);
        self.execute_routine(&routine, args).await
    }

    fn select_worker(&self) -> &B::Worker {
        // Return the first worker as an example (could be improved with load-balancing)
        &self.workers[0]
    }
}
