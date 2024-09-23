use workerpool::{
    routines::routine_registry::{ExecuteRoutineError, RoutineRegistryTrait},
    global::{get_registry, register_routine},
    Routine,
};
use workerpool_macro::global_routine;

#[global_routine]
fn add(args: Vec<u8>) -> Result<Vec<u8>, ()> {
    Ok(args.iter().map(|x| x + 1).collect())
}

// Function without the global_routine attribute
fn add2(args: Vec<u8>) -> Result<Vec<u8>, ()> {
    Ok(args.iter().map(|x| x + 2).collect())
}

#[test]
fn should_register_and_execute_global_routine() {
    let routine_name = Routine::new(add).name().to_owned();

    let result = get_registry().execute_routine(routine_name.as_str(), vec![1, 2, 3]);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), vec![2, 3, 4]);
}

#[test]
fn should_not_register_and_execute_function_without_attribute() {
    let routine_name = Routine::new(add2).name().to_owned();

    let result = get_registry().execute_routine(routine_name.as_str(), vec![1, 2, 3]);

    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        ExecuteRoutineError::RoutineNotFound(_)
    ));
}
