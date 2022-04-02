use bevy::prelude::*;
use tracing::Level;

use minecrevy_config::ConfigPlugin;
use minecrevy_game::entity::player::PlayerPlugins;
use minecrevy_net::plugin::ServerPlugin;

mod log;

fn main() {
    // Configure logging
    tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .with_timer(log::Timestamp)
        .init();

    // launch bevy engine
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugin(ConfigPlugin)
        .add_plugin(ServerPlugin)
        .add_plugins(PlayerPlugins)
        .run()
}
