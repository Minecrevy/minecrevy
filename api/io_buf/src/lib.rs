#![doc = include_str!("../README.md")]

#![forbid(missing_docs)]

#[cfg(feature = "blocking")]
pub use self::blocking::*;
pub use self::packet::*;
#[cfg(feature = "async-tokio")]
pub use self::tokio::*;

#[cfg(feature = "blocking")]
mod blocking;
mod packet;
#[cfg(feature = "async-tokio")]
mod tokio;
pub mod util;
