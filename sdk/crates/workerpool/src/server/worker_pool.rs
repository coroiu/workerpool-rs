use crate::{
    server::worker_backend::WorkerBackend,
    task::{TaskRequest, TaskResponse},
};

pub struct WorkerPool<B: WorkerBackend> {
    backend: B,              // The backend that the workers will use to execute tasks
    workers: Vec<B::Worker>, // A pool of workers created by the backend
}

impl<B: WorkerBackend> WorkerPool<B> {
    pub fn new(backend: B, size: usize) -> Self {
        let mut workers = Vec::with_capacity(size);
        for _ in 0..size {
            workers.push(B::spawn_worker()); // Create and add workers to the pool
        }
        WorkerPool { backend, workers }
    }

    pub async fn execute_task(
        &self,
        request: TaskRequest<B::Input>,
    ) -> TaskResponse<B::Output, B::Error> {
        let worker = self.select_worker();
        self.backend.execute_task(&worker, request).await // Send the task and await the result
    }

    fn select_worker(&self) -> &B::Worker {
        // Return the first worker as an example (could be improved with load-balancing)
        &self.workers[0]
    }
}
