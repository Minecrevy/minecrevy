//! A library for encoding/decoding Minecraft packets and data types in the style of `serde`.
//!
//! # Example
//!
//! ```
//! use minecrevy_io::{McRead, McWrite};
//!
//! #[derive(McRead, McWrite)]
//! pub struct Handshake {
//!     #[args(varint = true)]
//!     pub version: i32,
//!     #[args(max_len = 255)]
//!     pub address: String,
//!     pub port: u16,
//!     #[args(varint = true)]
//!     pub next: i32,
//! }
//! ```

#![warn(missing_docs)]
#![allow(clippy::module_name_repetitions)]

use std::io;

pub mod prelude {
    //! Re-exports important traits, types, and functions.

    pub use crate::{
        ext::{ReadMinecraftExt, WriteMinecraftExt},
        packet::RawPacket,
        util::varint_bytes,
        McRead, McWrite,
    };
}

pub mod args;
pub mod ext;
mod impls;
pub mod packet;
pub mod util;

/// A trait for reading a type from a stream of bytes.
pub trait McRead: Sized {
    /// The arguments for reading this type.
    type Args: Clone + Default;

    /// Reads this type from the given reader.
    ///
    /// # Errors
    ///
    /// If the reader returns an error, this function will return that error.
    fn read(reader: impl io::Read, args: Self::Args) -> io::Result<Self>;

    /// Reads this type from the given reader with default arguments.
    ///
    /// # Errors
    ///
    /// If the reader returns an error, this function will return that error.
    fn read_default(reader: impl io::Read) -> io::Result<Self> {
        Self::read(reader, Self::Args::default())
    }
}

/// A trait for writing a type to a stream of bytes.
pub trait McWrite: Sized {
    /// The arguments for writing this type.
    type Args: Clone + Default;

    /// Writes this type to the given writer.
    ///
    /// # Errors
    ///
    /// If the writer returns an error, this function will return that error.
    fn write(&self, writer: impl io::Write, args: Self::Args) -> io::Result<()>;

    /// Writes this type to the given writer with default arguments.
    ///
    /// # Errors
    ///
    /// If the writer returns an error, this function will return that error.
    fn write_default(&self, writer: impl io::Write) -> io::Result<()> {
        self.write(writer, Self::Args::default())
    }
}
