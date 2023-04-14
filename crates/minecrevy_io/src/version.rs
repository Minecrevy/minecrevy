use std::fmt;

use crate::{McRead, McWrite};

/// The Minecraft protocol version sent during protocol handshake.
#[derive(McRead, McWrite)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct ProtocolVersion(#[options(varint = true)] pub i32);

impl fmt::Display for ProtocolVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
