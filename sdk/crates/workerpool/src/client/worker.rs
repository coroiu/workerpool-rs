use crate::task::TaskRequest;

pub trait Worker {
    type Input;
    type Output;
    type Error;

    async fn execute_task(
        &self,
        task: TaskRequest<Self::Input>,
    ) -> Result<Self::Output, Self::Error>;
}
