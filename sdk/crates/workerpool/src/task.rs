use crate::Routine;

pub struct TaskRequest<A> {
    pub request_id: u32,
    pub routine_name: String,
    pub args: A,
}

pub struct TaskResponse<R, E> {
    pub request_id: u32,
    pub result: Result<R, E>,
}

impl<A> TaskRequest<A>
where
    A: Send + 'static,
{
    pub fn new<R, E>(request_id: u32, routine: Routine<A, R, E>, args: A) -> Self {
        TaskRequest {
            request_id,
            routine_name: routine.name().to_owned(),
            args,
        }
    }
}

impl<R, E> TaskResponse<R, E>
where
    R: Send + 'static,
    E: Send + 'static,
{
    pub fn new(request_id: u32, result: Result<R, E>) -> Self {
        TaskResponse { request_id, result }
    }
}
