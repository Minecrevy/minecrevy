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
//! #[derive(McRead)]
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

#![cfg_attr(feature = "glam", feature(maybe_uninit_uninit_array))]
#![cfg_attr(feature = "glam", feature(maybe_uninit_array_assume_init))]
#![forbid(missing_docs)]

#[cfg(feature = "glam")]
pub use self::glam::*;
#[cfg(feature = "hematite-nbt")]
pub use self::nbt::*;
pub use self::options::*;
pub use self::read::*;
pub use self::write::*;

#[cfg(feature = "glam")]
mod glam;
#[cfg(feature = "hematite-nbt")]
mod nbt;
mod options;
mod read;
mod write;

#[derive(McRead, McWrite)]
struct Test {
    #[mcio(varint)]
    len: i32,
    #[mcio(max_len = 16)]
    name: String,
    b: u8,
    #[mcio(length = "varint", inner::max_len = 16)]
    values: Vec<String>,
}
