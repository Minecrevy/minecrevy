use bevy::prelude::*;
use tokio::runtime::Runtime;

use minecrevy_config::Config;
use minecrevy_ecs::label::Networking;
use minecrevy_protocol_latest::server::codec;
use minecrevy_protocol_latest::PacketCodec;
use minecrevy_tcp::{Server, ServerEvent};

use crate::disconnect::Disconnect;
use crate::socket::{Handshake, Login, Play, RawSocket};

pub struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        let runtime = Runtime::new().expect("failed to create network server runtime");

        app.insert_resource(Server::new(runtime));
        app.insert_resource(codec());

        app.add_startup_system(Self::listen);

        app.add_system(Self::handle_events.label(Networking));
        app.add_system(Disconnect::system::<Login>.before(Networking));
        app.add_system(Disconnect::system::<Play>.before(Networking));
    }
}

impl ServerPlugin {
    fn listen(config: Res<Config>, mut server: ResMut<Server>) {
        server.listen(config.network.address.clone());
    }

    fn handle_events(
        mut commands: Commands,
        server: Res<Server>,
        codec: Res<PacketCodec>,
        conns: Query<(Entity, &RawSocket)>,
    ) {
        for event in server.events() {
            match event {
                ServerEvent::Bind(Ok(addr)) => {
                    tracing::info!("network server bound to {}", addr);
                }
                ServerEvent::Bind(Err(e)) => {
                    tracing::error!("failed to bind network server: {}", e);
                }
                ServerEvent::Accept(stream, addr) => {
                    let socket = RawSocket::new(&server, stream, addr);

                    tracing::info!("client connected: {}", socket.id());

                    commands
                        .spawn()
                        .insert(socket)
                        .insert(Handshake::default())
                        .insert(codec.clone());
                }
                ServerEvent::Disconnect(id) => {
                    for (entity, socket) in conns.iter() {
                        if socket.id() == id {
                            commands.entity(entity).insert(Disconnect::default());

                            tracing::info!("client disconnected: {}", socket.id());
                            break;
                        }
                    }
                }
                ServerEvent::Close => {
                    for (entity, socket) in conns.iter() {
                        commands.entity(entity).insert(Disconnect::default());

                        tracing::info!("client disconnected: {}", socket.id());
                    }

                    tracing::info!("network server closed, stopping all active connections");
                }
            }
        }
    }
}
