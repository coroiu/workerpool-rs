use workerpool_rs::{backends::SameThreadBackend, global, WorkerPool};

pub async fn global_same_thread_test() {
    println!("Running test: Backend = same-thread; Registration = global");

    let backend = SameThreadBackend::new(global::get_registry());
    let pool = WorkerPool::new(backend, 4);

    println!("Executing sleep_then_add([3, 1, 1])");
    let args = vec![3, 1, 1];
    let result = pool
        .execute_function(crate::routines::sleep_then_add, args)
        .await;

    println!("Result: {:?}", result);
}
