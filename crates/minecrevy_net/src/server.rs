//! This module contains the [`ServerPlugin`], which handles server-side communication.

use std::{fmt, io, net::SocketAddr, sync::Arc};

use bevy::prelude::*;
use flume::{Receiver, Sender};
use futures_util::{SinkExt, StreamExt};
use minecrevy_io::packet::{
    codec::{PacketCodecSettings, RawPacketCodec},
    RawPacket,
};
use tokio::{
    net::{TcpListener, TcpStream, ToSocketAddrs},
    runtime::Runtime,
    sync::{mpsc::UnboundedReceiver, oneshot},
    task::JoinHandle,
};
use tokio_util::codec::Framed;

use crate::{
    client::{Client, ClientAddressIndex, ProtocolState, WriteOp},
    packet::IncomingPacketHandlers,
};

/// [`SystemSet`]s for the [`NetServerPlugin`].
#[derive(SystemSet, Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum ServerSets {
    /// The set of systems that spawn new [`Client`]s as entities.
    ///
    /// This set is configured to run before [`NetworkSets::ReadPackets`].
    SpawnClients,
    /// The set of systems that read incoming packets from clients and trigger
    /// them as observer events.
    EmitPacketEvents,
    /// The set of systems that despawn [`Client`]s that have errored.
    DespawnClients,
}

/// [`Plugin`] for server-side networking.
pub struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        // Not listening by default.
        app.init_resource::<Server>();
        app.init_resource::<ClientAddressIndex>();

        app.configure_sets(
            PreUpdate,
            (ServerSets::SpawnClients, ServerSets::EmitPacketEvents).chain(),
        );

        // ServerSets::SpawnClients
        app.add_systems(
            PreUpdate,
            Self::spawn_clients.in_set(ServerSets::SpawnClients),
        );

        // ServerSets::EmitPacketEvents
        app.add_systems(
            PreUpdate,
            Self::trigger_incoming_packets.in_set(ServerSets::EmitPacketEvents),
        );

        // ServerSets::DespawnClients
        app.add_systems(
            PostUpdate,
            Self::despawn_errored_clients.in_set(ServerSets::DespawnClients),
        );
    }
}

impl ServerPlugin {
    /// [`System`] that spawns new [`Client`]s as entities.
    fn spawn_clients(mut commands: Commands, server: Res<Server>) {
        for client in server.iter_new_clients() {
            commands.spawn(client);
        }
    }

    /// [`System`] that reads incoming packets from clients and triggers them as observer events.
    fn trigger_incoming_packets(
        server: Res<Server>,
        index: Res<ClientAddressIndex>,
        mut commands: Commands,
    ) {
        for (addr, packet) in server.iter_incoming() {
            let Some(client_entity) = index.entity(addr) else {
                warn!("No client entity for {addr}");
                continue;
            };

            commands.queue(move |world: &mut World| {
                // The protocol state needs to be fetched when the command is executed,
                // as it may have changed after a previous command was executed.
                let Some(&state) = world.get::<ProtocolState>(client_entity) else {
                    // The client may have disconnected.
                    return;
                };
                let Some(func) = world
                    .resource::<IncomingPacketHandlers>()
                    .get(state, packet.id)
                else {
                    warn!("No handler for packet {} in state {state:?}", packet.id);
                    return;
                };

                (func)(world, client_entity, packet);
            });
        }
    }

    /// [`System`] that despawns [`Client`]s that have errored.
    fn despawn_errored_clients(mut commands: Commands, mut clients: Query<(Entity, &mut Client)>) {
        for (entity, mut client) in clients.iter_mut() {
            if let Ok(error) = client.errors.try_recv() {
                error!(
                    "Client {addr} errored: {error}",
                    addr = client.addr(),
                    error = error
                );
                commands.entity(entity).despawn();
            }
        }
    }
}

/// [`Resource`] for the network server.
#[derive(Resource)]
pub struct Server {
    /// The [`Runtime`] used to spawn the server and handle clients.
    runtime: Runtime,
    /// The [`JoinHandle`] for the TCP network listener.
    listener: Option<JoinHandle<()>>,
    /// The [`Receiver`] for new clients.
    new_clients: Receiver<Client>,
    // The [`Sender`] for incoming packets.
    incoming_tx: Sender<(SocketAddr, RawPacket)>,
    // The [`Receiver`] for incoming packets.
    incoming_rx: Receiver<(SocketAddr, RawPacket)>,
    /// The codec settings used for the server.
    pub codec: Arc<PacketCodecSettings>,
}

impl Default for Server {
    fn default() -> Self {
        let (incoming_tx, incoming_rx) = flume::unbounded();
        // Off by default.
        Self {
            runtime: Runtime::new().unwrap(),
            listener: None,
            new_clients: flume::unbounded().1,
            incoming_tx,
            incoming_rx,
            codec: Arc::new(PacketCodecSettings::default()),
        }
    }
}

