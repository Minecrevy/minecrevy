use std::{
    net::{Ipv4Addr, SocketAddr},
    time::Duration,
};

use bevy::{app::ScheduleRunnerPlugin, log::LogPlugin, prelude::*};
use minecrevy_net::{start_server, NetworkServerPlugins};
use minecrevy_protocol::ServerProtocolPlugin;
use minecrevy_std::{
    config::ConfigPlugin, handshake::HandshakePlugin, login::LoginPlugin, play::PlayPlugin,
    status::StatusPlugin, CorePlugin,
};
use tracing::Level;

fn main() {
    App::new()
        .add_plugins(
            MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f32(
                1. / 20.,
            ))),
        )
        .add_plugins(AssetPlugin::default())
        .add_plugins(LogPlugin {
            level: Level::TRACE,
            ..Default::default()
        })
        .add_plugins(NetworkServerPlugins)
        .add_plugins(ServerProtocolPlugin {
            handshake: true,
            login: true,
            play: true,
            status: true,
            config: true,
        })
        .add_plugins(CorePlugin)
        .add_plugins(HandshakePlugin)
        .add_plugins(StatusPlugin { favicon_path: None })
        .add_plugins(LoginPlugin)
        .add_plugins(ConfigPlugin)
        .add_plugins(PlayPlugin)
        .add_systems(
            Startup,
            start_server(SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 25565)),
        )
        .run();
}
