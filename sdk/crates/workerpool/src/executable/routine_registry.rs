use super::routine::Routine;

pub struct RoutineRegistry<A, R>
where
    A: Send + 'static,
    R: Send + 'static,
{
    routines: Vec<Routine<A, R>>,
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

    pub fn register_routine(&mut self, routine: Routine<A, R>) {
        self.routines.push(routine);
    }

    pub fn get_routine(&self, name: &str) -> Option<&Routine<A, R>> {
        self.routines.iter().find(|r| r.name() == name)
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
        let routine = Routine::new(add);
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
