//! This module contains the [`LoginPlugin`], which handles login packets.

use bevy::prelude::*;
use minecrevy_net::{
    client::{ClientQ, ClientQReadOnly, Paused, ProtocolState},
    packet::Recv,
};
use minecrevy_protocol::{
    login::{LoginAcknowledged, LoginStart, LoginSuccess},
    PacketHandlerSet, ServerProtocolPlugin,
};

use crate::{config::EnterConfig, profile::Profile, CorePlugin};

/// [`Plugin`] for handling login packets.
#[derive(Default)]
pub struct LoginPlugin;

impl Plugin for LoginPlugin {
    fn build(&self, app: &mut App) {
        assert!(
            app.is_plugin_added::<ServerProtocolPlugin>(),
            "{} must be added before {}",
            std::any::type_name::<ServerProtocolPlugin>(),
            std::any::type_name::<Self>(),
        );
        assert!(
            app.is_plugin_added::<CorePlugin>(),
            "{} must be added before {}",
            std::any::type_name::<CorePlugin>(),
            std::any::type_name::<Self>(),
        );

        app.add_systems(
            Update,
            (
                Self::handle_login_starts,
                apply_deferred,
                Self::handle_login_acks,
            )
                .chain()
                .in_set(PacketHandlerSet::Login),
        );
    }
}

impl LoginPlugin {
    /// [`System`] that handles the beginning of the login process.
    pub fn handle_login_starts(
        mut commands: Commands,
        mut starts: EventReader<Recv<LoginStart>>,
        clients: Query<ClientQReadOnly>,
    ) {
        for Recv {
            client: client_ent,
            packet,
        } in starts.read()
        {
            let Ok(client) = clients.get(*client_ent) else {
                continue;
            };

            trace!("{} entered login state", client.client.addr());

            commands.entity(*client_ent).insert(Profile {
                name: packet.username.clone(),
                uuid: packet.uuid,
                properties: Vec::new(),
            });

            client.send(LoginSuccess {
                uuid: packet.uuid,
                username: packet.username.clone(),
                properties: Vec::new(),
            });
        }
    }

    /// [`System`] that handles the end of the login process.
    pub fn handle_login_acks(
        mut acks: EventReader<Recv<LoginAcknowledged>>,
        mut enter_configs: EventWriter<EnterConfig>,
        mut clients: Query<(ClientQ, &mut Paused), With<Profile>>,
    ) {
        for Recv {
            client: client_ent,
            packet: _,
        } in acks.read()
        {
            let Ok((mut client, mut paused)) = clients.get_mut(*client_ent) else {
                continue;
            };

            *client.state = ProtocolState::Config;
            paused.0 = false;

            enter_configs.send(EnterConfig {
                client: *client_ent,
            });
        }
    }
}
