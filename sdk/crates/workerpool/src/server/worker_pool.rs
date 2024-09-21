use atomic_counter::{AtomicCounter, ConsistentCounter};

use crate::{
    server::worker_backend::WorkerBackend,
    task::{TaskRequest, TaskResponse},
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
    ) -> TaskResponse<B::Output, B::Error> {
        let worker = self.select_worker();
        self.backend.execute_task(&worker, request).await // Send the task and await the result
    }

    pub async fn execute_routine(
        &self,
        routine: &Routine<B::Input, B::Output, B::Error>,
        args: B::Input,
    ) -> Result<B::Output, B::Error> {
        let request_id = self.request_counter.inc();
        let request = TaskRequest::new(request_id, routine, args);
        let result = self.execute_task(request).await;
        result.result
    }

    fn select_worker(&self) -> &B::Worker {
        // Return the first worker as an example (could be improved with load-balancing)
        &self.workers[0]
    }
}
