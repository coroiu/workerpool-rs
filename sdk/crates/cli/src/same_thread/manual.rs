use workerpool_rs::{
    backends::SameThreadBackend,
    routines::routine_registry::{RoutineRegistry, RoutineRegistryTrait},
    Routine, WorkerPool,
};

pub async fn manual_same_thread_test() {
    println!("Running test: Backend = same-thread; Registration = manual");

    let mut registry = RoutineRegistry::new();
    registry.register_routine(Routine::from(crate::routines::sleep_then_add));

    let backend = SameThreadBackend::new(registry);
    let pool = WorkerPool::new(backend, 4);

    let args = vec![3, 1, 1];
    println!("Executing sleep_then_add([3, 1, 1])");
    let result = pool
        .execute_function(crate::routines::sleep_then_add, args)
        .await;

    println!("Result: {:?}", result);
}
