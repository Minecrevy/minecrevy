use minecrevy_io::{McRead, McWrite, Packet};

/// Sent by the client to begin Minecraft protocol communication.
#[derive(Packet, McRead, McWrite, Clone, PartialEq, Debug)]
pub struct Handshake {
    /// The protocol version.
    #[options(varint = true)]
    pub version: i32,
    /// The server address.
    #[options(max_len = 255)]
    pub address: String,
    /// The server port.
    pub port: u16,
    /// The intended handshake flow.
    pub next: NextState,
}

/// The intended handshake flow sent by the [`Handshake`] packet.
#[derive(McRead, McWrite, Clone, Copy, PartialEq, Eq, Debug, Hash)]
#[io_repr(varint)]
pub enum NextState {
    /// The client just wants server information.
    Status = 1,
    /// The client wants to login and play.
    Login = 2,
}
