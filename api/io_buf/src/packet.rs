use crate::util::var_i32_bytes;

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
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct RawPacket {
    /// The packet id.
    pub id: i32,
    /// The packet content.
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
}
