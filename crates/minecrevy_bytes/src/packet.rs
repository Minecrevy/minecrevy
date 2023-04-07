use std::{
    fmt::{self, Write as _},
    io::{Cursor, Read, Write},
};

use crate::var_i32_bytes;

/// A single packet in the Minecraft protocol.
///
/// # [Packet format][1]
/// | Field Name | Field Type | Notes                        |
/// |------------|------------|------------------------------|
/// | Length     | VarInt     | Length of (Packet ID + Data) |
/// | Packet ID  | VarInt     |                              |
/// | Data       | Byte Array | Contents depend on Packet ID |
///
/// [1]: https://wiki.vg/Protocol#Packet_format
#[derive(Clone, PartialEq, Eq)]
pub struct RawPacket {
    /// The ID of the packet.
    pub id: i32,
    /// The packet's contents.
    pub body: Vec<u8>,
}

impl RawPacket {
    /// Returns the length of the packet.
    ///
    /// This corresponds to the Length field in the packet format mentioned [here][RawPacket].
    pub fn len(&self) -> usize {
        let id_len = var_i32_bytes(self.id);
        let body_len = self.body.len();
        id_len + body_len
    }

    /// Returns an opaque [`Read`] for reading from this packet's body.
    pub fn reader(&self) -> impl Read + '_ {
        Cursor::new(&self.body)
    }

    /// Returns an opaque [`Write`] for writing to this packet's body.
    pub fn writer(&mut self) -> impl Write + '_ {
        Cursor::new(&mut self.body)
    }
}

impl fmt::Debug for RawPacket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut data = String::new();
        for &byte in &self.body {
            write!(data, "{:02X}", byte)?;
        }

        write!(f, "RawPacket({:#X}, {})", self.id, data)
    }
}
