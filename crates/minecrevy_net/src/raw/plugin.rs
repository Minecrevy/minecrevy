use bevy::prelude::*;

use crate::raw::server::RawServer;

/// Inserts the [`RawServer`] resource and adds the neccessary systems for it to receive connections.
#[derive(Default)]
pub struct RawNetworkPlugin;

impl Plugin for RawNetworkPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(RawServer::default());

        app.add_systems(
            PreUpdate,
            Self::accept_clients.run_if(Self::is_accepting_clients),
        );
    }
}

impl RawNetworkPlugin {
    /// A [`Condition`] that returns `true` if the [`RawServer`] is accepting new clients.
    pub fn is_accepting_clients(server: Res<RawServer>) -> bool {
        server.is_listening()
    }

    /// A [`System`] that accepts new [`RawClient`]s from the [`RawServer`] and spawns them as entities.
    ///
    /// [`RawClient`]: crate::raw::RawClient
    pub fn accept_clients(mut commands: Commands, server: Res<RawServer>) {
        while let Some(client) = server.try_accept() {
            commands.spawn(client);
        }
    }
}
