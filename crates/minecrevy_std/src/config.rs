//! This module contains the [`ConfigPlugin`], which handles config packets.

use std::sync::OnceLock;

use bevy::prelude::*;
use minecrevy_net::{
    client::{ClientQ, ClientQReadOnly, Paused, ProtocolState},
    packet::Recv,
};
use minecrevy_protocol::{
    config::{ClientInformation, Finish, RegistryData},
    PacketHandlerSet, ServerProtocolPlugin,
};

use crate::{play::EnterPlay, profile::Profile, CorePlugin};

/// [`Plugin`] for handling config packets.
pub struct ConfigPlugin;

impl Plugin for ConfigPlugin {
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

        app.add_event::<EnterConfig>();

        app.add_systems(
            Update,
            (Self::handle_enter_config, Self::handle_finish)
                .chain()
                .in_set(PacketHandlerSet::Config),
        );
    }
}

impl ConfigPlugin {
    /// [`System`] that handles the beginning of the config state.
    pub fn handle_enter_config(
        mut enters: EventReader<EnterConfig>,
        clients: Query<ClientQReadOnly>,
    ) {
        static DEFAULT_REGISTRY_JSON: &'static str =
            include_str!("../../../assets/default_registry.json");
        static DEFAULT_REGISTRY: OnceLock<RegistryData> = OnceLock::new();

        for EnterConfig { client } in enters.read() {
            let Ok(client) = clients.get(*client) else {
                continue;
            };

            trace!("{} entered config state", client.client.addr());

            let registry_data = DEFAULT_REGISTRY
                .get_or_init(|| serde_json::from_str(DEFAULT_REGISTRY_JSON).unwrap());

            client.send(registry_data.clone()).send(Finish);
        }
    }

    /// [`System`] that handles the client information packet.
    pub fn handle_client_info(mut infos: EventReader<Recv<ClientInformation>>) {
        for Recv {
            client: _,
            packet: _,
        } in infos.read()
        {}
    }

    /// [`System`] that handles the end of the config process.
    pub fn handle_finish(
        mut finishes: EventReader<Recv<Finish>>,
        mut enter_plays: EventWriter<EnterPlay>,
        mut clients: Query<(ClientQ, &mut Paused), With<Profile>>,
    ) {
        for Recv {
            client: client_ent,
            packet: _,
        } in finishes.read()
        {
            let Ok((mut client, mut paused)) = clients.get_mut(*client_ent) else {
                continue;
            };

            *client.state = ProtocolState::Play;
            paused.0 = false;

            enter_plays.send(EnterPlay {
                client: *client_ent,
            });
        }
    }
}

/// [`Event`] that indicates a client has entered the config state.
#[derive(Event)]
pub struct EnterConfig {
    /// The client that entered the config state.
    pub client: Entity,
}
