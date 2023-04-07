use std::io;

use minecrevy_io::{options::StringOptions, McRead, McWrite, Packet};
use minecrevy_text::Text;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub type PingResponse = crate::c2s::status::PingRequest;

#[derive(Packet, McRead, McWrite, Clone, PartialEq, Debug)]
pub struct StatusResponse(pub Response);

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Response {
    pub version: ResponseVersion,
    pub players: ResponsePlayers,
    pub description: Text,
    pub favicon: String,
    pub enforces_secure_chat: bool,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct ResponseVersion {
    pub name: String,
    pub protocol: i32,
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
