use std::io;
use std::io::Cursor;

use minecrevy_io_str::{McRead, McWrite};
pub use minecrevy_protocol_derive::Packet;

/// A Minecraft protocol packet sent between the client and server.
pub trait Packet: 'static {
    /// Decodes a packet's content from a slice of bytes.
    fn decode(body: &[u8]) -> io::Result<Self>
    where
        Self: McRead
    {
        McRead::read(Cursor::new(body), <Self as McRead>::Options::default())
    }

    /// Encodes a packet's content as a slice of bytes.
    fn encode(&self) -> io::Result<Vec<u8>>
    where
        Self: McWrite
    {
        let mut body = Vec::new();
        McWrite::write(self, &mut body, <Self as McWrite>::Options::default())?;
        Ok(body)
    }
}
