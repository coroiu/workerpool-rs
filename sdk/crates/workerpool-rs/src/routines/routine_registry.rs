use std::sync::{Arc, Mutex};

use thiserror::Error;

use super::routine::Routine;

#[derive(Error, Debug)]
pub enum ExecuteRoutineError<E> {
    #[error("No routine named `{0}` was found")]
    RoutineNotFound(String),

    #[error("Routine was successfully executed but returned an error: {0}")]
    VendorError(#[from] E),
}

pub struct RoutineRegistry<A, R, E>
where
    A: Send + 'static,
    R: Send + 'static,
    E: Send + 'static,
{
    routines: Vec<Routine<A, R, E>>,
}

impl<A, R, E> RoutineRegistry<A, R, E>
where
    A: Send + 'static,
    R: Send + 'static,
    E: Send + 'static,
{
    pub fn new() -> Self {
        RoutineRegistry {
            routines: Vec::new(),
        }
    }
}

pub trait RoutineRegistryTrait: Send {
    type Input: Send + 'static;
    type Output: Send + 'static;
    type Error: Send + 'static;

    fn register_routine(&mut self, routine: Routine<Self::Input, Self::Output, Self::Error>);
    fn get_routine(&self, name: &str) -> Option<Routine<Self::Input, Self::Output, Self::Error>>;
    fn execute_routine(
        &self,
        name: &str,
        args: Self::Input,
    ) -> Result<Self::Output, ExecuteRoutineError<Self::Error>>;
    fn execute_function<
        F: Fn(Self::Input) -> Result<Self::Output, Self::Error> + Send + Sync + 'static,
    >(
        &self,
        function: F,
        args: Self::Input,
    ) -> Result<Self::Output, ExecuteRoutineError<Self::Error>> {
        let routine = Routine::new(function);
        Ok(routine.execute(args)?)
    }
}

impl<A, R, E> RoutineRegistryTrait for RoutineRegistry<A, R, E>
where
    A: Send + 'static,
    R: Send + 'static,
    E: Send + 'static,
{
    type Input = A;
    type Output = R;
    type Error = E;

    fn register_routine(&mut self, routine: Routine<A, R, E>) {
        self.routines.push(routine);
    }

    fn get_routine(&self, name: &str) -> Option<Routine<A, R, E>> {
        self.routines.iter().find(|r| r.name() == name).cloned()
    }

    fn execute_routine(&self, name: &str, args: A) -> Result<R, ExecuteRoutineError<E>> {
        let routine = self
            .get_routine(name)
            .ok_or(ExecuteRoutineError::RoutineNotFound(name.to_owned()))?;
        Ok(routine.execute(args)?)
    }
}

impl<A, R, E> RoutineRegistryTrait for Arc<Mutex<RoutineRegistry<A, R, E>>>
where
    A: Send + 'static,
    R: Send + 'static,
    E: Send + 'static,
{
    type Input = A;
    type Output = R;
    type Error = E;

    fn register_routine(&mut self, routine: Routine<A, R, E>) {
        self.lock().unwrap().register_routine(routine);
    }

    fn get_routine(&self, name: &str) -> Option<Routine<A, R, E>> {
        self.lock().unwrap().get_routine(name)
    }

    fn execute_routine(&self, name: &str, args: A) -> Result<R, ExecuteRoutineError<E>> {
        self.lock().unwrap().execute_routine(name, args)
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
    #[derive(Debug, PartialEq)]
    struct Error;

    fn add(args: Input) -> Result<Output, Error> {
        Ok(args.a + args.b)
    }

    #[test]
    fn should_return_routine() {
        let mut registry = RoutineRegistry::<Input, Output, Error>::new();
        let routine = Routine::new(add);
        let routine_name = routine.name().to_owned();
        registry.register_routine(routine);

        let result = registry.get_routine(routine_name.as_str());
        assert!(result.is_some());
    }

    #[test]
    fn should_not_return_unknown_routine() {
        let registry = RoutineRegistry::<Input, Output, Error>::new();
        let result = registry.get_routine("unknown");
        assert!(result.is_none());
    }

    #[test]
    fn should_execute_routine() {
        let mut registry = RoutineRegistry::<Input, Output, Error>::new();
        let routine = Routine::new(add);
        let routine_name = routine.name().to_owned();
        registry.register_routine(routine);

        let result = registry.execute_routine(routine_name.as_str(), Input { a: 2, b: 2 });
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 4);
    }

    #[test]
    fn should_return_error_on_unknown_routine() {
        let registry = RoutineRegistry::<Input, Output, Error>::new();
        let result = registry.execute_routine("unknown", Input { a: 2, b: 2 });
        assert!(matches!(
            result.unwrap_err(),
            ExecuteRoutineError::RoutineNotFound(_)
        ));
    }

    #[test]
    fn should_return_error_on_execute() {
        let mut registry = RoutineRegistry::<Input, Output, Error>::new();
        let routine = Routine::new(|_| Err(Error));
        let routine_name = routine.name().to_owned();
        registry.register_routine(routine);

        let result = registry.execute_routine(routine_name.as_str(), Input { a: 2, b: 2 });
        assert!(matches!(
            result.unwrap_err(),
            ExecuteRoutineError::VendorError(Error)
        ));
    }
}
