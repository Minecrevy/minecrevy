use std::io;
use std::io::{Read, Write};
use flexstr::SharedStr;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use minecrevy_io_buf::WriteMinecraftExt;
use minecrevy_io_str::{McRead, McWrite};
use minecrevy_protocol::Packet;
use minecrevy_text::{Color, Style, Text};

/// Sends server information to the client, including player count, maximum player count, an MOTD, etc.
#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct StatusResponse(pub Motd);

/// Replies to the client for latency testing.
pub type StatusPong = crate::client::StatusPing;

/// Basic server information sent by the server.
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Motd {
    /// The protocol version that the server supports.
    pub version: MotdVersion,
    /// The player statistic information.
    pub players: MotdPlayers,
    /// The server MOTD text.
    pub description: Text,
    /// The server icon.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub favicon: Option<SharedStr>,
}

/// The protocol version that a server supports, and the human readable name for said version.
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct MotdVersion {
    /// The human readable name of the protocol version.
    pub name: SharedStr,
    /// The protocol number.
    pub protocol: i32,
}

/// The maximum player count, current player count, and sample of online players.
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct MotdPlayers {
    /// The maximum player count.
    pub max: i32,
    /// The current player count.
    pub online: i32,
    /// The sample of current online players.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sample: Vec<MotdPlayerSample>,
}

/// A sample player.
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct MotdPlayerSample {
    /// The player's name.
    pub name: SharedStr,
    /// The player's UUID.
    pub id: Uuid,
}

impl Default for Motd {
    fn default() -> Self {
        Self {
            version: MotdVersion { name: SharedStr::from_static("1.18.1"), protocol: 757 },
            players: MotdPlayers {
                max: 20,
                online: 0,
                sample: vec![],
            },
            description: Text::str("A Minecraft Server", Style::empty())
                .with_child(Text::str(", written in Rust", Style::color(Color::DARK_RED)))
                .with_child(Text::str(".", Style::empty())),
            favicon: None,
        }
    }
}

impl McRead for Motd {
    type Options = ();

    fn read<R: Read>(reader: R, (): Self::Options) -> std::io::Result<Self> {
        serde_json::from_reader(reader)
            .map_err(|e| io::Error::new(
                io::ErrorKind::InvalidData,
                format!("invalid MOTD json: {}", e),
            ))
    }
}

impl McWrite for Motd {
    type Options = ();

    fn write<W: Write>(&self, mut writer: W, (): Self::Options) -> std::io::Result<()> {
        writer.write_string(serde_json::to_string(&self).unwrap().as_str())?;
        Ok(())
    }
}
