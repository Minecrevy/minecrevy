use std::fmt;
use std::net::SocketAddr;
use std::sync::atomic::{AtomicU32, Ordering};

/// Uniquely identifies a protocol connection, usually for servers.
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "bevy", derive(bevy::prelude::Component))]
pub struct SocketId(SocketAddr, u32);

impl SocketId {
    /// Creates a new id from the provided [`SocketAddr`].
    pub fn new(addr: SocketAddr) -> Self {
        static COUNTER: AtomicU32 = AtomicU32::new(0);
        Self(addr, COUNTER.fetch_add(1, Ordering::Relaxed))
    }

    /// Returns the associated [`SocketAddr`].
    #[inline]
    pub fn addr(&self) -> SocketAddr {
        self.0
    }

    /// Returns the incremental unique ID.
    #[inline]
    pub fn id(&self) -> u32 {
        self.1
    }
}

impl fmt::Debug for SocketId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.0, self.1)
    }
}

impl fmt::Display for SocketId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.0, self.1)
    }
}
