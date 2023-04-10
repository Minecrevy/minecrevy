use std::{
    any::TypeId,
    collections::{BTreeMap, HashMap},
    marker::PhantomData,
    ops::RangeBounds,
    sync::Arc,
};

use bevy::prelude::*;
use minecrevy_io::{McRead, McWrite, Packet};

use crate::protocol::{state::ProtocolState, version::ProtocolVersion};

/// A read-only [`Resource`] map that stores a registry of incoming and outgoing
/// `Packet Type -> Packet ID` entries, grouped by [`ProtocolVersion`].
#[derive(Resource)]
pub struct VersionedPackets<S: ProtocolState>(BTreeMap<ProtocolVersion, Arc<Packets<S>>>);

impl<S: ProtocolState> VersionedPackets<S> {
    /// Gets the minimum supported [`ProtocolVersion`] and its packet registry.
    pub fn min(&self) -> Option<(ProtocolVersion, &Arc<Packets<S>>)> {
        self.0.first_key_value().map(|(v, registry)| (*v, registry))
    }

    #[inline]
    pub fn get(&self, version: ProtocolVersion) -> Option<&Arc<Packets<S>>> {
        self.0.get(&version)
    }

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

/// A mutable version of [`VersionedPackets`], only available during [`Startup`].
#[derive(Resource)]
pub struct VersionedPacketsBuilder<S: ProtocolState>(BTreeMap<ProtocolVersion, Packets<S>>);

impl<S: ProtocolState> VersionedPacketsBuilder<S> {
    /// Constructs a packet registry builder that supports the specified [`ProtocolVersion`]s.
    pub fn new(versions: impl IntoIterator<Item = ProtocolVersion>) -> Self {
        Self(
            versions
                .into_iter()
                .map(|v| (v, Packets::default()))
                .collect(),
        )
    }

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
    pub fn build(self) -> VersionedPackets<S> {
        VersionedPackets(
            self.0
                .into_iter()
                .map(|(version, registry)| (version, Arc::new(registry)))
                .collect(),
        )
    }
}

pub struct Packets<S: ProtocolState> {
    incoming: HashMap<TypeId, i32>,
    outgoing: HashMap<TypeId, i32>,
    _state: PhantomData<S>,
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
