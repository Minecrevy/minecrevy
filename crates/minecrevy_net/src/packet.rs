use std::{
    any::TypeId,
    collections::{BTreeMap, VecDeque},
    marker::PhantomData,
    ops::RangeBounds,
    sync::Arc,
};

use bevy::{prelude::*, utils::HashMap};
use minecrevy_io::{packet::RawPacket, McRead, McWrite, Packet};
use minecrevy_protocol::version::ProtocolVersion;

/// A [`Plugin`] that provides a [`VersionedPackets`] [`Resource`].
///
/// During startup, register via the [`VersionedPacketsBuilder`] [`Resource`],
/// as the former one is read-only.
pub struct PacketsPlugin;

impl Plugin for PacketsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(VersionedPacketsBuilder::<Handshake>::default())
            .insert_resource(VersionedPacketsBuilder::<Status>::default())
            .insert_resource(VersionedPacketsBuilder::<Login>::default())
            .insert_resource(VersionedPacketsBuilder::<Play>::default());

        app.add_systems(
            PostStartup,
            (
                Self::build_packet_registry::<Handshake>,
                Self::build_packet_registry::<Status>,
                Self::build_packet_registry::<Login>,
                Self::build_packet_registry::<Play>,
            ),
        );
    }
}

impl PacketsPlugin {
    fn build_packet_registry<S: ProtocolState>(
        mut commands: Commands,
        builder: Res<VersionedPacketsBuilder<S>>,
    ) {
        commands.insert_resource(builder.build());
        commands.remove_resource::<VersionedPacketsBuilder<S>>();
    }
}

/// A read-only [`Resource`] map that stores a registry of incoming and outgoing
/// `Packet Type -> Packet ID` entries, grouped by [`ProtocolVersion`].
#[derive(Resource)]
pub struct VersionedPackets<S: ProtocolState>(BTreeMap<ProtocolVersion, Arc<Packets<S>>>);

impl<S: ProtocolState> VersionedPackets<S> {
    /// Gets the ID of the provided [`Packet`] type for the specified [`ProtocolVersion`].
    pub fn incoming<T: Packet + McRead>(&self, version: ProtocolVersion) -> Option<i32> {
        self.0
            .get(&version)
            .and_then(|registry| registry.incoming::<T>())
    }

    /// Gets the ID of the provided [`Packet`] type for the specified [`ProtocolVersion`].
    pub fn outgoing<T: Packet + McWrite>(&self, version: ProtocolVersion) -> Option<i32> {
        self.0
            .get(&version)
            .and_then(|registry| registry.outgoing::<T>())
    }
}

impl<S: ProtocolState> Default for VersionedPackets<S> {
    fn default() -> Self {
        Self(BTreeMap::default())
    }
}

#[derive(Resource)]
pub struct VersionedPacketsBuilder<S: ProtocolState>(BTreeMap<ProtocolVersion, Packets<S>>);

impl<S: ProtocolState> VersionedPacketsBuilder<S> {
    /// Registers the specified [`Packet`] type to the specified packet ID, for the specified [`ProtocolVersion`]s.
    pub fn add_incoming<T: Packet + McRead>(
        &mut self,
        id: i32,
        versions: impl RangeBounds<ProtocolVersion>,
    ) -> &mut Self {
        for (_, registry) in self.0.range_mut(versions) {
            registry.add_incoming::<T>(id);
        }
        self
    }

    /// Registers the specified [`Packet`] type to the specified packet ID, for the specified [`ProtocolVersion`]s.
    pub fn add_outgoing<T: Packet + McWrite>(
        &mut self,
        id: i32,
        versions: impl RangeBounds<ProtocolVersion>,
    ) -> &mut Self {
        for (_, registry) in self.0.range_mut(versions) {
            registry.add_outgoing::<T>(id);
        }
        self
    }

    /// Constructs a read-only [`VersionedPackets`] registry.
    pub fn build(&self) -> VersionedPackets<S> {
        VersionedPackets(
            self.0
                .iter()
                .map(|(version, registry)| (*version, Arc::new(registry.clone())))
                .collect(),
        )
    }
}

impl<S: ProtocolState> Default for VersionedPacketsBuilder<S> {
    fn default() -> Self {
        Self(BTreeMap::default())
    }
}

pub trait ProtocolState: 'static + Send + Sync {
    type Buffer: PacketBuffer + Default;
}

pub struct Handshake;

impl ProtocolState for Handshake {
    type Buffer = ();
}

pub struct Status;

impl ProtocolState for Status {
    type Buffer = HashMap<i32, VecDeque<RawPacket>>;
}

pub struct Login;

impl ProtocolState for Login {
    type Buffer = HashMap<i32, VecDeque<RawPacket>>;
}

pub struct Play;

impl ProtocolState for Play {
    type Buffer = HashMap<i32, VecDeque<RawPacket>>;
}

/// A trait for types that can store [`RawPacket`]s, grouped by their ID.
pub trait PacketBuffer: 'static + Send + Sync {
    fn next(&mut self, id: i32) -> Option<RawPacket>;

    fn push(&mut self, packet: RawPacket);
}

impl PacketBuffer for () {
    #[inline]
    fn next(&mut self, _id: i32) -> Option<RawPacket> {
        None
    }

    #[inline]
    fn push(&mut self, packet: RawPacket) {
        drop(packet)
    }
}

impl PacketBuffer for HashMap<i32, VecDeque<RawPacket>> {
    fn next(&mut self, id: i32) -> Option<RawPacket> {
        self.get_mut(&id).and_then(|queue| queue.pop_front())
    }

    fn push(&mut self, packet: RawPacket) {
        let queue = self.entry(packet.id).or_default();
        queue.push_back(packet);
    }
}

pub struct Packets<S: ProtocolState> {
    incoming: HashMap<TypeId, i32>,
    outgoing: HashMap<TypeId, i32>,
    _state: PhantomData<S>,
}

impl<S: ProtocolState> Clone for Packets<S> {
    fn clone(&self) -> Self {
        Self {
            incoming: self.incoming.clone(),
            outgoing: self.outgoing.clone(),
            _state: self._state.clone(),
        }
    }
}

impl<S: ProtocolState> Default for Packets<S> {
    fn default() -> Self {
        Self {
            incoming: Default::default(),
            outgoing: Default::default(),
            _state: Default::default(),
        }
    }
}

impl<S: ProtocolState> Packets<S> {
    pub fn incoming<T: Packet + McRead>(&self) -> Option<i32> {
        self.incoming.get(&TypeId::of::<T>()).copied()
    }

    pub fn outgoing<T: Packet + McWrite>(&self) -> Option<i32> {
        self.outgoing.get(&TypeId::of::<T>()).copied()
    }

    pub fn add_incoming<T: Packet + McRead>(&mut self, id: i32) -> &mut Self {
        self.incoming.insert(TypeId::of::<T>(), id);
        self
    }

    pub fn add_outgoing<T: Packet + McWrite>(&mut self, id: i32) -> &mut Self {
        self.outgoing.insert(TypeId::of::<T>(), id);
        self
    }
}
