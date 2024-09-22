use lazy_static::lazy_static;
use std::sync::Mutex;

pub use crate::{executable::routine_registry::RoutineRegistry, Routine};

type GlobalInput = Vec<u8>;
type GlobalOutput = Vec<u8>;
type GlobalError = (); // TODO: Implement error handling

lazy_static! {
    static ref ROUTINE_REGISTRY: Mutex<RoutineRegistry<GlobalInput, GlobalOutput, GlobalError>> =
        Mutex::new(RoutineRegistry::new());
}

pub fn register_routine(routine: Routine<GlobalInput, GlobalOutput, GlobalError>) {
    let mut registry = ROUTINE_REGISTRY.lock().unwrap();
    registry.register_routine(routine);
}

pub fn execute_routine(name: &str, args: GlobalInput) -> Option<Result<GlobalOutput, GlobalError>> {
    let registry = ROUTINE_REGISTRY.lock().unwrap();
    todo!()
    // registry.execute_routine(name, args)
}

#[cfg(test)]
mod test {
    use super::*;

    // #[global_routine]
    fn add(args: Vec<u8>) -> Result<Vec<u8>, ()> {
        Ok(args.iter().map(|x| x + 1).collect())
    }

    #[test]
    fn should_execute_global_routine() {
        let routine_name = Routine::new(add).name().to_owned();
        register_routine(Routine::new(add));

        let result = execute_routine(routine_name.as_str(), vec![1, 2, 3]);

        assert!(result.is_some());
        assert_eq!(result.unwrap(), Ok(vec![2, 3, 4]));
    }
}
