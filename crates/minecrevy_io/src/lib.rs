//! A library for encoding/decoding Minecraft packets and data types in the style of `serde`.
//!
//! # Example
//!
//! ```rust
//! use minecrevy_io::{McRead, McWrite};
//!
//! #[derive(McRead, McWrite)]
//! pub struct Handshake {
//!     #[options(varint = true)]
//!     pub version: i32,
//!     #[options(max_len = 255)]
//!     pub address: String,
//!     pub port: u16,
//!     #[options(varint = true)]
//!     pub next: i32,
//! }
//! ```

#![deny(missing_docs)]
#![feature(array_try_from_fn)]

use std::io::{self, Read, Write};

pub use minecrevy_io_macros::{McRead, McWrite, Packet};

mod impls;
pub mod options;

/// The `McRead` trait allows for constructing data types from bytes.
pub trait McRead: Sized {
    /// The associated type of configurable options for the read operation.
    type Options: Clone + Default;

    /// Returns a value constructed from a series of bytes received from the specified reader,
    /// optionally configured via the specified options.
    fn read<R: Read>(reader: R, options: Self::Options) -> io::Result<Self>;

    /// Returns a value constructed from a series of bytes received from the specified reader,
    /// using the default options.
    fn read_default<R: Read>(reader: R) -> io::Result<Self> {
        Self::read(reader, Self::Options::default())
    }
}

/// The `McWrite` trait allows for converting data types into bytes.
pub trait McWrite: Sized {
    /// The associated type of configurable options for the write operation.
    type Options: Clone + Default;

    /// Writes this value as a series of bytes to the specified writer,
    /// optionally configured via the specified options.
    fn write<W: Write>(&self, writer: W, options: Self::Options) -> io::Result<()>;

    /// Writes this value as a series of bytes to the specified writer,
    /// using the default options.
    fn write_default<W: Write>(&self, writer: W) -> io::Result<()> {
        self.write(writer, Self::Options::default())
    }
}

/// A generic packet trait.
pub trait Packet: 'static {
    /// The socket metadata associated with this packet type.
    fn meta() -> Option<PacketMeta>
    where
        Self: Sized,
    {
        None
    }
}

/// The socket metadata related to a [`Packet`].
pub enum PacketMeta {
    /// A packet that also enables packet compression thereafter.
    EnableCompression,
    /// A packet that also enables packet encryption thereafter.
    EnableEncryption,
}
