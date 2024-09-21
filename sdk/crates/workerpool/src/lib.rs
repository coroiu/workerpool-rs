pub mod backends;
mod client;
mod executable;
pub mod global;
mod server;
mod task;

pub use executable::routine::Routine;
pub use server::worker_backend::WorkerBackend;
pub use server::worker_pool::WorkerPool;

#[cfg(test)]
mod tests {
    // use super::*;

    // #[test]
    // fn it_works() {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    // }
}
