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

/// [`Query`] filter for clients who are currently in [`ProtocolState`] `S`.
pub type ClientIn<S> = With<PacketQueue<S>>;

/// A [`WorldQuery`] for ergonomically interacting with Minecraft clients.
#[derive(WorldQuery)]
#[world_query(mutable)]
pub struct Client<S: ProtocolState> {
    /// The socket connection.
    pub(crate) raw: &'static RawClient,
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
        self.raw.addr()
    }

    pub fn read<T: Packet + McRead>(&mut self) -> Option<T> {
        let Some(id) = self.registry.incoming::<T>() else {
            let version = self
                .info
                .map(|i| i.version)
                .unwrap_or(ReleaseVersion::V1_7_2.v());
            self.raw.error(ClientError::unregistered::<T>(version));
            return None;
        };

        // check the queue for a matching packet
        if let Some(packet) = self.queue.pop(id) {
            match T::read_default(packet.reader()) {
                Ok(decoded) => return Some(decoded),
                Err(error) => {
                    self.raw.error(ClientError::PacketIo(error));
                    return None;
                }
            }
            // if let Some(meta) = T::meta() { TODO
            //     self.connection.meta(meta);
            // }
        }

        // if the queue was empty,
        // check the socket for a matching packet
        for packet in self.raw.iter() {
            if packet.id != id {
                // TODO: only save registered packets?
                self.queue.push(packet);
                continue;
            }

            match T::read_default(packet.reader()) {
                Ok(decoded) => return Some(decoded),
                Err(error) => {
                    self.raw.error(ClientError::PacketIo(error));
                    return None;
                }
            }
            // if let Some(meta) = T::meta() { TODO
            //     self.connection.meta(meta);
            // }
        }

        None
    }

    pub fn write<T: Packet + McWrite>(&mut self, packet: T) {
        let Some(id) = self.registry.outgoing::<T>() else {
            let version = self
                .info
                .map(|i| i.version)
                .unwrap_or(ReleaseVersion::V1_7_2.v());
            self.raw.error(ClientError::unregistered::<T>(version));
            return;
        };

        let mut raw = RawPacket {
            id,
            body: Vec::new(),
        };
        if let Err(error) = packet.write_default(raw.writer()) {
            self.raw.error(ClientError::PacketIo(error));
            return;
        }

        self.raw.send(raw);
        if let Some(meta) = T::meta() {
            self.raw.meta(meta);
        }
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
