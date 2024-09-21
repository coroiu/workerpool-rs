mod executable;
pub mod global;
mod task;
mod worker;
mod worker_backend;
mod worker_pool;

pub use executable::routine::Routine;
pub use worker_backend::WorkerBackend;
pub use worker_pool::WorkerPool;

#[cfg(test)]
mod tests {
    // use super::*;

    // #[test]
    // fn it_works() {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    // }
}
