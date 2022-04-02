use bevy::prelude::*;
use bevy::utils::tracing;
use flexstr::SharedStr;

use minecrevy_auth::Profile;
use minecrevy_ecs::label::Networking;
use minecrevy_net::socket::{Handshake, Login, Play, Socket, Status};
use minecrevy_protocol_latest::client::NextState;
use minecrevy_protocol_latest::server::Motd;
use minecrevy_protocol_latest::{client, server};

/// Initial connection info sent by the client in the [`client::Handshake`] packet.
#[derive(Component, Clone, Debug)]
pub struct ConnectionInfo {
    // TODO: strongly typed version number
    /// The protocol version that the client is connecting with.
    pub version: i32,
    /// The address specified by the client in the address bar.
    pub address: SharedStr,
    /// The port specified by the client in the address bar.
    pub port: u16,
}

/// Handles any logic related to a client's *connection*, rather than game features.
/// This includes [`Handshake`] transitions, [`Status`] pings, and [`Logic`] flow.
pub struct ConnectionPlugin;

impl Plugin for ConnectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(Self::handle_handshake.label(Networking));

        app.insert_resource(Motd::default());
        app.add_system(Self::handle_status.label(Networking));

        app.add_system(Self::handle_login.label(Networking));
    }
}

impl ConnectionPlugin {
    /// Performs [handshake][1] transitions.
    ///
    /// [1]: https://wiki.vg/Protocol#Handshaking
    pub fn handle_handshake(
        mut commands: Commands,
        mut clients: Query<(Entity, Socket<Handshake>)>,
    ) {
        for (entity, mut socket) in clients.iter_mut() {
            if let Some(packet) = socket.recv::<client::Handshake>() {
                tracing::debug!("client {} transitioning to {:?}", socket.id(), packet.next);

                let mut entity = commands.entity(entity);
                entity.remove::<Handshake>();
                entity.insert(ConnectionInfo {
                    version: packet.version,
                    address: packet.address,
                    port: packet.port,
                });

                match packet.next {
                    NextState::Status => {
                        entity.insert(Status::default());
                    }
                    NextState::Login => {
                        entity.insert(Login::default());
                    }
                }
            }
        }
    }

    /// Handles [server list ping][1] logic.
    ///
    /// See [`Self::handle_handshake`] for handshake logic.
    ///
    /// [1]: https://wiki.vg/Server_List_Ping
    pub fn handle_status(motd: Res<Motd>, mut clients: Query<Socket<Status>>) {
        for mut socket in clients.iter_mut() {
            if let Some(_) = socket.recv::<client::StatusRequest>() {
                socket.send(server::StatusResponse(motd.as_ref().clone()));
            }
            if let Some(packet) = socket.recv::<client::StatusPing>() {
                // type alias, cast added for clarity
                socket.send(packet as server::StatusPong);
            }
        }
    }

    /// Performs [login][1] flow.
    ///
    /// See [`Self::handle_handshake`] for handshake logic.
    ///
    /// [1]: https://wiki.vg/Protocol#Login
    pub fn handle_login(mut commands: Commands, mut clients: Query<(Entity, Socket<Login>)>) {
        for (entity, mut socket) in clients.iter_mut() {
            // TODO: support encryption and compression

            if let Some(packet) = socket.recv::<client::LoginStart>() {
                let profile = Profile::new_offline(packet.name.clone());

                socket.send(dbg!(server::LoginSuccess {
                    id: profile.id(),
                    name: packet.name,
                }));

                commands
                    .entity(entity)
                    .remove::<Login>()
                    .insert(Play::default())
                    .insert(profile);

                tracing::debug!("client {} transitioning to Play", socket.id());
            }
        }
    }
}
