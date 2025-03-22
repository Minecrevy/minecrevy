use std::{io, net::SocketAddr, time::Duration};

use bevy_ecs::component::Component;
use crossbeam_channel::Receiver;
use tokio::{net::TcpListener, runtime::Handle, task::JoinHandle};

use crate::client::Client;

/// A TCP server that listens for incoming connections and returns [`Client`]s.
#[derive(Component)]
pub struct Server {
    /// The address of the server.
    addr: SocketAddr,
    /// The connection timeout of each client.
    client_timeout: Duration,
    /// The task that handles the server's network I/O.
    task: JoinHandle<()>,
    /// The channel for receiving clients via the server's I/O task.
    incoming: Receiver<IncomingServerEvent>,
}

impl Server {
    /// Listens for incoming connections on the given address.
    pub fn listen(runtime: &Handle, addr: SocketAddr, client_timeout: Duration) -> Self {
        let (incoming_tx, incoming_rx) = crossbeam_channel::bounded(64);
        Self {
            addr,
            client_timeout,
            task: runtime.spawn(async move {
                let listener = match TcpListener::bind(addr).await {
                    Ok(listener) => listener,
                    Err(e) => {
                        incoming_tx.try_send(IncomingServerEvent::Error(e)).ok();
                        return;
                    }
                };

                loop {
                    match listener.accept().await {
                        Ok((stream, addr)) => {
                            let client = Client::new(stream, addr, client_timeout);
                            incoming_tx
                                .try_send(IncomingServerEvent::Connected(client))
                                .ok();
                        }
                        Err(e) => {
                            incoming_tx.try_send(IncomingServerEvent::Error(e)).ok();
                            return;
                        }
                    }
                }
            }),
            incoming: incoming_rx,
        }
    }

    /// Returns the address of the server.
    pub fn addr(&self) -> SocketAddr {
        self.addr
    }

    /// Returns the connection timeout of each client.
    pub fn client_timeout(&self) -> Duration {
        self.client_timeout
    }

    /// Returns whether the server is alive.
    pub fn is_alive(&self) -> bool {
        !self.task.is_finished()
    }

    /// Returns incoming events from the server.
    pub fn incoming(&self) -> impl Iterator<Item = IncomingServerEvent> + '_ {
        self.incoming.try_iter()
    }
}

impl Drop for Server {
    fn drop(&mut self) {
        self.task.abort();
    }
}

/// Events that are returned by a [`Server`].
pub enum IncomingServerEvent {
    /// A client has connected to the server.
    Connected(Client),
    /// An error occurred while listening for incoming connections.
    Error(io::Error),
}
