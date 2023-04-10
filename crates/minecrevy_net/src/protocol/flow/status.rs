use std::io;

use bevy::prelude::*;
use minecrevy_io::{options::StringOptions, McRead, McWrite, Packet};
use minecrevy_text::Text;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::protocol::{
    client::Client,
    flow::handshake::ClientInfo,
    registry::VersionedPacketsBuilder,
    state::Status,
    version::{ProtocolVersion, ReleaseVersion},
};

/// A [`SystemSet`] for handling packets as part of the status flow.
#[derive(SystemSet, Clone, PartialEq, Eq, Debug, Hash)]
pub struct StatusFlow;

/// Adds systems to handle the Minecraft protocol status flow.
pub struct StatusFlowPlugin;

impl Plugin for StatusFlowPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, Self::register_packets);

        app.add_systems(PreUpdate, Self::status.in_set(StatusFlow));
    }
}

/// ECS Systems
impl StatusFlowPlugin {
    fn register_packets(mut status: ResMut<VersionedPacketsBuilder<Status>>) {
        status.add_incoming::<StatusRequest>(0x00, ReleaseVersion::V1_7_2.v()..);
        status.add_incoming::<PingRequest>(0x01, ReleaseVersion::V1_7_2.v()..);

        status.add_outgoing::<StatusResponse>(0x00, ReleaseVersion::V1_7_2.v()..);
        status.add_outgoing::<PingResponse>(0x01, ReleaseVersion::V1_7_2.v()..);
    }

    fn status(
        mut commands: Commands,
        global_resp: Option<Res<Response>>,
        mut clients: Query<(Entity, Client<Status>, &ClientInfo, Option<&Response>)>,
    ) {
        let num_clients = clients.iter().count();

        for (entity, mut client, info, client_resp) in &mut clients {
            let mut entity = commands.entity(entity);

            if let Some(ping) = client.read::<PingRequest>() {
                let Ok(ping) = ping else {
                    entity.despawn();
                    continue;
                };

                client.write(ping).unwrap();
            }
            if let Some(req) = client.read::<StatusRequest>() {
                let Ok(_) = req else {
                    entity.despawn();
                    continue;
                };

                let response =
                    client_resp
                        .or(global_resp.as_deref())
                        .cloned()
                        .unwrap_or(Response {
                            version: info.version.into(),
                            players: ResponsePlayers {
                                max: 1000,
                                online: num_clients as i32,
                                sample: vec![],
                            },
                            description: Text::str("A Minecraft server."),
                            favicon: None,
                            enforces_secure_chat: false,
                        });
                client.write(StatusResponse(response)).unwrap();
            }
        }
    }
}

/// Sent by the client to request latency information.
#[derive(Packet, McRead, McWrite, Clone, PartialEq, Debug)]
pub struct PingRequest(pub i64);

pub type PingResponse = PingRequest;

/// Sent by the client to request server information.
#[derive(Packet, McRead, McWrite, Clone, PartialEq, Debug)]
pub struct StatusRequest;

#[derive(Packet, McRead, McWrite, Clone, PartialEq, Debug)]
pub struct StatusResponse(pub Response);

#[derive(Component, Resource, Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Response {
    pub version: ResponseVersion,
    pub players: ResponsePlayers,
    pub description: Text,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub favicon: Option<String>,
    pub enforces_secure_chat: bool,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct ResponseVersion {
    pub name: String,
    pub protocol: i32,
}

impl From<ProtocolVersion> for ResponseVersion {
    fn from(version: ProtocolVersion) -> Self {
        if let Some(release) = version.release() {
            Self {
                name: release.to_string(),
                protocol: version.0,
            }
        } else {
            Self {
                name: format!("Unknown"),
                protocol: version.0,
            }
        }
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct ResponsePlayers {
    pub max: i32,
    pub online: i32,
    pub sample: Vec<ResponsePlayerSample>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct ResponsePlayerSample {
    pub name: String,
    pub id: Uuid,
}

impl McRead for Response {
    type Options = ();

    fn read<R: io::Read>(reader: R, _: Self::Options) -> io::Result<Self> {
        let str = String::read(
            reader,
            StringOptions {
                max_len: Some(32767),
            },
        )?;
        serde_json::from_str(&str).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }
}

impl McWrite for Response {
    type Options = ();

    fn write<W: io::Write>(&self, writer: W, _: Self::Options) -> io::Result<()> {
        let str = serde_json::to_string(self)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        String::write(
            &str,
            writer,
            StringOptions {
                max_len: Some(32767),
            },
        )
    }
}
