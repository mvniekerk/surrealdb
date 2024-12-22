#[cfg(not(all(target_arch = "wasm32", not(target_os = "wasi"))))]
pub use spawn::spawn;
pub use try_join_all_buffered::try_join_all_buffered;

mod spawn;
mod try_join_all_buffered;
