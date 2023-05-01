use std::collections::VecDeque;

use dashmap::DashMap;
use minecrevy_io::packet::RawPacket;

/// A client connection state in the Minecraft protocol.
pub trait ProtocolState: 'static + Send + Sync {
    type Queue: ProtocolQueue + Default;
}

/// The initial client connection [`ProtocolState`].
pub struct Handshake;

impl ProtocolState for Handshake {
    type Queue = ();
}

/// A potential client connection [`ProtocolState`] after [`Handshake`].
pub struct Status;

impl ProtocolState for Status {
    type Queue = DashMap<i32, VecDeque<RawPacket>>;
}

/// A potential client connection [`ProtocolState`] after [`Handhskae`].
pub struct Login;

impl ProtocolState for Login {
    type Queue = DashMap<i32, VecDeque<RawPacket>>;
}

/// The client connection [`ProtocolState`] after a successful [`Login`].
pub struct Play;

impl ProtocolState for Play {
    type Queue = DashMap<i32, VecDeque<RawPacket>>;
}

/// A trait for types that can store [`RawPacket`]s, grouped by their ID.
pub trait ProtocolQueue: 'static + Send + Sync {
    fn pop(&self, id: i32) -> Option<RawPacket>;

    fn push(&self, packet: RawPacket);
}

impl ProtocolQueue for () {
    #[inline]
    fn pop(&self, _id: i32) -> Option<RawPacket> {
        None
    }

    #[inline]
    fn push(&self, packet: RawPacket) {
        drop(packet)
    }
}

impl ProtocolQueue for DashMap<i32, VecDeque<RawPacket>> {
    fn pop(&self, id: i32) -> Option<RawPacket> {
        let mut queue = self.get_mut(&id)?;
        queue.pop_front()
    }

    fn push(&self, packet: RawPacket) {
        let mut queue = self.entry(packet.id).or_default();
        queue.push_back(packet);
    }
}
