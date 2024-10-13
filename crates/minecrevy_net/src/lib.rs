//! A Minecraft networking library, integrated with [Bevy](bevy).

#![warn(missing_docs)]

use std::fmt;

use bevy::{app::PluginGroupBuilder, prelude::*};
use minecrevy_io::{McRead, McWrite};
use tokio::net::ToSocketAddrs;

use crate::{
    client::ProtocolState,
    packet::{IncomingPacketHandlers, OutgoingPacketIds},
    server::{Server, ServerPlugin},
};

pub mod client;
pub mod packet;
pub mod server;

/// [`PluginGroup`] for the [`NetworkPlugin`] and [`ServerPlugin`].
pub struct NetworkServerPlugins;

impl PluginGroup for NetworkServerPlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>().add(ServerPlugin)
    }
}

/// Extension trait for [`App`] to add network-related functionality.
pub trait AppNetworkExt {
    /// Registers the given incoming packet type with the given [`ProtocolState`] and packet ID.
    ///
    /// If `stateful` is `true`, then the client will be paused until the packet is handled.
    fn add_incoming_packet<T: McRead + Send + Sync + 'static>(
        &mut self,
        state: ProtocolState,
        id: i32,
    ) -> &mut Self;

    /// Registers the given outgoing packet type with the given [`ProtocolState`] and packet ID.
    fn add_outgoing_packet<T: McWrite + Send + Sync + 'static>(
        &mut self,
        state: ProtocolState,
        id: i32,
    ) -> &mut Self;
}

impl AppNetworkExt for App {
    fn add_incoming_packet<T: McRead + Send + Sync + 'static>(
        &mut self,
        state: ProtocolState,
        id: i32,
    ) -> &mut Self {
        let mut handlers = self
            .world_mut()
            .get_resource_or_init::<IncomingPacketHandlers>();
        handlers.insert::<T>(state, id);

        self
    }

    fn add_outgoing_packet<T: McWrite + Send + Sync + 'static>(
        &mut self,
        state: ProtocolState,
        id: i32,
    ) -> &mut Self {
        let mut ids = self.world_mut().get_resource_or_init::<OutgoingPacketIds>();
        ids.insert::<T>(state, id);

        self
    }
}

/// [`System`] supplier that tells the [`Server`](server::Server) to start listening for connections.
pub fn start_server(
    address: impl ToSocketAddrs + Clone + fmt::Display + Send + 'static,
) -> impl FnMut(ResMut<Server>) {
    move |mut server: ResMut<Server>| {
        server.start(address.clone());
    }
}
