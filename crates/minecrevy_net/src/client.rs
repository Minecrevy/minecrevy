use std::{net::SocketAddr, sync::Arc};

use bevy::{ecs::query::WorldQuery, prelude::*};
use minecrevy_io::{packet::RawPacket, McRead, McWrite, Packet};

use crate::{
    packet::{PacketBuffer, Packets, ProtocolState},
    raw::RawClient,
};

pub type ClientConnected = Added<RawClient>;
pub type ClientEntered<S> = Added<ProtocolBuffer<S>>;

pub type Error = crate::error::ClientError;

/// A [`WorldQuery`] for ergonomically interacting with Minecraft clients.
#[derive(WorldQuery)]
#[world_query(mutable)]
pub struct Client<S: ProtocolState> {
    /// The client's socket connection.
    connection: &'static RawClient,
    /// The client's buffer for incoming packets.
    buffer: &'static mut ProtocolBuffer<S>,
    /// The client's map of Packet Type => Packet ID.
    registry: &'static PacketsArc<S>,
}

impl<S: ProtocolState> ClientItem<'_, S> {
    /// Returns the [`SocketAddr`] of this client.
    pub fn addr(&self) -> SocketAddr {
        self.connection.addr()
    }

    pub fn read<T: Packet + McRead>(&mut self) -> Result<Option<T>, Error> {
        let id = self
            .registry
            .incoming::<T>()
            .ok_or_else(|| Error::unregistered::<T>())?;

        if let Some(packet) = self.buffer.next(id) {
            let decoded = T::read_default(packet.reader()).map_err(Error::PacketIo)?;
            if let Some(meta) = T::meta() {
                self.connection.meta(meta);
            }
            return Ok(Some(decoded));
        }

        for packet in self.connection.iter() {
            if packet.id != id {
                self.buffer.push(packet);
                continue;
            }

            let decoded = T::read_default(packet.reader()).map_err(Error::PacketIo)?;
            if let Some(meta) = T::meta() {
                self.connection.meta(meta);
            }
            return Ok(Some(decoded));
        }

        Ok(None)
    }

    pub fn write<T: Packet + McWrite>(&mut self, packet: T) -> Result<(), Error> {
        let id = self
            .registry
            .outgoing::<T>()
            .ok_or_else(|| Error::unregistered::<T>())?;

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

#[derive(Component, Deref, DerefMut, Default)]
pub struct ProtocolBuffer<S: ProtocolState>(S::Buffer);

#[derive(Component, Deref, Default)]
pub struct PacketsArc<S: ProtocolState>(pub Arc<Packets<S>>);
