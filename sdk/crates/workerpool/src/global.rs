use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};

pub use crate::routines::routine::Routine;
use crate::routines::routine_registry::{RoutineRegistry, RoutineRegistryTrait};

type GlobalInput = Vec<u8>;
type GlobalOutput = Vec<u8>;
type GlobalError = (); // TODO: Implement error handling

lazy_static! {
    static ref ROUTINE_REGISTRY: Arc<Mutex<RoutineRegistry<GlobalInput, GlobalOutput, GlobalError>>> =
        Arc::new(Mutex::new(RoutineRegistry::new()));
}

pub fn get_registry() -> Arc<Mutex<RoutineRegistry<GlobalInput, GlobalOutput, GlobalError>>> {
    ROUTINE_REGISTRY.clone()
}

pub fn register_routine(routine: Routine<GlobalInput, GlobalOutput, GlobalError>) {
    let mut registry = ROUTINE_REGISTRY.lock().unwrap();
    registry.register_routine(routine);
}

// #[cfg(test)]
// mod test {
//     use super::*;

//     // #[global_routine]
//     fn add(args: Vec<u8>) -> Result<Vec<u8>, ()> {
//         Ok(args.iter().map(|x| x + 1).collect())
//     }

//     // #[test]
//     // fn should_execute_global_routine() {
//     //     let routine_name = Routine::new(add).name().to_owned();
//     //     register_routine(Routine::new(add));

//     //     let result = execute_routine(routine_name.as_str(), vec![1, 2, 3]);

//     //     assert!(result.is_some());
//     //     assert_eq!(result.unwrap(), Ok(vec![2, 3, 4]));
//     // }
// }
