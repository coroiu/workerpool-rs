use crate::{
    executable::routine_registry::{ExecuteRoutineError, RoutineRegistryTrait},
    task::{TaskRequest, TaskResponse},
    WorkerBackend,
};

// An included implementation using the existing thread as a worker backend
pub struct SameThreadBackend<R>
where
    R: RoutineRegistryTrait,
{
    registry: R,
}

impl<R> SameThreadBackend<R>
where
    R: RoutineRegistryTrait,
{
    // Usually backends don't use a registry since that's the job of the client worker
    // but since backed is using the same thread as the server, it needs to know the routines.
    pub fn new(registry: R) -> Self {
        Self { registry }
    }
}

impl<R> WorkerBackend for SameThreadBackend<R>
where
    R: RoutineRegistryTrait + Sync,
    R::Input: Send + 'static,
    R::Output: Send + 'static,
    R::Error: Send + 'static,
{
    type Worker = ();
    type Input = R::Input;
    type Output = R::Output;
    type BackendError = (); // This backend cannot itself fail
    type Error = R::Error;

    fn spawn_worker() -> () {
        ()
    }

    async fn execute_task(
        &self,
        _worker: &(),
        request: TaskRequest<Self::Input>,
    ) -> Result<TaskResponse<Self::Output, ExecuteRoutineError<Self::Error>>, Self::BackendError>
    {
        let result = self
            .registry
            .execute_routine(request.routine_name.as_str(), request.args);

        Ok(TaskResponse::new(request.request_id, result))
    }
}
