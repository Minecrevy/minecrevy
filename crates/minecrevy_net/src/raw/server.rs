use std::{net::SocketAddr, sync::Arc};

use bevy::prelude::*;
use flume::Sender;
use minecrevy_core::channel::Channel;
use minecrevy_io::packet::CodecSettings;
use tokio::{
    net::{TcpListener, TcpStream},
    runtime::Runtime,
    task::JoinHandle,
};

use crate::{error::ServerError, raw::client::RawClient};

/// A [`Resource`] that asynchronously accepts Minecraft client connections.
///
/// # Example
///
/// ```
/// # use bevy::prelude::*;
/// let mut server = RawServer::new(CodecSettings {
///     compression_threshold: None,
///     encryption_key: None,
/// });
/// server.listen("127.0.0.1:25565".parse().unwrap());
/// world.insert_resource(server);
///
/// fn accept_clients(server: Res<RawServer>) {
///     while let Some(client) = server.accept() {
///         // do something with the client, like make it an entity...
///     }
/// }
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
    errors: Channel<ServerError>,
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

    pub fn runtime(&self) -> &Runtime {
        &self.runtime
    }

    pub fn codec(&self) -> &Arc<CodecSettings> {
        &self.codec
    }

    /// Returns `true` if the server is currently accepting new clients.
    pub fn is_listening(&self) -> bool {
        self.listener.is_some()
    }

    /// Starts listening for new clients at the provided [`SocketAddr`].
    pub fn listen(&mut self, addr: SocketAddr) {
        self.close();

        info!("RawServer now listening on {addr}.");

        let new_connections = self.new_connections.send.clone();
        let errors = self.errors.send.clone();

        let listener = async move { Self::listener(addr, new_connections, errors).await };
        self.listener = Some(self.runtime.spawn(listener));
    }

    /// Stops listening for new clients and drops any incoming connections.
    pub fn close(&mut self) {
        if let Some(listener) = self.listener.take() {
            listener.abort();
            self.new_connections.recv.drain();
            warn!("RawServer closed.");
        }
    }

    /// Accepts a new client connection, if available.
    pub fn try_accept(&self) -> Option<RawClient> {
        if !self.is_listening() {
            return None;
        }

        let (stream, addr) = self.new_connections.recv.try_recv().ok()?;

        Some(RawClient::accept(self, stream, addr))
    }
}

/// Async Tasks
impl RawServer {
    async fn listener(
        addr: SocketAddr,
        new_connections: Sender<(TcpStream, SocketAddr)>,
        errors: Sender<ServerError>,
    ) {
        let listener = match TcpListener::bind(addr).await {
            Ok(listener) => listener,
            Err(e) => {
                errors
                    .try_send(ServerError::Bind(e))
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
                        .try_send(ServerError::Accept(e))
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
