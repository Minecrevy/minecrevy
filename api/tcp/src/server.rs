use std::{fmt, io};
use std::net::SocketAddr;

use flume::Sender;
use tokio::net::{TcpListener, TcpStream, ToSocketAddrs};
use tokio::runtime::{Handle, Runtime};
use tokio::task::JoinHandle;

use crate::util::Channel;

/// An event received from the server task.
#[derive(Debug)]
pub enum ServerEvent {
    /// Sent after attempting to bind to an address.
    Bind(io::Result<SocketAddr>),
    /// Sent when a new client connection is received.
    Accept(TcpStream, SocketAddr),
    /// Sent when a client connection has closed.
    Disconnect(
        #[cfg(feature = "bevy")]
        bevy::prelude::Entity,
        #[cfg(not(feature = "bevy"))]
        SocketAddr,
    ),
    /// Sent when the server has stopped accepting connections.
    Close,
}

/// A Minecraft protocol server.
#[derive(Debug)]
pub struct Server {
    runtime: Runtime,
    listener: Option<JoinHandle<()>>,
    /// The [`flume`] channel for [`ServerEvent`]s.
    pub events: Channel<ServerEvent>,
}

impl Server {
    /// Constructs a new, unconnected [`Server`].
    pub fn new(runtime: Runtime) -> Self {
        Self {
            runtime,
            listener: None,
            events: flume::unbounded::<ServerEvent>().into(),
        }
    }

    /// Returns a [Handle] to the tokio runtime managed by this client.
    pub fn runtime(&self) -> &Handle {
        self.runtime.handle()
    }

    /// Starts listening for new connections.
    ///
    /// If there are any active connections, they will be closed.
    pub fn listen(&mut self, addr: impl ToSocketAddrs + fmt::Display + Send + 'static) {
        self.stop();

        let events_send = self.events.send.clone();
        self.listener = Some(self.runtime.spawn(async move {
            Self::listener(addr, events_send).await;
        }));
    }

    /// Stops listening for new connections and stops all active connections.
    pub fn stop(&mut self) {
        if let Some(listener) = self.listener.take() {
            listener.abort();

            // Send a disconnect event to ourselves so calling stop() doesn't have to be handled manually
            self.events.send.try_send(ServerEvent::Close)
                .expect("failed to send server event");
        }
    }

    /// Returns an iterator of all incoming [ServerEvent]s.
    pub fn events(&self) -> impl Iterator<Item=ServerEvent> + '_ {
        self.events.recv.try_iter()
    }
}

impl Server {
    async fn listener(
        address: impl ToSocketAddrs + fmt::Display + Send + 'static,
        events: Sender<ServerEvent>,
    ) {
        const EVENT_FAIL: &'static str = "failed to send server event";

        let listener = match TcpListener::bind(address).await {
            Ok(l) => l,
            Err(e) => {
                events.send_async(ServerEvent::Bind(Err(e)))
                    .await.expect(EVENT_FAIL);
                return;
            }
        };

        events.send_async(ServerEvent::Bind(listener.local_addr()))
            .await.expect(EVENT_FAIL);

        while let Ok((stream, addr)) = listener.accept().await {
            events.send_async(ServerEvent::Accept(stream, addr))
                .await.expect(EVENT_FAIL);
        }

        events.send_async(ServerEvent::Close)
            .await.expect(EVENT_FAIL);
    }
}
