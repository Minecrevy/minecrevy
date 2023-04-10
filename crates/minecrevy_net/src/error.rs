use std::{io, net::SocketAddr};

use minecrevy_io::Packet;
use thiserror::Error;

use crate::protocol::version::ProtocolVersion;

#[derive(Error, Debug)]
pub enum ServerError {
    #[error("error during bind(): {0}")]
    Bind(#[source] io::Error),
    #[error("error during accept(): {0}")]
    Accept(#[source] io::Error),
}

#[derive(Error, Debug)]
pub enum ClientError {
    #[error("client disconnected: {0}")]
    Disconnected(SocketAddr),
    #[error("unregistered packet type: {0}")]
    UnregisteredPacket(&'static str, ProtocolVersion),
    #[error(transparent)]
    PacketIo(#[from] io::Error),
}

impl ClientError {
    pub fn unregistered<T: Packet>(version: ProtocolVersion) -> Self {
        Self::UnregisteredPacket(std::any::type_name::<T>(), version)
    }
}
