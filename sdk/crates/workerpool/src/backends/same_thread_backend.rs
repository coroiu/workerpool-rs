use crate::{
    executable,
    global::RoutineRegistry,
    task::{TaskRequest, TaskResponse},
    WorkerBackend,
};

pub enum Error<RE> {
    RoutineNotFound,
    RoutineError(RE),
}

// An included implementation using the existing thread as a worker backend
pub struct SameThreadBackend<A, R, E>
where
    A: Send + 'static,
    R: Send + 'static,
    E: Send + 'static,
{
    registry: executable::routine_registry::RoutineRegistry<A, R, E>,
}

impl<A, R, E> SameThreadBackend<A, R, E>
where
    A: Send + 'static,
    R: Send + 'static,
    E: Send + 'static,
{
    // Usually backends don't use a registry since that's the job of the client worker
    // but since backed is using the same thread as the server, it needs to know the routines.
    pub fn new(registry: RoutineRegistry<A, R, E>) -> Self {
        Self { registry }
    }
}

impl<A, R, E> WorkerBackend for SameThreadBackend<A, R, E>
where
    A: Send + 'static,
    R: Send + 'static,
    E: Send + 'static,
{
    type Worker = ();
    type Input = A;
    type Output = R;
    type Error = Error<E>;

    fn spawn_worker() -> () {
        ()
    }

    async fn execute_task(
        &self,
        _worker: &(),
        request: TaskRequest<Self::Input>,
        // ) -> impl std::future::Future<Output = TaskResponse<Self::Output, Self::Error>> + Send {
    ) -> TaskResponse<Self::Output, Self::Error> {
        // async move {
        let result = self
            .registry
            .execute_routine(request.routine_name.as_str(), request.args);

        match result {
            Some(result) => TaskResponse::new(
                request.request_id,
                result.map_err(|e| Error::RoutineError(e)),
            ),
            None => TaskResponse::new(request.request_id, Err(Error::RoutineNotFound)),
        }
    }
}
