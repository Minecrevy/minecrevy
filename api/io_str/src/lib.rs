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

use std::io::{self, Read, Write};

pub use minecrevy_io_str_derive::{McRead, McWrite};

pub use self::bitset::*;
pub use self::impls::*;
pub use self::options::*;

mod bitset;
mod impls;
mod options;

#[cfg(test)]
mod test;

/// The `McRead` trait allows for constructing data types from bytes.
///
/// Implementors of the `McRead` trait are typically packets or primitive data types.
pub trait McRead: Sized {
    /// The type of options available to configure the read operation.
    type Options: Clone + Default;

    /// Returns a value constructed from a series of bytes received from the specified reader,
    /// optionally configured via the specified options.
    fn read<R: Read>(reader: R, options: Self::Options) -> io::Result<Self>;
}

/// The `McWrite` trait allows for converting data types into bytes.
///
/// Implementors of the `McWrite` trait are typically packets or primitive data types.
pub trait McWrite: Sized {
    /// The type of options available to configure the write operation.
    type Options: Clone + Default;

    /// Writes this value as a series of bytes to the specified writer,
    /// optionally configured via the specified options.
    fn write<W: Write>(&self, writer: W, options: Self::Options) -> io::Result<()>;
}
