/// All possible states of a Minecraft protocol client.
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum ProtocolState {
    /// The initial state used for negotiation.
    Handshake,
    /// The state where plugin list data is collected.
    Status,
    /// The state where player login handshake is performed.
    Login,
    /// The state where normal plugin operation occurs.
    Play,
}

impl Default for ProtocolState {
    fn default() -> Self {
        Self::Handshake
    }
}

/// A workaround for the incomplete compiler feature 'adt_const_params'.
pub trait ProtocolStateDesc: Send + Sync + 'static {
    /// The state this descriptor represents.
    const STATE: ProtocolState;
}

/// See [`State::Handshake`] for info.
pub struct Handshake;

impl ProtocolStateDesc for Handshake {
    const STATE: ProtocolState = ProtocolState::Handshake;
}

/// See [`State::Status`] for info.
pub struct Status;

impl ProtocolStateDesc for Status {
    const STATE: ProtocolState = ProtocolState::Status;
}

/// See [`State::Login`] for info.
pub struct Login;

impl ProtocolStateDesc for Login {
    const STATE: ProtocolState = ProtocolState::Login;
}

/// See [`State::Play`] for info.
pub struct Play;

impl ProtocolStateDesc for Play {
    const STATE: ProtocolState = ProtocolState::Play;
}
