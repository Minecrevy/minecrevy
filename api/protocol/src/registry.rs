use std::any::TypeId;
use std::collections::HashMap;

use crate::{Packet, ProtocolState};

/// A dual-[`PacketRegistry`] wrapper for bi-directional registration.
#[derive(Clone, Debug, Default)]
pub struct PacketCodec {
    /// The [`PacketRegistry`] for inbound packets.
    pub incoming: PacketRegistry,
    /// The [`PacketRegistry`] for outbound packets.
    pub outgoing: PacketRegistry,
}

impl PacketCodec {
    /// Constructs a new, empty [`PacketCodec`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Flips the incoming and outgoing [`PacketRegistry`]s in this codec, allowing
    /// easy conversion between client-side and server-side packet processing.
    pub fn flip(mut self) -> Self {
        std::mem::swap(&mut self.incoming, &mut self.outgoing);
        self
    }
}

/// A map of a [`packet type`][`Packet`] and [`State`] to a `packet id`.
#[derive(Clone, Debug, Default)]
pub struct PacketRegistry {
    inner: HashMap<(TypeId, ProtocolState), i32>,
}

impl PacketRegistry {
    /// Constructs a new, empty registry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the packet id corresponding to the given [`Packet`] type and [`State`].
    pub fn get<T: Packet>(&self, state: ProtocolState) -> Option<i32> {
        self.inner
            .get(&(TypeId::of::<T>(), state))
            .copied()
    }

    /// Returns `true` if the registry has an entry for the given [`Packet`] type and [`State`].
    pub fn contains<T: Packet>(&self, state: ProtocolState) -> bool {
        self.get::<T>(state).is_some()
    }

    /// Adds the given [`Packet`] type, [`State`], and a packet id to the registry.
    pub fn register<T: Packet>(&mut self, state: ProtocolState, id: i32) {
        self.inner
            .insert((TypeId::of::<T>(), state), id);
    }
}