impl Server {
    /// Starts the server on the given address.
    pub fn start(&mut self, address: impl ToSocketAddrs + fmt::Display + Send + 'static) {
        self.stop();

        info!("Starting network server on {}", address);

        let codec = self.codec.clone();
        let (new_clients_tx, new_clients_rx) = flume::unbounded::<Client>();
        let incoming = self.incoming_tx.clone();

        self.listener =
            Some(self.runtime.spawn(async move {
                Self::listener(address, new_clients_tx, incoming, codec).await
            }));
        self.new_clients = new_clients_rx;
    }

    /// Stops the server.
    pub fn stop(&mut self) {
        if let Some(listener) = self.listener.take() {
            info!("Stopping network server");
            listener.abort();
        }
    }

    /// Returns an iterator over newly connected clients.
    pub fn iter_new_clients(&self) -> impl Iterator<Item = Client> + '_ {
        self.new_clients.try_iter()
    }

    /// Returns the [`Receiver`] for incoming packets.
    pub fn incoming(&self) -> Receiver<(SocketAddr, RawPacket)> {
        self.incoming_rx.clone()
    }

    /// Returns an iterator over incoming packets.
    pub fn iter_incoming(&self) -> impl Iterator<Item = (SocketAddr, RawPacket)> + '_ {
        self.incoming_rx.try_iter()
    }

    /// Processes incoming connections.
    async fn listener(
        addr: impl ToSocketAddrs + fmt::Display,
        new_clients: Sender<Client>,
        incoming: Sender<(SocketAddr, RawPacket)>,
        codec: Arc<PacketCodecSettings>,
    ) {
        info!("Starting network server on {addr}");

        let listener = TcpListener::bind(addr).await.unwrap();

        while let Ok((stream, addr)) = listener.accept().await {
            let incoming = incoming.clone();
            // Tokio's MPSC channels are cancel safe, so we use those instead for tokio::select! {}
            let (outgoing_tx, outgoing_rx) = tokio::sync::mpsc::unbounded_channel::<WriteOp>();
            let (errors_tx, errors_rx) = oneshot::channel::<io::Error>();
            let codec = codec.clone();

            tokio::spawn(async move {
                Self::handle_client(addr, stream, codec, incoming, outgoing_rx, errors_tx).await
            });

            new_clients
                .try_send(Client::new(addr, outgoing_tx, errors_rx))
                .ok();

            trace!("Client {addr} connected");
        }

        info!("Network server stopped");
    }

    /// Handles I/O for the given client.
    async fn handle_client(
        addr: SocketAddr,
        stream: TcpStream,
        codec: Arc<PacketCodecSettings>,
        incoming: Sender<(SocketAddr, RawPacket)>,
        mut outgoing: UnboundedReceiver<WriteOp>,
        errors: oneshot::Sender<io::Error>,
    ) {
        let mut stream = Framed::new(stream, RawPacketCodec::new(Arc::clone(&codec)));

        loop {
            tokio::select! {
                _ = tokio::time::sleep(codec.timeout) => {
                    if let Err(e) = stream.flush().await {
                        // failed to flush remaining packets
                        errors.send(e).ok();
                        break;
                    } else {
                        // timed out
                        errors.send(io::Error::new(io::ErrorKind::TimedOut, "Client timed out")).ok();
                        break;
                    }
                }
                Some(packet) = stream.next() => {
                    match packet {
                        Ok(packet) => {
                            incoming.try_send((addr, packet)).ok();
                        }
                        Err(e) => {
                            errors.send(e).ok();
                            break;
                        }
                    }
                }
                Some(op) = outgoing.recv() => {
                    match op {
                        WriteOp::Send(packet) => {
                            if let Err(e) = stream.feed(packet).await {
                                errors.send(e).ok();
                                break;
                            }
                        }
                        WriteOp::Flush => {
                            if let Err(e) = stream.flush().await {
                                errors.send(e).ok();
                                break;
                            }
                        }
                        WriteOp::EnableCompression => {
                            stream.codec_mut().enable_compression();
                        }
                        WriteOp::EnableEncryption => {
                            stream.codec_mut().enable_encryption();
                        }
                        WriteOp::Disconnect => {
                            errors.send(io::Error::new(io::ErrorKind::ConnectionAborted, "Client disconnected")).ok();
                            break;
                        }
                    }
                }
                else => {
                    // I/O disconnected
                    errors.send(io::Error::new(io::ErrorKind::ConnectionAborted, "Client disconnected")).ok();
                    break;
                }
            }
        }
    }
}
