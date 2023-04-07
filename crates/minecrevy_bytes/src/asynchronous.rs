use std::io::{self, Cursor, Read};

use async_trait::async_trait;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

use crate::{blocking::ReadMinecraftExt, packet::RawPacket};

#[async_trait]
pub trait AsyncReadMinecraftExt: AsyncRead + Unpin {
    /// Reads an unsigned 8 bit integer from the underlying reader.
    async fn read_u8(&mut self) -> io::Result<u8> {
        let mut buf = [0; 1];
        self.read_exact(&mut buf).await?;
        Ok(buf[0])
    }

    /// Reads a signed 32 bit integer from the underlying reader, using variable-length encoding.
    async fn read_var_i32(&mut self) -> io::Result<i32> {
        let mut result: i32 = 0;
        let mut len: u8 = 0;
        loop {
            let byte: u8 = self.read_u8().await?;
            result |= i32::from(byte & 0x7F) << (7 * len);

            len += 1;
            if len > 5 {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "exceeded maximum VarInt byte length",
                ));
            }

            if byte & 0x80 == 0 {
                break;
            }
        }
        return Ok(result);
    }

    /// Reads a signed 32 bit integer as a usize from the underlying reader, using variable-length encoding.
    async fn read_var_i32_len(&mut self) -> io::Result<usize> {
        let value = self.read_var_i32().await?;
        let value = usize::try_from(value).map_err(|_| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("invalid VarInt value as length: {}", value),
            )
        })?;
        Ok(value)
    }

    /// Reads a single [packet][`RawPacket`] from the underlying reader.
    async fn read_packet(&mut self) -> io::Result<RawPacket> {
        let mut outer = {
            let len = self.read_var_i32_len().await?;
            let mut content = vec![0; len];
            self.read_exact(&mut content).await?;
            Cursor::new(content)
        };
        let id = ReadMinecraftExt::read_var_i32(&mut outer)?;
        let mut body = Vec::new();
        Read::read_to_end(&mut outer, &mut body)?;
        Ok(RawPacket { id, body })
    }
}

impl<T: AsyncRead + Unpin> AsyncReadMinecraftExt for T {}

#[async_trait]
pub trait AsyncWriteMinecraftExt: AsyncWrite + Unpin {
    /// Writes an unsigned 8 bit integer to the underlying writer.
    async fn write_u8(&mut self, v: u8) -> io::Result<()> {
        self.write_all(&[v]).await
    }

    /// Writes a signed 32 bit integer to the underlying writer, using variable-length encoding.
    async fn write_var_i32(&mut self, v: i32) -> io::Result<()> {
        let mut v = v as u32;
        loop {
            if (v & !0x7F) == 0 {
                self.write_u8(v as u8).await?;
                return Ok(());
            }

            self.write_u8(((v & 0x7F) | 0x80) as u8).await?;
            v >>= 7;
        }
    }

    /// Writes a usize as a signed 32 bit integer to the underlying writer, using variable-length encoding.
    async fn write_var_i32_len(&mut self, v: usize) -> io::Result<()> {
        let v = i32::try_from(v).map_err(|_| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("invalid VarInt value as length: {}", v),
            )
        })?;
        self.write_var_i32(v).await
    }

    /// Writes a single [packet][`RawPacket`] to the underlying writer.
    async fn write_packet(&mut self, packet: &RawPacket) -> io::Result<()> {
        self.write_var_i32_len(packet.len()).await?;
        self.write_var_i32(packet.id).await?;
        self.write_all(&packet.body).await?;
        Ok(())
    }
}

impl<T: AsyncWrite + Unpin> AsyncWriteMinecraftExt for T {}
