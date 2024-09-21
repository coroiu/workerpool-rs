pub trait Worker {
    type Task;
    type Output;
    type Error;

    async fn execute_task(&self, task: Self::Task) -> Result<Self::Output, Self::Error>;
}
