#[cfg(target_arch="wasm32")]
mod wasm;
#[cfg(target_arch="wasm32")]
pub use self::wasm::precise_time_s;

#[cfg(not(target_arch="wasm32"))]
mod os;
#[cfg(not(target_arch="wasm32"))]
pub use self::os::precise_time_s;
