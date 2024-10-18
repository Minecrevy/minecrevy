//! This module contains the [`PlayPlugin`], which handles the play state.

use bevy::{math::DVec3, prelude::*};
use enumflags2::BitFlags;
use minecrevy_net::client::PacketWriter;
use minecrevy_protocol::play::{GameEvent, Login, SyncPlayerPosition};

use crate::PlayerCount;

/// [`Event`] that's triggered when a client has entered the play state.
#[derive(Event, Clone, Copy, PartialEq, Eq, Debug)]
pub struct EnterPlay;

/// [`Plugin`] for handling the play state.
#[derive(Default)]
pub struct PlayPlugin;

impl Plugin for PlayPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(Self::on_enter_play);
    }
}

impl PlayPlugin {
    /// [`Observer`] [`System`] that handles the play state being entered.
    pub fn on_enter_play(
        trigger: Trigger<EnterPlay>,
        mut writer: PacketWriter,
        players: Res<PlayerCount>,
    ) {
        let client = trigger.entity();

        writer
            .client(client)
            .send(&Login {
                entity_id: 0,                                               // TODO
                is_hardcore: false,                                         // TODO
                dimensions: Vec::from_iter(["minecraft:overworld".into()]), // TODO
                max_players: players.max,
                view_distance: 10,                            // TODO
                simulation_distance: 10,                      // TODO
                reduced_debug_info: false,                    // TODO
                enable_respawn_screen: true,                  // TODO
                do_limited_crafting: false,                   // TODO
                dimension_type: 0,                            // TODO
                dimension_name: "minecraft:overworld".into(), // TODO
                hashed_seed: 0,                               // TODO
                game_mode: 2,                                 // TODO
                previous_game_mode: -1,                       // TODO
                is_debug: false,                              // TODO
                is_flat: false,                               // TODO
                death_dimension_name_and_location: None,      // TODO
                portal_cooldown: 0,                           // TODO
                enforces_secure_chat: false,                  // TODO
            })
            // TODO: seems to prevent getting past the loading screen
            // .send(&SyncPlayerAbilities {
            //     flags: PlayerAbilities::Flying | PlayerAbilities::AllowFlying, // TODO
            //     flying_speed: 0.05,                                            // TODO
            //     fov_modifier: 0.0,                                             // TODO
            // })
            .send(&GameEvent::WAIT_ON_CHUNKS)
            .send(&SyncPlayerPosition {
                position: DVec3::ZERO,      // TODO
                rotation: Vec2::ZERO,       // TODO
                flags: BitFlags::default(), // TODO
                teleport_id: 0,             // TODO
            });
    }
}
