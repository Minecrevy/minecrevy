//! An untyped, raw Minecraft packet.

use std::{
    fmt::{self, Write as _},
    io::{Cursor, Read, Write},
};

use crate::util::varint_bytes;

/// A single packet in the Minecraft protocol.
///
/// # [Packet format][1]
/// | Field Name | Field Type | Notes                        |
/// |------------|------------|------------------------------|
/// | Length     | `VarInt`   | Length of (Packet ID + Data) |
/// | Packet ID  | `VarInt`   |                              |
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
    #[allow(clippy::len_without_is_empty)]
    #[must_use]
    pub fn len(&self) -> usize {
        let id_len = varint_bytes(self.id);
        let body_len = self.body.len();
        id_len + body_len
    }

    /// Returns an opaque [`Read`] for reading from this packet's body.
    #[must_use]
    pub fn reader(&self) -> impl Read + '_ {
        Cursor::new(&self.body)
    }

    /// Returns an opaque [`Write`] for writing to this packet's body.
    #[must_use]
    pub fn writer(&mut self) -> impl Write + '_ {
        Cursor::new(&mut self.body)
    }
}

impl fmt::Debug for RawPacket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut data = String::new();
        for &byte in &self.body {
            write!(data, "{byte:02X}")?;
        }

        write!(f, "RawPacket({:#X}, {})", self.id, data)
    }
}

#[cfg(feature = "codec")]
pub mod codec {
    //! [`Encoder`] and [`Decoder`] for [`RawPacket`]s.

    use std::{
        io::{self, Cursor},
        sync::Arc,
        time::Duration,
    };

    use bytes::Buf;
    use tokio_util::codec::{Decoder, Encoder};

    use crate::prelude::{RawPacket, ReadMinecraftExt, WriteMinecraftExt};

    /// Settings for a [`RawPacketCodec`].
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    pub struct PacketCodecSettings {
        /// The timeout for reading packets.
        pub timeout: Duration,
        /// How large a packet must be before it is compressed.
        pub compression_threshold: Option<i32>,
        /// The public key used to encrypt packets.
        pub encryption_key: Option<[u8; 16]>,
    }

    impl Default for PacketCodecSettings {
        fn default() -> Self {
            Self {
                timeout: Duration::from_secs(30),
                compression_threshold: None,
                encryption_key: None,
            }
        }
    }

    /// [`Encoder`] and [`Decoder`] for [`RawPacket`]s.
    pub struct RawPacketCodec {
        /// The settings for this codec.
        pub settings: Arc<PacketCodecSettings>,
        /// True if packets should be compressed/decompressed.
        pub compress: bool,
        /// True if packets should be encrypted/decrypted.
        pub encrypt: bool,
    }

    impl RawPacketCodec {
        /// Creates a new [`RawPacketCodec`] with the given settings.
        #[must_use]
        pub fn new(settings: Arc<PacketCodecSettings>) -> Self {
            Self {
                settings,
                compress: false,
                encrypt: false,
            }
        }

        /// Enables compression for this codec.
        pub fn enable_compression(&mut self) {
            self.compress = true;
        }

        /// Enables encryption for this codec.
        pub fn enable_encryption(&mut self) {
            self.encrypt = true;
        }
    }

    impl Encoder<RawPacket> for RawPacketCodec {
        type Error = io::Error;

        fn encode(
            &mut self,
            packet: RawPacket,
            dst: &mut bytes::BytesMut,
        ) -> Result<(), Self::Error> {
            let mut bytes = Vec::new();
            bytes.write_packet(&packet)?;

            if self.compress {
                // TODO
            }

            if self.encrypt {
                // TODO
            }

            dst.extend_from_slice(&bytes);

            Ok(())
        }
    }

    impl Decoder for RawPacketCodec {
        type Item = RawPacket;
        type Error = io::Error;

        fn decode(&mut self, src: &mut bytes::BytesMut) -> Result<Option<Self::Item>, Self::Error> {
            // TODO: compression and encryption
            let mut cursor = Cursor::<&[u8]>::new(src);
            match cursor.read_packet() {
                Ok(packet) => {
                    // reading was successful, advance the outer buffer and return
                    src.advance(cursor.position() as usize);
                    Ok(Some(packet))
                }
                Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => Ok(None),
                Err(e) => Err(e),
            }
        }
    }
}
