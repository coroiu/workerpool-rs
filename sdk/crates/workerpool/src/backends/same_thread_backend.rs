use crate::{
    executable::{self, routine_registry::ExecuteRoutineError},
    global::RoutineRegistry,
    task::{TaskRequest, TaskResponse},
    WorkerBackend,
};

// An included implementation using the existing thread as a worker backend
pub struct SameThreadBackend<'a, A, R, E>
where
    A: Send + 'static,
    R: Send + 'static,
    E: Send + 'static,
{
    registry: &'a executable::routine_registry::RoutineRegistry<A, R, E>,
}

impl<'a, A, R, E> SameThreadBackend<'a, A, R, E>
where
    A: Send + 'static,
    R: Send + 'static,
    E: Send + 'static,
{
    // Usually backends don't use a registry since that's the job of the client worker
    // but since backed is using the same thread as the server, it needs to know the routines.
    pub fn new(registry: &'a RoutineRegistry<A, R, E>) -> Self {
        Self { registry }
    }
}

impl<'a, A, R, E> WorkerBackend for SameThreadBackend<'a, A, R, E>
where
    A: Send + 'static,
    R: Send + 'static,
    E: Send + 'static,
{
    type Worker = ();
    type Input = A;
    type Output = R;
    type BackendError = (); // This backend cannot itself fail
    type Error = E;

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
