use std::io::{self, Read, Write};

use minecrevy_io_buf::{ReadMinecraftExt, WriteMinecraftExt};
use minecrevy_io_str::{McRead, McWrite};

use crate::ProtocolState;

/// The first packet sent by the client, telling the plugin if it expects to login or simply fetch plugin information.
#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct Handshake {
    /// The protocol version that the client is connecting with.
    #[mcio(varint)]
    pub version: i32,
    /// The address specified in the address bar by the client.
    ///
    /// This may be useful for proxies to determine target plugin.
    #[mcio(max_len = 255)]
    pub address: String,
    /// The port specified in the address bar by the client.
    pub port: u16,
    /// The transition state the client expects.
    pub next: NextState,
}

impl crate::Packet for Handshake {}

/// The state that a client expects to transition into after [HandshakePacket] is sent.
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum NextState {
    /// Tells the plugin that the client wants to fetch plugin information and exit.
    Status,
    /// Tells the plugin that the client wants to login and play.
    Login,
}

impl McRead for NextState {
    type Options = ();

    fn read<R: Read>(mut reader: R, _options: Self::Options) -> io::Result<Self> {
        match reader.read_var_i32()? {
            1 => Ok(NextState::Status),
            2 => Ok(NextState::Login),
            n => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("expected next state of 1 or 2, encountered {}", n),
            )),
        }
    }
}

impl McWrite for NextState {
    type Options = ();

    fn write<W: Write>(&self, mut writer: W, _options: Self::Options) -> io::Result<()> {
        match self {
            NextState::Status => writer.write_var_i32(1),
            NextState::Login => writer.write_var_i32(2),
        }
    }
}

impl From<NextState> for ProtocolState {
    fn from(next: NextState) -> Self {
        match next {
            NextState::Status => Self::Status,
            NextState::Login => Self::Login,
        }
    }
}
