//! Modern Minecraft protocol packet definitions.

#![warn(missing_docs)]

use bevy::prelude::*;
use minecrevy_net::{client::ProtocolState, AppNetworkExt, NetworkPlugin};

pub mod config;
pub mod handshake;
pub mod login;
pub mod play;
pub mod status;

/// [`Plugin`] for automatically registering Minecraft protocol packets for
/// server-side communication.
pub struct ServerProtocolPlugin {
    /// Whether to register [`handshake`] packets.
    pub handshake: bool,
    /// Whether to register [`login`] packets.
    pub login: bool,
    /// Whether to register [`play`] packets.
    pub play: bool,
    /// Whether to register [`status`] packets.
    pub status: bool,
    /// Whether to register [`config`] packets.
    pub config: bool,
}

impl Plugin for ServerProtocolPlugin {
    fn build(&self, app: &mut App) {
        assert!(
            app.is_plugin_added::<NetworkPlugin>(),
            "{} must be added before {}",
            std::any::type_name::<NetworkPlugin>(),
            std::any::type_name::<Self>(),
        );

        app.configure_sets(
            Update,
            (
                PacketHandlerSet::Handshake,
                PacketHandlerSet::HandshakeApply,
                PacketHandlerSet::Login,
                PacketHandlerSet::LoginApply,
                PacketHandlerSet::Config,
                PacketHandlerSet::ConfigApply,
                PacketHandlerSet::Play,
                PacketHandlerSet::PlayApply,
                PacketHandlerSet::Status,
            )
                .chain(),
        );

        if self.handshake {
            app.add_handshake_packet();
            app.add_systems(
                Update,
                apply_deferred.in_set(PacketHandlerSet::HandshakeApply),
            );
        }
        if self.login {
            app.add_login_packets();
            app.add_systems(Update, apply_deferred.in_set(PacketHandlerSet::LoginApply));
        }
        if self.play {
            app.add_play_packets();
            app.add_systems(Update, apply_deferred.in_set(PacketHandlerSet::PlayApply));
        }
        if self.status {
            app.add_status_packets();
        }
        if self.config {
            // TODO
        }
    }
}

/// [`SystemSet`]s for handling packet events.
#[derive(SystemSet)]
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum PacketHandlerSet {
    /// [`SystemSet`] for handling [`handshake`] packets.
    Handshake,
    /// [`SystemSet`] for applying commands queued during [`PacketHandlerSet::Handshake`].
    HandshakeApply,
    /// [`SystemSet`] for handling [`login`] packets.
    Login,
    /// [`SystemSet`] for applying commands queued during [`PacketHandlerSet::Login`].
    LoginApply,
    /// [`SystemSet`] for handling [`config`] packets.
    Config,
    /// [`SystemSet`] for applying commands queued during [`PacketHandlerSet::Config`].
    ConfigApply,
    /// [`SystemSet`] for handling [`play`] packets.
    Play,
    /// [`SystemSet`] for applying commands queued during [`PacketHandlerSet::Play`].
    PlayApply,
    /// [`SystemSet`] for handling [`status`] packets.
    Status,
}

/// Extension trait for [`App`]s to register Minecraft protocol packets.
pub trait AppProtocolExt {
    /// Adds the [`handshake::Handshake`] packet to the given [`App`].
    fn add_handshake_packet(&mut self) -> &mut Self;

    /// Adds the [`login`] packets to the given [`App`].
    fn add_login_packets(&mut self) -> &mut Self;

    /// Adds the [`play`] packets to the given [`App`].
    fn add_play_packets(&mut self) -> &mut Self;

    /// Adds the [`status`] packets to the given [`App`].
    fn add_status_packets(&mut self) -> &mut Self;

    /// Adds the [`config`] packets to the given [`App`].
    fn add_config_packets(&mut self) -> &mut Self;
}

impl AppProtocolExt for App {
    fn add_handshake_packet(&mut self) -> &mut Self {
        self.add_incoming_packet::<handshake::Handshake>(ProtocolState::Handshake, 0x00, true)
    }

    fn add_login_packets(&mut self) -> &mut Self {
        self.add_outgoing_packet::<login::Disconnect>(ProtocolState::Login, 0x00)
    }

    fn add_play_packets(&mut self) -> &mut Self {
        self
    }

    fn add_status_packets(&mut self) -> &mut Self {
        self.add_incoming_packet::<status::Request>(ProtocolState::Status, 0x00, false)
            .add_incoming_packet::<status::Ping>(ProtocolState::Status, 0x01, false)
            .add_outgoing_packet::<status::Response>(ProtocolState::Status, 0x00)
            .add_outgoing_packet::<status::Ping>(ProtocolState::Status, 0x01)
    }

    fn add_config_packets(&mut self) -> &mut Self {
        self
    }
}
