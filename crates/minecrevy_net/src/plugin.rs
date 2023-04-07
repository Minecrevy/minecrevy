use bevy::{app::PluginGroupBuilder, prelude::*};

use crate::{
    flow::{handshake::HandshakeFlowPlugin, login::LoginFlowPlugin, status::StatusFlowPlugin},
    packet::PacketsPlugin,
    raw::RawServer,
};

/// This [`PluginGroup`] will add all the default networking plugins needed to handle
/// pre-gameplay Minecraft protocol flow. Gameplay flow is specifically excluded as it
/// requires a much larger breadth of features to fully implement.
///
/// See [`MinimalNetworkPlugins`] if interested in providing a custom Status and Login flow.
pub struct DefaultNetworkPlugins;

impl PluginGroup for DefaultNetworkPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(PacketsPlugin)
            .add(ServerPlugin)
            .add(HandshakeFlowPlugin)
            .add(StatusFlowPlugin)
            .add(LoginFlowPlugin)
    }
}

/// This [`PluginGroup`] will add the minimal networking plugins required to handle
/// Minecraft protocol handshake flow. Everything else after (status and login)
/// are left for implementation.
///
/// See [`DefaultNetworkPlugins`] if interested in a more holistic builtin pre-gameplay flow.
pub struct MinimalNetworkPlugins;

impl PluginGroup for MinimalNetworkPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(PacketsPlugin)
            .add(ServerPlugin)
            .add(HandshakeFlowPlugin)
    }
}

/// Inserts the [`RawServer`] resource and adds the neccessary systems for it to receive connections.
#[derive(Default)]
pub struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(RawServer::default());

        app.add_systems(
            PreUpdate,
            RawServer::accept_clients.run_if(RawServer::is_accepting_connections),
        );
    }
}

impl ServerPlugin {
    /// A [`Condition`] that returns `true` if the [`RawServer`] is accepting new clients.
    pub fn is_accepting_clients(server: Res<RawServer>) -> bool {
        server.is_listening()
    }

    /// A [`System`] that accepts new [`RawClient`]s from the [`RawServer`] and spawns them as entities.
    ///
    /// [`RawClient`]: crate::raw::RawClient
    pub fn accept_clients(mut commands: Commands, server: Res<RawServer>) {
        while let Some(client) = server.accept() {
            commands.spawn(client);
        }
    }
}
