use std::{
    net::{IpAddr, SocketAddr},
    path::PathBuf,
    time::Duration,
};

use bevy::{app::ScheduleRunnerPlugin, log::LogPlugin, prelude::*};
use clap::Parser;
use minecrevy_net::{start_server, NetworkServerPlugins};
use minecrevy_protocol::ServerProtocolPlugin;
use minecrevy_std::{
    handshake::{AllowLogin, HandshakePlugin},
    status::{Motd, PlayerSample, ServerProtocol, ServerProtocolName, StatusPlugin},
    CorePlugin, PlayerCount,
};
use minecrevy_text::Text;
use tracing::Level;

/// A Minecraft server list advertiser.
/// Does not support joining the server.
#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// The address to bind the server to.
    #[arg(short, long, default_value = "0.0.0.0")]
    pub address: IpAddr,
    /// The port to bind the server to.
    #[arg(short, long, default_value_t = 25565)]
    pub port: u16,
    /// How many ticks per second to run the server at.
    #[arg(short, long, default_value_t = 20.)]
    pub tick_rate: f32,
    /// The message of the day displayed in the server list.
    #[arg(short, long, default_value = "A Minecraft Server")]
    pub motd: Text,
    /// The number of players to display in the server list.
    #[arg(long = "online", default_value_t = 0)]
    pub online_players: i32,
    /// The maximum number of players to display in the server list.
    #[arg(long = "max", default_value_t = 20)]
    pub max_players: i32,
    /// The list of sample player names to display in the server list.
    #[arg(long = "sample")]
    pub sample_players: Vec<String>,
    /// The filename of the favicon to display in the server list.
    /// It will be automatically resized to 64x64 pixels.
    #[arg(long, default_value = "server-icon.png")]
    pub favicon: PathBuf,
    /// The message to display when a client tries to log in.
    #[arg(long, default_value = "Logins are not enabled.")]
    pub deny_login: Text,
    /// The protocol version to advertise to clients.
    /// If not set, the client's protocol version will be echoed back.
    #[arg(long)]
    pub protocol_version: Option<i32>,
    /// The name of the server protocol version to advertise to clients.
    #[arg(long, default_value = "Ping Server")]
    pub protocol_name: String,
}

fn main() {
    let args = Args::parse();

    App::new()
        .add_plugins(
            MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f32(
                1. / args.tick_rate,
            ))),
        )
        .add_plugins(AssetPlugin::default())
        .add_plugins(LogPlugin {
            level: Level::DEBUG,
            ..Default::default()
        })
        .add_plugins(NetworkServerPlugins)
        .add_plugins(ServerProtocolPlugin {
            handshake: true,
            login: true,
            play: false,
            status: true,
            config: false,
        })
        .add_plugins(CorePlugin)
        .add_plugins(HandshakePlugin)
        .add_plugins(StatusPlugin {
            favicon_path: Some(args.favicon),
        })
        .insert_resource(ServerProtocolName(args.protocol_name))
        .insert_resource(
            args.protocol_version
                .map_or(ServerProtocol::Echo, ServerProtocol::Version),
        )
        .insert_resource(Motd(args.motd))
        .insert_resource(PlayerSample(args.sample_players))
        .insert_resource(PlayerCount {
            online: args.online_players,
            max: args.max_players,
        })
        .insert_resource(AllowLogin(Err(Some(args.deny_login))))
        .add_systems(
            Startup,
            start_server(SocketAddr::new(args.address, args.port)),
        )
        .run();
}
