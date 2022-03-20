use minecrevy_io_str::{McRead, McWrite};

/// Tells the server to send server information.
///
/// See [`crate::server::ResponseStatusPacket`] for what kind of information is received.
#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct StatusRequest;

impl crate::Packet for StatusRequest {}

/// Tells the server that the client wants to test latency.
#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct StatusPing(pub i64);

impl crate::Packet for StatusPing {}
