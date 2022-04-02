use std::io;
use std::net::SocketAddr;

use bevy::prelude::*;
use flume::{Receiver, Sender, TrySendError};
use tokio::io::{AsyncBufRead, AsyncBufReadExt, AsyncWrite, AsyncWriteExt, BufReader, BufWriter};
use tokio::net::TcpStream;
use tokio::task::JoinHandle;

use minecrevy_io_buf::util::hex_string;
use minecrevy_io_buf::{AsyncReadMinecraftExt, AsyncWriteMinecraftExt, RawPacket};
use minecrevy_tcp::{Server, ServerEvent, SocketId};

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum WriteOp {
    Send(RawPacket),
    Flush,
}

#[derive(Component, Debug)]
pub struct RawSocket {
    id: SocketId,
    reader: Receiver<RawPacket>,
    writer: Sender<WriteOp>,
    reader_task: JoinHandle<()>,
    writer_task: JoinHandle<()>,
}

impl RawSocket {
    pub fn new(server: &Server, stream: TcpStream, addr: SocketAddr) -> Self {
        let id = SocketId::new(addr);
        let (reader, writer) = stream.into_split();
        let (reader_send, reader_recv) = flume::unbounded::<RawPacket>();
        let (writer_send, writer_recv) = flume::unbounded::<WriteOp>();
        let events = server.events.send.clone();

        Self {
            id,
            reader: reader_recv,
            writer: writer_send,
            reader_task: server.runtime().spawn(async move {
                Self::read(id, BufReader::new(reader), reader_send, events).await;
            }),
            writer_task: server.runtime().spawn(async move {
                Self::write(id, BufWriter::new(writer), writer_recv).await;
            }),
        }
    }

    #[inline]
    pub fn id(&self) -> SocketId {
        self.id
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = RawPacket> + '_ {
        self.reader.try_iter()
    }

    pub fn send(&self, packet: RawPacket) {
        match self.writer.try_send(WriteOp::Send(packet)) {
            Err(TrySendError::Full(_)) => {
                tracing::warn!("channel is full");
            }
            Err(TrySendError::Disconnected(_)) => {
                tracing::warn!("channel is disconnected");
            }
            Ok(_) => {}
        }
    }

    pub fn flush(&self) {
        match self.writer.try_send(WriteOp::Flush) {
            Err(TrySendError::Full(_)) => {
                tracing::warn!("channel is full");
            }
            Err(TrySendError::Disconnected(_)) => {
                tracing::warn!("channel is disconnected");
            }
            Ok(_) => {}
        }
    }
}

impl RawSocket {
    #[tracing::instrument(skip(reader, packets, events))]
    async fn read(
        client: SocketId,
        mut reader: impl AsyncBufRead + Unpin + Send,
        packets: Sender<RawPacket>,
        events: Sender<ServerEvent>,
    ) {
        match reader.fill_buf().await {
            Ok([0xFE, 0x01, ..]) => {
                tracing::debug!("legacy ping is unsupported");
                return;
            }
            Ok([]) => {
                tracing::warn!("no data received");
                return;
            }
            Ok(_) => { /* normal operation */ }
            Err(e) => {
                tracing::warn!("failed to read from socket: {}", e);
                return;
            }
        }

        loop {
            let packet = match reader.read_packet().await {
                Ok(packet) => packet,
                Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => {
                    // disconnected
                    break;
                }
                Err(e) => {
                    tracing::warn!("failed to read from socket: {}", e);
                    break;
                }
            };
            tracing::trace!(
                "PACKET-INC {:#02X} (len: {}): {}",
                packet.id,
                packet.len(),
                hex_string(&packet.body)
            );

            packets
                .send_async(packet)
                .await
                .expect("failed to receive packet from socket");
        }

        events
            .send_async(ServerEvent::Disconnect(client))
            .await
            .expect("failed to notify socket disconnect");
    }

    #[tracing::instrument(skip(writer, packets))]
    async fn write(
        #[allow(unused_variables)] // its used in the instrumented span
        client: SocketId,
        mut writer: impl AsyncWrite + Unpin + Send,
        packets: Receiver<WriteOp>,
    ) {
        while let Ok(op) = packets.recv_async().await {
            match op {
                WriteOp::Send(packet) => {
                    tracing::trace!(
                        "PACKET-OUT {:#02X} (len: {}): {}",
                        packet.id,
                        packet.len(),
                        hex_string(&packet.body)
                    );

                    if let Err(e) = writer.write_packet(&packet).await {
                        tracing::warn!("failed to write to socket: {}", e);
                    }
                }
                WriteOp::Flush => {
                    if let Err(e) = writer.flush().await {
                        tracing::warn!("failed to flush socket: {}", e);
                    }
                }
            }
        }

        // flush remaining packets
        if let Err(e) = writer.flush().await {
            tracing::warn!("failed to flush socket: {}", e);
        }

        // We don't notify disconnect here as reading/writing will always be closed together.
        // If we did, the server would receive double disconnect events.
    }
}

impl Drop for RawSocket {
    fn drop(&mut self) {
        // If you drop the socket, we stop reading/writing.
        self.reader_task.abort();
        self.writer_task.abort();
    }
}
