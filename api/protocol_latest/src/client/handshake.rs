use flexstr::SharedStr;
use minecrevy_io_str::{McRead, McWrite};
use minecrevy_protocol::Packet;

use crate::ProtocolState;

/// The first packet sent by the client, telling the server if it expects to login or simply fetch server information.
#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct Handshake {
    /// The protocol version that the client is connecting with.
    #[options(varint = true)]
    pub version: i32,
    /// The address specified in the address bar by the client.
    ///
    /// This may be useful for proxies to determine target server.
    #[options(max_len = 255)]
    pub address: SharedStr,
    /// The port specified in the address bar by the client.
    pub port: u16,
    /// The transition state the client expects.
    pub next: NextState,
}

/// The state that a client expects to transition into after [HandshakePacket] is sent.
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, McRead, McWrite)]
#[io_repr(varint)]
pub enum NextState {
    /// Tells the server that the client wants to fetch server information and exit.
    Status = 1,
    /// Tells the server that the client wants to login and play.
    Login = 2,
}

impl From<NextState> for ProtocolState {
    fn from(next: NextState) -> Self {
        match next {
            NextState::Status => Self::Status,
            NextState::Login => Self::Login,
        }
    }
}
