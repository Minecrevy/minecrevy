//! This crate defines a TCP [`Client`] and [`Server`].
//!
//! These allow us to bridge the ASYNC/[`tokio`] world with the SYNC/[`bevy`] world,
//! using [`flume`] to communicate between them.

#![warn(missing_docs)]

pub use self::client::*;
pub use self::server::*;
pub use self::socket::*;

mod client;
mod server;
mod socket;

pub mod util;
