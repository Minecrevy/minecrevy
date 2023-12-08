//! This module contains the [`HandshakePlugin`], which handles handshake packets.

use std::ops::Deref;

use bevy::prelude::*;
use minecrevy_net::{
    client::{Client, Paused, ProtocolState},
    packet::{PacketIds, Recv},
};
use minecrevy_protocol::{
    handshake::Handshake, login::Disconnect, PacketHandlerSet, ServerProtocolPlugin,
};
use minecrevy_text::Text;

/// [`Plugin`] that handles the Minecraft protocol handshake.
pub struct HandshakePlugin {
    /// Whether or not to allow clients to log in.
    pub allow_login: bool,
}

impl Plugin for HandshakePlugin {
    fn build(&self, app: &mut App) {
        assert!(
            app.is_plugin_added::<ServerProtocolPlugin>(),
            "{} must be added before {}",
            std::any::type_name::<ServerProtocolPlugin>(),
            std::any::type_name::<Self>(),
        );

        app.insert_resource(AllowLogin(self.allow_login));

        app.add_systems(
            Update,
            Self::handle_handshakes.in_set(PacketHandlerSet::Handshake),
        );
    }
}

impl HandshakePlugin {
    /// [`System`] that handles [`Handshake`] packets.
    pub fn handle_handshakes(
        allow_login: Res<AllowLogin>,
        ids: Res<PacketIds>,
        mut commands: Commands,
        mut packets: EventReader<Recv<Handshake>>,
        mut clients: Query<(&Client, &mut ProtocolState, &mut Paused)>,
    ) {
        for Recv {
            client: client_ent,
            packet,
        } in packets.read()
        {
            let Ok((client, mut state, mut paused)) = clients.get_mut(*client_ent) else {
                continue;
            };

            *state = match packet.next_state {
                1 => ProtocolState::Status,
                2 => ProtocolState::Login,
                _ => {
                    // unknown state
                    continue;
                }
            };

            if *state == ProtocolState::Login && !allow_login.0 {
                client.send(
                    &*ids,
                    &state,
                    Disconnect {
                        reason: Text::from("Logins are disabled."),
                    },
                );
                continue;
            }

            paused.0 = false;

            commands.entity(*client_ent).insert((
                ids.deref().clone(),
                ClientInfo {
                    protocol_version: packet.protocol_version,
                    server_address: packet.server_address.clone(),
                    server_port: packet.server_port,
                },
            ));
        }
    }
}

/// [`Component`] that stores information about the client's handshake.
#[derive(Component)]
pub struct ClientInfo {
    /// The protocol version of the client.
    pub protocol_version: i32,
    /// The address of the server the client is connecting to.
    pub server_address: String,
    /// The port of the server the client is connecting to.
    pub server_port: u16,
}

/// [`Resource`] that stores whether or not clients are allowed to log in.
#[derive(Resource, Deref, DerefMut)]
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct AllowLogin(pub bool);
