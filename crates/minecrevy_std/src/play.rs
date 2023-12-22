//! This module contains the [`PlayPlugin`], which handles play packets.

use bevy::prelude::*;
use minecrevy_net::client::ClientQReadOnly;
use minecrevy_protocol::{
    play::{GameMode, Login, PreviousGameMode, SetDefaultSpawnPosition, SynchronizePlayerPosition},
    PacketHandlerSet, ServerProtocolPlugin,
};

use crate::{CorePlugin, PlayerCount};

/// [`Plugin`] for handling play packets.
pub struct PlayPlugin;

impl Plugin for PlayPlugin {
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

        app.add_event::<EnterPlay>();

        app.add_systems(
            Update,
            (Self::handle_enter_play,)
                .chain()
                .in_set(PacketHandlerSet::Play),
        );
    }
}

impl PlayPlugin {
    /// [`System`] that handles the beginning of the play state.
    pub fn handle_enter_play(
        counts: Res<PlayerCount>,
        mut enters: EventReader<EnterPlay>,
        clients: Query<ClientQReadOnly>,
    ) {
        for EnterPlay { client } in enters.read() {
            let Ok(client) = clients.get(*client) else {
                continue;
            };

            trace!("{} entered play state", client.client.addr());

            client
                .send(Login {
                    entity_id: 1,                                             // TODO
                    is_hardcore: false,                                       // TODO: configurable
                    dimensions: vec!["minecraft:overworld".parse().unwrap()], // TODO
                    max_players: counts.max,                                  //
                    view_distance: 8,                                         // TODO: configurable
                    simulation_distance: 8,                                   // TODO: configurable
                    reduced_debug_info: false,                                // TODO: configurable
                    enable_respawn_screen: true,                              // TODO: configurable
                    do_limited_crafting: false,                               // TODO: configurable
                    dimension_type: "minecraft:overworld".parse().unwrap(),   // TODO
                    dimension_name: "minecraft:overworld".parse().unwrap(),   // TODO
                    hashed_seed: 0,                                           // TODO
                    game_mode: GameMode::Creative,                            // TODO
                    previous_game_mode: PreviousGameMode(None),               // TODO
                    is_debug: false,                                          // TODO
                    is_flat: false,                                           // TODO
                    death_location: None,                                     // TODO
                    portal_cooldown: 0,                                       // TODO
                })
                .send(SynchronizePlayerPosition {
                    x: 0.,
                    y: 5000.,
                    z: 0.,
                    yaw: 0.,
                    pitch: 0.,
                    flags: 0,
                    teleport_id: 0, // TODO
                })
                .send(SetDefaultSpawnPosition {
                    location: IVec3 { x: 0, y: 64, z: 0 },
                    angle: 0.,
                });
        }
    }
}

/// [`Event`] that indicates a client has entered the play state.
#[derive(Event)]
pub struct EnterPlay {
    /// The client that entered the play state.
    pub client: Entity,
}
