#[cfg(not(feature = "shared-memory"))]
mod no_shared_memory;

#[cfg(not(feature = "shared-memory"))]
pub use no_shared_memory::WasmWorkerBackend;
