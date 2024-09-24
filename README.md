# workerpool-rs

Early example of a basic task-delegation framework for offloading CPU intensive tasks to threads and web workers.

## Getting started: How workerpool-rs is used

### Using the global registry with automatically registered routines

Note: WASM doesn't support the `ctor` crate, so routines must be registered manually.
You can still use the global registry to store routines, but you can't use the `workerpool-macro` helpers.

```rust
// In /src/routines.rs
// Define a routine
#[global_routine]
pub fn sleep_then_add(args: Input) -> Output {
    // Do something
}

// In /src/main.rs
pub fn main() {
    // Create a thread backend
    // `SameThreadBackend` is a basic backend that runs tasks on the main thread which is included in the workerpool-rs crate.
    // For more advanced backends, implement your own backend by implementing the `WorkerBackend` trait.
    let backend = SameThreadBackend::new(registry);

    // Create a worker pool using the backend, and specify the number of workers to use
    let pool = WorkerPool::new(backend, 4);

    // Execute a function with the worker pool
    let args = "<your args here>";
    let result = pool
        .execute_function(crate::routines::sleep_then_add, args)
        .await;
}
```

### Using local registry with manually registered routines

```rust
// In /src/routines.rs
// Define a routine
pub fn sleep_then_add(args: Input) -> Output {
    // Do something
}

// In /src/main.rs
pub fn main() {
    // Create a registry to store executable routines
    let mut registry = RoutineRegistry::new();

    // Register the routine with the registry
    registry.register_routine(Routine::from(crate::routines::sleep_then_add));

    // Create a thread backend
    // `SameThreadBackend` is a basic backend that runs tasks on the main thread which is included in the workerpool-rs crate.
    // For more advanced backends, implement your own backend by implementing the `WorkerBackend` trait.
    let backend = SameThreadBackend::new(registry);

    // Create a worker pool using the backend, and specify the number of workers to use
    let pool = WorkerPool::new(backend, 4);

    // Execute a function with the worker pool
    let args = "<your args here>";
    let result = pool
        .execute_function(crate::routines::sleep_then_add, args)
        .await;
}
```

## Run

```bash
# Install dependencies
npm install

# Start the development server
npm start
```
