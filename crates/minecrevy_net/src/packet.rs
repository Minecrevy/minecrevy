//! Packet registration and event types.

use std::{
    any::TypeId,
    ops::{Deref, DerefMut},
};

use bevy::{prelude::*, utils::HashMap};
use minecrevy_io::{packet::RawPacket, McRead, McWrite};

use crate::client::ProtocolState;

/// [`Event`] emitted for each incoming packet.
#[derive(Event)]
#[repr(transparent)]
pub struct Recv<T: McRead>(pub T);

impl<T: McRead> Deref for Recv<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: McRead> DerefMut for Recv<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Function pointer for triggering events for incoming packets.
///
/// See [`IncomingPacketHandlers`] for where these are stored.
pub type PacketHandler = fn(&mut World, Entity, RawPacket);

/// [`Resource`] that stores [`PacketHandler`]s for triggering [`Event`]s for incoming packets.
#[derive(Resource, Default)]
pub struct IncomingPacketHandlers(HashMap<(ProtocolState, i32), PacketHandler>);

impl IncomingPacketHandlers {
    /// Returns the [`PacketHandler`] for the given packet ID and
    /// [`ProtocolState`], if any.
    pub fn get(&self, state: ProtocolState, id: i32) -> Option<PacketHandler> {
        self.0.get(&(state, id)).copied()
    }

    /// Inserts a [`PacketHandler`] for the given packet ID and
    /// [`ProtocolState`], which deserializes the [`RawPacket`] into the given
    /// type `T` and triggers a [`Recv<T>`] event.
    pub fn insert<T: McRead + Send + Sync + 'static>(&mut self, state: ProtocolState, id: i32) {
        self.0.insert((state, id), |world, client, packet| {
            let Ok(packet) = T::read_default(packet.reader()) else {
                warn!(
                    "Failed to read packet from client {client}: {:?}",
                    std::any::type_name::<T>()
                );
                return;
            };

            world.trigger_targets(Recv(packet), client);
        });
    }
}

/// [`Resource`] that stores the IDs for packets that are sent to the client,
/// based on the packet type and [`ProtocolState`].
#[derive(Resource, Default)]
pub struct OutgoingPacketIds(HashMap<(ProtocolState, TypeId), i32>);

impl OutgoingPacketIds {
    /// Returns the ID of the given packet type `T` for the given
    /// [`ProtocolState`], if any.
    pub fn get<T: McWrite + 'static>(&self, state: ProtocolState) -> Option<i32> {
        self.0.get(&(state, TypeId::of::<T>())).copied()
    }

    /// Inserts the packet ID for the given packet type `T` and [`ProtocolState`].
    pub fn insert<T: McWrite + 'static>(&mut self, state: ProtocolState, id: i32) {
        self.0.insert((state, TypeId::of::<T>()), id);
    }
}
