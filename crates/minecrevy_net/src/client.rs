//! This module contains the [`ClientPlugin`], which handles client communication, both client-side and server-side.

use std::{io, net::SocketAddr};

use bevy::{
    ecs::{
        component::ComponentId, entity::EntityHashMap, query::QueryEntityError,
        system::SystemParam, world::DeferredWorld,
    },
    prelude::*,
    utils::HashMap,
};
use minecrevy_io::{packet::RawPacket, McWrite};
use tokio::sync::{mpsc::UnboundedSender, oneshot};

use crate::packet::OutgoingPacketIds;

/// [`Plugin`] for client-side network functionality.
pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, _app: &mut App) {
        todo!()
    }
}

/// [`Resource`] that maps [`SocketAddr`] to [`Entity`] and vice versa.
#[derive(Resource, Default)]
pub struct ClientAddressIndex {
    addr_to_entity: HashMap<SocketAddr, Entity>,
    entity_to_addr: EntityHashMap<SocketAddr>,
}

impl ClientAddressIndex {
    /// Returns the [`SocketAddr`] of the given [`Entity`].
    pub fn address(&self, entity: Entity) -> Option<SocketAddr> {
        self.entity_to_addr.get(&entity).copied()
    }

    /// Returns the [`Entity`] of the given [`SocketAddr`].
    pub fn entity(&self, addr: SocketAddr) -> Option<Entity> {
        self.addr_to_entity.get(&addr).copied()
    }

    /// Inserts the given [`SocketAddr`] and [`Entity`] into the index.
    pub(crate) fn insert(&mut self, addr: SocketAddr, entity: Entity) {
        self.addr_to_entity.insert(addr, entity);
        self.entity_to_addr.insert(entity, addr);
    }

    /// Removes the given [`SocketAddr`] and [`Entity`] from the index.
    pub(crate) fn remove(&mut self, addr: SocketAddr, entity: Entity) {
        self.addr_to_entity.remove(&addr);
        self.entity_to_addr.remove(&entity);
    }
}

/// [`SystemParam`] for writing packets to clients.
#[derive(SystemParam)]
pub struct PacketWriter<'w, 's> {
    clients: Query<'w, 's, (&'static Client, &'static mut ProtocolState)>,
    outgoing_ids: Res<'w, OutgoingPacketIds>,
}

impl PacketWriter<'_, '_> {
    /// Returns a [`ClientPacketWriter`] for the given client.
    ///
    /// # Panics
    ///
    /// Panics if the client does not exist.
    pub fn client(&mut self, client: Entity) -> ClientPacketWriter<'_> {
        self.get_client(client).unwrap()
    }

    /// Returns a [`ClientPacketWriter`] for the given client.
    ///
    /// # Errors
    ///
    /// Returns an error if the client does not exist.
    pub fn get_client(
        &mut self,
        client: Entity,
    ) -> Result<ClientPacketWriter<'_>, QueryEntityError> {
        let outgoing_ids = &self.outgoing_ids;
        self.clients
            .get_mut(client)
            .map(move |(client, state)| ClientPacketWriter {
                client,
                state,
                outgoing_ids,
            })
    }

    /// Sends the given packet to the given client.
    pub fn send<T: McWrite + 'static>(&mut self, client: Entity, packet: &T) -> &mut Self {
        let client = self.get_client(client).unwrap();
        client.send(packet);
        drop(client);
        self
    }
}

/// A writer for sending packets to a client.
pub struct ClientPacketWriter<'w> {
    client: &'w Client,
    state: Mut<'w, ProtocolState>,
    outgoing_ids: &'w OutgoingPacketIds,
}

impl ClientPacketWriter<'_> {
    /// Sends the given packet to the client.
    pub fn send<T: McWrite + 'static>(&self, packet: &T) -> &Self {
        let id = self.outgoing_ids.get::<T>(*self.state).unwrap_or_else(|| {
            panic!(
                "Packet {:?} is not registered for state {:?}",
                std::any::type_name::<T>(),
                self.state
            )
        });
        self.client.send(id, packet);
        self
    }

    /// Returns the [`Client`]'s current [`ProtocolState`].
    pub fn state(&self) -> ProtocolState {
        *self.state
    }

    /// Changes the [`Client`]'s [`ProtocolState`].
    pub fn set_state(&mut self, state: ProtocolState) {
        *self.state = state;
    }
}

impl Drop for ClientPacketWriter<'_> {
    fn drop(&mut self) {
        let _ = self.client.outgoing.send(WriteOp::Flush);
    }
}

/// A client connected to the server.
#[derive(Component)]
#[require(ProtocolState)]
#[component(on_add = Self::on_add, on_remove = Self::on_remove)]
pub struct Client {
    /// The address of the client.
    addr: SocketAddr,
    /// The [`UnboundedSender`] for outgoing packets.
    outgoing: UnboundedSender<WriteOp>,
    /// The [`Receiver`] for I/O errors.
    pub(crate) errors: oneshot::Receiver<io::Error>,
}

impl Client {
    /// Creates a new [`Client`] with the given address, I/O task, and channels.
    pub(crate) fn new(
        addr: SocketAddr,
        outgoing: UnboundedSender<WriteOp>,
        errors: oneshot::Receiver<io::Error>,
    ) -> Self {
        Self {
            addr,
            outgoing,
            errors,
        }
    }

    /// Returns the address of the client.
    pub fn addr(&self) -> SocketAddr {
        self.addr
    }

    /// Sends the given packet to the client.
    ///
    /// Prefer using [`PacketWriter`] or [`ClientPacketWriter`] instead.
    fn send<T: McWrite + 'static>(&self, id: i32, packet: &T) {
        let mut body = Vec::new();
        packet.write_default(&mut body).unwrap();

        let _ = self.outgoing.send(WriteOp::Send(RawPacket { id, body }));
    }

    fn on_add(mut world: DeferredWorld, entity: Entity, _: ComponentId) {
        let Some(addr) = world.get::<Client>(entity).map(|c| c.addr()) else {
            return;
        };
        let Some(mut index) = world.get_resource_mut::<ClientAddressIndex>() else {
            return;
        };

        index.insert(addr, entity);
    }

    fn on_remove(mut world: DeferredWorld, entity: Entity, _: ComponentId) {
        let Some(addr) = world.get::<Client>(entity).map(|c| c.addr()) else {
            return;
        };
        let Some(mut index) = world.get_resource_mut::<ClientAddressIndex>() else {
            return;
        };

        index.remove(addr, entity);
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        let _ = self.outgoing.send(WriteOp::Disconnect);
    }
}

/// An operation to perform on the client's socket.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum WriteOp {
    /// Writes the given packet to the client's outgoing packet buffer.
    Send(RawPacket),
    /// Flushes the client's outgoing packet buffer.
    Flush,
    /// Enables compression for the client.
    EnableCompression,
    /// Enables encryption for the client.
    EnableEncryption,
    /// Disconnects the client.
    Disconnect,
}

/// [`Component`] for [`Client`]s current protocol state.
#[derive(Component)]
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, Default)]
pub enum ProtocolState {
    /// The first state of the protocol, where the client sends its initial
    /// handshake.
    #[default]
    Handshake,
    /// One of the second states of the protocol, where the client requests
    /// server list information.
    Status,
    /// One of the second states of the protocol, where the client begins the
    /// login process.
    Login,
    /// The final state of the protocol, where the client plays the game.
    Play,
    /// An intermediate state of the protocol, where the client sends/requests
    /// new network configuration.
    Config,
}
