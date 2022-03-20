//! A library for structurally encoding and decoding Minecraft packets and data types.
//!
//! While this library may be useful for general structural I/O,
//! all of the configuration defaults are geared towards working with the Minecraft protocol.
//!
//! # Reading (structurally)
//!
//! Reading from bytes to construct a data type begins with the foundational [`McRead`] trait.
//! All of the primitive types in the Rust language are already implemented, including [`String`].
//!
//! Now all you have to do (usually), is to start deriving:
//!
//! ```rust
//! #[derive(minecrevy_io_str::McRead)]
//! pub struct HandshakePacket {
//!     #[mcio(varint)]
//!     pub version: i32,
//!     #[mcio(max_len = 255)]
//!     pub address: String,
//!     pub port: u16,
//!     #[mcio(varint)]
//!     pub next: i32,
//! }
//! ```
//!
//! # Writing (structurally)
//!
//! Writing a data type by converting it to bytes begins with the foundational [`McWrite`] trait.
//!
//! Making a [`McWrite`]able type is as simple as reading;
//! just swap `#[derive(McRead)]` with `#[derive(McWrite)]` and you're good to go.
//! Or maybe you want to be able to read AND write a type! Just derive both then!

#![feature(maybe_uninit_uninit_array)]
#![feature(maybe_uninit_array_assume_init)]
#![warn(missing_docs)]

pub use self::bitset::*;
#[cfg(feature = "enumflags2")]
pub use self::enumflags2::*;
#[cfg(feature = "glam")]
pub use self::glam::*;
pub use self::macros::*;
#[cfg(feature = "nbt")]
pub use self::nbt::*;
pub use self::options::*;
pub use self::read::*;
pub use self::write::*;

mod bitset;
#[cfg(feature = "enumflags2")]
mod enumflags2;
#[cfg(feature = "glam")]
mod glam;
mod macros;
#[cfg(feature = "nbt")]
mod nbt;
mod options;
mod read;
mod write;

#[cfg(test)]
mod test;
