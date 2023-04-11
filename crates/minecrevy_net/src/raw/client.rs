use std::{io, net::SocketAddr, sync::Arc};

use bevy::prelude::*;
use flume::{Receiver, Sender};
use futures_util::SinkExt;
use minecrevy_io::{
    packet::{CodecSettings, RawPacket, RawPacketCodec},
    PacketMeta,
};
use tokio::{
    net::TcpStream,
    sync::mpsc::{UnboundedReceiver as TokioReceiver, UnboundedSender as TokioSender},
};
use tokio_stream::StreamExt;
use tokio_util::codec::Framed;

use crate::raw::server::RawServer;

/// A Minecraft client connection.
#[derive(Component)]
pub struct RawClient {
    /// The address of this connection.
    addr: SocketAddr,
    /// The channel for receiving packets from the `io_task`.
    incoming: Receiver<RawPacket>,
    /// The channel for sending packets to the `io_task`.
    ///
    /// Flume's `Sender` doesn't have async cancellation safety, but Tokio's does.
    /// We need it for [`tokio::select!`].
    outgoing: TokioSender<WriteOp>,
    /// The channel for receiving errors from the `io_task`.
    errors: Receiver<io::Error>,
}

/// Public API
impl RawClient {
    pub(crate) fn accept(server: &RawServer, stream: TcpStream, addr: SocketAddr) -> Self {
        let (incoming_tx, incoming_rx) = flume::unbounded();
        let (outgoing_tx, outgoing_rx) = tokio::sync::mpsc::unbounded_channel();
        let (errors_tx, errors_rx) = flume::unbounded();

        let codec = Arc::clone(server.codec());

        server.runtime().spawn(async move {
            Self::client_handler(stream, codec, incoming_tx, outgoing_rx, errors_tx).await
        });

        Self {
            addr,
            incoming: incoming_rx,
            outgoing: outgoing_tx,
            errors: errors_rx,
        }
    }

    /// Returns `true` if this client is an active connection.
    pub fn is_connected(&self) -> bool {
        !self.incoming.is_disconnected()
    }

    /// Returns the [`SocketAddr`] corresponding to this connection.
    pub fn addr(&self) -> SocketAddr {
        self.addr
    }

    /// Sends a [`RawPacket`] to the connected peer.
    pub fn send(&self, packet: RawPacket) -> FlushGuard {
        FlushGuard(&self.outgoing).send(packet)
    }

    /// Sets certain socket metadata for this connection.
    pub fn meta(&self, meta: PacketMeta) {
        self.outgoing
            .send(WriteOp::Meta(meta))
            .unwrap_or_else(|e| panic!("{e}"));
    }

    /// Returns an iterator of all received packets.
    pub fn iter(&self) -> impl Iterator<Item = RawPacket> + '_ {
        self.incoming.try_iter()
    }

    /// Returns an iterator ofa ll received I/O errors.
    pub fn errors(&self) -> impl Iterator<Item = io::Error> + '_ {
        self.errors.try_iter()
    }
}

/// Async Tasks
impl RawClient {
    async fn client_handler(
        stream: TcpStream,
        codec: Arc<CodecSettings>,
        incoming: Sender<RawPacket>,
        mut outgoing: TokioReceiver<WriteOp>,
        errors: Sender<io::Error>,
    ) {
        let mut stream = Framed::new(stream, RawPacketCodec::new(codec));

        loop {
            tokio::select! {
                packet = stream.next() => {
                    match packet {
                        Some(Ok(packet)) => {
                            incoming.try_send(packet).unwrap_or_else(|e| unreachable!("{e}"));
                        }
                        Some(Err(e)) => {
                            errors.try_send(e).unwrap_or_else(|e| unreachable!("{e}"));
                        }
                        None => {
                            let error = io::Error::new(io::ErrorKind::ConnectionAborted, "connection closed");
                            errors.try_send(error).unwrap_or_else(|e| unreachable!("{e}"));
                            break;
                        }
                    }
                }
                op = outgoing.recv() => {
                    match op {
                        Some(WriteOp::Packet(packet)) => {
                            if let Err(e) = stream.send(packet).await {
                                errors.try_send(e).unwrap_or_else(|e| unreachable!("{e}"));
                            }
                        }
                        Some(WriteOp::Meta(PacketMeta::EnableCompression)) => {
                            stream.codec_mut().enable_compression();
                        }
                        Some(WriteOp::Meta(PacketMeta::EnableEncryption)) => {
                            stream.codec_mut().enable_encryption();
                        }
                        Some(WriteOp::Flush) => {
                            if let Err(e) = stream.flush().await {
                                errors.try_send(e).unwrap_or_else(|e| unreachable!("{e}"));
                            }
                        }
                        None => {
                            let error = io::Error::new(io::ErrorKind::ConnectionAborted, "connection closed");
                            errors.try_send(error).ok();
                            break;
                        }
                    }
                }
            }
        }
    }
}

/// A guard type that ensures packets are eagerly flushed to the socket after they have been sent.
pub struct FlushGuard<'a>(&'a TokioSender<WriteOp>);

impl FlushGuard<'_> {
    pub fn send(self, packet: RawPacket) -> Self {
        self.0
            .send(WriteOp::Packet(packet))
            .unwrap_or_else(|e| panic!("{e}"));
        self
    }
}

impl Drop for FlushGuard<'_> {
    fn drop(&mut self) {
        self.0
            .send(WriteOp::Flush)
            .unwrap_or_else(|e| panic!("{e}"));
    }
}

pub enum WriteOp {
    Packet(RawPacket),
    Meta(PacketMeta),
    Flush,
}
