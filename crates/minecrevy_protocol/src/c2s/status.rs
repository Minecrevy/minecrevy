use minecrevy_io::{McRead, McWrite, Packet};

/// Sent by the client to request latency information.
#[derive(Packet, McRead, McWrite, Clone, PartialEq, Debug)]
pub struct PingRequest(pub i64);

/// Sent by the client to request server information.
#[derive(Packet, McRead, McWrite, Clone, PartialEq, Debug)]
pub struct StatusRequest;
