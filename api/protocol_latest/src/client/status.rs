use minecrevy_io_str::{McRead, McWrite};
use minecrevy_protocol::Packet;

/// Tells the server to send server information.
///
/// See [`crate::server::ResponseStatusPacket`] for what kind of information is received.
#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct StatusRequest;

/// Tells the server that the client wants to test latency.
#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct StatusPing(pub i64);
