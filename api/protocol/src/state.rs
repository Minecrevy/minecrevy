/// All possible states of a Minecraft protocol client.
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum ProtocolState {
    /// The initial state used for negotiation.
    Handshake,
    /// The state where server list data is collected.
    Status,
    /// The state where player login handshake is performed.
    Login,
    /// The state where normal server operation occurs.
    Play,
}

impl Default for ProtocolState {
    fn default() -> Self {
        Self::Handshake
    }
}
