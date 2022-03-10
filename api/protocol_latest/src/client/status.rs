use minecrevy_io_str::{McRead, McWrite};

/// Tells the plugin to send plugin information.
///
/// See [`crate::plugin::ResponseStatusPacket`] for what kind of information is received.
#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct StatusRequest;

impl crate::Packet for StatusRequest {}

/// Tells the plugin that the client wants to test latency.
#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct StatusPing(pub i64);

impl crate::Packet for StatusPing {}
