use workerpool::{backends::SameThreadBackend, global::RoutineRegistry, Routine, WorkerPool};

pub async fn global_same_thread_test() {
    // println!("Running test: Backend = same-thread; Registration = global");

    // // let mut registry = RoutineRegistry::new();
    // // let routine = Routine::from(crate::routines::sleep_then_add);
    // // registry.register_routine(routine);

    // let backend = SameThreadBackend::new(&registry);
    // let pool = WorkerPool::new(backend, 4);

    // let args = vec![3, 1, 1];
    // let routine = Routine::from(crate::routines::sleep_then_add);

    // println!("Executing sleep_then_add([3, 1, 1])");
    // let result = pool.execute_routine(&routine, args).await;

    // println!("Result: {:?}", result);
}
