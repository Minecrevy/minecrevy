use std::any::type_name;
use std::collections::VecDeque;
use std::fmt;
use std::ops::Deref;

use bevy::ecs::query::WorldQuery;
use bevy::prelude::*;

use minecrevy_io_buf::RawPacket;
use minecrevy_io_str::{McRead, McWrite};
use minecrevy_protocol_latest::{Packet, PacketCodec, ProtocolState};
use minecrevy_tcp::SocketId;

pub use self::raw::*;

mod raw;

pub trait StateBuffer: Component {
    const STATE: ProtocolState;

    fn buffer(&mut self) -> &mut VecDeque<RawPacket>;
}

/// See [`State::Handshake`] for info.
#[derive(Component, Debug, Default)]
pub struct Handshake(VecDeque<RawPacket>);

impl StateBuffer for Handshake {
    const STATE: ProtocolState = ProtocolState::Handshake;

    #[inline(always)]
    fn buffer(&mut self) -> &mut VecDeque<RawPacket> {
        &mut self.0
    }
}

/// See [`State::Status`] for info.
#[derive(Component, Debug, Default)]
pub struct Status(VecDeque<RawPacket>);

impl StateBuffer for Status {
    const STATE: ProtocolState = ProtocolState::Status;

    #[inline(always)]
    fn buffer(&mut self) -> &mut VecDeque<RawPacket> {
        &mut self.0
    }
}

/// See [`State::Login`] for info.
#[derive(Component, Debug, Default)]
pub struct Login(VecDeque<RawPacket>);

impl StateBuffer for Login {
    const STATE: ProtocolState = ProtocolState::Login;

    #[inline(always)]
    fn buffer(&mut self) -> &mut VecDeque<RawPacket> {
        &mut self.0
    }
}

/// See [`State::Play`] for info.
#[derive(Component, Debug, Default)]
pub struct Play(VecDeque<RawPacket>);

impl StateBuffer for Play {
    const STATE: ProtocolState = ProtocolState::Play;

    #[inline(always)]
    fn buffer(&mut self) -> &mut VecDeque<RawPacket> {
        &mut self.0
    }
}

#[derive(WorldQuery)]
#[world_query(mutable, derive(Debug))]
pub struct Socket<'w, S: StateBuffer> {
    raw: &'w RawSocket,
    state: &'w mut S,
    codec: &'w PacketCodec,
}

impl<'w, S: StateBuffer> SocketItem<'w, S> {
    #[inline]
    pub fn id(&self) -> SocketId {
        self.raw.id()
    }

    pub fn recv<T: Packet + McRead>(&mut self) -> Option<T> {
        let id = match self.codec.incoming.get::<T>(S::STATE) {
            Some(id) => id,
            None => {
                tracing::warn!(
                    "tried to receive unregistered packet {:?}",
                    type_name::<T>()
                );
                return None;
            }
        };
        let raw = self.recv_raw(id)?;

        match T::decode(&raw.body) {
            Ok(packet) => Some(packet),
            Err(e) => {
                tracing::warn!("failed to decode packet: {:?}", e);
                None
            }
        }
    }

    fn recv_raw(&mut self, id: i32) -> Option<RawPacket> {
        // First, check recv_buf
        for i in 0..self.state.buffer().len() {
            if self.state.buffer()[i].id == id {
                // matched: remove and return
                return self.state.buffer().remove(i);
            }
        }
        // Second, check socket io
        for raw in self.raw.iter() {
            if raw.id == id {
                // matched: return
                return Some(raw);
            } else {
                // not one we're looking for, lets store it for later
                self.state.buffer().push_front(raw);
            }
        }
        // No matching packet found
        None
    }

    pub fn send<T: Packet + McWrite>(&mut self, packet: T) -> FlushGuard<'_, 'w, S> {
        let id = match self.codec.outgoing.get::<T>(S::STATE) {
            Some(id) => id,
            None => {
                tracing::warn!("tried to send unregistered packet: {}", type_name::<T>());
                return FlushGuard(self);
            }
        };

        tracing::trace!("sending {} packet to client {}", type_name::<T>(), self.id());

        match T::encode(&packet) {
            Ok(body) => {
                self.raw.send(RawPacket { id, body });
            }
            Err(e) => {
                tracing::warn!("failed to encode packet: {:?}", e);
            }
        }

        FlushGuard(self)
    }

    #[inline]
    pub fn flush(&mut self) {
        self.raw.flush();
    }
}

impl<'w, S: StateBuffer> fmt::Display for Socket<'w, S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.raw.id().fmt(f)
    }
}

/// Guard type that flushes all packets on drop.
pub struct FlushGuard<'a, 'w, S: StateBuffer>(&'a SocketItem<'w, S>);

impl<'w, S: StateBuffer> Drop for FlushGuard<'_, 'w, S> {
    #[inline]
    fn drop(&mut self) {
        self.0.raw.flush();
    }
}

impl<'w, S: StateBuffer> Deref for FlushGuard<'_, 'w, S> {
    type Target = SocketItem<'w, S>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
