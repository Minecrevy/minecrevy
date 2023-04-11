use bevy::{prelude::*, utils::HashSet};
use minecrevy_core::key::Key;
use minecrevy_io::{
    options::{ListLength, OptionTag},
    McRead, McWrite, Packet,
};
use minecrevy_text::Text;
use uuid::Uuid;

use crate::{
    error::ClientError,
    protocol::{
        client::{Client, ClientEntered, PacketQueue, PacketRegistry},
        flow::handshake::ClientInfo,
        registry::{Packets, VersionedPackets, VersionedPacketsBuilder},
        state::{Login, Play},
        version::ReleaseVersion,
    },
};

const NAMESPACE_OFFLINE_PLAYER: Uuid = Uuid::from_bytes([
    0x29, 0x9c, 0xd4, 0x21, 0x35, 0x72, 0x45, 0x74, 0x95, 0x07, 0xd3, 0x2d, 0x9e, 0xb2, 0x4d, 0x8f,
]);

/// Adds systems to handle the Minecraft protocol login flow.
pub struct LoginFlowPlugin;

impl Plugin for LoginFlowPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LoginHandlers::default());

        app.add_systems(Startup, Self::register_packets);
        app.add_systems(PreUpdate, (Self::begin_login, Self::finish_login).chain());
    }
}

/// ECS Systems
impl LoginFlowPlugin {
    fn register_packets(mut login: ResMut<VersionedPacketsBuilder<Login>>) {
        login.add_incoming::<Start>(0x00, ReleaseVersion::V1_19_3.v()..);

        login.add_outgoing::<Success>(0x02, ReleaseVersion::V1_19.v()..);
    }

    pub fn begin_login(
        mut commands: Commands,
        mut clients: Query<(Entity, Client<Login>), ClientEntered<Login>>,
    ) {
        // Begin channel negotiations
        for (entity, mut client) in &mut clients {
            if let Some(start) = client.read::<Start>() {
                let mut entity = commands.entity(entity);

                entity.insert((
                    PlayerInfo {
                        name: start.name,
                        id: start.id,
                    },
                    FinishedHandlers::default(),
                ));
            }
        }
    }

    pub fn finish_login(
        mut commands: Commands,
        all_channels: Res<LoginHandlers>,
        play: Res<VersionedPackets<Play>>,
        mut clients: Query<
            (
                Entity,
                Client<Login>,
                &ClientInfo,
                &PlayerInfo,
                &FinishedHandlers,
            ),
            Changed<FinishedHandlers>,
        >,
    ) {
        for (entity, mut client, info, player, channels) in &mut clients {
            let _net = info_span!("net", client = %client.addr()).entered();

            // Send Login Success if all channels are finished negotiating.
            if channels.is_finished(&all_channels) {
                let mut entity = commands.entity(entity);

                let Some(registry) = play.get(info.version) else {
                    let msg = format!(
                        "{} registry does not exist, cannot enter {} flow without one.",
                        std::any::type_name::<Packets<Play>>(),
                        std::any::type_name::<Play>()
                    );
                    client.raw.error(ClientError::ISE(msg));
                    return;
                };

                let player = Player {
                    name: player.name.clone(),
                    id: player.id.unwrap_or(Uuid::new_v3(
                        &NAMESPACE_OFFLINE_PLAYER,
                        player.name.as_bytes(),
                    )),
                };

                info!(player = %player.name, "player joined");

                client.write(Success {
                    id: player.id,
                    username: player.name.clone(),
                    properties: vec![],
                });

                entity
                    .insert((
                        player,
                        PacketQueue::<Play>::default(),
                        PacketRegistry(registry.clone()),
                    ))
                    .remove::<(PlayerInfo, PacketQueue<Login>, PacketRegistry<Login>)>();
            }
        }
    }
}

pub trait AppLoginFlowExt {
    /// Adds a login handler that **MUST** signal finish for the login flow to end.
    fn add_login_handler(&mut self, channel: LoginHandler) -> &mut Self;
}

