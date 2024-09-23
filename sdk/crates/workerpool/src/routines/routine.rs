use std::sync::Arc;

/// A worker routine that can be executed by a worker backend.
/// It is intended to be a "smart pointer" to a function that can be executed by a worker.
pub struct Routine<A, R, E> {
    // TODO: consider Cow<'static, str>
    name: String,
    // TODO: this should be fn(A) -> Result<R, E>
    function: Arc<Box<dyn Fn(A) -> Result<R, E> + Send + Sync + 'static>>,
}

// Manual implementation of Clone because the default derive implementation
// would require A, R, and E to be Clone.
impl<A, R, E> Clone for Routine<A, R, E> {
    fn clone(&self) -> Self {
        Routine {
            name: self.name.clone(),
            function: self.function.clone(),
        }
    }
}

impl<A, R, E> Routine<A, R, E> {
    /// Returns the name of the worker routine.
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl<A, R, E> Routine<A, R, E>
where
    A: Send + 'static,
    R: Send + 'static,
    E: Send + 'static,
{
    pub fn new<F>(function: F) -> Self
    where
        F: Fn(A) -> Result<R, E> + Send + Sync + 'static,
    {
        Routine {
            name: std::any::type_name::<F>().to_owned(),
            function: Arc::new(Box::new(function)),
        }
    }

    /// Executes the worker routine with the given arguments.
    pub fn execute(&self, args: A) -> Result<R, E> {
        (self.function)(args)
    }
}

impl<A, R, E, F> From<F> for Routine<A, R, E>
where
    A: Send + 'static,
    R: Send + 'static,
    E: Send + 'static,
    F: Fn(A) -> Result<R, E> + Send + Sync + 'static,
{
    fn from(function: F) -> Self {
        Routine::new(function)
    }
}
