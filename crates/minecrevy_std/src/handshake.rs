//! This module contains the [`HandshakePlugin`], which handles handshake packets.

use bevy::prelude::*;
use minecrevy_net::{
    client::{PacketWriter, ProtocolState},
    packet::Recv,
};
use minecrevy_protocol::{handshake::Handshake, login::Disconnect};
use minecrevy_text::Text;

/// [`Plugin`] that handles the Minecraft protocol handshake.
///
/// Configurable [`Resource`]s:
/// - [`AllowLogin`]: Whether or not clients are allowed to log in.
pub struct HandshakePlugin;

impl Plugin for HandshakePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AllowLogin>();

        app.add_observer(Self::on_handshake);
    }
}

impl HandshakePlugin {
    /// [`Observer`] [`System`] that handles incoming handshake packets.
    pub fn on_handshake(
        trigger: Trigger<Recv<Handshake>>,
        mut writer: PacketWriter,
        allow_login: Res<AllowLogin>,
        mut commands: Commands,
    ) {
        let packet = &trigger.event().0;
        let mut writer = writer.client(trigger.entity());

        writer.set_state(match packet.next_state {
            1 => ProtocolState::Status,
            2 => ProtocolState::Login,
            // unknown state
            _ => return,
        });

        if writer.state() == ProtocolState::Login {
            if let Err(reason) = &allow_login.0 {
                writer.send(&Disconnect {
                    reason: reason
                        .clone()
                        .unwrap_or_else(|| Text::from("Logins are disabled.")),
                });
                commands.entity(trigger.entity()).despawn();
                return;
            }
        }

        commands.entity(trigger.entity()).insert(ConnectionInfo {
            protocol_version: packet.protocol_version,
            server_address: packet.server_address.clone(),
            server_port: packet.server_port,
        });
    }
}

/// [`Component`] that stores information about the client's handshake.
#[derive(Component)]
pub struct ConnectionInfo {
    /// The protocol version of the client.
    pub protocol_version: i32,
    /// The address of the server the client is connecting to.
    pub server_address: String,
    /// The port of the server the client is connecting to.
    pub server_port: u16,
}

/// [`Resource`] that stores whether or not clients are allowed to log in.
#[derive(Resource, Deref, DerefMut)]
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AllowLogin(pub Result<(), Option<Text>>);

impl Default for AllowLogin {
    fn default() -> Self {
        Self(Ok(()))
    }
}
