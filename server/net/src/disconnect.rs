use bevy::prelude::*;

use minecrevy_protocol_latest::server;
use minecrevy_text::{Style, Text};

use crate::socket::{Socket, StateBuffer};

#[derive(Clone, Debug, Component)]
pub struct Disconnect(pub Text);

impl Disconnect {
    pub const DEFAULT_REASON: Text = Text::str("connection reset by peer", Style::empty());
}

impl Default for Disconnect {
    fn default() -> Self {
        Self(Self::DEFAULT_REASON)
    }
}

impl Disconnect {
    pub(crate) fn system<S: StateBuffer>(
        mut commands: Commands,
        mut clients: Query<(Entity, Socket<S>, &Disconnect), Added<Disconnect>>,
    ) {
        for (entity, mut socket, disconnect) in clients.iter_mut() {
            let reason = disconnect.0.clone();

            socket.send(server::Disconnect(reason));

            commands.entity(entity).despawn(); // TODO: despawn_recursive?
        }
    }
}
