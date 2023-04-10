use std::time::Duration;

use bevy::{app::ScheduleRunnerSettings, prelude::*};
use minecrevy_net::{
    protocol::{packet::PacketsPlugin, plugin::DefaultNetworkPlugins, version::ReleaseVersion},
    raw::RawServer,
};

/// How often the server loop should tick.
const TICKS_PER_SECOND: f64 = 20.0;

fn start_listening(mut server: ResMut<RawServer>) {
    server.listen("127.0.0.1:25565".parse().unwrap());
}

fn main() {
    let tick_duration = Duration::from_secs_f64(1.0 / TICKS_PER_SECOND);

    App::new()
        .insert_resource(ScheduleRunnerSettings::run_loop(tick_duration))
        .add_plugins(MinimalPlugins)
        .add_plugin(PacketsPlugin::new([ReleaseVersion::V1_19_4.v()]))
        .add_plugins(DefaultNetworkPlugins)
        .add_systems(PostStartup, start_listening)
        .run();
}
