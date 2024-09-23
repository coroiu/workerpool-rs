pub mod backends;
pub mod global;
pub mod routines;
mod task;
mod workers;

pub use routines::routine::Routine;
pub use workers::worker_backend::WorkerBackend;
pub use workers::worker_pool::WorkerPool;

#[cfg(test)]
mod tests {
    // use super::*;

    // #[test]
    // fn it_works() {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    // }
}
