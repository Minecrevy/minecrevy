use bevy::prelude::*;
use bevy::utils::tracing;
use nanorand::{Rng, WyRand};

use minecrevy_ecs::label::Networking;
use minecrevy_io_str::Nbt;
use minecrevy_key::key;
use minecrevy_math::vector::Vector;
use minecrevy_net::disconnect::Disconnect;
use minecrevy_net::socket::{Play, Socket};
use minecrevy_protocol_latest::types::PreviousGameMode;
use minecrevy_protocol_latest::{client, server};
use minecrevy_util::{GameMode, MainHand};

use crate::entity::living::LivingEntityBundle;
use crate::entity::player::{ClientSettings, KeepAlive, PlayerBundle, SpawnPosition};
use crate::entity::{EntityBundle, Position, Rotation};
use crate::util::ChangeTracked;
use crate::world::{default_dimension_registry, default_dimension_type};

/// Also includes keep-alive handling during [`Play`].
pub struct PlayPlugin;

impl Plugin for PlayPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(Self::finish_login.label(Networking));

        app.insert_resource(WyRand::new()); // TODO: move to a 'core' plugin
        app.add_system(Self::handle_keepalive.label(Networking));

        app.add_system(Self::save_client_settings.label(Networking));
        app.add_system(Self::update_spawn_position.label(Networking));
        app.add_system(Self::update_position_and_look.label(Networking));
        app.add_system(Self::update_gamemode.label(Networking));
    }
}

impl PlayPlugin {
    /// Performs player initialization when they have finished logging in.
    ///
    /// See [`crate::entity::player::conn::ConnectionPlugin::handle_login`] for login logic.
    pub fn finish_login(
        mut commands: Commands,
        mut players: Query<(Entity, Socket<Play>), Added<Play>>,
    ) {
        for (entity, mut socket) in players.iter_mut() {
            let player = PlayerBundle {
                living: LivingEntityBundle {
                    entity: EntityBundle {
                        ..Default::default()
                    },
                    ..Default::default()
                },
                ..Default::default()
            };

            socket.send(server::JoinGame {
                id: player.living.entity.id.val(),
                hardcore: false,
                gamemode: player.gamemode,
                previous_gamemode: PreviousGameMode(None),
                worlds: vec![],
                dimension_registry: Nbt(default_dimension_registry().clone()),
                dimension_type: Nbt(default_dimension_type().clone()),
                world: key!("minecraft:overworld"),
                seed: 0,
                max_players: 20,
                view_dst: 8,
                sim_dst: 8,
                reduced_debug_info: false,
                respawn_screen: true,
                debug: false,
                flat: false,
            });
            socket.send(server::PluginMessage::brand("minecrevy"));

            commands.entity(entity).insert_bundle(player);
        }
    }

    fn handle_keepalive(
        mut commands: Commands,
        mut rng: ResMut<WyRand>,
        time: Res<Time>,
        mut clients: Query<(Entity, Socket<Play>, &mut KeepAlive)>,
    ) {
        for (entity, mut socket, mut keepalive) in clients.iter_mut() {
            // Tick the client's keep-alive timer.
            keepalive.timer.tick(time.delta());

            match keepalive.id {
                Some(id) => {
                    if keepalive.timer.just_finished() {
                        // client failed to respond
                        commands
                            .entity(entity)
                            .insert(Disconnect(KeepAlive::FAILED));
                    } else if let Some(packet) = socket.recv::<client::KeepAlive>() {
                        if packet.0 == id {
                            // client responded correctly
                            keepalive.id = None;
                        } else {
                            // client responded with wrong id
                            tracing::warn!(
                                "client {} responded with wrong keep-alive id {} (expected: {})",
                                socket.id(),
                                packet.0,
                                id
                            );
                        }
                    }
                }
                None => {
                    if keepalive.timer.just_finished() {
                        // send out a new keep-alive packet
                        let id = rng.generate::<i64>();
                        socket.send(server::KeepAlive(id));
                        keepalive.id = Some(id);
                    } else {
                        // client has already responded, just waiting to send the next keep-alive.
                    }
                }
            }
        }
    }

    pub fn save_client_settings(
        mut commands: Commands,
        mut players: Query<(Entity, Socket<Play>, &mut MainHand), Without<ClientSettings>>,
    ) {
        for (entity, mut socket, mut main_hand) in players.iter_mut() {
            if let Some(packet) = socket.recv::<client::ClientSettings>() {
                *main_hand = packet.main_hand;

                commands.entity(entity).insert(ClientSettings {
                    locale: packet.locale,
                    view_dst: packet.view_dst,
                    visibility: packet.visibility,
                    colors: packet.chat_colors,
                    skin_parts: packet.skin_parts,
                    main_hand: packet.main_hand,
                    filter_text: packet.filter_text,
                    shown_on_tablist: packet.shown_on_tablist,
                });
            }
        }
    }

    /// Performs [change detection][1] for a player's [`SpawnPosition`] component,
    /// and sends a [`server::SpawnPosition`] packet if it changed.
    ///
    /// [1]: https://bevy-cheatbook.github.io/programming/change-detection.html
    pub fn update_spawn_position(
        mut players: Query<(Socket<Play>, &SpawnPosition), Changed<SpawnPosition>>,
    ) {
        for (mut socket, spawn_position) in players.iter_mut() {
            socket.send(server::SpawnPosition {
                position: spawn_position.position,
                pitch: spawn_position.pitch,
            });
        }
    }

    /// Performs [change detection][1] for a player's [`Position`] and [`Rotation`] components,
    /// and sends a [`server::PlayerPositionAndRotation`] packet if either have changed.
    ///
    /// [1]: https://bevy-cheatbook.github.io/programming/change-detection.html
    pub fn update_position_and_look(
        mut players: Query<(
            Socket<Play>,
            ChangeTracked<Position>,
            ChangeTracked<Rotation>,
        )>,
    ) {
        for (mut socket, position, rotation) in players.iter_mut() {
            if position.is_changed() || rotation.is_changed() {
                let Vector([yaw, pitch, _]) = rotation.value.axes_angles_deg();

                // TODO: tp id confirm
                socket.send(server::PlayerPositionAndRotation {
                    position: position.value.0,
                    yaw: yaw as f32,
                    pitch: pitch as f32,
                    flags: 0,
                    tp_id: 0,
                    dismount: false,
                });
            }
        }
    }

    /// Performs [change detection][1] for a player's [`GameMode`] component,
    /// and sends a [`server::GameStateUpdate::GameMode`] packet if it changed.
    ///
    /// [1]: https://bevy-cheatbook.github.io/programming/change-detection.html
    pub fn update_gamemode(mut players: Query<(Socket<Play>, &GameMode), Changed<GameMode>>) {
        for (mut socket, gamemode) in players.iter_mut() {
            socket.send(server::GameStateUpdate::GameMode((*gamemode).into()));
        }
    }
}
