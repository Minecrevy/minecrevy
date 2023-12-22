//! This module contains the [`ClientPlugin`], which handles client communication, both client-side and server-side.

use std::{io, net::SocketAddr};

use bevy::{ecs::query::WorldQuery, prelude::*};
use flume::Receiver;
use minecrevy_io::{packet::RawPacket, McWrite};
use tokio::{
    sync::{mpsc::UnboundedSender, oneshot},
    task::JoinHandle,
};

use crate::packet::PacketIds;

/// [`Plugin`] for client-side network functionality.
pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, _app: &mut App) {
        todo!()
    }
}

/// [`WorldQuery`] for ergonomic access to [`Client`] components.
#[derive(WorldQuery)]
#[world_query(mutable)]
pub struct ClientQ {
    /// The [`Client`] component.
    pub client: &'static Client,
    /// The [`PacketIds`] component.
    pub ids: &'static PacketIds,
    /// The [`ProtocolState`] component.
    pub state: &'static mut ProtocolState,
}

impl ClientQItem<'_> {
    /// Sends the given packet to the client.
    pub fn send<T: McWrite + 'static>(&self, packet: T) -> FlushOnDrop<'_> {
        self.client.send(self.ids, &self.state, packet)
    }
}

impl ClientQReadOnlyItem<'_> {
    /// Sends the given packet to the client.
    pub fn send<T: McWrite + 'static>(&self, packet: T) -> FlushOnDrop<'_> {
        self.client.send(self.ids, &self.state, packet)
    }
}

/// [`Bundle`] for all [`Client`]-related components.
#[derive(Bundle)]
pub struct ClientBundle {
    /// The [`Client`] component.
    pub client: Client,
    /// The [`ProtocolState`] component.
    pub state: ProtocolState,
    /// The [`Paused`] component.
    pub paused: Paused,
}

/// A client connected to the server.
#[derive(Component)]
pub struct Client {
    /// The address of the client.
    addr: SocketAddr,
    /// The [`JoinHandle`] for the I/O task.
    io_task: JoinHandle<()>,
    /// The [`Receiver`] for incoming packets.
    pub(crate) incoming: Receiver<RawPacket>,
    /// The [`UnboundedSender`] for outgoing packets.
    outgoing: UnboundedSender<WriteOp>,
    /// The [`Receiver`] for I/O errors.
    pub(crate) errors: oneshot::Receiver<io::Error>,
}

impl Client {
    /// Creates a new [`Client`] with the given address, I/O task, and channels.
    pub fn new(
        addr: SocketAddr,
        io_task: JoinHandle<()>,
        incoming: Receiver<RawPacket>,
        outgoing: UnboundedSender<WriteOp>,
        errors: oneshot::Receiver<io::Error>,
    ) -> Self {
        Self {
            addr,
            io_task,
            incoming,
            outgoing,
            errors,
        }
    }

    /// Returns the address of the client.
    pub fn addr(&self) -> SocketAddr {
        self.addr
    }

    /// Sends the given packet to the client.
    pub fn send<'w, T: McWrite + 'static>(
        &'w self,
        ids: &'w PacketIds,
        state: &'w ProtocolState,
        packet: T,
    ) -> FlushOnDrop<'w> {
        FlushOnDrop {
            client: self,
            ids,
            state,
        }
        .send(packet)
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        self.io_task.abort();
    }
}

/// A guard that flushes the outgoing packet buffer when dropped.
pub struct FlushOnDrop<'w> {
    client: &'w Client,
    ids: &'w PacketIds,
    state: &'w ProtocolState,
}

impl FlushOnDrop<'_> {
    /// Sends the given packet to the client.
    pub fn send<T: McWrite + 'static>(self, packet: T) -> Self {
        let id = self.ids.outgoing::<T>(*self.state).unwrap_or_else(|| {
            panic!(
                "Packet {:?} is not registered for state {:?}",
                std::any::type_name::<T>(),
                self.state
            )
        });
        let mut body = Vec::new();
        packet.write_default(&mut body).unwrap();

        self.client
            .outgoing
            .send(WriteOp::Send(RawPacket { id, body }))
            .ok();

        self
    }
}

impl Drop for FlushOnDrop<'_> {
    fn drop(&mut self) {
        self.client.outgoing.send(WriteOp::Flush).ok();
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

/// [`Component`] for [`Client`]s with pending [`ProtocolState`] changes.
///
/// When `true`, no packets will be read from the client until reset to `false`.
/// This is used to allow the server to change the client's protocol state
/// before reading any packets from the new state.
#[derive(Component, Deref, DerefMut)]
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, Default)]
pub struct Paused(pub bool);
