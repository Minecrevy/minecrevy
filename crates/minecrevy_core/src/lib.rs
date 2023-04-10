//! This crate provides ubiquitous ("core") functionality for Minecrevy.

pub mod bow;
pub mod channel;
pub mod color;
pub mod ecs;
pub mod key;
pub mod math;

pub mod str {
    pub use compact_str::*;
}
