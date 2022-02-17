use std::fmt::Display;
use std::io;
use std::net::SocketAddr;

use flume::Sender;
use tokio::net::{TcpStream, ToSocketAddrs};
use tokio::runtime::{Handle, Runtime};

use crate::SocketId;
use crate::util::Channel;

/// An event received from the client task.
#[derive(Debug)]
pub enum ClientEvent {
    /// Sent after attempting to connect to a server.
    Connect(io::Result<(TcpStream, SocketAddr)>),
    /// Sent when a server connection has closed.
    Disconnect(SocketId),
}

/// A Minecraft protocol client.
#[derive(Debug)]
pub struct Client {
    runtime: Runtime,
    /// Message channel for receiving and sending events.
    pub events: Channel<ClientEvent>,
}

impl Client {
    /// Constructs a new, unconnected [`Client`].
    pub fn new(runtime: Runtime) -> Self {
        Self {
            runtime,
            events: flume::unbounded::<ClientEvent>().into(),
        }
    }

    /// Returns a [Handle] to the tokio runtime managed by this client.
    pub fn runtime(&self) -> &Handle {
        self.runtime.handle()
    }

    /// Connects to a server at the specified address.
    ///
    /// If the client has an active connection, it will be closed.
    pub fn connect(&self, addr: impl ToSocketAddrs + Display + Send + 'static) {
        let events = self.events.send.clone();

        self.runtime.spawn(async move {
            Self::connector(addr, events).await;
        });
    }

    /// Returns an iterator of all incoming [ClientEvent]s.
    pub fn events(&self) -> impl Iterator<Item=ClientEvent> + '_ {
        self.events.recv.try_iter()
    }
}

impl Client {
    async fn connector(
        address: impl ToSocketAddrs + Display + Send + 'static,
        events: Sender<ClientEvent>,
    ) {
        const EVENT_FAIL: &'static str = "failed to send client event";

        let stream = match TcpStream::connect(address).await {
            Ok(s) => s,
            Err(e) => {
                events.send_async(ClientEvent::Connect(Err(e)))
                    .await.expect(EVENT_FAIL);
                return;
            }
        };

        let result = stream.peer_addr()
            .map(|addr| (stream, addr));

        events.send_async(ClientEvent::Connect(result))
            .await.expect(EVENT_FAIL);
    }
}
