use crate::worker_backend::WorkerBackend;

pub struct WorkerPool<B: WorkerBackend> {
    workers: Vec<B>, // A pool of workers that implement the WorkerBackend trait
}

impl<B: WorkerBackend> WorkerPool<B> {
    pub fn new(size: usize) -> Self {
        let mut workers = Vec::with_capacity(size);
        for _ in 0..size {
            workers.push(B::spawn_worker()); // Create and add workers to the pool
        }
        WorkerPool { workers }
    }

    pub async fn execute_task(&mut self, task: B::Task) -> Result<B::Output, String> {
        let worker = self.select_worker();
        worker.execute_task(task).await // Send the task and await the result
    }

    fn select_worker(&mut self) -> &B {
        // Return the first worker as an example (could be improved with load-balancing)
        &self.workers[0]
    }
}
