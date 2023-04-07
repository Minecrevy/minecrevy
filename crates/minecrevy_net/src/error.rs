use std::{io, net::SocketAddr};

use minecrevy_io::Packet;
use thiserror::Error;

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
    UnregisteredPacket(&'static str),
    #[error(transparent)]
    PacketIo(#[from] io::Error),
}

impl ClientError {
    pub fn unregistered<T: Packet>() -> Self {
        Self::UnregisteredPacket(std::any::type_name::<T>())
    }
}
