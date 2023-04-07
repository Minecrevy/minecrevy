/// A client connection state in the Minecraft protocol.
pub trait ProtocolState: 'static + Send + Sync {}

/// The initial client connection [`ProtocolState`].
pub struct Handshake;

impl ProtocolState for Handshake {}

/// A potential client connection [`ProtocolState`] after [`Handshake`].
pub struct Status;

impl ProtocolState for Status {}

/// A potential client connection [`ProtocolState`] after [`Handhskae`].
pub struct Login;

impl ProtocolState for Login {}

/// The client connection [`ProtocolState`] after a successful [`Login`].
pub struct Play;

impl ProtocolState for Play {}
