/// A worker routine that can be executed by a worker backend.
pub struct Routine<A, R> {
    name: String,
    function: Box<dyn Fn(A) -> R + Send + Sync + 'static>,
}

impl<A, R> Routine<A, R>
where
    A: Send + 'static,
    R: Send + 'static,
{
    pub fn new<F>(function: F) -> Self
    where
        F: Fn(A) -> R + Send + Sync + 'static,
    {
        Routine {
            name: std::any::type_name::<F>().to_owned(),
            function: Box::new(function),
        }
    }

    /// Returns the name of the worker routine.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Executes the worker routine with the given arguments.
    pub fn execute(&self, args: A) -> R {
        (self.function)(args)
    }
}
