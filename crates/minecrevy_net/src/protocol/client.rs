use std::{net::SocketAddr, sync::Arc};

use bevy::{ecs::query::WorldQuery, prelude::*};
use minecrevy_io::{packet::RawPacket, McRead, McWrite, Packet};

use crate::{
    error::ClientError,
    protocol::{
        flow::handshake::ClientInfo,
        registry::Packets,
        state::{ProtocolQueue, ProtocolState},
        version::ReleaseVersion,
    },
    raw::RawClient,
};

/// [`Query`] filter for clients who've only just connected to the server.
pub type ClientConnected = Added<RawClient>;

/// [`Query`] filter for clients who've entered a new [`ProtocolState`] `S`.
pub type ClientEntered<S> = Added<PacketQueue<S>>;

/// A [`WorldQuery`] for ergonomically interacting with Minecraft clients.
#[derive(WorldQuery)]
#[world_query(mutable)]
pub struct Client<S: ProtocolState> {
    /// The socket connection.
    connection: &'static RawClient,
    /// The queue for incoming packets.
    queue: &'static mut PacketQueue<S>,
    /// The map of Packet Type => Packet ID.
    registry: &'static PacketRegistry<S>,
    /// Client info available after the initial handshake.
    info: Option<&'static ClientInfo>,
}

impl<S: ProtocolState> ClientItem<'_, S> {
    /// Returns the [`SocketAddr`] of this client.
    pub fn addr(&self) -> SocketAddr {
        self.connection.addr()
    }

    pub fn read<T: Packet + McRead>(&mut self) -> Option<Result<T, ClientError>> {
        let Some(id) = self.registry.incoming::<T>() else {
            let version = self
                .info
                .map(|i| i.version)
                .unwrap_or(ReleaseVersion::V1_7_2.v());
            return Some(Err(ClientError::unregistered::<T>(version)));
        };

        // check the queue for a matching packet
        if let Some(packet) = self.queue.pop(id) {
            let decoded = T::read_default(packet.reader()).map_err(ClientError::PacketIo);
            // if let Some(meta) = T::meta() { TODO
            //     self.connection.meta(meta);
            // }
            return Some(decoded);
        }

        // if the queue was empty,
        // check the socket for a matching packet
        for packet in self.connection.iter() {
            if packet.id != id {
                // TODO: only save registered packets?
                self.queue.push(packet);
                continue;
            }

            let decoded = T::read_default(packet.reader()).map_err(ClientError::PacketIo);
            // if let Some(meta) = T::meta() { TODO
            //     self.connection.meta(meta);
            // }
            return Some(decoded);
        }

        None
    }

    pub fn write<T: Packet + McWrite>(&mut self, packet: T) -> Result<(), ClientError> {
        let Some(id) = self.registry.outgoing::<T>() else {
            let version = self
                .info
                .map(|i| i.version)
                .unwrap_or(ReleaseVersion::V1_7_2.v());
            return Err(ClientError::unregistered::<T>(version));
        };

        let mut raw = RawPacket {
            id,
            body: Vec::new(),
        };
        packet.write_default(raw.writer())?;

        self.connection.send(raw);
        if let Some(meta) = T::meta() {
            self.connection.meta(meta);
        }

        Ok(())
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct PacketQueue<S: ProtocolState>(S::Queue);

impl<S: ProtocolState> Default for PacketQueue<S> {
    fn default() -> Self {
        Self(S::Queue::default())
    }
}

#[derive(Component, Deref)]
pub struct PacketRegistry<S: ProtocolState>(pub Arc<Packets<S>>);
