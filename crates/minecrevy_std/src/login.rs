//! This module contains the [`LoginPlugin`], which handles the login process.

use bevy::prelude::*;

use minecrevy_net::{
    client::{PacketWriter, ProtocolState},
    packet::Recv,
};
use minecrevy_protocol::login::{Acknowledged, Start, Success};
use uuid::Uuid;

use crate::config::EnterConfig;

/// [`Plugin`] for handling the login process.
#[derive(Default)]
pub struct LoginPlugin;

impl Plugin for LoginPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(Self::on_login_start);
        app.add_observer(Self::on_login_acknowledged);
    }
}

impl LoginPlugin {
    /// [`Observer`] [`System`] that handles incoming [`Start`] packets.
    pub fn on_login_start(
        trigger: Trigger<Recv<Start>>,
        mut writer: PacketWriter,
        mut commands: Commands,
    ) {
        let packet = &trigger.event().0;
        let client = trigger.entity();

        commands.entity(client).insert(UserProfile {
            uuid: packet.uuid,
            name: packet.username.clone(),
        });

        writer.send(
            client,
            &Success {
                uuid: packet.uuid,
                username: packet.username.clone(),
                properties: Vec::new(),
                strict_error_handling: true,
            },
        );
    }

    /// [`Observer`] [`System`] that handles incoming [`Acknowledged`] packets.
    pub fn on_login_acknowledged(
        trigger: Trigger<Recv<Acknowledged>>,
        mut writer: PacketWriter,
        mut commands: Commands,
    ) {
        let client = trigger.entity();

        writer.client(client).set_state(ProtocolState::Config);
        commands.entity(client).trigger(EnterConfig {
            previous_state: ProtocolState::Login,
        });
    }
}

/// The user profile of a player.
#[derive(Component, Clone, PartialEq, Eq, Debug)]
pub struct UserProfile {
    /// The UUID of the user.
    pub uuid: Uuid,
    /// The name of the user, cannot exceed 16 characters.
    pub name: String,
}
