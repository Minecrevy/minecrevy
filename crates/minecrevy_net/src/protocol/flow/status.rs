use std::io;

use bevy::prelude::*;
use minecrevy_io::{options::StringOptions, McRead, McWrite, Packet, ProtocolVersion};
use minecrevy_text::Text;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::protocol::{
    client::{Client, ClientIn},
    flow::handshake::ClientInfo,
    registry::VersionedPacketsBuilder,
    state::{Play, Status},
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
        status.add_incoming::<StatusRequest>(0x00, ProtocolVersion::V1_7_2..);
        status.add_incoming::<PingRequest>(0x01, ProtocolVersion::V1_7_2..);

        status.add_outgoing::<StatusResponse>(0x00, ProtocolVersion::V1_7_2..);
        status.add_outgoing::<PingResponse>(0x01, ProtocolVersion::V1_7_2..);
    }

    fn status(
        global_resp: Option<Res<Response>>,
        mut clients: Query<(Client<Status>, &ClientInfo, Option<&Response>)>,
        players: Query<(), ClientIn<Play>>,
    ) {
        let num_players = players.iter().count();

        for (client, info, client_resp) in &mut clients {
            if let Some(ping) = client.read::<PingRequest>() {
                client.write(ping);
            }
            if let Some(_) = client.read::<StatusRequest>() {
                let response =
                    client_resp
                        .or(global_resp.as_deref())
                        .cloned()
                        .unwrap_or(Response {
                            version: info.version.into(),
                            players: ResponsePlayers {
                                max: 1000,
                                online: num_players as i32,
                                sample: vec![],
                            },
                            description: Text::str("A Minecraft server."),
                            favicon: None,
                            enforces_secure_chat: false,
                        });
                client.write(StatusResponse(response));
            }
        }
    }
}

pub struct Motd {
    pub description: Text,
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
        Self {
            name: version.to_string(),
            protocol: version.get(),
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

    fn read<R: io::Read>(
        reader: R,
        _: Self::Options,
        version: ProtocolVersion,
    ) -> io::Result<Self> {
        let str = String::read(
            reader,
            StringOptions {
                max_len: Some(32767),
            },
            version,
        )?;
        serde_json::from_str(&str).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }
}

impl McWrite for Response {
    type Options = ();

    fn write<W: io::Write>(
        &self,
        writer: W,
        _: Self::Options,
        version: ProtocolVersion,
    ) -> io::Result<()> {
        let str = serde_json::to_string(self)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        String::write(
            &str,
            writer,
            StringOptions {
                max_len: Some(32767),
            },
            version,
        )
    }
}
