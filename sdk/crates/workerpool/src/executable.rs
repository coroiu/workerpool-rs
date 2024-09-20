pub struct Task {
    routine: String,
    args: Vec<u8>,
}

pub struct RoutineRegistry<A, R>
where
    A: Send + 'static,
    R: Send + 'static,
{
    routines: Vec<WorkerRoutine<A, R>>,
}

impl<A, R> RoutineRegistry<A, R>
where
    A: Send + 'static,
    R: Send + 'static,
{
    pub fn new() -> Self {
        RoutineRegistry {
            routines: Vec::new(),
        }
    }

    pub fn register_routine(&mut self, routine: WorkerRoutine<A, R>) {
        self.routines.push(routine);
    }

    pub fn get_routine(&self, name: &str) -> Option<&WorkerRoutine<A, R>> {
        self.routines.iter().find(|r| r.name() == name)
    }
}

/// A worker routine that can be executed by a worker backend.
pub struct WorkerRoutine<A, R> {
    name: String,
    function: Box<dyn Fn(A) -> R + 'static>,
}

impl<A, R> WorkerRoutine<A, R>
where
    A: Send + 'static,
    R: Send + 'static,
{
    pub fn new<F>(function: F) -> Self
    where
        F: Fn(A) -> R + 'static,
    {
        WorkerRoutine {
            name: std::any::type_name::<F>().to_owned(),
            function: Box::new(function),
        }
    }

    /// Returns the name of the worker routine.
    pub fn name(&self) -> &str {
        &self.name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Input {
        a: i32,
        b: i32,
    }
    type Output = i32;

    fn add(args: Input) -> Output {
        args.a + args.b
    }

    #[test]
    fn routine_registry_should_return_routine() {
        let mut registry = RoutineRegistry::<Input, Output>::new();
        let routine = WorkerRoutine::new(add);
        registry.register_routine(routine);

        let result = registry.get_routine("workerpool::executable::tests::add");
        assert!(result.is_some());
    }

    #[test]
    fn routine_registry_should_not_return_unknown_routine() {
        let registry = RoutineRegistry::<Input, Output>::new();
        let result = registry.get_routine("unknown");
        assert!(result.is_none());
    }
}
