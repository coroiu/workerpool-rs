use workerpool::{
    global::{execute_routine, register_routine},
    Routine,
};
use workerpool_macro::global_routine;

#[global_routine]
fn add(args: Vec<u8>) -> Vec<u8> {
    args.iter().map(|x| x + 1).collect()
}

// Function without the global_routine attribute
fn add2(args: Vec<u8>) -> Vec<u8> {
    args.iter().map(|x| x + 2).collect()
}

#[test]
fn should_register_and_execute_global_routine() {
    let routine_name = Routine::new(add).name().to_owned();

    let result = execute_routine(routine_name.as_str(), vec![1, 2, 3]);

    assert!(result.is_some());
    assert_eq!(result.unwrap(), vec![2, 3, 4]);
}

#[test]
fn should_not_register_and_execute_function_without_attribute() {
    let routine_name = Routine::new(add2).name().to_owned();

    let result = execute_routine(routine_name.as_str(), vec![1, 2, 3]);

    assert!(result.is_none());
}
