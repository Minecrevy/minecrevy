//! A crate providing Minecraft protocol packet primitives.
//!
//! # Adding a new packet type
//!
//! - Derive or manually implement [`McRead`][`minecrevy_io_str::McRead`]
//!   and [`McWrite`][`minecrevy_io_str::McWrite`].
//! - Implement [`Packet`].
//! -

#![warn(missing_docs)]

pub use self::registry::*;
pub use self::packet::*;
pub use self::state::*;

mod registry;
mod packet;
mod state;
