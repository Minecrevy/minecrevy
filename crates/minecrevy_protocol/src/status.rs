//! Minecraft protocol packet definitions in the `Status` state.

use std::io::{self, Read, Write};

use minecrevy_io::{
    args::{IntArgs, StringArgs},
    McRead, McWrite,
};
use minecrevy_text::Text;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A packet sent by the client to determine latency.
///
/// The server should respond with a [`Ping`] packet containing the same payload.
#[derive(Clone, PartialEq, Debug)]
pub struct Ping(pub i64);

impl McRead for Ping {
    type Args = ();

    fn read(reader: impl io::Read, (): Self::Args) -> io::Result<Self> {
        Ok(Self(i64::read(reader, IntArgs { varint: false })?))
    }
}

impl McWrite for Ping {
    type Args = ();

    fn write(&self, writer: impl io::Write, (): Self::Args) -> io::Result<()> {
        i64::write(&self.0, writer, IntArgs { varint: false })
    }
}

/// A packet sent by the client to request server MOTD, favicon, and player info.
#[derive(Clone, PartialEq, Debug)]
pub struct Request;

impl McRead for Request {
    type Args = ();

    fn read(_reader: impl io::Read, (): Self::Args) -> io::Result<Self> {
        Ok(Self)
    }
}

/// A packet sent by the server to respond to a [`Request`].
#[derive(Clone, PartialEq, Debug)]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    /// The version of the server.
    pub version: ResponseVersion,
    /// The players on the server.
    pub players: ResponsePlayers,
    /// The message to display in the server list.
    pub description: Text,
    /// The icon to display in the server list.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub favicon: Option<String>,
    /// Whether the server enforces secure chat.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enforces_secure_chat: Option<bool>,
    /// Whether the server previews chat.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub previews_chat: Option<bool>,
}

/// [`Response`] version information.
#[derive(Clone, PartialEq, Debug)]
#[derive(Serialize, Deserialize)]
pub struct ResponseVersion {
    /// The name of the version of the server.
    pub name: String,
    /// The protocol version of the server.
    pub protocol: i32,
}

/// [`Response`] player information.
#[derive(Clone, PartialEq, Debug)]
#[derive(Serialize, Deserialize)]
pub struct ResponsePlayers {
    /// The maximum number of players allowed on the server at once.
    pub max: i32,
    /// The number of players currently on the server.
    pub online: i32,
    /// The players currently on the server.
    pub sample: Vec<ResponseProfile>,
}

/// [`Response`] player profile information.
#[derive(Clone, PartialEq, Debug)]
#[derive(Serialize, Deserialize)]
pub struct ResponseProfile {
    /// The name of the player.
    pub name: String,
    /// The UUID of the player.
    pub id: Uuid,
}

impl McRead for Response {
    type Args = ();

    fn read(reader: impl Read, (): Self::Args) -> io::Result<Self> {
        let json = String::read(
            reader,
            StringArgs {
                max_len: Some(32767),
            },
        )?;

        serde_json::from_str::<Response>(&json)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }
}

impl McWrite for Response {
    type Args = ();

    fn write(&self, writer: impl Write, (): Self::Args) -> io::Result<()> {
        let json = serde_json::to_string::<Response>(self)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        String::write(
            &json,
            writer,
            StringArgs {
                max_len: Some(32767),
            },
        )
    }
}
