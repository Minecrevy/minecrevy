use std::time::Duration;

use bevy::{
    app::ScheduleRunnerSettings,
    log::{Level, LogPlugin},
    prelude::*,
    MinimalPlugins,
};
use minecrevy_net::protocol::{
    packet::PacketsPlugin,
    plugin::{listen, DefaultNetworkPlugins},
    version::ReleaseVersion,
};

/// How often the server loop should run.
const TICKS_PER_SECOND: f64 = 50.0;

fn main() {
    let tick_duration = Duration::from_secs_f64(1.0 / TICKS_PER_SECOND);

    App::new()
        .insert_resource(ScheduleRunnerSettings::run_loop(tick_duration))
        .add_plugins(MinimalPlugins)
        .add_plugin(LogPlugin {
            filter: "".into(),
            level: Level::DEBUG,
        })
        .add_plugin(PacketsPlugin::new([ReleaseVersion::V1_19_4.v()]))
        .add_plugins(DefaultNetworkPlugins)
        .add_systems(PostStartup, listen("127.0.0.1:25565".parse().unwrap()))
        .run();
}
