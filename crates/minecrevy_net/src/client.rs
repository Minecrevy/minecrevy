use std::{io, net::SocketAddr, time::Duration};

use bevy_ecs::component::Component;
use crossbeam_channel::{Receiver, Sender};
use futures_util::{SinkExt, StreamExt};
use tokio::{
    io::{AsyncRead, AsyncWrite, join},
    net::TcpStream,
    runtime::Handle,
    sync::mpsc::{UnboundedReceiver, UnboundedSender},
    task::JoinHandle,
    time::sleep,
};
use tokio_util::codec::Framed;

use crate::packet::{RawPacket, RawPacketCodec};

/// A TCP client that connects to a [`Server`] and sends/receives packets.
///
/// [`Server`]: crate::server::Server
#[derive(Component)]
pub struct Client {
    /// The address of the client.
    addr: SocketAddr,
    /// The client's connection timeout.
    timeout: Duration,
    /// The task that handles the client's network I/O.
    task: JoinHandle<()>,
    /// The channel for receiving packets via the client's I/O task.
    incoming: Receiver<IncomingClientEvent>,
    /// The channel for sending packets via the client's I/O task.
    outgoing: UnboundedSender<OutgoingClientEvent>,
}

impl Client {
    /// Connects to a server at the given address.
    pub fn connect(runtime: &Handle, addr: SocketAddr, timeout: Duration) -> Self {
        let (incoming_tx, incoming_rx) = crossbeam_channel::bounded(64);
        let (outgoing_tx, outgoing_rx) = tokio::sync::mpsc::unbounded_channel();
        Self {
            addr,
            timeout,
            task: runtime.spawn(async move {
                let mut stream = match TcpStream::connect(addr).await {
                    Ok(stream) => stream,
                    Err(e) => {
                        incoming_tx.try_send(IncomingClientEvent::Error(e)).ok();
                        return;
                    }
                };
                let (reader, writer) = stream.split();
                Self::io(addr, reader, writer, incoming_tx, outgoing_rx, timeout).await;
            }),
            incoming: incoming_rx,
            outgoing: outgoing_tx,
        }
    }

    pub(crate) fn new(mut stream: TcpStream, addr: SocketAddr, timeout: Duration) -> Self {
        let (incoming_tx, incoming_rx) = crossbeam_channel::bounded(64);
        let (outgoing_tx, outgoing_rx) = tokio::sync::mpsc::unbounded_channel();
        Self {
            addr,
            timeout,
            task: tokio::spawn(async move {
                let (reader, writer) = stream.split();
                Client::io(addr, reader, writer, incoming_tx, outgoing_rx, timeout).await;
            }),
            incoming: incoming_rx,
            outgoing: outgoing_tx,
        }
    }

    /// Returns the address of this client.
    pub fn addr(&self) -> SocketAddr {
        self.addr
    }

    /// Returns the client's connection timeout.
    pub fn timeout(&self) -> Duration {
        self.timeout
    }

    /// Returns whether this client is alive.
    pub fn is_alive(&self) -> bool {
        self.task.is_finished()
    }

    /// Returns incoming events from the server.
    pub fn incoming(&self) -> impl Iterator<Item = IncomingClientEvent> + '_ {
        self.incoming.try_iter()
    }

    /// Sends a packet to the server and flushes the output buffer.
    pub fn send(&self, packet: RawPacket) {
        self.outgoing.send(OutgoingClientEvent::Packet(packet)).ok();
        self.outgoing.send(OutgoingClientEvent::Flush).ok();
    }

    /// Sends multiple packets to the server, then flushes the output buffer.
    pub fn send_all(&self, packets: impl IntoIterator<Item = RawPacket>) {
        for packet in packets {
            self.outgoing.send(OutgoingClientEvent::Packet(packet)).ok();
        }
        self.outgoing.send(OutgoingClientEvent::Flush).ok();
    }
}

/// Private API
impl Client {
    pub(crate) async fn io(
        addr: SocketAddr,
        reader: impl AsyncRead + Unpin,
        writer: impl AsyncWrite + Unpin,
        incoming: Sender<IncomingClientEvent>,
        mut outgoing: UnboundedReceiver<OutgoingClientEvent>,
        timeout: Duration,
    ) {
        let mut frames = Framed::new(
            join(reader, writer),
            RawPacketCodec {
                compression_threshold: None,
            },
        );
        loop {
            tokio::select! {
                event = outgoing.recv() => {
                    match event {
                        Some(OutgoingClientEvent::Packet(packet)) => {
                            frames.send(packet).await.ok();
                        }
                        Some(OutgoingClientEvent::Flush) => {
                            frames.flush().await.ok();
                        }
                        Some(OutgoingClientEvent::SetCompression { threshold }) => {
                            frames.codec_mut().compression_threshold = Some(threshold);
                            todo!();
                        }
                        None => {
                            // The sender was dropped.
                            incoming.try_send(IncomingClientEvent::Error(io::Error::new(
                                io::ErrorKind::ConnectionAborted,
                                format!("client {addr} disconnected (dropped)"),
                            ))).ok();
                            return;
                        }
                    }
                }
                packet = frames.next() => {
                    match packet {
                        Some(Ok(packet)) => {
                            incoming.try_send(IncomingClientEvent::Packet(packet)).ok();
                        }
                        Some(Err(e)) => {
                            incoming.try_send(IncomingClientEvent::Error(e)).ok();
                            return;
                        }
                        None => {
                            // The client disconnected.
                            incoming.try_send(IncomingClientEvent::Error(io::Error::new(
                                io::ErrorKind::ConnectionAborted,
                                format!("client {addr} disconnected (EOF)"),
                            ))).ok();
                            return;
                        }
                    }
                }
                _ = sleep(timeout) => {
                    // The client timed out.
                    incoming.try_send(IncomingClientEvent::Error(io::Error::new(
                        io::ErrorKind::TimedOut,
                        format!("client {addr} timed out"),
                    ))).ok();
                    return;
                }
            }
        }
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        self.task.abort();
    }
}

/// Events that are returned by a [`Client`].
pub enum IncomingClientEvent {
    Packet(RawPacket),
    Error(io::Error),
}

/// Events that are sent to a [`Client`].
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum OutgoingClientEvent {
    Packet(RawPacket),
    Flush,
    SetCompression { threshold: i32 },
}
