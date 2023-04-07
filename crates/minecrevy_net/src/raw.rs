use std::{io, net::SocketAddr, sync::Arc};

use bevy::prelude::*;
use flume::{Receiver, Sender};
use futures_util::SinkExt;
use minecrevy_bytes::{
    codec::{CodecSettings, RawPacketCodec},
    packet::RawPacket,
};
use minecrevy_core::channel::Channel;
use minecrevy_io::{
    packet::{CodecSettings, RawPacket, RawPacketCodec},
    PacketMeta,
};
use tokio::{
    net::{TcpListener, TcpStream},
    runtime::Runtime,
    sync::mpsc::{UnboundedReceiver as TokioReceiver, UnboundedSender as TokioSender},
    task::JoinHandle,
};
use tokio_stream::StreamExt;
use tokio_util::codec::Framed;

pub type Error = crate::error::ServerError;

/// A [`Resource`] that asynchronously accepts Minecraft client connections.
///
/// # Example
///
/// ```
/// # use bevy::prelude::*;
/// # let mut world = World::default();
/// # let mut schedule = Schedule::new();
/// let mut server = RawServer::new(CodecSettings {
///     compression_threshold: None,
///     encryption_key: None,
/// });
/// server.listen("127.0.0.1:25565".parse().unwrap());
///
/// world.insert_resource(server);
///
/// fn accept_clients(server: Res<RawServer>) {
///     while let Some(client) = server.accept() {
///         // do something with the client, like make it an entity...
///     }
/// }
/// # schedule.add_systems(accept_clients);
/// # schedule.run(&mut world);
/// ```
#[derive(Resource)]
pub struct RawServer {
    /// The async Tokio runtime.
    runtime: Runtime,
    /// Settings used by every new client.
    codec: Arc<CodecSettings>,
    /// The async task that accepts new clients.
    listener: Option<JoinHandle<()>>,
    /// The channel connected to the async task to pull new clients from.
    new_connections: Channel<(TcpStream, SocketAddr)>,
    /// The channel connected to the async task to pull I/O errors from.
    errors: Channel<Error>,
}

/// Public API
impl RawServer {
    /// Constructs a new raw server.
    pub fn new(codec: CodecSettings) -> Self {
        Self {
            runtime: Runtime::new().expect("failed to create network runtime"),
            codec: Arc::new(codec),
            listener: None,
            new_connections: Channel::from(flume::unbounded()),
            errors: Channel::from(flume::unbounded()),
        }
    }

    /// Returns `true` if the server is currently accepting new clients.
    pub fn is_listening(&self) -> bool {
        self.listener.is_some()
    }

    /// Starts listening for new clients at the provided [`SocketAddr`].
    pub fn listen(&mut self, addr: SocketAddr) {
        self.close();

        let new_connections = self.new_connections.send.clone();
        let errors = self.errors.send.clone();

        let listener = async move { Self::listener(addr, new_connections, errors).await };
        self.listener = Some(self.runtime.spawn(listener));
    }

    /// Stops listening for new clients and drops any incoming connections.
    pub fn close(&mut self) {
        if let Some(listener) = self.listener.take() {
            listener.abort();
        }

        self.new_connections.recv.drain();
    }

    /// Accepts a new client connection, if available.
    pub fn accept(&self) -> Option<RawClient> {
        if !self.is_listening() {
            return None;
        }

        let (stream, addr) = self.new_connections.recv.try_recv().ok()?;

        let (incoming_tx, incoming_rx) = flume::unbounded();
        let (outgoing_tx, outgoing_rx) = tokio::sync::mpsc::unbounded_channel();
        let (errors_tx, errors_rx) = flume::unbounded();

        let codec = Arc::clone(&self.codec);

        self.runtime.spawn(async move {
            RawClient::client_handler(stream, codec, incoming_tx, outgoing_rx, errors_tx).await
        });

        Some(RawClient {
            addr,
            incoming: incoming_rx,
            outgoing: outgoing_tx,
            errors: errors_rx,
        })
    }
}

/// ECS Systems
impl RawServer {
    /// A [`Condition`] that returns `true` if the server is currently accepting new client
    /// connections.
    pub fn is_accepting_connections(server: Res<Self>) -> bool {
        server.is_listening()
    }

    pub fn accept_clients(mut commands: Commands, server: Res<Self>) {
        while let Some(client) = server.accept() {
            commands.spawn(client);
        }
    }
}

/// Async Tasks
impl RawServer {
    async fn listener(
        addr: SocketAddr,
        new_connections: Sender<(TcpStream, SocketAddr)>,
        errors: Sender<Error>,
    ) {
        let listener = match TcpListener::bind(addr).await {
            Ok(listener) => listener,
            Err(e) => {
                errors
                    .try_send(Error::Bind(e))
                    .unwrap_or_else(|e| unreachable!("{e}"));
                return;
            }
        };

        loop {
            match listener.accept().await {
                Ok((stream, addr)) => {
                    new_connections
                        .try_send((stream, addr))
                        .unwrap_or_else(|e| unreachable!("{e}"));
                }
                Err(e) => {
                    errors
                        .try_send(Error::Accept(e))
                        .unwrap_or_else(|e| unreachable!("{e}"));
                    break;
                }
            }
        }
    }
}

impl Default for RawServer {
    fn default() -> Self {
        Self::new(CodecSettings::default())
    }
}

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
                            errors.try_send(error).unwrap_or_else(|e| unreachable!("{e}"));
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
