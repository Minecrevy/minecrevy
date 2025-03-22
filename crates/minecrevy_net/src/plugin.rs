use bevy_app::{App, Plugin, PreUpdate};
use bevy_ecs::{
    component::Component,
    entity::Entity,
    system::{Commands, Query},
};

use crate::server::{IncomingServerEvent, Server};

pub struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, Self::spawn_clients);
    }
}

impl ServerPlugin {
    pub fn spawn_clients(mut commands: Commands, servers: Query<(Entity, &Server)>) {
        for (entity, server) in servers.iter() {
            for event in server.incoming() {
                match event {
                    IncomingServerEvent::Connected(client) => {
                        commands.spawn((client, ConnectedTo(entity)));
                    }
                    IncomingServerEvent::Error(error) => {
                        tracing::error!(server = %server.addr(), "Server error: {}", error);
                        commands.entity(entity).despawn();
                    }
                }
            }
        }
    }
}

#[derive(Component)]
#[relationship(relationship_target = Connections)]
pub struct ConnectedTo(pub Entity);

#[derive(Component)]
#[relationship_target(relationship = ConnectedTo)]
pub struct Connections(Vec<Entity>);