impl AppLoginFlowExt for App {
    fn add_login_handler(&mut self, channel: LoginHandler) -> &mut Self {
        let mut channels = self.world.resource_mut::<LoginHandlers>();
        channels.insert(channel);

        self
    }
}

/// The set of [`LoginHandler`]s that have finished negotation for a given client.
/// Login flow for a client will not be ended until this set is a superset of the
/// [set of all registered channels](LoginHandlers).
///
/// A login flow channel can signal it is finished negotating by inserting its [`ChannelKey`]
/// with [`FinishedHandlers::finish`].
#[derive(Component, Deref, Default)]
pub struct FinishedHandlers(HashSet<LoginHandler>);

impl FinishedHandlers {
    /// Adds the given channel to the set of finished channels.
    pub fn finish(&mut self, channel: LoginHandler) {
        self.0.insert(channel);
    }

    /// Returns `true` when all registered channels have finished negotation.
    pub fn is_finished(&self, all_channels: &LoginHandlers) -> bool {
        self.0.is_superset(&all_channels.0)
    }
}

/// The set of all registered [`LoginHandler`]s.
#[derive(Resource, Deref, Default)]
pub struct LoginHandlers(HashSet<LoginHandler>);

impl LoginHandlers {
    pub fn insert(&mut self, channel: LoginHandler) {
        self.0.insert(channel);
    }
}

/// The identifier of a login handler.
#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct LoginHandler(pub Key);

#[derive(Component, Debug)]
pub struct PlayerInfo {
    pub name: String,
    pub id: Option<Uuid>,
}

#[derive(Component, Debug)]
pub struct Player {
    pub name: String,
    pub id: Uuid,
}

#[derive(Packet, McRead, McWrite, Clone, PartialEq, Debug)]
pub struct Start {
    #[options(max_len = 16)]
    pub name: String,
    #[options(tag = OptionTag::Bool)]
    pub id: Option<Uuid>,
}

#[derive(Packet, McRead, McWrite, Clone, PartialEq, Debug)]
#[meta(EnableEncryption)]
pub struct EncryptionResponse {
    #[options(length = ListLength::VarInt)]
    pub shared_secret: Vec<u8>,
    #[options(length = ListLength::VarInt)]
    pub verify_token: Vec<u8>,
}

#[derive(Packet, McRead, McWrite, Clone, PartialEq, Debug)]
pub struct PluginResponse {
    pub message_id: i32,
    #[options(tag = OptionTag::Bool)]
    pub data: Option<Vec<u8>>,
}

#[derive(Packet, McRead, McWrite, Clone, PartialEq, Debug)]
pub struct Disconnect(pub Text);

#[derive(Packet, McRead, McWrite, Clone, PartialEq, Debug)]
pub struct EncryptionRequest {
    #[options(max_len = 20)]
    pub server_id: String,
    #[options(length = ListLength::VarInt)]
    pub public_key: Vec<u8>,
    #[options(length = ListLength::VarInt)]
    pub verify_token: Vec<u8>,
}

#[derive(Packet, McRead, McWrite, Clone, PartialEq, Debug)]
pub struct Success {
    pub id: Uuid,
    #[options(max_len = 16)]
    pub username: String,
    #[options(length = ListLength::VarInt)]
    pub properties: Vec<ProfileProperty>,
}

#[derive(McRead, McWrite, Clone, PartialEq, Debug)]
pub struct ProfileProperty {
    #[options(max_len = 32767)]
    pub name: String,
    #[options(max_len = 32767)]
    pub value: String,
    #[options(tag = OptionTag::Bool, inner.max_len = 32767)]
    pub signature: Option<String>,
}

#[derive(Packet, McRead, McWrite, Clone, PartialEq, Debug)]
#[meta(EnableCompression)]
pub struct SetCompression {
    #[options(varint = true)]
    pub threshold: i32,
}

#[derive(Packet, McRead, McWrite, Clone, PartialEq, Debug)]
pub struct PluginRequest {
    #[options(varint = true)]
    pub message_id: i32,
    pub channel: Key,
    #[options(length = ListLength::Remaining)]
    pub data: Vec<u8>,
}
