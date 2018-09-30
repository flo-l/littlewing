#[cfg(target="wasm")]
mod wasm;
#[cfg(target="wasm")]
pub use self::wasm::precise_time_s;

#[cfg(not(target="wasm"))]
mod os;
#[cfg(not(target="wasm"))]
pub use self::os::precise_time_s;
