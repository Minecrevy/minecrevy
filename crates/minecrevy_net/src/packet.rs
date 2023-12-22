//! Packet registration and event types.

use std::{
    any::TypeId,
    io::{self, Cursor},
    sync::Arc,
};

use bevy::{
    prelude::*,
    utils::{HashMap, HashSet},
};
use dashmap::DashMap;
use minecrevy_io::{packet::RawPacket, McRead, McWrite};

use crate::client::ProtocolState;

/// [`Event`] emitted for each incoming packet.
#[derive(Event)]
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Recv<T: McRead> {
    /// The entity with the [`crate::client::Client`] component.
    pub client: Entity,
    /// The packet.
    pub packet: T,
}

/// [`Resource`] that stores the IDs of incoming and outgoing packets.
#[derive(Resource, Component)]
#[derive(Clone, Default)]
pub struct PacketIds(Arc<PacketIdsInner>);

impl std::ops::Deref for PacketIds {
    type Target = PacketIdsInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for PacketIds {
    fn deref_mut(&mut self) -> &mut Self::Target {
        Arc::make_mut(&mut self.0)
    }
}

/// [`Resource`] that stores the IDs of incoming packets.
#[derive(Clone, Default)]
pub struct PacketIdsInner {
    incoming: HashMap<(TypeId, ProtocolState), i32>,
    outgoing: HashMap<(TypeId, ProtocolState), i32>,
    stateful: HashSet<(ProtocolState, i32)>,
}

impl PacketIdsInner {
    /// Registers the given incoming packet type `T` with the given [`ProtocolState`] and packet ID.
    ///
    /// If `stateful` is `true`, then the client will be paused until the packet is handled.
    pub fn insert_incoming<T: McRead + 'static>(
        &mut self,
        state: ProtocolState,
        id: i32,
        stateful: bool,
    ) {
        if self
            .incoming
            .iter()
            .any(|(&(_, pstate), &v)| pstate == state && id == v)
        {
            panic!(
                "Packet ID {id:x} is already registered for state {:?}",
                state
            );
        }

        self.incoming.insert((TypeId::of::<T>(), state), id);

        if stateful {
            self.stateful.insert((state, id));
        }
    }

    /// Returns the ID of the given incoming packet type `T` for the given [`ProtocolState`].
    pub fn incoming<T: McRead + 'static>(&self, state: ProtocolState) -> Option<i32> {
        self.incoming.get(&(TypeId::of::<T>(), state)).copied()
    }

    /// Registers the given outgoing packet type `T` with the given [`ProtocolState`] and packet ID.
    pub fn insert_outgoing<T: McWrite + 'static>(
        &mut self,
        state: ProtocolState,
        id: i32,
        stateful: bool,
    ) {
        if self
            .outgoing
            .iter()
            .any(|(&(_, pstate), &v)| pstate == state && id == v)
        {
            panic!(
                "Packet ID {id:x} is already registered for state {:?}",
                state
            );
        }

        self.outgoing.insert((TypeId::of::<T>(), state), id);

        if stateful {
            self.stateful.insert((state, id));
        }
    }

    /// Returns the ID of the given outgoing packet type `T` for the given [`ProtocolState`].
    pub fn outgoing<T: McWrite + 'static>(&self, state: ProtocolState) -> Option<i32> {
        self.outgoing.get(&(TypeId::of::<T>(), state)).copied()
    }

    /// Returns whether the client should be paused until the given packet is handled.
    pub fn is_stateful(&self, state: ProtocolState, id: i32) -> bool {
        self.stateful.contains(&(state, id))
    }
}

/// [`Resource`] for buffering incoming packets for later event emission.
#[derive(Resource, Default)]
pub struct IncomingPackets {
    /// A map of (packet state, packet ID) pairs to a list of (client, packet body) pairs.
    packets: DashMap<(ProtocolState, i32), Vec<(Entity, Vec<u8>)>>,
}

impl IncomingPackets {
    /// Buffers the given packet for later event emission.
    pub fn insert(&mut self, state: ProtocolState, packet: RawPacket, client: Entity) {
        self.packets
            .entry((state, packet.id))
            .or_insert_with(Vec::new)
            .push((client, packet.body));
    }

    /// Removes and returns all packets for the given state and ID.
    ///
    /// The returned iterator yields (client entity ID, decoded packet) pairs.
    pub fn drain<T: McRead>(
        &self,
        state: ProtocolState,
        id: i32,
    ) -> impl Iterator<Item = (Entity, io::Result<T>)> {
        self.packets
            .remove(&(state, id))
            .into_iter()
            .flat_map(|(_, packets)| {
                packets
                    .into_iter()
                    .map(|(entity, data)| (entity, T::read_default(&mut Cursor::new(data))))
            })
    }
}
