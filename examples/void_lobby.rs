use std::time::Duration;

use bevy::{app::ScheduleRunnerPlugin, log::LogPlugin, prelude::*};
use minecrevy_net::{start_server, NetworkServerPlugins};
use minecrevy_protocol::ServerProtocolPlugin;
use minecrevy_std::{
    config::ConfigPlugin, handshake::HandshakePlugin, login::LoginPlugin, play::PlayPlugin,
    status::StatusPlugin, CorePlugin,
};
use tracing::Level;

/// 20 ticks per second.
const TICK_RATE: f32 = 1. / 20.;

fn main() {
    App::new()
        .add_plugins(
            MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f32(
                TICK_RATE,
            ))),
        )
        .add_plugins(AssetPlugin::default())
        .add_plugins(LogPlugin {
            filter: String::new(),
            level: Level::TRACE,
        })
        .add_plugins(NetworkServerPlugins)
        .add_plugins(ServerProtocolPlugin {
            handshake: true,
            login: true,
            play: true,
            status: true,
            config: true,
        })
        .add_plugins(CorePlugin { max_players: 20 })
        .add_plugins(HandshakePlugin { allow_login: true })
        .add_plugins(StatusPlugin {
            motd: Some("A Ping Server".into()),
            favicon_filename: Some("server-icon.png".into()),
            show_players: false,
        })
        .add_plugins(LoginPlugin)
        .add_plugins(ConfigPlugin)
        .add_plugins(PlayPlugin)
        .add_systems(Startup, start_server("127.0.0.1", 25565))
        .run();
}
