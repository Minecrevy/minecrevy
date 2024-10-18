//! Modern Minecraft protocol packet definitions.

#![warn(missing_docs)]

use std::io;

use bevy::prelude::*;
use minecrevy_io::McWrite;
use minecrevy_net::{client::ProtocolState::*, AppNetworkExt};
use minecrevy_text::Text;

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
        if self.handshake {
            app.add_handshake_packet();
        }
        if self.login {
            app.add_login_packets();
        }
        if self.play {
            app.add_play_packets();
        }
        if self.status {
            app.add_status_packets();
        }
        if self.config {
            app.add_config_packets();
        }
    }
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
        self.add_incoming_packet::<handshake::Handshake>(Handshake, 0x00);
        self
    }

    fn add_login_packets(&mut self) -> &mut Self {
        self.add_incoming_packet::<login::Start>(Login, 0x00)
            .add_incoming_packet::<login::Acknowledged>(Login, 0x03);
        self.add_outgoing_packet::<login::Disconnect>(Login, 0x00)
            .add_outgoing_packet::<login::Success>(Login, 0x02);
        self
    }

    fn add_play_packets(&mut self) -> &mut Self {
        self.add_incoming_packet::<play::ConfirmTeleport>(Play, 0x00)
            .add_incoming_packet::<play::SetPlayerPosition>(Play, 0x1A)
            .add_incoming_packet::<play::SetPlayerPositionAndRotation>(Play, 0x1B)
            .add_incoming_packet::<play::SetPlayerRotation>(Play, 0x1C)
            .add_incoming_packet::<play::SetPlayerOnGround>(Play, 0x1D);
        self.add_outgoing_packet::<play::GameEvent>(Play, 0x22)
            .add_outgoing_packet::<play::Login>(Play, 0x2B)
            .add_outgoing_packet::<play::SyncPlayerAbilities>(Play, 0x38)
            .add_outgoing_packet::<play::SyncPlayerPosition>(Play, 0x40);
        self
    }

    fn add_status_packets(&mut self) -> &mut Self {
        self.add_incoming_packet::<status::Request>(Status, 0x00)
            .add_incoming_packet::<status::Ping>(Status, 0x01);
        self.add_outgoing_packet::<status::Response>(Status, 0x00)
            .add_outgoing_packet::<status::Ping>(Status, 0x01);
        self
    }

    fn add_config_packets(&mut self) -> &mut Self {
        self.add_incoming_packet::<config::ClientInformation>(Config, 0x00)
            .add_incoming_packet::<config::AcknowledgeFinish>(Config, 0x03)
            .add_incoming_packet::<config::KnownDataPacks>(Config, 0x07);
        self.add_outgoing_packet::<config::Disconnect>(Config, 0x02)
            .add_outgoing_packet::<config::Finish>(Config, 0x03)
            .add_outgoing_packet::<config::RegistryData<()>>(Config, 0x07)
            .add_outgoing_packet::<config::KnownDataPacks>(Config, 0x0E);
        self
    }
}

/// A packet sent by the server to disconnect the client with a reason.
#[derive(Clone, PartialEq, Debug)]
pub struct Disconnect {
    /// The reason for the disconnect.
    pub reason: Text,
}

impl Default for Disconnect {
    fn default() -> Self {
        Self {
            reason: Text::from("Disconnected"),
        }
    }
}

impl McWrite for Disconnect {
    type Args = ();

    fn write(&self, writer: impl io::Write, (): Self::Args) -> io::Result<()> {
        self.reason.write_default(writer)
    }
}
