#[path = "pb/antelope.atomicmarket.v1.rs"]
#[allow(dead_code)]
pub mod atomicmarket;
pub use self::atomicmarket::*;

mod abi;
mod maps;
mod sinks;